use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

use ethers::prelude::*;

use crate::bindings::AggregatorV3Interface;
use crate::bindings::PriceOracle as Oracle;
use crate::bindings::{DataCompressor, OpenCreditAccountFilter};
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;
use crate::token_service::service::TokenService;

pub struct PriceOracle<M: Middleware, S: Signer> {
    price_feeds: HashMap<Address, AggregatorV3Interface<SignerMiddleware<M, S>>>,
    prices: HashMap<Address, U256>,
    contract: Oracle<SignerMiddleware<M, S>>,
    client: std::sync::Arc<SignerMiddleware<M, S>>,
    decimal_multipliers: HashMap<Address, U256>,
    decimal_dividers: HashMap<Address, U256>,
    weth_token: Address,
}

impl<M: Middleware, S: Signer> PriceOracle<M, S> {
    pub fn new(client: std::sync::Arc<SignerMiddleware<M, S>>, address: Address) -> Self {
        let contract = Oracle::new(address, client.clone());

        PriceOracle {
            price_feeds: HashMap::new(),
            prices: HashMap::new(),
            contract,
            client: client.clone(),
            decimal_multipliers: HashMap::new(),
            decimal_dividers: HashMap::new(),
            weth_token: Address::default(),
        }
    }

    pub async fn load_price_feeds(&mut self, tokens: &HashSet<Address>) {
        println!("load price feeds");

        let weth_token = self.contract.weth_address().call().await.unwrap();

        for token in tokens {
            if *token != weth_token {
                let price_feed_addr = self.contract.price_feeds(*token).call().await.unwrap();
                self.price_feeds.insert(
                    *token,
                    AggregatorV3Interface::new(price_feed_addr, self.client.clone()),
                );
                self.prices.insert(*token, 0.into());
                let decimal_multiplier = self
                    .contract
                    .decimals_multipliers(*token)
                    .call()
                    .await
                    .unwrap();
                let decimal_divider = self
                    .contract
                    .decimals_dividers(*token)
                    .call()
                    .await
                    .unwrap();
                self.decimal_multipliers.insert(*token, decimal_multiplier);
                self.decimal_dividers.insert(*token, decimal_divider);
            }
        }

        self.decimal_multipliers.insert(weth_token, U256::from(1));
        self.decimal_dividers
            .insert(weth_token, U256::from(10).pow(U256::from(18)));
        self.weth_token = weth_token;
    }

    pub async fn update_prices(&mut self) -> Result<(), LiquidationError> {
        println!("update_prices");
        for (token, feed) in self.price_feeds.iter() {
            *self.prices.get_mut(token).unwrap() = U256::try_from(
                feed.latest_round_data()
                    .call()
                    .await
                    .map_err(|err| NetError("cant get price oracle".to_string()))?
                    .1,
            )
            .unwrap();
        }

        dbg!(&self.prices);
        Ok(())
    }

    pub fn convert(
        &self,
        amount: U256,
        from: Address,
        to: Address,
    ) -> Result<U256, LiquidationError> {
        if !self.decimal_multipliers.contains_key(&from) {
            println!("FROM PRONLE: {:?}", &from);
            return Err(NetError(
                format!("Can find a key for {:?}", from).to_string(),
            ));
        }

        if !self.decimal_dividers.contains_key(&to) {
            println!("TO PRONLE: {:?}", &to);
            return Err(NetError(format!("Can find a key for {:?}", to).to_string()));
        }

        Ok(
            amount * self.decimal_multipliers[&from] * self.get_last_price(from, to)
                / self.decimal_dividers[&to],
        )
    }

    /// @dev Gets token rate with 18 decimals. Reverts if priceFeed doesn't exist
    /// @param tokenFrom Converts from token address
    /// @param tokenTo Converts to token address
    /// @return Rate in WAD format
    fn get_last_price(&self, from: Address, to: Address) -> U256 {
        let wad = U256::from(10).pow(U256::from(18));
        if from == to {
            return wad;
        }

        // price = wad * price[ETH] / price[token_to] = wad^2 / price[token_to]
        if from == self.weth_token {
            return wad * wad / self.prices[&to];
        }

        // price = wad * price[token_from] / price[ETH] = wad * price[token_from] / wad = price[token_from]
        if to == self.weth_token {
            return self.prices[&from];
        }

        wad * self.prices[&from] / self.prices[&to]
    }
}
