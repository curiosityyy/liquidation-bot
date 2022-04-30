pub use iclaimzap_mod::*;
#[allow(clippy::too_many_arguments)]
mod iclaimzap_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IClaimZap was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICLAIMZAP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"rewardContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"extraRewardContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokenRewardContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokenRewardTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositCrvMaxAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositCvxMaxAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"spendCvxAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"options\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICLAIMZAP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IClaimZap<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IClaimZap<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IClaimZap<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IClaimZap))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IClaimZap<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICLAIMZAP_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                ICLAIMZAP_ABI.clone(),
                ICLAIMZAP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `claimRewards` (0x5a7b87f2) function"]
        pub fn claim_rewards(
            &self,
            reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
            extra_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
            token_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
            token_reward_tokens: ::std::vec::Vec<ethers::core::types::Address>,
            deposit_crv_max_amount: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            deposit_cvx_max_amount: ethers::core::types::U256,
            spend_cvx_amount: ethers::core::types::U256,
            options: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [90, 123, 135, 242],
                    (
                        reward_contracts,
                        extra_reward_contracts,
                        token_reward_contracts,
                        token_reward_tokens,
                        deposit_crv_max_amount,
                        min_amount_out,
                        deposit_cvx_max_amount,
                        spend_cvx_amount,
                        options,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IClaimZap<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(address[],address[],address[],address[],uint256,uint256,uint256,uint256,uint256)` and selector `[90, 123, 135, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "claimRewards",
        abi = "claimRewards(address[],address[],address[],address[],uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ClaimRewardsCall {
        pub reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
        pub extra_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
        pub token_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
        pub token_reward_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub deposit_crv_max_amount: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub deposit_cvx_max_amount: ethers::core::types::U256,
        pub spend_cvx_amount: ethers::core::types::U256,
        pub options: ethers::core::types::U256,
    }
}
