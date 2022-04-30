use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use async_recursion::async_recursion;
use ethers::prelude::*;
use gearbox::credit_facade::CreditFacade as CreditFacadeContract;
use gearbox::data_compressor::DataCompressor as DataCompressorContract;
use gearbox::credit_facade::CreditFacadeEvents;

use crate::ampq_service::AmpqService;
use crate::config::config::str_to_address;
use crate::credit_service::credit_account::CreditAccount;
use crate::credit_service::credit_manager::CreditManager;
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;
use crate::path_finder::PathFinder;
use crate::terminator_service::terminator::TerminatorJob;

pub struct CreditFacade<M: Middleware, S: Signer> {
    added_to_job: HashMap<Address, u8>,

    address: ethers::core::types::Address,
    underlying_token: ethers::core::types::Address,
    contract: CreditFacadeContract<SignerMiddleware<M, S>>,
    data_compressor: DataCompressorContract<SignerMiddleware<M, S>>,
    credit_manager: CreditManager<M, S>, 
    yearn_tokens: HashMap<Address, Address>,
    ampq_service: AmpqService,
    charts_url: String,
}

impl<M: Middleware, S: Signer> CreditFacade<M, S> {
    async fn liquidate(
        &mut self,
        address: &Address,
        path_finder: &PathFinder<SignerMiddleware<M, S>>,
    ) -> Result<TerminatorJob, LiquidationError> {
        let account = &self.credit_manager.credit_accounts[&address];
        println!("Preparing to liquidate {:?}", &address);

        let mut paths: Vec<(
            ethers::core::types::U256,
            Vec<ethers::core::types::Address>,
            ethers::core::types::U256,
        )> = Vec::new();

        let mut balances = account.balances.clone();

        for y_token in self.yearn_tokens.iter() {
            *balances.get_mut(&y_token.1).unwrap() = balances[&y_token.0] + balances[&y_token.1];
            *balances.get_mut(&y_token.0).unwrap() = U256::zero();
        }

        for asset in self.credit_manager.allowed_tokens.iter() {
            let trade_path = path_finder
                .get_best_rate(*asset, self.underlying_token, balances[&asset])
                .await?;
            paths.push((balances[&asset], trade_path.path, trade_path.amount_out_min));
        }

        dbg!(&account);
        dbg!(&paths);

        let repay_amount = self
            .contract
            .calc_repay_amount(*address, true)
            .call()
            .await
            .map_err(|e| NetError("cant get repay amount".to_string()))?;

        Ok(TerminatorJob {
            credit_manager: self.address,
            borrower: *address,
            router: path_finder.router,
            paths,
            underlying_token: self.underlying_token,
            repay_amount,
        })
    }
    
    #[async_recursion]
    async fn load_events(
        &mut self,
        from_block: &U64,
        to_block: &U64,
    ) -> Vec<(CreditFacadeEvents, LogMeta)> {
        let events = self
            .contract
            .events()
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await;

        match events {
            Ok(result) => result,
            Err(err) => {
                println!("Query err: {:?}", err);

                let mid_block = (from_block + to_block) / 2u64;
                if mid_block == *from_block || mid_block == *to_block {
                    panic!("range is already narrow");
                }

                let mut left_part = self.load_events(from_block, &mid_block).await;
                let mut right_part = self.load_events(&(mid_block + 1u64), to_block).await;
                left_part.append(&mut right_part);
                left_part
            }
        }
    }

