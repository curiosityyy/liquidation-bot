pub use curvelp3pricefeed_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvelp3pricefeed_mod {
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
    #[doc = "CurveLP3PriceFeed was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVELP3PRICEFEED_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_curvePool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed1\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed2\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed3\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_description\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"curvePool\",\"outputs\":[{\"internalType\":\"contract ICurvePool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed1\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed2\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed3\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVELP3PRICEFEED_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101006040523480156200001257600080fd5b5060405162000aee38038062000aee8339810160408190526200003591620001b1565b6001600160a01b03851615806200005357506001600160a01b038416155b806200006657506001600160a01b038316155b806200007957506001600160a01b038216155b156200009857604051635919af9760e11b815260040160405180910390fd5b6001600160a01b0380861660805284811660a05283811660c052821660e0528051620000cc906000906020840190620000d8565b50505050505062000318565b828054620000e690620002db565b90600052602060002090601f0160209004810192826200010a576000855562000155565b82601f106200012557805160ff191683800117855562000155565b8280016001018555821562000155579182015b828111156200015557825182559160200191906001019062000138565b506200016392915062000167565b5090565b5b8082111562000163576000815560010162000168565b80516001600160a01b03811681146200019657600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b600080600080600060a08688031215620001ca57600080fd5b620001d5866200017e565b94506020620001e68188016200017e565b9450620001f6604088016200017e565b935062000206606088016200017e565b60808801519093506001600160401b03808211156200022457600080fd5b818901915089601f8301126200023957600080fd5b8151818111156200024e576200024e6200019b565b604051601f8201601f19908116603f011681019083821181831017156200027957620002796200019b565b816040528281528c868487010111156200029257600080fd5b600093505b82841015620002b6578484018601518185018701529285019262000297565b82841115620002c85760008684830101525b8096505050505050509295509295909350565b600181811c90821680620002f057607f821691505b602082108114156200031257634e487b7160e01b600052602260045260246000fd5b50919050565b60805160a05160c05160e0516107826200036c6000396000818160fb01526103e40152600081816101be015261033e01526000818161019701526102a8015260008181609d015261049301526107826000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c80637284e416116100665780637284e416146101335780639a6fc8f514610148578063ab0ca0e114610192578063e5693f41146101b9578063feaf968c146101e057600080fd5b8063218751b214610098578063313ce567146100dc578063427cb6fe146100f657806354fd4d501461011d575b600080fd5b6100bf7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100e4600881565b60405160ff90911681526020016100d3565b6100bf7f000000000000000000000000000000000000000000000000000000000000000081565b610125600181565b6040519081526020016100d3565b61013b6101e8565b6040516100d39190610535565b61015b6101563660046105a5565b610276565b6040805169ffffffffffffffffffff968716815260208101959095528401929092526060830152909116608082015260a0016100d3565b6100bf7f000000000000000000000000000000000000000000000000000000000000000081565b6100bf7f000000000000000000000000000000000000000000000000000000000000000081565b61015b610297565b600080546101f5906105c9565b80601f0160208091040260200160405190810160405280929190818152602001828054610221906105c9565b801561026e5780601f106102435761010080835404028352916020019161026e565b820191906000526020600020905b81548152906001019060200180831161025157829003601f168201915b505050505081565b600080600080600060405163024e46f760e41b815260040160405180910390fd5b6000806000806000806000806000807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610304573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103289190610604565b809a50819b50829c50839d50849e5050505050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa15801561039a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103be9190610604565b939850919650945092509050888412156103e2578499508398508297508196508095505b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610440573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104649190610604565b93985091965094509250905088841215610488578499508398508297508196508095505b670de0b6b3a76400007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bb7b8b806040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104ef573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610513919061065c565b61051d908b61068b565b6105279190610710565b985050505050509091929394565b600060208083528351808285015260005b8181101561056257858101830151858201604001528201610546565b81811115610574576000604083870101525b50601f01601f1916929092016040019392505050565b69ffffffffffffffffffff811681146105a257600080fd5b50565b6000602082840312156105b757600080fd5b81356105c28161058a565b9392505050565b600181811c908216806105dd57607f821691505b602082108114156105fe57634e487b7160e01b600052602260045260246000fd5b50919050565b600080600080600060a0868803121561061c57600080fd5b85516106278161058a565b80955050602086015193506040860151925060608601519150608086015161064e8161058a565b809150509295509295909350565b60006020828403121561066e57600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b60006001600160ff1b03818413828413808216868404861116156106b1576106b1610675565b600160ff1b60008712828116878305891216156106d0576106d0610675565b600087129250878205871284841616156106ec576106ec610675565b8785058712818416161561070257610702610675565b505050929093029392505050565b60008261072d57634e487b7160e01b600052601260045260246000fd5b600160ff1b82146000198414161561074757610747610675565b50059056fea26469706673582212208f4308c32262b8f9ed02ec16fc105f40518ae747eae42328853a54218eb6ade464736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CurveLP3PriceFeed<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveLP3PriceFeed<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveLP3PriceFeed<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveLP3PriceFeed))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveLP3PriceFeed<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CURVELP3PRICEFEED_ABI.clone(), client)
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
                CURVELP3PRICEFEED_ABI.clone(),
                CURVELP3PRICEFEED_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `priceFeed3` (0x427cb6fe) function"]
        pub fn price_feed_3(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([66, 124, 182, 254], ())
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
        for CurveLP3PriceFeed<M>
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
    #[doc = "Container type for all input parameters for the `priceFeed3`function with signature `priceFeed3()` and selector `[66, 124, 182, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeed3", abi = "priceFeed3()")]
    pub struct PriceFeed3Call;
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
    pub enum CurveLP3PriceFeedCalls {
        CurvePool(CurvePoolCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        PriceFeed1(PriceFeed1Call),
        PriceFeed2(PriceFeed2Call),
        PriceFeed3(PriceFeed3Call),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for CurveLP3PriceFeedCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CurvePoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::CurvePool(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::PriceFeed1(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::PriceFeed2(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::PriceFeed3(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP3PriceFeedCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveLP3PriceFeedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveLP3PriceFeedCalls::CurvePool(element) => element.encode(),
                CurveLP3PriceFeedCalls::Decimals(element) => element.encode(),
                CurveLP3PriceFeedCalls::Description(element) => element.encode(),
                CurveLP3PriceFeedCalls::GetRoundData(element) => element.encode(),
                CurveLP3PriceFeedCalls::LatestRoundData(element) => element.encode(),
                CurveLP3PriceFeedCalls::PriceFeed1(element) => element.encode(),
                CurveLP3PriceFeedCalls::PriceFeed2(element) => element.encode(),
                CurveLP3PriceFeedCalls::PriceFeed3(element) => element.encode(),
                CurveLP3PriceFeedCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveLP3PriceFeedCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveLP3PriceFeedCalls::CurvePool(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::Decimals(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::Description(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::GetRoundData(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::LatestRoundData(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::PriceFeed1(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::PriceFeed2(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::PriceFeed3(element) => element.fmt(f),
                CurveLP3PriceFeedCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CurvePoolCall> for CurveLP3PriceFeedCalls {
        fn from(var: CurvePoolCall) -> Self {
            CurveLP3PriceFeedCalls::CurvePool(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CurveLP3PriceFeedCalls {
        fn from(var: DecimalsCall) -> Self {
            CurveLP3PriceFeedCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for CurveLP3PriceFeedCalls {
        fn from(var: DescriptionCall) -> Self {
            CurveLP3PriceFeedCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for CurveLP3PriceFeedCalls {
        fn from(var: GetRoundDataCall) -> Self {
            CurveLP3PriceFeedCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for CurveLP3PriceFeedCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            CurveLP3PriceFeedCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<PriceFeed1Call> for CurveLP3PriceFeedCalls {
        fn from(var: PriceFeed1Call) -> Self {
            CurveLP3PriceFeedCalls::PriceFeed1(var)
        }
    }
    impl ::std::convert::From<PriceFeed2Call> for CurveLP3PriceFeedCalls {
        fn from(var: PriceFeed2Call) -> Self {
            CurveLP3PriceFeedCalls::PriceFeed2(var)
        }
    }
    impl ::std::convert::From<PriceFeed3Call> for CurveLP3PriceFeedCalls {
        fn from(var: PriceFeed3Call) -> Self {
            CurveLP3PriceFeedCalls::PriceFeed3(var)
        }
    }
    impl ::std::convert::From<VersionCall> for CurveLP3PriceFeedCalls {
        fn from(var: VersionCall) -> Self {
            CurveLP3PriceFeedCalls::Version(var)
        }
    }
}
