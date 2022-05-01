use crate::ampq_service::AmpqService;
use ethers::prelude::*;
use futures::{StreamExt};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use gearbox::data_compressor::DataCompressor as DataCompressorContract;
use gearbox::shared_types::CreditManagerData;
use gearbox::credit_manager::CreditManager as CreditManagerContract;
use terminator::shared_types::MultiCall;
use terminator::terminator::UniV2Params;

use crate::config::config::str_to_address;
use crate::credit_service::credit_account::CreditAccount;
use crate::credit_service::credit_facade::CreditFacade;
use crate::credit_service::credit_configurator::CreditConfigurator;
use crate::credit_service::pool::PoolService;
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;
use crate::path_finder::PathFinder;
use crate::price_oracle::oracle::PriceOracle;
use crate::terminator_service::terminator::TerminatorJob;

pub struct CreditManager<M: Middleware, S: Signer> {
    pub credit_accounts: HashMap<Address, CreditAccount>,
    pub allowed_tokens: Vec<ethers::core::types::Address>,

    liquidation_thresholds: HashMap<Address, U256>,
    added_to_job: HashMap<Address, u8>,
    address: ethers::core::types::Address,
    underlying_token: ethers::core::types::Address,
    is_weth: bool,
    contract: CreditManagerContract<SignerMiddleware<M, S>>,
    data_compressor: DataCompressorContract<SignerMiddleware<M, S>>,

    credit_facade: CreditFacade<M, S>,
    pool_service: PoolService<SignerMiddleware<M, S>>,
    credit_configurator: CreditConfigurator<SignerMiddleware<M, S>>,
    yearn_tokens: HashMap<Address, Address>,
    ampq_service: AmpqService,
    charts_url: String,
}

impl<M: Middleware, S: Signer> CreditManager<M, S> {
    pub async fn new(
        client: std::sync::Arc<SignerMiddleware<M, S>>,
        credit_manager_data: &CreditManagerData,
        data_compressor: DataCompressorContract<SignerMiddleware<M, S>>,
        chain_id: u64,
        ampq_service: AmpqService,
        charts_url: String,
    ) -> Self {
        let contract = CreditManagerContract::new(credit_manager_data.addr, client.clone());
        let pool_service_address = contract.pool_service().call().await.unwrap();
        let pool_service = PoolService::new(pool_service_address, client.clone());

        let credit_configurator_address = contract.credit_configurator().call().await.unwrap();
        let credit_configurator = CreditConfigurator::new(credit_configurator_address, client.clone());

        let credit_facade_address = contract.credit_facade().call().await.unwrap();
        let credit_facade = CreditFacade::new(credit_facade_address, client.clone());

        let mut yearn_tokens: HashMap<Address, Address> = HashMap::new();

        if chain_id == 42 {
            // KOVAN YEARN TOKENS
            // DAI
            yearn_tokens.insert(
                str_to_address(String::from("0x67A022C14E1e6517F45E92BF7C76249c0967569d")),
                str_to_address(String::from("0x9DC7B33C3B63fc00ed5472fBD7813eDDa6a64752")),
            );

            yearn_tokens.insert(
                str_to_address(String::from("0xe5267045739E4d6FcA15BB4a79190012F146893b")),
                str_to_address(String::from("0x9DC7B33C3B63fc00ed5472fBD7813eDDa6a64752")),
            );

            // USDC
            yearn_tokens.insert(
                str_to_address(String::from("0x3B55a47d6ffE0b7bb1762109faFa5B84180c1111")),
                str_to_address(String::from("0x31EeB2d0F9B6fD8642914aB10F4dD473677D80df")),
            );

            yearn_tokens.insert(
                str_to_address(String::from("0x980E4d8A22105c2a2fA2252B7685F32fc7564512")),
                str_to_address(String::from("0x31EeB2d0F9B6fD8642914aB10F4dD473677D80df")),
            );
        } else {
            // MAINNET YEARN TOKENS
            // DAI
            yearn_tokens.insert(
                str_to_address(String::from("0xdA816459F1AB5631232FE5e97a05BBBb94970c95")),
                str_to_address(String::from("0x6B175474E89094C44Da98b954EedeAC495271d0F")),
            );

            // USDC
            yearn_tokens.insert(
                str_to_address(String::from("0xa354f35829ae975e850e23e9615b11da1b3dc4de")),
                str_to_address(String::from("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")),
            );
        }

        CreditManager {
            credit_accounts: HashMap::new(),
            added_to_job: HashMap::new(),
            contract,
            address: credit_manager_data.addr,
            underlying_token: credit_manager_data.underlying,
            is_weth: credit_manager_data.is_weth,
            // can_borrow: payload.4,
            // borrow_rate: payload.5,
            // min_amount: payload.6,
            // max_amount: payload.7,
            // max_leverage_factor: payload.8,
            // available_liquidity: payload.9,
            allowed_tokens: credit_manager_data.allowed_tokens.clone(),
            liquidation_thresholds: HashMap::new(),
            // adapters: payload.11.clone(),
            data_compressor,
            credit_facade,
            pool_service,
            credit_configurator,
            yearn_tokens,
            ampq_service,
            charts_url
        }
    }

