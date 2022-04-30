use std::collections::HashMap;

use ethers::abi::Address;
use ethers::core::abi::ethereum_types::U64;
use ethers::prelude::{Middleware, U256};
use gearbox::credit_configurator::CreditConfigurator as CreditConfiguratorContract;

pub struct CreditConfigurator<M: Middleware> {
    contract: CreditConfiguratorContract<M>,
}

impl<M: Middleware> CreditConfigurator<M> {
    pub fn new(address: Address, client: std::sync::Arc<M>) -> Self {
        let contract = CreditConfiguratorContract::new(address, client.clone());
        CreditConfigurator {
            contract,
        }
    }

    pub async fn update(&mut self, from_block: &U64, to_block: &U64, liquidation_thresholds: &mut HashMap<Address, U256>) {

        // get token allowed list
        let events = self
            .contract
            .token_allowed_filter()
            .from_block(from_block)
            .to_block(to_block)
            .query()
            .await
            .unwrap();

        for event in events {
            if !liquidation_thresholds.contains_key(&event.token) {
                liquidation_thresholds
                    .insert(event.token, U256::zero());
            }
        }
        
        let events = self
            .contract
            .token_liquidation_threshold_updated_filter()
            .from_block(from_block)
            .to_block(to_block)
            .query()
            .await
            .unwrap();

        for event in events {
            if liquidation_thresholds.contains_key(&event.token) {
                *liquidation_thresholds.get_mut(&event.token).unwrap() =
                    event.liquidity_threshold;
            } else {
                assert!(false, "token should be in allowed list");
            }
        }

        println!(
            "Got liquidataion thresolds {:?}",
            &liquidation_thresholds
        );
    }
}
