pub use icreditfacadeevents_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditfacadeevents_mod {
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
    #[doc = "ICreditFacadeEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITFACADEEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddCollateral\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CloseCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DecreaseBorrowedAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IncreaseBorrowedAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"remainingFunds\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"MultiCallFinished\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MultiCallStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OpenCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TransferAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferAccountAllowed\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITFACADEEVENTS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditFacadeEvents<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditFacadeEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditFacadeEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditFacadeEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditFacadeEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICREDITFACADEEVENTS_ABI.clone(), client)
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
                ICREDITFACADEEVENTS_ABI.clone(),
                ICREDITFACADEEVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Gets the contract's `AddCollateral` event"]
        pub fn add_collateral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CloseCreditAccount` event"]
        pub fn close_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CloseCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DecreaseBorrowedAmount` event"]
        pub fn decrease_borrowed_amount_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DecreaseBorrowedAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseBorrowedAmount` event"]
        pub fn increase_borrowed_amount_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreaseBorrowedAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidateCreditAccount` event"]
        pub fn liquidate_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidateCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MultiCallFinished` event"]
        pub fn multi_call_finished_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MultiCallFinishedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MultiCallStarted` event"]
        pub fn multi_call_started_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MultiCallStartedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OpenCreditAccount` event"]
        pub fn open_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OpenCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAccount` event"]
        pub fn transfer_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAccountAllowed` event"]
        pub fn transfer_account_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferAccountAllowedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ICreditFacadeEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICreditFacadeEvents<M>
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
    #[ethevent(name = "AddCollateral", abi = "AddCollateral(address,address,uint256)")]
    pub struct AddCollateralFilter {
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
        name = "CloseCreditAccount",
        abi = "CloseCreditAccount(address,address)"
    )]
    pub struct CloseCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
        name = "DecreaseBorrowedAmount",
        abi = "DecreaseBorrowedAmount(address,uint256)"
    )]
    pub struct DecreaseBorrowedAmountFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
        name = "IncreaseBorrowedAmount",
        abi = "IncreaseBorrowedAmount(address,uint256)"
    )]
    pub struct IncreaseBorrowedAmountFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
        name = "LiquidateCreditAccount",
        abi = "LiquidateCreditAccount(address,address,address,uint256)"
    )]
    pub struct LiquidateCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub liquidator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub remaining_funds: ethers::core::types::U256,
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
    #[ethevent(name = "MultiCallFinished", abi = "MultiCallFinished()")]
    pub struct MultiCallFinishedFilter();
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "MultiCallStarted", abi = "MultiCallStarted(address)")]
    pub struct MultiCallStartedFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
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
        name = "OpenCreditAccount",
        abi = "OpenCreditAccount(address,address,uint256,uint256)"
    )]
    pub struct OpenCreditAccountFilter {
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub credit_account: ethers::core::types::Address,
        pub borrow_amount: ethers::core::types::U256,
        pub referral_code: ethers::core::types::U256,
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
    #[ethevent(name = "TransferAccount", abi = "TransferAccount(address,address)")]
    pub struct TransferAccountFilter {
        #[ethevent(indexed)]
        pub old_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        name = "TransferAccountAllowed",
        abi = "TransferAccountAllowed(address,address,bool)"
    )]
    pub struct TransferAccountAllowedFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub state: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditFacadeEventsEvents {
        AddCollateralFilter(AddCollateralFilter),
        CloseCreditAccountFilter(CloseCreditAccountFilter),
        DecreaseBorrowedAmountFilter(DecreaseBorrowedAmountFilter),
        IncreaseBorrowedAmountFilter(IncreaseBorrowedAmountFilter),
        LiquidateCreditAccountFilter(LiquidateCreditAccountFilter),
        MultiCallFinishedFilter(MultiCallFinishedFilter),
        MultiCallStartedFilter(MultiCallStartedFilter),
        OpenCreditAccountFilter(OpenCreditAccountFilter),
        TransferAccountFilter(TransferAccountFilter),
        TransferAccountAllowedFilter(TransferAccountAllowedFilter),
    }
    impl ethers::contract::EthLogDecode for ICreditFacadeEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddCollateralFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::AddCollateralFilter(decoded));
            }
            if let Ok(decoded) = CloseCreditAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::CloseCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = DecreaseBorrowedAmountFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::DecreaseBorrowedAmountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = IncreaseBorrowedAmountFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::IncreaseBorrowedAmountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LiquidateCreditAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::LiquidateCreditAccountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = MultiCallFinishedFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::MultiCallFinishedFilter(decoded));
            }
            if let Ok(decoded) = MultiCallStartedFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::MultiCallStartedFilter(decoded));
            }
            if let Ok(decoded) = OpenCreditAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::OpenCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::TransferAccountFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountAllowedFilter::decode_log(log) {
                return Ok(ICreditFacadeEventsEvents::TransferAccountAllowedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ICreditFacadeEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditFacadeEventsEvents::AddCollateralFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::CloseCreditAccountFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::DecreaseBorrowedAmountFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::IncreaseBorrowedAmountFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::LiquidateCreditAccountFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::MultiCallFinishedFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::MultiCallStartedFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::OpenCreditAccountFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::TransferAccountFilter(element) => element.fmt(f),
                ICreditFacadeEventsEvents::TransferAccountAllowedFilter(element) => element.fmt(f),
            }
        }
    }
}
