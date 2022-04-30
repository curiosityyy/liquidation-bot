pub use ipriceoracle_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipriceoracle_mod {
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
    #[doc = "IPriceOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPRICEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"PriceFeedDecimalsNotEqual8Exception\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PriceOracleNotExistsException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TokenDecimalsGreater18ForbiddenException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroPriceException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"priceFeed\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewPriceFeed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenFrom\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenTo\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convert\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertFromUSD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToUSD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountFrom\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenFrom\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTo\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenTo\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fastCheck\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralFrom\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeeds\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IPRICEORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IPriceOracle<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IPriceOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPriceOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IPriceOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPRICEORACLE_ABI.clone(), client).into()
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
                IPRICEORACLE_ABI.clone(),
                IPRICEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `convert` (0xb66102df) function"]
        pub fn convert(
            &self,
            amount: ethers::core::types::U256,
            token_from: ethers::core::types::Address,
            token_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 97, 2, 223], (amount, token_from, token_to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertFromUSD` (0x7afb0104) function"]
        pub fn convert_from_usd(
            &self,
            amount: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 251, 1, 4], (amount, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertToUSD` (0xf9a65030) function"]
        pub fn convert_to_usd(
            &self,
            amount: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([249, 166, 80, 48], (amount, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fastCheck` (0x5cecbd0e) function"]
        pub fn fast_check(
            &self,
            amount_from: ethers::core::types::U256,
            token_from: ethers::core::types::Address,
            amount_to: ethers::core::types::U256,
            token_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [92, 236, 189, 14],
                    (amount_from, token_from, amount_to, token_to),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceFeeds` (0x9dcb511a) function"]
        pub fn price_feeds(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([157, 203, 81, 26], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewPriceFeed` event"]
        pub fn new_price_feed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPriceFeedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewPriceFeedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPriceOracle<M> {
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
    #[ethevent(name = "NewPriceFeed", abi = "NewPriceFeed(address,address)")]
    pub struct NewPriceFeedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub price_feed: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convert`function with signature `convert(uint256,address,address)` and selector `[182, 97, 2, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "convert", abi = "convert(uint256,address,address)")]
    pub struct ConvertCall {
        pub amount: ethers::core::types::U256,
        pub token_from: ethers::core::types::Address,
        pub token_to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convertFromUSD`function with signature `convertFromUSD(uint256,address)` and selector `[122, 251, 1, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "convertFromUSD", abi = "convertFromUSD(uint256,address)")]
    pub struct ConvertFromUSDCall {
        pub amount: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convertToUSD`function with signature `convertToUSD(uint256,address)` and selector `[249, 166, 80, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "convertToUSD", abi = "convertToUSD(uint256,address)")]
    pub struct ConvertToUSDCall {
        pub amount: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `fastCheck`function with signature `fastCheck(uint256,address,uint256,address)` and selector `[92, 236, 189, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fastCheck", abi = "fastCheck(uint256,address,uint256,address)")]
    pub struct FastCheckCall {
        pub amount_from: ethers::core::types::U256,
        pub token_from: ethers::core::types::Address,
        pub amount_to: ethers::core::types::U256,
        pub token_to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `priceFeeds`function with signature `priceFeeds(address)` and selector `[157, 203, 81, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeeds", abi = "priceFeeds(address)")]
    pub struct PriceFeedsCall {
        pub token: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPriceOracleCalls {
        Convert(ConvertCall),
        ConvertFromUSD(ConvertFromUSDCall),
        ConvertToUSD(ConvertToUSDCall),
        FastCheck(FastCheckCall),
        PriceFeeds(PriceFeedsCall),
    }
    impl ethers::core::abi::AbiDecode for IPriceOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ConvertCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleCalls::Convert(decoded));
            }
            if let Ok(decoded) =
                <ConvertFromUSDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleCalls::ConvertFromUSD(decoded));
            }
            if let Ok(decoded) =
                <ConvertToUSDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleCalls::ConvertToUSD(decoded));
            }
            if let Ok(decoded) =
                <FastCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleCalls::FastCheck(decoded));
            }
            if let Ok(decoded) =
                <PriceFeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPriceOracleCalls::PriceFeeds(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPriceOracleCalls::Convert(element) => element.encode(),
                IPriceOracleCalls::ConvertFromUSD(element) => element.encode(),
                IPriceOracleCalls::ConvertToUSD(element) => element.encode(),
                IPriceOracleCalls::FastCheck(element) => element.encode(),
                IPriceOracleCalls::PriceFeeds(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPriceOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPriceOracleCalls::Convert(element) => element.fmt(f),
                IPriceOracleCalls::ConvertFromUSD(element) => element.fmt(f),
                IPriceOracleCalls::ConvertToUSD(element) => element.fmt(f),
                IPriceOracleCalls::FastCheck(element) => element.fmt(f),
                IPriceOracleCalls::PriceFeeds(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ConvertCall> for IPriceOracleCalls {
        fn from(var: ConvertCall) -> Self {
            IPriceOracleCalls::Convert(var)
        }
    }
    impl ::std::convert::From<ConvertFromUSDCall> for IPriceOracleCalls {
        fn from(var: ConvertFromUSDCall) -> Self {
            IPriceOracleCalls::ConvertFromUSD(var)
        }
    }
    impl ::std::convert::From<ConvertToUSDCall> for IPriceOracleCalls {
        fn from(var: ConvertToUSDCall) -> Self {
            IPriceOracleCalls::ConvertToUSD(var)
        }
    }
    impl ::std::convert::From<FastCheckCall> for IPriceOracleCalls {
        fn from(var: FastCheckCall) -> Self {
            IPriceOracleCalls::FastCheck(var)
        }
    }
    impl ::std::convert::From<PriceFeedsCall> for IPriceOracleCalls {
        fn from(var: PriceFeedsCall) -> Self {
            IPriceOracleCalls::PriceFeeds(var)
        }
    }
}
