pub use curvelp2pricefeed_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvelp2pricefeed_mod {
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
    #[doc = "CurveLP2PriceFeed was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVELP2PRICEFEED_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_curvePool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed1\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed2\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_description\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"curvePool\",\"outputs\":[{\"internalType\":\"contract ICurvePool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed1\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed2\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVELP2PRICEFEED_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e06040523480156200001157600080fd5b50604051620009d5380380620009d5833981016040819052620000349162000196565b6001600160a01b03841615806200005257506001600160a01b038316155b806200006557506001600160a01b038216155b156200008457604051635919af9760e11b815260040160405180910390fd5b6001600160a01b0380851660805283811660a052821660c0528051620000b2906000906020840190620000bd565b5050505050620002e8565b828054620000cb90620002ab565b90600052602060002090601f016020900481019282620000ef57600085556200013a565b82601f106200010a57805160ff19168380011785556200013a565b828001600101855582156200013a579182015b828111156200013a5782518255916020019190600101906200011d565b50620001489291506200014c565b5090565b5b808211156200014857600081556001016200014d565b80516001600160a01b03811681146200017b57600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b60008060008060808587031215620001ad57600080fd5b620001b88562000163565b93506020620001c981870162000163565b9350620001d96040870162000163565b60608701519093506001600160401b0380821115620001f757600080fd5b818801915088601f8301126200020c57600080fd5b81518181111562000221576200022162000180565b604051601f8201601f19908116603f011681019083821181831017156200024c576200024c62000180565b816040528281528b868487010111156200026557600080fd5b600093505b828410156200028957848401860151818501870152928501926200026a565b828411156200029b5760008684830101525b989b979a50959850505050505050565b600181811c90821680620002c057607f821691505b60208210811415620002e257634e487b7160e01b600052602260045260246000fd5b50919050565b60805160a05160c0516106a96200032c6000396000818161018c015261030d015260008181610165015261026f015260008181609201526103ba01526106a96000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c80639a6fc8f51161005b5780639a6fc8f514610116578063ab0ca0e114610160578063e5693f4114610187578063feaf968c146101ae57600080fd5b8063218751b21461008d578063313ce567146100d157806354fd4d50146100eb5780637284e41614610101575b600080fd5b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100d9600881565b60405160ff90911681526020016100c8565b6100f3600181565b6040519081526020016100c8565b6101096101b6565b6040516100c8919061045c565b6101296101243660046104cc565b610244565b6040805169ffffffffffffffffffff968716815260208101959095528401929092526060830152909116608082015260a0016100c8565b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b610129610265565b600080546101c3906104f0565b80601f01602080910402602001604051908101604052809291908181526020018280546101ef906104f0565b801561023c5780601f106102115761010080835404028352916020019161023c565b820191906000526020600020905b81548152906001019060200180831161021f57829003601f168201915b505050505081565b600080600080600060405163024e46f760e41b815260040160405180910390fd5b60008060008060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa1580156102cb573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102ef919061052b565b809550819650829750839850849950505050505060008060008060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610369573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061038d919061052b565b94509450945094509450888412156103af578499508398508297508196508095505b670de0b6b3a76400007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bb7b8b806040518163ffffffff1660e01b8152600401602060405180830381865afa158015610416573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061043a9190610583565b610444908b6105b2565b61044e9190610637565b985050505050509091929394565b600060208083528351808285015260005b818110156104895785810183015185820160400152820161046d565b8181111561049b576000604083870101525b50601f01601f1916929092016040019392505050565b69ffffffffffffffffffff811681146104c957600080fd5b50565b6000602082840312156104de57600080fd5b81356104e9816104b1565b9392505050565b600181811c9082168061050457607f821691505b6020821081141561052557634e487b7160e01b600052602260045260246000fd5b50919050565b600080600080600060a0868803121561054357600080fd5b855161054e816104b1565b809550506020860151935060408601519250606086015191506080860151610575816104b1565b809150509295509295909350565b60006020828403121561059557600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b60006001600160ff1b03818413828413808216868404861116156105d8576105d861059c565b600160ff1b60008712828116878305891216156105f7576105f761059c565b600087129250878205871284841616156106135761061361059c565b878505871281841616156106295761062961059c565b505050929093029392505050565b60008261065457634e487b7160e01b600052601260045260246000fd5b600160ff1b82146000198414161561066e5761066e61059c565b50059056fea2646970667358221220cd8fbb09536b29799da14e202861056c79132bb298dc1a60c12373bde4df7fe664736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CurveLP2PriceFeed<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveLP2PriceFeed<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveLP2PriceFeed<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveLP2PriceFeed))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveLP2PriceFeed<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CURVELP2PRICEFEED_ABI.clone(), client)
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
                CURVELP2PRICEFEED_ABI.clone(),
                CURVELP2PRICEFEED_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `curvePool` (0x218751b2) function"]
        pub fn curve_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([33, 135, 81, 178], ())
                .expect("method not found (this should never happen)")
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
            p0: u128,
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
                .method_hash([154, 111, 200, 245], p0)
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
        #[doc = "Calls the contract's `priceFeed1` (0xab0ca0e1) function"]
        pub fn price_feed_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([171, 12, 160, 225], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceFeed2` (0xe5693f41) function"]
        pub fn price_feed_2(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([229, 105, 63, 65], ())
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
        for CurveLP2PriceFeed<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `curvePool`function with signature `curvePool()` and selector `[33, 135, 81, 178]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "curvePool", abi = "curvePool()")]
    pub struct CurvePoolCall;
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
    pub struct GetRoundDataCall(pub u128);
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
    #[doc = "Container type for all input parameters for the `priceFeed1`function with signature `priceFeed1()` and selector `[171, 12, 160, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeed1", abi = "priceFeed1()")]
    pub struct PriceFeed1Call;
    #[doc = "Container type for all input parameters for the `priceFeed2`function with signature `priceFeed2()` and selector `[229, 105, 63, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeed2", abi = "priceFeed2()")]
    pub struct PriceFeed2Call;
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
    pub enum CurveLP2PriceFeedCalls {
        CurvePool(CurvePoolCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        PriceFeed1(PriceFeed1Call),
        PriceFeed2(PriceFeed2Call),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for CurveLP2PriceFeedCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CurvePoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::CurvePool(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::PriceFeed1(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::PriceFeed2(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP2PriceFeedCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveLP2PriceFeedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveLP2PriceFeedCalls::CurvePool(element) => element.encode(),
                CurveLP2PriceFeedCalls::Decimals(element) => element.encode(),
                CurveLP2PriceFeedCalls::Description(element) => element.encode(),
                CurveLP2PriceFeedCalls::GetRoundData(element) => element.encode(),
                CurveLP2PriceFeedCalls::LatestRoundData(element) => element.encode(),
                CurveLP2PriceFeedCalls::PriceFeed1(element) => element.encode(),
                CurveLP2PriceFeedCalls::PriceFeed2(element) => element.encode(),
                CurveLP2PriceFeedCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveLP2PriceFeedCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveLP2PriceFeedCalls::CurvePool(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::Decimals(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::Description(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::GetRoundData(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::LatestRoundData(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::PriceFeed1(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::PriceFeed2(element) => element.fmt(f),
                CurveLP2PriceFeedCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CurvePoolCall> for CurveLP2PriceFeedCalls {
        fn from(var: CurvePoolCall) -> Self {
            CurveLP2PriceFeedCalls::CurvePool(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CurveLP2PriceFeedCalls {
        fn from(var: DecimalsCall) -> Self {
            CurveLP2PriceFeedCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for CurveLP2PriceFeedCalls {
        fn from(var: DescriptionCall) -> Self {
            CurveLP2PriceFeedCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for CurveLP2PriceFeedCalls {
        fn from(var: GetRoundDataCall) -> Self {
            CurveLP2PriceFeedCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for CurveLP2PriceFeedCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            CurveLP2PriceFeedCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<PriceFeed1Call> for CurveLP2PriceFeedCalls {
        fn from(var: PriceFeed1Call) -> Self {
            CurveLP2PriceFeedCalls::PriceFeed1(var)
        }
    }
    impl ::std::convert::From<PriceFeed2Call> for CurveLP2PriceFeedCalls {
        fn from(var: PriceFeed2Call) -> Self {
            CurveLP2PriceFeedCalls::PriceFeed2(var)
        }
    }
    impl ::std::convert::From<VersionCall> for CurveLP2PriceFeedCalls {
        fn from(var: VersionCall) -> Self {
            CurveLP2PriceFeedCalls::Version(var)
        }
    }
}
