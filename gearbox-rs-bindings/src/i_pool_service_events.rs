pub use ipoolserviceevents_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipoolserviceevents_mod {
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
    #[doc = "IPoolServiceEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOLSERVICEEVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddLiquidity\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowForbidden\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewCreditManagerConnected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newLimit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewExpectedLiquidityLimit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewWithdrawFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RemoveLiquidity\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"profit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"loss\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Repay\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"loss\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UncoveredLoss\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IPOOLSERVICEEVENTS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IPoolServiceEvents<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IPoolServiceEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPoolServiceEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPoolServiceEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IPoolServiceEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOOLSERVICEEVENTS_ABI.clone(), client)
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
                IPOOLSERVICEEVENTS_ABI.clone(),
                IPOOLSERVICEEVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Gets the contract's `AddLiquidity` event"]
        pub fn add_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers::contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowForbidden` event"]
        pub fn borrow_forbidden_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCreditManagerConnected` event"]
        pub fn new_credit_manager_connected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCreditManagerConnectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewExpectedLiquidityLimit` event"]
        pub fn new_expected_liquidity_limit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewExpectedLiquidityLimitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewInterestRateModel` event"]
        pub fn new_interest_rate_model_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewInterestRateModelFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewWithdrawFee` event"]
        pub fn new_withdraw_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewWithdrawFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemoveLiquidity` event"]
        pub fn remove_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RemoveLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repay` event"]
        pub fn repay_filter(&self) -> ethers::contract::builders::Event<M, RepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UncoveredLoss` event"]
        pub fn uncovered_loss_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UncoveredLossFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IPoolServiceEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IPoolServiceEvents<M>
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
    #[ethevent(
        name = "AddLiquidity",
        abi = "AddLiquidity(address,address,uint256,uint256)"
    )]
    pub struct AddLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "Borrow", abi = "Borrow(address,address,uint256)")]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub credit_account: ethers::core::types::Address,
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
    #[ethevent(name = "BorrowForbidden", abi = "BorrowForbidden(address)")]
    pub struct BorrowForbiddenFilter {
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
    #[ethevent(
        name = "NewCreditManagerConnected",
        abi = "NewCreditManagerConnected(address)"
    )]
    pub struct NewCreditManagerConnectedFilter {
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
    #[ethevent(
        name = "NewExpectedLiquidityLimit",
        abi = "NewExpectedLiquidityLimit(uint256)"
    )]
    pub struct NewExpectedLiquidityLimitFilter {
        pub new_limit: ethers::core::types::U256,
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
    #[ethevent(name = "NewInterestRateModel", abi = "NewInterestRateModel(address)")]
    pub struct NewInterestRateModelFilter {
        #[ethevent(indexed)]
        pub new_interest_rate_model: ethers::core::types::Address,
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
    #[ethevent(name = "NewWithdrawFee", abi = "NewWithdrawFee(uint256)")]
    pub struct NewWithdrawFeeFilter {
        pub fee: ethers::core::types::U256,
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
        name = "RemoveLiquidity",
        abi = "RemoveLiquidity(address,address,uint256)"
    )]
    pub struct RemoveLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
    #[ethevent(name = "Repay", abi = "Repay(address,uint256,uint256,uint256)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
        pub borrowed_amount: ethers::core::types::U256,
        pub profit: ethers::core::types::U256,
        pub loss: ethers::core::types::U256,
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
    #[ethevent(name = "UncoveredLoss", abi = "UncoveredLoss(address,uint256)")]
    pub struct UncoveredLossFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
        pub loss: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolServiceEventsEvents {
        AddLiquidityFilter(AddLiquidityFilter),
        BorrowFilter(BorrowFilter),
        BorrowForbiddenFilter(BorrowForbiddenFilter),
        NewCreditManagerConnectedFilter(NewCreditManagerConnectedFilter),
        NewExpectedLiquidityLimitFilter(NewExpectedLiquidityLimitFilter),
        NewInterestRateModelFilter(NewInterestRateModelFilter),
        NewWithdrawFeeFilter(NewWithdrawFeeFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        RepayFilter(RepayFilter),
        UncoveredLossFilter(UncoveredLossFilter),
    }
    impl ethers::contract::EthLogDecode for IPoolServiceEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = BorrowForbiddenFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::BorrowForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewCreditManagerConnectedFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::NewCreditManagerConnectedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewExpectedLiquidityLimitFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::NewExpectedLiquidityLimitFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewInterestRateModelFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::NewInterestRateModelFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewWithdrawFeeFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::NewWithdrawFeeFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = UncoveredLossFilter::decode_log(log) {
                return Ok(IPoolServiceEventsEvents::UncoveredLossFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPoolServiceEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolServiceEventsEvents::AddLiquidityFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::BorrowFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::BorrowForbiddenFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::NewCreditManagerConnectedFilter(element) => {
                    element.fmt(f)
                }
                IPoolServiceEventsEvents::NewExpectedLiquidityLimitFilter(element) => {
                    element.fmt(f)
                }
                IPoolServiceEventsEvents::NewInterestRateModelFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::NewWithdrawFeeFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::RemoveLiquidityFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::RepayFilter(element) => element.fmt(f),
                IPoolServiceEventsEvents::UncoveredLossFilter(element) => element.fmt(f),
            }
        }
    }
}
