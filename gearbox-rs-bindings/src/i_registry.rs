pub use iregistry_mod::*;
#[allow(clippy::too_many_arguments)]
mod iregistry_mod {
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
    #[doc = "IRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge_controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_address\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_gauges\",\"outputs\":[{\"internalType\":\"address[10]\",\"name\":\"\",\"type\":\"address[10]\",\"components\":[]},{\"internalType\":\"uint128[10]\",\"name\":\"\",\"type\":\"uint128[10]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_registry\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IREGISTRY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IRegistry<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IREGISTRY_ABI.clone(), client).into()
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
                IREGISTRY_ABI.clone(),
                IREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `gauge_controller` (0xd8b9a018) function"]
        pub fn gauge_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([216, 185, 160, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_address` (0x493f4f74) function"]
        pub fn get_address(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([73, 63, 79, 116], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_gauges` (0x56059ffb) function"]
        pub fn get_gauges(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ([ethers::core::types::Address; 10usize], [u128; 10usize]),
        > {
            self.0
                .method_hash([86, 5, 159, 251], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_lp_token` (0x37951049) function"]
        pub fn get_lp_token(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([55, 149, 16, 73], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_registry` (0xa262904b) function"]
        pub fn get_registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([162, 98, 144, 75], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IRegistry<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `gauge_controller`function with signature `gauge_controller()` and selector `[216, 185, 160, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gauge_controller", abi = "gauge_controller()")]
    pub struct GaugeControllerCall;
    #[doc = "Container type for all input parameters for the `get_address`function with signature `get_address(uint256)` and selector `[73, 63, 79, 116]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_address", abi = "get_address(uint256)")]
    pub struct GetAddressCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `get_gauges`function with signature `get_gauges(address)` and selector `[86, 5, 159, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_gauges", abi = "get_gauges(address)")]
    pub struct GetGaugesCall(pub ethers::core::types::Address);
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
    pub struct GetLpTokenCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `get_registry`function with signature `get_registry()` and selector `[162, 98, 144, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_registry", abi = "get_registry()")]
    pub struct GetRegistryCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IRegistryCalls {
        GaugeController(GaugeControllerCall),
        GetAddress(GetAddressCall),
        GetGauges(GetGaugesCall),
        GetLpToken(GetLpTokenCall),
        GetRegistry(GetRegistryCall),
    }
    impl ethers::core::abi::AbiDecode for IRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GaugeControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRegistryCalls::GaugeController(decoded));
            }
            if let Ok(decoded) =
                <GetAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRegistryCalls::GetAddress(decoded));
            }
            if let Ok(decoded) =
                <GetGaugesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRegistryCalls::GetGauges(decoded));
            }
            if let Ok(decoded) =
                <GetLpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRegistryCalls::GetLpToken(decoded));
            }
            if let Ok(decoded) =
                <GetRegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRegistryCalls::GetRegistry(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IRegistryCalls::GaugeController(element) => element.encode(),
                IRegistryCalls::GetAddress(element) => element.encode(),
                IRegistryCalls::GetGauges(element) => element.encode(),
                IRegistryCalls::GetLpToken(element) => element.encode(),
                IRegistryCalls::GetRegistry(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRegistryCalls::GaugeController(element) => element.fmt(f),
                IRegistryCalls::GetAddress(element) => element.fmt(f),
                IRegistryCalls::GetGauges(element) => element.fmt(f),
                IRegistryCalls::GetLpToken(element) => element.fmt(f),
                IRegistryCalls::GetRegistry(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GaugeControllerCall> for IRegistryCalls {
        fn from(var: GaugeControllerCall) -> Self {
            IRegistryCalls::GaugeController(var)
        }
    }
    impl ::std::convert::From<GetAddressCall> for IRegistryCalls {
        fn from(var: GetAddressCall) -> Self {
            IRegistryCalls::GetAddress(var)
        }
    }
    impl ::std::convert::From<GetGaugesCall> for IRegistryCalls {
        fn from(var: GetGaugesCall) -> Self {
            IRegistryCalls::GetGauges(var)
        }
    }
    impl ::std::convert::From<GetLpTokenCall> for IRegistryCalls {
        fn from(var: GetLpTokenCall) -> Self {
            IRegistryCalls::GetLpToken(var)
        }
    }
    impl ::std::convert::From<GetRegistryCall> for IRegistryCalls {
        fn from(var: GetRegistryCall) -> Self {
            IRegistryCalls::GetRegistry(var)
        }
    }
}
