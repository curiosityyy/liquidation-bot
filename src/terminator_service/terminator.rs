use ethers::abi::Address;
use ethers::prelude::{Middleware, Signer, SignerMiddleware, TransactionReceipt, U256};
use terminator::shared_types::MultiCall;
use terminator::terminator::{Terminator, UniV2Params};

use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;

#[derive(Debug)]
pub struct TerminatorJob {
    pub(crate) credit_facade: Address,
    pub(crate) borrower: Address,
    pub(crate) skip_token_mask: U256,
    pub(crate) covert_weth: bool,
    pub(crate) calls: Vec<MultiCall>,
    pub(crate) router: Address,
    pub(crate) paths: Vec<UniV2Params>,
    pub repay_amount: U256,
    pub underlying_token: Address,
}

pub struct TerminatorService<M: Middleware, S: Signer> {
    contract: Terminator<SignerMiddleware<M, S>>,
}

impl<M: Middleware, S: Signer> TerminatorService<M, S> {
    pub async fn new(
        address: &Address,
        client: std::sync::Arc<SignerMiddleware<M, S>>,
        liquidator_enabled: bool,
    ) -> Self {
        let contract = Terminator::new(*address, client.clone());

        if liquidator_enabled {
            let is_executor = contract.executors(client.address()).call().await.unwrap();

            if !is_executor {
                let tx = contract
                    .allow_executor(client.address())
                    .send()
                    .await
                    .unwrap()
                    .await
                    .unwrap()
                    .unwrap();

                println!("Allow executor {}", tx.transaction_hash);
            } else {
                println!("Executor {} is already allowed", &client.address())
            }
        }

        TerminatorService { contract }
    }

    pub async fn liquidate(
        &mut self,
        job: &TerminatorJob,
    ) -> Result<TransactionReceipt, LiquidationError> {
        dbg!(&job);
        println!("Length: {}", &job.paths.len());

        let result = self
            .contract
            .liquidate_and_sell_on_v2(
                job.credit_facade,
                job.borrower,
                job.skip_token_mask,
                job.covert_weth,
                job.calls.clone(),
                job.router,
                job.paths.clone(),
            )
            .gas(5_000_000)
            .send()
            .await
            .map_err(|err| NetError(format!("Cant execute liquidation {:?}", &job).into()))?
            .await
            .map_err(|err| NetError(format!("Cant execute liquidation {:?}", &job).into()))?
            .ok_or(NetError(
                format!("Cant execute liquidation {:?}", &job).into(),
            ))?;
        Ok(result)
    }
}
