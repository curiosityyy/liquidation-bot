use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use async_recursion::async_recursion;
use ethers::prelude::*;
use gearbox::credit_facade::CreditFacade as CreditFacadeContract;
use gearbox::credit_facade::CreditFacadeEvents;

use crate::config::config::str_to_address;
use crate::credit_service::credit_account::CreditAccount;

pub struct CreditFacade<M: Middleware, S: Signer> {
    address: ethers::core::types::Address,
    pub contract: CreditFacadeContract<SignerMiddleware<M, S>>,
}

impl<M: Middleware, S: Signer> CreditFacade<M, S> {
    pub fn new(address: Address, client: std::sync::Arc<SignerMiddleware<M, S>>) -> Self {
        let contract = CreditFacadeContract::new(address, client.clone());
        CreditFacade {
            address,
            contract,
        }
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

    pub async fn update_accounts(&mut self, from_block: &U64, to_block: &U64, added_to_job: &mut HashMap<Address, u8>, credit_accounts: &mut HashMap<Address, CreditAccount>) -> HashSet<Address> {
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

                    added_to_job.remove(&data.owner);
                    credit_accounts.remove(&data.owner);
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
                    added_to_job.remove(&data.owner);
                    credit_accounts.remove(&data.owner);
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
                    added_to_job.remove(&data.on_behalf_of);
                    updated.insert(data.on_behalf_of);
                }
                CreditFacadeEvents::TransferAccountFilter(data) => {
                    // println!("Transfer, {:?} ", &event.0);

                    if data.new_owner == selected {
                        println!("[{}]: TRANSFER, {:?}", &event.1.block_number, data);
                    }
                    credit_accounts.remove(&data.old_owner);
                    added_to_job.remove(&data.old_owner);
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
        updated
    }
}