    async fn update_accounts(&mut self, from_block: &U64, to_block: &U64) {
        let mut updated: HashSet<Address> = HashSet::new();

        let events = self.load_events(from_block, to_block).await;

        // let mut counter: u64 = 0;
        // let mut oper_by_user: HashMap<Address, u64> = HashMap::new();

        let selected = str_to_address("0xEB2902acd8021Fb93b92a9CFaa5F3cf3758b4318".to_string());

        println!("Credit account: {}", self.address);

        for event in events {
            match &event.0 {
                CreditFacadeEvents::OpenCreditAccountFilter(data) => {
                    // println!("OPEN, {:?}  {:?} ", &event.0, data);
                    if data.on_behalf_of == selected {
                        println!("[{}]: OPEN: {:?}", &event.1.block_number, data);
                    }
                    updated.insert(data.on_behalf_of);
                }
                CreditFacadeEvents::CloseCreditAccountFilter(data) => {
                    // println!("Close credit account, {:?} ", &event.0);
                    if data.owner == selected {
                        println!("[{}]: CLOSE: {:?} ", &event.1.block_number, data);
                    }

                    self.added_to_job.remove(&data.owner);
                    self.credit_manager.credit_accounts.remove(&data.owner);
                    updated.remove(&data.owner);
                }
                // CreditFacadeEvents::RepayCreditAccountFilter(data) => {
                //     if data.owner == selected {
                //         println!("[{}]: REPAY: {:?} ", &event.1.block_number, data);
                //     }
                //     self.added_to_job.remove(&data.owner);
                //     self.credit_manager.credit_accounts.remove(&data.owner);
                //     updated.remove(&data.owner);
                // }
                CreditFacadeEvents::LiquidateCreditAccountFilter(data) => {
                    if data.owner == selected {
                        println!("[{}]: LIQUIDATE: {:?} ", &event.1.block_number, data);
                    }
                    self.added_to_job.remove(&data.owner);
                    self.credit_manager.credit_accounts.remove(&data.owner);
                    updated.remove(&data.owner);
                }
                CreditFacadeEvents::IncreaseBorrowedAmountFilter(data) => {
                    // println!("Incresae borrowing, {:?} ", &event.0);
                    if data.borrower == selected {
                        println!(
                            "[{}]: INCREASE BORROWING: {:?}",
                            &event.1.block_number, data
                        );
                    }
                    updated.insert(data.borrower);
                }
                CreditFacadeEvents::DecreaseBorrowedAmountFilter(data) => {
                    // println!("Incresae borrowing, {:?} ", &event.0);
                    if data.borrower == selected {
                        println!(
                            "[{}]: INCREASE BORROWING: {:?}",
                            &event.1.block_number, data
                        );
                    }
                    updated.insert(data.borrower);
                }
                CreditFacadeEvents::AddCollateralFilter(data) => {
                    if data.on_behalf_of == selected {
                        println!("[{}]: ADD COLLATERAL:  {:?} ", &event.1.block_number, data);
                    }
                    self.added_to_job.remove(&data.on_behalf_of);
                    updated.insert(data.on_behalf_of);
                }
                CreditFacadeEvents::TransferAccountFilter(data) => {
                    // println!("Transfer, {:?} ", &event.0);

                    if data.new_owner == selected {
                        println!("[{}]: TRANSFER, {:?}", &event.1.block_number, data);
                    }
                    self.credit_manager.credit_accounts.remove(&data.old_owner);
                    self.added_to_job.remove(&data.old_owner);
                    updated.remove(&data.old_owner);
                    updated.insert(data.new_owner);
                }
                CreditFacadeEvents::MultiCallStartedFilter(data) => {
                    println!("MultiCall Started , {:?} ", &event.0)
                }
                CreditFacadeEvents::MultiCallFinishedFilter(data) => {
                    println!("MultiCall Finished , {:?} ", &event.0)
                }
                CreditFacadeEvents::TransferAccountAllowedFilter(data) => {
                    if data.from == selected || data.to == selected {
                        println!("[{}]: transfersAllowed , {:?}", &event.1.block_number, data);
                    } 
                }

                _ => {}
            }
        }
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

            if self.credit_manager.credit_accounts.contains_key(&borrower) {
                // dbg!(data.unwrap().0);
                *self.credit_manager.credit_accounts.get_mut(&borrower).unwrap() = ca;
            } else {
                self.credit_manager.credit_accounts.insert(*borrower, ca);
            }
        }

        println!("\nTotal accs: {}", &self.credit_manager.credit_accounts.len());

        println!("Credit acc data is updated");
    }
}
