use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet};
use std::process::exit;
use std::rc::Rc;
use std::sync::Arc;
use std::{thread, time};
use std::ops::Div;

use ethers::prelude::*;

use crate::ampq_service::AmpqService;
use crate::bindings::data_compressor::DataCompressor;
use crate::bindings::CreditManager as CM;
use crate::config::config::str_to_address;
use crate::config::Config;
use crate::credit_service::credit_manager::CreditManager;
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;
use crate::path_finder::PathFinder;
use crate::price_oracle::oracle::PriceOracle;
use crate::terminator_service::terminator::{TerminatorJob, TerminatorService};
use crate::token_service::service::TokenService;

pub struct CreditService<M: Middleware, S: Signer> {
    credit_managers: Vec<CreditManager<M, S>>,
    token_service: TokenService<SignerMiddleware<M, S>>,
    price_oracle: PriceOracle<M, S>,
    dc: DataCompressor<SignerMiddleware<M, S>>,
    client: Arc<SignerMiddleware<M, S>>,
    last_block_synced: U64,
    provider: Provider<Http>,
    path_finder: PathFinder<SignerMiddleware<M, S>>,
    ampq_service: AmpqService,
    bot_address: Address,
    terminator_service: TerminatorService<M, S>,
    chain_id: u64,
    etherscan: String,
    charts_url: String,
    liquidator_enabled: bool,
}

impl<M: Middleware, S: Signer> CreditService<M, S> {
    pub async fn new(
        config: &Config,
        data_compressor: H160,
        client: Arc<SignerMiddleware<M, S>>,
        token_service: TokenService<SignerMiddleware<M, S>>,
        price_oracle: PriceOracle<M, S>,
        provider: Provider<Http>,
    ) -> CreditService<M, S> {
        let dc = DataCompressor::new(data_compressor, client.clone());
        let credit_managers = Vec::new();
        let path_finder = PathFinder::new(&*config, client.clone());
        let ampq_service = AmpqService::new(config).await;
        let terminator_service = TerminatorService::new(
            &config.terminator_address,
            &config.terminator_flash_address,
            client.clone(),
            config.liquidator_enabled,
        )
        .await;
        CreditService {
            credit_managers,
            token_service,
            price_oracle,
            dc,
            client,
            last_block_synced: 0.into(),
            provider,
            path_finder,
            ampq_service,
            bot_address: *&config.terminator_address,
            terminator_service,
            chain_id: config.chain_id,
            etherscan: config.etherscan.clone(),
            charts_url: config.charts_url.clone(),
            liquidator_enabled: config.liquidator_enabled,
        }
    }

    pub async fn launch(&mut self) {
        let cm_list = self
            .dc
            .get_credit_managers_list(self.dc.address())
            .call()
            .await
            .unwrap();

        // let addr = str_to_address("0x968f9a68a98819e2e6bb910466e191a7b6cf02f0".into());

        for cm in cm_list {
            let credit_manager: CreditManager<M, S> = CreditManager::new(
                self.client.clone(),
                &cm,
                DataCompressor::new(self.dc.address(), self.client.clone()),
                self.chain_id,
                self.ampq_service.clone(),
                self.charts_url.clone(),
            )
            .await;
            // if cm.0 == addr {
            self.credit_managers.push(credit_manager);
            // }
        }

        let tokens = self.get_tokens();
        self.price_oracle.load_price_feeds(&tokens).await;
        self.token_service.add_token(&tokens).await;

        self.ampq_service
            .send(String::from("Liquidation bot started!"))
            .await;

        self.update().await;

        let watcher = self.client.clone();
        let mut on_block = watcher
            .watch_blocks()
            .await
            .map_err(ContractError::MiddlewareError::<SignerMiddleware<M, S>>)
            .unwrap()
            .stream();

        while on_block.next().await.is_some() {
            match self.update().await {
                Err(e) => {
                    println!("{}", &e);
                    self.ampq_service.send(e.to_string()).await;
                }
                _ => {}
            }

            if !self.liquidator_enabled {
                println!("zzzzz...");
                let delay = time::Duration::from_secs(20);
                thread::sleep(delay);
            }
        }
    }

