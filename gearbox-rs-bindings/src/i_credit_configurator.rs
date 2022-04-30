pub use icreditconfigurator_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditconfigurator_mod {
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
    #[doc = "ICreditConfigurator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITCONFIGURATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AdapterHasIncorrectCreditManagerException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AdapterUsedTwiceException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ChiThresholdMoreOneException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ContractNotInAllowedList\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CreditManagerOrFacadeUsedAsAllowContractsException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FastCheckNotCoverCollateralDropException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectFeesException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectLiquidationThresholdException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectPriceFeedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectTokenContractException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SetLTForUnderlyingException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"protocol\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"adapter\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"protocol\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractForbidden\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newCreditConfigurator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CreditConfiguratorUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newCreditFacade\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CreditFacadeUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chiThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fastCheckDelay\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FastCheckParametersUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feeInterest\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feeLiquidation\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"liquidationPremium\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minBorrowedAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"maxBorrowedAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LimitsUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPriceOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PriceOracleUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokenAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TokenForbidden\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"liquidityThreshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenLiquidationThresholdUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addTokenToAllowedList\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"adapter\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"allowContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedContracts\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedContractsCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forbidContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationThreshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLiquidationThreshold\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITCONFIGURATOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditConfigurator<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditConfigurator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditConfigurator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditConfigurator))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditConfigurator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICREDITCONFIGURATOR_ABI.clone(), client)
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
                ICREDITCONFIGURATOR_ABI.clone(),
                ICREDITCONFIGURATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addTokenToAllowedList` (0xdadfb98b) function"]
        pub fn add_token_to_allowed_list(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 223, 185, 139], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowContract` (0x7bccacee) function"]
        pub fn allow_contract(
            &self,
            target_contract: ethers::core::types::Address,
            adapter: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 204, 172, 238], (target_contract, adapter))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContracts` (0x5094cb4f) function"]
        pub fn allowed_contracts(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([80, 148, 203, 79], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContractsCount` (0x50e036ff) function"]
        pub fn allowed_contracts_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([80, 224, 54, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidContract` (0x52438e54) function"]
        pub fn forbid_contract(
            &self,
            target_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 67, 142, 84], target_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidationThreshold` (0x0e30428d) function"]
        pub fn set_liquidation_threshold(
            &self,
            token: ethers::core::types::Address,
            liquidation_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 48, 66, 141], (token, liquidation_threshold))
                .expect("method not found (this should never happen)")
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ICreditConfiguratorEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICreditConfigurator<M>
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
    pub enum ICreditConfiguratorEvents {
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
    impl ethers::contract::EthLogDecode for ICreditConfiguratorEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ContractAllowedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::ContractAllowedFilter(decoded));
            }
            if let Ok(decoded) = ContractForbiddenFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::ContractForbiddenFilter(decoded));
            }
            if let Ok(decoded) = CreditConfiguratorUpgradedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::CreditConfiguratorUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CreditFacadeUpgradedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::CreditFacadeUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FastCheckParametersUpdatedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::FastCheckParametersUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FeesUpdatedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::FeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LimitsUpdatedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::LimitsUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleUpgradedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::PriceOracleUpgradedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TokenAllowedFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::TokenAllowedFilter(decoded));
            }
            if let Ok(decoded) = TokenForbiddenFilter::decode_log(log) {
                return Ok(ICreditConfiguratorEvents::TokenForbiddenFilter(decoded));
            }
            if let Ok(decoded) = TokenLiquidationThresholdUpdatedFilter::decode_log(log) {
                return Ok(
                    ICreditConfiguratorEvents::TokenLiquidationThresholdUpdatedFilter(decoded),
                );
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ICreditConfiguratorEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditConfiguratorEvents::ContractAllowedFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::ContractForbiddenFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::CreditConfiguratorUpgradedFilter(element) => {
                    element.fmt(f)
                }
                ICreditConfiguratorEvents::CreditFacadeUpgradedFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::FastCheckParametersUpdatedFilter(element) => {
                    element.fmt(f)
                }
                ICreditConfiguratorEvents::FeesUpdatedFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::LimitsUpdatedFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::PriceOracleUpgradedFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::TokenAllowedFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::TokenForbiddenFilter(element) => element.fmt(f),
                ICreditConfiguratorEvents::TokenLiquidationThresholdUpdatedFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addTokenToAllowedList`function with signature `addTokenToAllowedList(address)` and selector `[218, 223, 185, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addTokenToAllowedList", abi = "addTokenToAllowedList(address)")]
    pub struct AddTokenToAllowedListCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowContract`function with signature `allowContract(address,address)` and selector `[123, 204, 172, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowContract", abi = "allowContract(address,address)")]
    pub struct AllowContractCall {
        pub target_contract: ethers::core::types::Address,
        pub adapter: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowedContracts`function with signature `allowedContracts(uint256)` and selector `[80, 148, 203, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowedContracts", abi = "allowedContracts(uint256)")]
    pub struct AllowedContractsCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allowedContractsCount`function with signature `allowedContractsCount()` and selector `[80, 224, 54, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowedContractsCount", abi = "allowedContractsCount()")]
    pub struct AllowedContractsCountCall;
    #[doc = "Container type for all input parameters for the `forbidContract`function with signature `forbidContract(address)` and selector `[82, 67, 142, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "forbidContract", abi = "forbidContract(address)")]
    pub struct ForbidContractCall {
        pub target_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setLiquidationThreshold`function with signature `setLiquidationThreshold(address,uint256)` and selector `[14, 48, 66, 141]`"]
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
        name = "setLiquidationThreshold",
        abi = "setLiquidationThreshold(address,uint256)"
    )]
    pub struct SetLiquidationThresholdCall {
        pub token: ethers::core::types::Address,
        pub liquidation_threshold: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditConfiguratorCalls {
        AddTokenToAllowedList(AddTokenToAllowedListCall),
        AllowContract(AllowContractCall),
        AllowedContracts(AllowedContractsCall),
        AllowedContractsCount(AllowedContractsCountCall),
        ForbidContract(ForbidContractCall),
        SetLiquidationThreshold(SetLiquidationThresholdCall),
    }
    impl ethers::core::abi::AbiDecode for ICreditConfiguratorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddTokenToAllowedListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditConfiguratorCalls::AddTokenToAllowedList(decoded));
            }
            if let Ok(decoded) =
                <AllowContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditConfiguratorCalls::AllowContract(decoded));
            }
            if let Ok(decoded) =
                <AllowedContractsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditConfiguratorCalls::AllowedContracts(decoded));
            }
            if let Ok(decoded) =
                <AllowedContractsCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditConfiguratorCalls::AllowedContractsCount(decoded));
            }
            if let Ok(decoded) =
                <ForbidContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditConfiguratorCalls::ForbidContract(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidationThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditConfiguratorCalls::SetLiquidationThreshold(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICreditConfiguratorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICreditConfiguratorCalls::AddTokenToAllowedList(element) => element.encode(),
                ICreditConfiguratorCalls::AllowContract(element) => element.encode(),
                ICreditConfiguratorCalls::AllowedContracts(element) => element.encode(),
                ICreditConfiguratorCalls::AllowedContractsCount(element) => element.encode(),
                ICreditConfiguratorCalls::ForbidContract(element) => element.encode(),
                ICreditConfiguratorCalls::SetLiquidationThreshold(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICreditConfiguratorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditConfiguratorCalls::AddTokenToAllowedList(element) => element.fmt(f),
                ICreditConfiguratorCalls::AllowContract(element) => element.fmt(f),
                ICreditConfiguratorCalls::AllowedContracts(element) => element.fmt(f),
                ICreditConfiguratorCalls::AllowedContractsCount(element) => element.fmt(f),
                ICreditConfiguratorCalls::ForbidContract(element) => element.fmt(f),
                ICreditConfiguratorCalls::SetLiquidationThreshold(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddTokenToAllowedListCall> for ICreditConfiguratorCalls {
        fn from(var: AddTokenToAllowedListCall) -> Self {
            ICreditConfiguratorCalls::AddTokenToAllowedList(var)
        }
    }
    impl ::std::convert::From<AllowContractCall> for ICreditConfiguratorCalls {
        fn from(var: AllowContractCall) -> Self {
            ICreditConfiguratorCalls::AllowContract(var)
        }
    }
    impl ::std::convert::From<AllowedContractsCall> for ICreditConfiguratorCalls {
        fn from(var: AllowedContractsCall) -> Self {
            ICreditConfiguratorCalls::AllowedContracts(var)
        }
    }
    impl ::std::convert::From<AllowedContractsCountCall> for ICreditConfiguratorCalls {
        fn from(var: AllowedContractsCountCall) -> Self {
            ICreditConfiguratorCalls::AllowedContractsCount(var)
        }
    }
    impl ::std::convert::From<ForbidContractCall> for ICreditConfiguratorCalls {
        fn from(var: ForbidContractCall) -> Self {
            ICreditConfiguratorCalls::ForbidContract(var)
        }
    }
    impl ::std::convert::From<SetLiquidationThresholdCall> for ICreditConfiguratorCalls {
        fn from(var: SetLiquidationThresholdCall) -> Self {
            ICreditConfiguratorCalls::SetLiquidationThreshold(var)
        }
    }
}
