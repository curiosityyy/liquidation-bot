pub use iaclevents_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaclevents_mod {
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
    #[doc = "IACLEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IACLEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PausableAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PausableAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UnpausableAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UnpausableAdminRemoved\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IACLEVENTS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IACLEvents<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IACLEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IACLEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IACLEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IACLEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IACLEVENTS_ABI.clone(), client).into()
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
                IACLEVENTS_ABI.clone(),
                IACLEVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, IACLEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IACLEvents<M> {
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
    pub enum IACLEventsEvents {
        PausableAdminAddedFilter(PausableAdminAddedFilter),
        PausableAdminRemovedFilter(PausableAdminRemovedFilter),
        UnpausableAdminAddedFilter(UnpausableAdminAddedFilter),
        UnpausableAdminRemovedFilter(UnpausableAdminRemovedFilter),
    }
    impl ethers::contract::EthLogDecode for IACLEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PausableAdminAddedFilter::decode_log(log) {
                return Ok(IACLEventsEvents::PausableAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = PausableAdminRemovedFilter::decode_log(log) {
                return Ok(IACLEventsEvents::PausableAdminRemovedFilter(decoded));
            }
            if let Ok(decoded) = UnpausableAdminAddedFilter::decode_log(log) {
                return Ok(IACLEventsEvents::UnpausableAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = UnpausableAdminRemovedFilter::decode_log(log) {
                return Ok(IACLEventsEvents::UnpausableAdminRemovedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IACLEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IACLEventsEvents::PausableAdminAddedFilter(element) => element.fmt(f),
                IACLEventsEvents::PausableAdminRemovedFilter(element) => element.fmt(f),
                IACLEventsEvents::UnpausableAdminAddedFilter(element) => element.fmt(f),
                IACLEventsEvents::UnpausableAdminRemovedFilter(element) => element.fmt(f),
            }
        }
    }
}