    pub async fn update(
        &mut self,
        from_block: &U64,
        to_block: &U64,
        price_oracle: &PriceOracle<M, S>,
        path_finder: &PathFinder<SignerMiddleware<M, S>>,
        jobs: &mut Vec<TerminatorJob>,
    ) -> Result<(), LiquidationError> {
        self.credit_configurator.update(from_block, to_block, &mut self.liquidation_thresholds).await;
        self.update_accounts(from_block, to_block).await;
        let new_ci = self.pool_service.get_new_ci().await;

        println!("Credit manager: {:?}", &self.address);

        let mut accs_to_liquidate: HashSet<Address> = HashSet::new();
        for ca in self.credit_accounts.iter_mut() {
            let hf = ca.1.compute_hf(
                self.underlying_token,
                &new_ci,
                price_oracle,
                &self.liquidation_thresholds,
            )?;

            println!("{:?} : {:?}", &ca.1.borrower, &hf);

            if hf < 10000 {
                if self.added_to_job.contains_key(&ca.1.borrower) {
                    let bad_debt_blocks = self.added_to_job[&ca.1.borrower] + 1;
                    *self.added_to_job.get_mut(&ca.1.borrower).unwrap() = bad_debt_blocks;

                    if bad_debt_blocks >= 5 && bad_debt_blocks % 50 == 5{
                        self.ampq_service
                            .send(format!(
                                "BAD DEBT!: Credit manager: {:}\nborrower: {:?}\nCharts:{}/{:?}/{:?}",
                                &self.address, &ca.1.borrower,
                                &self.charts_url,
                                &self.address, &ca.1.borrower
                            ))
                            .await;
                    }
                } else {
                    self.added_to_job.insert(*&ca.1.borrower, 0u8);
                    accs_to_liquidate.insert(ca.1.borrower);
                }
            }
        }

        dbg!(&accs_to_liquidate);

        println!("Starting liquidation process:");

        for acc in accs_to_liquidate {
            jobs.push(self.liquidate(&acc, path_finder).await?);
        }
        Ok(())
    }
    
