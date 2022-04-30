pub use icreditconfiguratorevents_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditconfiguratorevents_mod {
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
    #[doc = "ICreditConfiguratorEvents was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITCONFIGURATOREVENTS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"protocol\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"adapter\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"protocol\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractForbidden\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newCreditConfigurator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CreditConfiguratorUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newCreditFacade\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CreditFacadeUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chiThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fastCheckDelay\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FastCheckParametersUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feeInterest\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feeLiquidation\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidationPremium\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minBorrowedAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"maxBorrowedAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LimitsUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PriceOracleUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokenAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokenForbidden\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"liquidityThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenLiquidationThresholdUpdated\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITCONFIGURATOREVENTS_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditConfiguratorEvents<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditConfiguratorEvents<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditConfiguratorEvents<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditConfiguratorEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditConfiguratorEvents<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ICREDITCONFIGURATOREVENTS_ABI.clone(),
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
                ICREDITCONFIGURATOREVENTS_ABI.clone(),
                ICREDITCONFIGURATOREVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Gets the contract's `ContractAllowed` event"]
        pub fn contract_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractForbidden` event"]
        pub fn contract_forbidden_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreditConfiguratorUpgraded` event"]
        pub fn credit_configurator_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreditConfiguratorUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreditFacadeUpgraded` event"]
        pub fn credit_facade_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreditFacadeUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FastCheckParametersUpdated` event"]
        pub fn fast_check_parameters_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FastCheckParametersUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FeesUpdated` event"]
        pub fn fees_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeesUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LimitsUpdated` event"]
        pub fn limits_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LimitsUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceOracleUpgraded` event"]
        pub fn price_oracle_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PriceOracleUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenAllowed` event"]
        pub fn token_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenForbidden` event"]
        pub fn token_forbidden_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenLiquidationThresholdUpdated` event"]
        pub fn token_liquidation_threshold_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenLiquidationThresholdUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, ICreditConfiguratorEventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICreditConfiguratorEvents<M>
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
    #[ethevent(name = "ContractAllowed", abi = "ContractAllowed(address,address)")]
    pub struct ContractAllowedFilter {
        #[ethevent(indexed)]
        pub protocol: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub adapter: ethers::core::types::Address,
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
    #[ethevent(name = "ContractForbidden", abi = "ContractForbidden(address)")]
    pub struct ContractForbiddenFilter {
        #[ethevent(indexed)]
        pub protocol: ethers::core::types::Address,
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
        name = "CreditConfiguratorUpgraded",
        abi = "CreditConfiguratorUpgraded(address)"
    )]
    pub struct CreditConfiguratorUpgradedFilter {
        #[ethevent(indexed)]
        pub new_credit_configurator: ethers::core::types::Address,
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
    #[ethevent(name = "CreditFacadeUpgraded", abi = "CreditFacadeUpgraded(address)")]
    pub struct CreditFacadeUpgradedFilter {
        #[ethevent(indexed)]
        pub new_credit_facade: ethers::core::types::Address,
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
        name = "FastCheckParametersUpdated",
        abi = "FastCheckParametersUpdated(uint256,uint256)"
    )]
    pub struct FastCheckParametersUpdatedFilter {
        pub chi_threshold: ethers::core::types::U256,
        pub fast_check_delay: ethers::core::types::U256,
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
    #[ethevent(name = "FeesUpdated", abi = "FeesUpdated(uint256,uint256,uint256)")]
    pub struct FeesUpdatedFilter {
        pub fee_interest: ethers::core::types::U256,
        pub fee_liquidation: ethers::core::types::U256,
        pub liquidation_premium: ethers::core::types::U256,
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
    #[ethevent(name = "LimitsUpdated", abi = "LimitsUpdated(uint256,uint256)")]
    pub struct LimitsUpdatedFilter {
        pub min_borrowed_amount: ethers::core::types::U256,
        pub max_borrowed_amount: ethers::core::types::U256,
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
    #[ethevent(name = "PriceOracleUpgraded", abi = "PriceOracleUpgraded(address)")]
    pub struct PriceOracleUpgradedFilter {
        #[ethevent(indexed)]
        pub new_price_oracle: ethers::core::types::Address,
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
    #[ethevent(name = "TokenAllowed", abi = "TokenAllowed(address)")]
    pub struct TokenAllowedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
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
    #[ethevent(name = "TokenForbidden", abi = "TokenForbidden(address)")]
    pub struct TokenForbiddenFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
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
        name = "TokenLiquidationThresholdUpdated",
        abi = "TokenLiquidationThresholdUpdated(address,uint256)"
    )]
    pub struct TokenLiquidationThresholdUpdatedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub liquidity_threshold: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditConfiguratorEventsEvents {
        ContractAllowedFilter(ContractAllowedFilter),
        ContractForbiddenFilter(ContractForbiddenFilter),
        CreditConfiguratorUpgradedFilter(CreditConfiguratorUpgradedFilter),
        CreditFacadeUpgradedFilter(CreditFacadeUpgradedFilter),
        FastCheckParametersUpdatedFilter(FastCheckParametersUpdatedFilter),
        FeesUpdatedFilter(FeesUpdatedFilter),
        LimitsUpdatedFilter(LimitsUpdatedFilter),
        PriceOracleUpgradedFilter(PriceOracleUpgradedFilter),
        TokenAllowedFilter(TokenAllowedFilter),
        TokenForbiddenFilter(TokenForbiddenFilter),
        TokenLiquidationThresholdUpdatedFilter(TokenLiquidationThresholdUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for ICreditConfiguratorEventsEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ContractAllowedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::ContractAllowedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ContractForbiddenFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::ContractForbiddenFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CreditConfiguratorUpgradedFilter::decode_log(log) {
                return Ok(
                    ICreditConfiguratorEventsEvents::CreditConfiguratorUpgradedFilter(decoded),
                );
            }
            if let Ok(decoded) = CreditFacadeUpgradedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::CreditFacadeUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FastCheckParametersUpdatedFilter::decode_log(log) {
                return Ok(
                    ICreditConfiguratorEventsEvents::FastCheckParametersUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = FeesUpdatedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::FeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LimitsUpdatedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::LimitsUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PriceOracleUpgradedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::PriceOracleUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenAllowedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::TokenAllowedFilter(decoded));
            }
            if let Ok(decoded) = TokenForbiddenFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEventsEvents::TokenForbiddenFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenLiquidationThresholdUpdatedFilter::decode_log(log) {
                return Ok(
                    ICreditConfiguratorEventsEvents::TokenLiquidationThresholdUpdatedFilter(
                        decoded,
                    ),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ICreditConfiguratorEventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditConfiguratorEventsEvents::ContractAllowedFilter(element) => element.fmt(f),
                ICreditConfiguratorEventsEvents::ContractForbiddenFilter(element) => element.fmt(f),
                ICreditConfiguratorEventsEvents::CreditConfiguratorUpgradedFilter(element) => {
                    element.fmt(f)
                }
                ICreditConfiguratorEventsEvents::CreditFacadeUpgradedFilter(element) => {
                    element.fmt(f)
                }
                ICreditConfiguratorEventsEvents::FastCheckParametersUpdatedFilter(element) => {
                    element.fmt(f)
                }
                ICreditConfiguratorEventsEvents::FeesUpdatedFilter(element) => element.fmt(f),
                ICreditConfiguratorEventsEvents::LimitsUpdatedFilter(element) => element.fmt(f),
                ICreditConfiguratorEventsEvents::PriceOracleUpgradedFilter(element) => {
                    element.fmt(f)
                }
                ICreditConfiguratorEventsEvents::TokenAllowedFilter(element) => element.fmt(f),
                ICreditConfiguratorEventsEvents::TokenForbiddenFilter(element) => element.fmt(f),
                ICreditConfiguratorEventsEvents::TokenLiquidationThresholdUpdatedFilter(
                    element,
                ) => element.fmt(f),
            }
        }
    }
}
