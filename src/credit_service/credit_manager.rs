use crate::ampq_service::AmpqService;
use ethers::prelude::*;
use futures::{StreamExt};
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use gearbox::data_compressor::DataCompressor;
use gearbox::shared_types::CreditManagerData;
use gearbox::credit_manager::CreditManager as CreditManagerContract;

use crate::config::config::str_to_address;
use crate::credit_service::credit_account::CreditAccount;
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
    pub liquidation_thresholds: HashMap<Address, U256>,

    added_to_job: HashMap<Address, u8>,
    address: ethers::core::types::Address,
    underlying_token: ethers::core::types::Address,
    is_weth: bool,
    contract: CM<SignerMiddleware<M, S>>,
    data_compressor: DataCompressor<SignerMiddleware<M, S>>,
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
        data_compressor: DataCompressor<SignerMiddleware<M, S>>,
        chain_id: u64,
        ampq_service: AmpqService,
        charts_url: String,
    ) -> Self {
        let contract = CM::new(payload.0, client.clone());
        let pool_service_address = contract.pool_service().call().await.unwrap();
        let pool_service = PoolService::new(pool_service_address, client.clone());

        let credit_configurator_address = contract.credit_configurator().call().await.unwrap();
        let credit_configurator = CreditConfigurator::new(credit_configurator_address, client.clone());

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
            address: payload.0,
            underlying_token: payload.2,
            is_weth: payload.3,
            // can_borrow: payload.4,
            // borrow_rate: payload.5,
            // min_amount: payload.6,
            // max_amount: payload.7,
            // max_leverage_factor: payload.8,
            // available_liquidity: payload.9,
            allowed_tokens: payload.10.clone(),
            liquidation_thresholds: HashMap::new(),
            // adapters: payload.11.clone(),
            data_compressor,
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

    

    pub fn print_accounts(&self) {
        for acc in self.credit_accounts.values() {
            dbg!(acc);
        }
    }
}
