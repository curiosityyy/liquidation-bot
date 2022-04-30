pub use icurveregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod icurveregistry_mod {
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
    #[doc = "ICurveRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICURVEREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_n_coins\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_pool_from_lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICURVEREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICurveRegistry<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICurveRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICurveRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICurveRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICurveRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICURVEREGISTRY_ABI.clone(), client)
                .into()
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
                ICURVEREGISTRY_ABI.clone(),
                ICURVEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `get_lp_token` (0x37951049) function"]
        pub fn get_lp_token(
            &self,
            pool: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([55, 149, 16, 73], pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_n_coins` (0x940494f1) function"]
        pub fn get_n_coins(
            &self,
            pool: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 4, 148, 241], pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_pool_from_lp_token` (0xbdf475c3) function"]
        pub fn get_pool_from_lp_token(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([189, 244, 117, 195], token)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICurveRegistry<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `get_lp_token`function with signature `get_lp_token(address)` and selector `[55, 149, 16, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_lp_token", abi = "get_lp_token(address)")]
    pub struct GetLpTokenCall {
        pub pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `get_n_coins`function with signature `get_n_coins(address)` and selector `[148, 4, 148, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_n_coins", abi = "get_n_coins(address)")]
    pub struct GetNCoinsCall {
        pub pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `get_pool_from_lp_token`function with signature `get_pool_from_lp_token(address)` and selector `[189, 244, 117, 195]`"]
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
        name = "get_pool_from_lp_token",
        abi = "get_pool_from_lp_token(address)"
    )]
    pub struct GetPoolFromLpTokenCall {
        pub token: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICurveRegistryCalls {
        GetLpToken(GetLpTokenCall),
        GetNCoins(GetNCoinsCall),
        GetPoolFromLpToken(GetPoolFromLpTokenCall),
    }
    impl ethers::core::abi::AbiDecode for ICurveRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetLpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveRegistryCalls::GetLpToken(decoded));
            }
            if let Ok(decoded) =
                <GetNCoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveRegistryCalls::GetNCoins(decoded));
            }
            if let Ok(decoded) =
                <GetPoolFromLpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveRegistryCalls::GetPoolFromLpToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICurveRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICurveRegistryCalls::GetLpToken(element) => element.encode(),
                ICurveRegistryCalls::GetNCoins(element) => element.encode(),
                ICurveRegistryCalls::GetPoolFromLpToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICurveRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICurveRegistryCalls::GetLpToken(element) => element.fmt(f),
                ICurveRegistryCalls::GetNCoins(element) => element.fmt(f),
                ICurveRegistryCalls::GetPoolFromLpToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetLpTokenCall> for ICurveRegistryCalls {
        fn from(var: GetLpTokenCall) -> Self {
            ICurveRegistryCalls::GetLpToken(var)
        }
    }
    impl ::std::convert::From<GetNCoinsCall> for ICurveRegistryCalls {
        fn from(var: GetNCoinsCall) -> Self {
            ICurveRegistryCalls::GetNCoins(var)
        }
    }
    impl ::std::convert::From<GetPoolFromLpTokenCall> for ICurveRegistryCalls {
        fn from(var: GetPoolFromLpTokenCall) -> Self {
            ICurveRegistryCalls::GetPoolFromLpToken(var)
        }
    }
}
