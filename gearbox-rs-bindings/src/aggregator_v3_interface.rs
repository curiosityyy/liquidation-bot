pub use aggregatorv3interface_mod::*;
#[allow(clippy::too_many_arguments)]
mod aggregatorv3interface_mod {
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
    #[doc = "AggregatorV3Interface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AGGREGATORV3INTERFACE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static AGGREGATORV3INTERFACE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct AggregatorV3Interface<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AggregatorV3Interface<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AggregatorV3Interface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AggregatorV3Interface))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AggregatorV3Interface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                AGGREGATORV3INTERFACE_ABI.clone(),
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
                AGGREGATORV3INTERFACE_ABI.clone(),
                AGGREGATORV3INTERFACE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `description` (0x7284e416) function"]
        pub fn description(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundData` (0x9a6fc8f5) function"]
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundData` (0xfeaf968c) function"]
        pub fn latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for AggregatorV3Interface<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `description`function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    #[doc = "Container type for all input parameters for the `getRoundData`function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    #[doc = "Container type for all input parameters for the `latestRoundData`function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `version`function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AggregatorV3InterfaceCalls {
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for AggregatorV3InterfaceCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorV3InterfaceCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorV3InterfaceCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorV3InterfaceCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorV3InterfaceCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AggregatorV3InterfaceCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AggregatorV3InterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AggregatorV3InterfaceCalls::Decimals(element) => element.encode(),
                AggregatorV3InterfaceCalls::Description(element) => element.encode(),
                AggregatorV3InterfaceCalls::GetRoundData(element) => element.encode(),
                AggregatorV3InterfaceCalls::LatestRoundData(element) => element.encode(),
                AggregatorV3InterfaceCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AggregatorV3InterfaceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AggregatorV3InterfaceCalls::Decimals(element) => element.fmt(f),
                AggregatorV3InterfaceCalls::Description(element) => element.fmt(f),
                AggregatorV3InterfaceCalls::GetRoundData(element) => element.fmt(f),
                AggregatorV3InterfaceCalls::LatestRoundData(element) => element.fmt(f),
                AggregatorV3InterfaceCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DecimalsCall> for AggregatorV3InterfaceCalls {
        fn from(var: DecimalsCall) -> Self {
            AggregatorV3InterfaceCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for AggregatorV3InterfaceCalls {
        fn from(var: DescriptionCall) -> Self {
            AggregatorV3InterfaceCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for AggregatorV3InterfaceCalls {
        fn from(var: GetRoundDataCall) -> Self {
            AggregatorV3InterfaceCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for AggregatorV3InterfaceCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            AggregatorV3InterfaceCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<VersionCall> for AggregatorV3InterfaceCalls {
        fn from(var: VersionCall) -> Self {
            AggregatorV3InterfaceCalls::Version(var)
        }
    }
}