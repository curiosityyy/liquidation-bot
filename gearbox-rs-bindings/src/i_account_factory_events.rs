pub use iaccountfactoryevents_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaccountfactoryevents_mod {
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
    #[doc = "IAccountFactoryEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IACCOUNTFACTORYEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AccountMinerChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"InitializeCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ReturnCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TakeForever\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IACCOUNTFACTORYEVENTS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAccountFactoryEvents<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAccountFactoryEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAccountFactoryEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAccountFactoryEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAccountFactoryEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IACCOUNTFACTORYEVENTS_ABI.clone(),
                client,
            )
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
                IACCOUNTFACTORYEVENTS_ABI.clone(),
                IACCOUNTFACTORYEVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Gets the contract's `AccountMinerChanged` event"]
        pub fn account_miner_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AccountMinerChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InitializeCreditAccount` event"]
        pub fn initialize_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializeCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCreditAccount` event"]
        pub fn new_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReturnCreditAccount` event"]
        pub fn return_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReturnCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TakeForever` event"]
        pub fn take_forever_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TakeForeverFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IAccountFactoryEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAccountFactoryEvents<M>
    {
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
    #[ethevent(name = "AccountMinerChanged", abi = "AccountMinerChanged(address)")]
    pub struct AccountMinerChangedFilter {
        #[ethevent(indexed)]
        pub miner: ethers::core::types::Address,
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
        name = "InitializeCreditAccount",
        abi = "InitializeCreditAccount(address,address)"
    )]
    pub struct InitializeCreditAccountFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
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
    #[ethevent(name = "NewCreditAccount", abi = "NewCreditAccount(address)")]
    pub struct NewCreditAccountFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "ReturnCreditAccount", abi = "ReturnCreditAccount(address)")]
    pub struct ReturnCreditAccountFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "TakeForever", abi = "TakeForever(address,address)")]
    pub struct TakeForeverFilter {
        #[ethevent(indexed)]
        pub credit_account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAccountFactoryEventsEvents {
        AccountMinerChangedFilter(AccountMinerChangedFilter),
        InitializeCreditAccountFilter(InitializeCreditAccountFilter),
        NewCreditAccountFilter(NewCreditAccountFilter),
        ReturnCreditAccountFilter(ReturnCreditAccountFilter),
        TakeForeverFilter(TakeForeverFilter),
    }
    impl ethers::contract::EthLogDecode for IAccountFactoryEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccountMinerChangedFilter::decode_log(log) {
                return Ok(IAccountFactoryEventsEvents::AccountMinerChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializeCreditAccountFilter::decode_log(log) {
                return Ok(IAccountFactoryEventsEvents::InitializeCreditAccountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewCreditAccountFilter::decode_log(log) {
                return Ok(IAccountFactoryEventsEvents::NewCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = ReturnCreditAccountFilter::decode_log(log) {
                return Ok(IAccountFactoryEventsEvents::ReturnCreditAccountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TakeForeverFilter::decode_log(log) {
                return Ok(IAccountFactoryEventsEvents::TakeForeverFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAccountFactoryEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAccountFactoryEventsEvents::AccountMinerChangedFilter(element) => element.fmt(f),
                IAccountFactoryEventsEvents::InitializeCreditAccountFilter(element) => {
                    element.fmt(f)
                }
                IAccountFactoryEventsEvents::NewCreditAccountFilter(element) => element.fmt(f),
                IAccountFactoryEventsEvents::ReturnCreditAccountFilter(element) => element.fmt(f),
                IAccountFactoryEventsEvents::TakeForeverFilter(element) => element.fmt(f),
            }
        }
    }
}