    pub fn get_tokens(&self) -> HashSet<Address> {
        let mut set: HashSet<Address> = HashSet::new();

        for credit_manager in self.credit_managers.iter() {
            for token in credit_manager.allowed_tokens.iter() {
                set.insert(*token);
            }
        }

        set
    }

    // Updates information for new blocks
    pub async fn update(&mut self) -> Result<(), LiquidationError> {
        // Gets the last block
        let to = self
            .client
            .provider()
            .get_block_number()
            .await
            .map_err(|r| NetError(format!("cant get last block {}", r.to_string())))?;

        if self.last_block_synced == to {
            return Ok(());
        }

        println!("Updating info from {} to {}", &self.last_block_synced, &to);

        // Load fresh prices from oracle
        self.price_oracle.update_prices().await?;

        let mut terminator_jobs: Vec<TerminatorJob> = Vec::new();

        // Updates info
        for cm in self.credit_managers.iter_mut() {
            cm.update(
                &self.last_block_synced,
                &to,
                &self.price_oracle,
                &self.path_finder,
                &mut terminator_jobs,
            )
            .await?
        }

        println!("Terminator jobs : {}", &terminator_jobs.len());


        let minimal_executor_eth_requirement = U256::from_dec_str("2000000000000000000").unwrap(); // 2 eth

        for job in &terminator_jobs {
            let balance = self
                .token_service
                .get_balance(&job.underlying_token, &self.bot_address)
                .await;

            let executor_balance: U256 = self.client.get_balance(self.client.address(), None).await.unwrap();
            println!("Executor ETH balance: {:?}", executor_balance);
            if executor_balance < minimal_executor_eth_requirement {
                let msg = format!(
                    "ETH balance of executor is not enough, remaining {}. Please keep executor's ETH balance more than 2 eth",
                    executor_balance
                );
                println!("{}", &msg);
                self.ampq_service.send(msg).await;
            }

            if self.liquidator_enabled && executor_balance >= minimal_executor_eth_requirement {
                let mut msg: String;

                let mut terminator_type = 1;

                if balance < job.repay_amount {
                    msg = format!(
                        "TERMINATOR 1 hasn't enough {} balance. Have {}, needed {}. Please refill {:?}\n\nStarting TERMINATOR_FLASH",
                        self.token_service.symbol(&job.underlying_token),
                        self.token_service.format_bn(&job.underlying_token, &balance),
                        self.token_service.format_bn(&job.underlying_token, &job.repay_amount),
                        self.bot_address
                    );
                    terminator_type = 2;
                } else {
                    msg = "STARTING TERMINATOR 1".into();
                }

                println!("{}", &msg);
                self.ampq_service.send(msg).await;

                let receipt = self
                    .terminator_service
                    .liquidate(job, terminator_type)
                    .await;

                match receipt {
                    Ok(receipt) => {
                        msg = format!(
                            "{} account {:?} was successfully liquidated. TxHash: {}/tx/{:?} . Gas used: {:?}\nBlock number: {}",
                            self.token_service.symbol(&job.underlying_token),
                            &job.borrower,
                            &self.etherscan,
                            &receipt.transaction_hash,
                            &receipt.gas_used.unwrap(),
                            &receipt.block_number.unwrap().as_u64()
                        );
                    }
                    Err(err) => {
                        msg = format!(
                            "WARN: Cant liquidate\nCredit manager: {:?}\naccount {:?}\n{:?}",
                            &job.credit_manager, &job.borrower, err
                        );
                    }
                }

                println!("{}", &msg);
                self.ampq_service.send(msg).await;
            } else {
                let msg = format!(
                    "Liquidation required:\ncredit manager {}: {}/address/{:?}\nborrower: {:?}\namount needed: {}",
                    self.token_service.symbol(&job.underlying_token),
                    &self.etherscan,
                    &job.credit_manager,
                    &job.borrower,
                    self.token_service.format_bn(&job.underlying_token, &job.repay_amount),
                    // self.bot_address
                );

                self.ampq_service.send(msg).await;
            }
        }

        // Updates the last block synced
        self.last_block_synced = to;
        Ok(())
    }
}
