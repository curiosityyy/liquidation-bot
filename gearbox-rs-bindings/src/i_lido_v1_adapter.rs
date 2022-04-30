pub use ilidov1adapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod ilidov1adapter_mod {
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
    #[doc = "ILidoV1Adapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ILIDOV1ADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"LimitIsOverException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewLimit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLimit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ILIDOV1ADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ILidoV1Adapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ILidoV1Adapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ILidoV1Adapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILidoV1Adapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ILidoV1Adapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ILIDOV1ADAPTER_ABI.clone(), client)
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
                ILIDOV1ADAPTER_ABI.clone(),
                ILIDOV1ADAPTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_gearboxAdapterType` (0xce30bbdb) function"]
        pub fn gearbox_adapter_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([206, 48, 187, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_gearboxAdapterVersion` (0x78aa73a4) function"]
        pub fn gearbox_adapter_version(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([120, 170, 115, 164], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditFacade` (0x2f7a1881) function"]
        pub fn credit_facade(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([47, 122, 24, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManager` (0xc12c21c0) function"]
        pub fn credit_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 44, 33, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLimit` (0x27ea6f2b) function"]
        pub fn set_limit(
            &self,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 234, 111, 43], limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetContract` (0xbd90df70) function"]
        pub fn target_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([189, 144, 223, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewLimit` event"]
        pub fn new_limit_filter(&self) -> ethers::contract::builders::Event<M, NewLimitFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewLimitFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ILidoV1Adapter<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "NewLimit", abi = "NewLimit(uint256)")]
    pub struct NewLimitFilter {
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_gearboxAdapterType`function with signature `_gearboxAdapterType()` and selector `[206, 48, 187, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_gearboxAdapterType", abi = "_gearboxAdapterType()")]
    pub struct GearboxAdapterTypeCall;
    #[doc = "Container type for all input parameters for the `_gearboxAdapterVersion`function with signature `_gearboxAdapterVersion()` and selector `[120, 170, 115, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_gearboxAdapterVersion", abi = "_gearboxAdapterVersion()")]
    pub struct GearboxAdapterVersionCall;
    #[doc = "Container type for all input parameters for the `creditFacade`function with signature `creditFacade()` and selector `[47, 122, 24, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditFacade", abi = "creditFacade()")]
    pub struct CreditFacadeCall;
    #[doc = "Container type for all input parameters for the `creditManager`function with signature `creditManager()` and selector `[193, 44, 33, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManager", abi = "creditManager()")]
    pub struct CreditManagerCall;
    #[doc = "Container type for all input parameters for the `setLimit`function with signature `setLimit(uint256)` and selector `[39, 234, 111, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLimit", abi = "setLimit(uint256)")]
    pub struct SetLimitCall {
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `targetContract`function with signature `targetContract()` and selector `[189, 144, 223, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "targetContract", abi = "targetContract()")]
    pub struct TargetContractCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ILidoV1AdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        SetLimit(SetLimitCall),
        TargetContract(TargetContractCall),
    }
    impl ethers::core::abi::AbiDecode for ILidoV1AdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoV1AdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoV1AdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoV1AdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoV1AdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <SetLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoV1AdapterCalls::SetLimit(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILidoV1AdapterCalls::TargetContract(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ILidoV1AdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ILidoV1AdapterCalls::GearboxAdapterType(element) => element.encode(),
                ILidoV1AdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                ILidoV1AdapterCalls::CreditFacade(element) => element.encode(),
                ILidoV1AdapterCalls::CreditManager(element) => element.encode(),
                ILidoV1AdapterCalls::SetLimit(element) => element.encode(),
                ILidoV1AdapterCalls::TargetContract(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ILidoV1AdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ILidoV1AdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                ILidoV1AdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                ILidoV1AdapterCalls::CreditFacade(element) => element.fmt(f),
                ILidoV1AdapterCalls::CreditManager(element) => element.fmt(f),
                ILidoV1AdapterCalls::SetLimit(element) => element.fmt(f),
                ILidoV1AdapterCalls::TargetContract(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for ILidoV1AdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            ILidoV1AdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for ILidoV1AdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            ILidoV1AdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for ILidoV1AdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            ILidoV1AdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for ILidoV1AdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            ILidoV1AdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<SetLimitCall> for ILidoV1AdapterCalls {
        fn from(var: SetLimitCall) -> Self {
            ILidoV1AdapterCalls::SetLimit(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for ILidoV1AdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            ILidoV1AdapterCalls::TargetContract(var)
        }
    }
}
