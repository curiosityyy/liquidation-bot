pub use iacl_mod::*;
#[allow(clippy::too_many_arguments)]
mod iacl_mod {
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
    #[doc = "IACL was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IACL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PausableAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PausableAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UnpausableAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UnpausableAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isConfigurator\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPausableAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isUnpausableAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IACL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IACL<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IACL<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IACL<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IACL))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IACL<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IACL_ABI.clone(), client).into()
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
                IACL_ABI.clone(),
                IACL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `isConfigurator` (0x5f259aba) function"]
        pub fn is_configurator(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([95, 37, 154, 186], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPausableAdmin` (0x3a41ec64) function"]
        pub fn is_pausable_admin(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([58, 65, 236, 100], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isUnpausableAdmin` (0xd4eb5db0) function"]
        pub fn is_unpausable_admin(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([212, 235, 93, 176], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `PausableAdminAdded` event"]
        pub fn pausable_admin_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausableAdminAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PausableAdminRemoved` event"]
        pub fn pausable_admin_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausableAdminRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UnpausableAdminAdded` event"]
        pub fn unpausable_admin_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UnpausableAdminAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UnpausableAdminRemoved` event"]
        pub fn unpausable_admin_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UnpausableAdminRemovedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IACLEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IACL<M> {
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
    #[ethevent(name = "PausableAdminAdded", abi = "PausableAdminAdded(address)")]
    pub struct PausableAdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "PausableAdminRemoved", abi = "PausableAdminRemoved(address)")]
    pub struct PausableAdminRemovedFilter {
        #[ethevent(indexed)]
        pub admin: ethers::core::types::Address,
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
    #[ethevent(name = "UnpausableAdminAdded", abi = "UnpausableAdminAdded(address)")]
    pub struct UnpausableAdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(
        name = "UnpausableAdminRemoved",
        abi = "UnpausableAdminRemoved(address)"
    )]
    pub struct UnpausableAdminRemovedFilter {
        #[ethevent(indexed)]
        pub admin: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IACLEvents {
        PausableAdminAddedFilter(PausableAdminAddedFilter),
        PausableAdminRemovedFilter(PausableAdminRemovedFilter),
        UnpausableAdminAddedFilter(UnpausableAdminAddedFilter),
        UnpausableAdminRemovedFilter(UnpausableAdminRemovedFilter),
    }
    impl ethers::contract::EthLogDecode for IACLEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PausableAdminAddedFilter::decode_log(log) {
                return Ok(IACLEvents::PausableAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = PausableAdminRemovedFilter::decode_log(log) {
                return Ok(IACLEvents::PausableAdminRemovedFilter(decoded));
            }
            if let Ok(decoded) = UnpausableAdminAddedFilter::decode_log(log) {
                return Ok(IACLEvents::UnpausableAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = UnpausableAdminRemovedFilter::decode_log(log) {
                return Ok(IACLEvents::UnpausableAdminRemovedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IACLEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IACLEvents::PausableAdminAddedFilter(element) => element.fmt(f),
                IACLEvents::PausableAdminRemovedFilter(element) => element.fmt(f),
                IACLEvents::UnpausableAdminAddedFilter(element) => element.fmt(f),
                IACLEvents::UnpausableAdminRemovedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `isConfigurator`function with signature `isConfigurator(address)` and selector `[95, 37, 154, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isConfigurator", abi = "isConfigurator(address)")]
    pub struct IsConfiguratorCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isPausableAdmin`function with signature `isPausableAdmin(address)` and selector `[58, 65, 236, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPausableAdmin", abi = "isPausableAdmin(address)")]
    pub struct IsPausableAdminCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isUnpausableAdmin`function with signature `isUnpausableAdmin(address)` and selector `[212, 235, 93, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isUnpausableAdmin", abi = "isUnpausableAdmin(address)")]
    pub struct IsUnpausableAdminCall {
        pub addr: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IACLCalls {
        IsConfigurator(IsConfiguratorCall),
        IsPausableAdmin(IsPausableAdminCall),
        IsUnpausableAdmin(IsUnpausableAdminCall),
    }
    impl ethers::core::abi::AbiDecode for IACLCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsConfiguratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLCalls::IsConfigurator(decoded));
            }
            if let Ok(decoded) =
                <IsPausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLCalls::IsPausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsUnpausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IACLCalls::IsUnpausableAdmin(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IACLCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IACLCalls::IsConfigurator(element) => element.encode(),
                IACLCalls::IsPausableAdmin(element) => element.encode(),
                IACLCalls::IsUnpausableAdmin(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IACLCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IACLCalls::IsConfigurator(element) => element.fmt(f),
                IACLCalls::IsPausableAdmin(element) => element.fmt(f),
                IACLCalls::IsUnpausableAdmin(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsConfiguratorCall> for IACLCalls {
        fn from(var: IsConfiguratorCall) -> Self {
            IACLCalls::IsConfigurator(var)
        }
    }
    impl ::std::convert::From<IsPausableAdminCall> for IACLCalls {
        fn from(var: IsPausableAdminCall) -> Self {
            IACLCalls::IsPausableAdmin(var)
        }
    }
    impl ::std::convert::From<IsUnpausableAdminCall> for IACLCalls {
        fn from(var: IsUnpausableAdminCall) -> Self {
            IACLCalls::IsUnpausableAdmin(var)
        }
    }
}