    async fn liquidate(
        &mut self,
        address: &Address,
        path_finder: &PathFinder<SignerMiddleware<M, S>>,
    ) -> Result<TerminatorJob, LiquidationError> {
        let account = &self.credit_accounts[&address];
        println!("Preparing to liquidate {:?}", &address);

        let mut paths: Vec<UniV2Params> = Vec::new();

        let mut balances = account.balances.clone();

        for y_token in self.yearn_tokens.iter() {
            *balances.get_mut(&y_token.1).unwrap() = balances[&y_token.0] + balances[&y_token.1];
            *balances.get_mut(&y_token.0).unwrap() = U256::zero();
        }

        for asset in self.allowed_tokens.iter() {
            let trade_path = path_finder
                .get_best_rate(*asset, self.underlying_token, balances[&asset])
                .await?;
            paths.push(UniV2Params{
                amount_in: balances[&asset], 
                path: trade_path.path, 
                amount_out_min: trade_path.amount_out_min,
            });
        }

        dbg!(&account);
        dbg!(&paths);

        let (borrowed_amount, borrowed_amount_with_interest) = self.contract.calc_credit_account_accrued_interest(*address).call().await.unwrap();
        let (total_value, _) = self.credit_facade.contract.calc_total_value(*address).call().await.unwrap();
        let (amount_to_pool, remaining_funds, _, _) = self.contract.calc_close_payments(total_value, true, borrowed_amount, borrowed_amount_with_interest).call().await.unwrap();
        

        Ok(TerminatorJob {
            credit_facade: self.address,
            borrower: *address,
            skip_token_mask: U256::zero(),
            convert_weth: false,
            calls: Vec::<MultiCall>::new(),
            router: path_finder.router,
            paths,
            underlying_token: self.underlying_token,
            repay_amount: amount_to_pool + remaining_funds,
        })
    }

    async fn update_accounts(&mut self, from_block: &U64, to_block: &U64) {

        let updated: HashSet<Address> = self.credit_facade.update_accounts(from_block, to_block, &mut self.added_to_job, &mut self.credit_accounts).await;
        // println!("Got operations: {}", &counter);
        // println!("Got operations: {:?}", &oper_by_user.keys().len());
        println!("\n\nUnderlying token: {:?}", &self.underlying_token);
        println!("\n\nCredit manager address: {:?}", &self.address);
        println!("Credit acc data is loaded");

        let function = &self
            .data_compressor
            .abi()
            .functions
            .get("getCreditAccountDataExtended")
            .unwrap()[0];

        dbg!(&updated);

        // let tx =self.data_compressor
        //     .get_credit_account_data_extended(self.address, *updated.iter().next().unwrap())
        //     .tx.clone();
        //
        // let jobs = stream::iter(updated.clone().iter()).map(|b| {
        //     async move {
        //
        //
        //
        //         self
        //             .data_compressor
        //             .client()
        //             .call(&tx, BlockId::from(to_block.as_u64()).into())
        //             .await
        //             .unwrap()
        //     }
        // }).buffer_unordered(3);
        //
        // jobs.for_each(|f| async {
        //     dbg!(f);
        // }).await;

        for borrower in updated.clone().iter() {
            print!(". {}", borrower);
            // let payload =
            //     self
            //     .data_compressor
            //     .get_credit_account_data_extended(self.address, *borrower)
            //     .call()
            //     .await
            //     .unwrap();

            let tx = self
                .data_compressor
                .get_credit_account_data(self.address, *borrower)
                .tx;

            let response = self
                .data_compressor
                .client()
                .call(&tx, BlockId::from(to_block.as_u64()).into())
                .await
                .unwrap();

            let payload: (
                ethers::core::types::Address,
                ethers::core::types::Address,
                bool,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                Vec<(ethers::core::types::Address, ethers::core::types::U256, bool)>,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ) = decode_function_data(function, response, false).unwrap();

            let health_factor = payload.7.as_u64();

            let ca = CreditAccount {
                contract: payload.0,
                borrower: *borrower,
                borrowed_amount: payload.13,
                cumulative_index_at_open: payload.14,
                balances: HashMap::from_iter(payload.9.into_iter().map(|elm| (elm.0, elm.1))),
                health_factor,
            };

            // if health_factor > 100_000 {
            //     dbg!(&ca);
            // }

            if self.credit_accounts.contains_key(&borrower) {
                // dbg!(data.unwrap().0);
                *self.credit_accounts.get_mut(&borrower).unwrap() = ca;
            } else {
                self.credit_accounts.insert(*borrower, ca);
            }
        }

        println!("\nTotal accs: {}", &self.credit_accounts.len());

        println!("Credit acc data is updated");
    }
    

    pub fn print_accounts(&self) {
        for acc in self.credit_accounts.values() {
            dbg!(acc);
        }
    }
}
