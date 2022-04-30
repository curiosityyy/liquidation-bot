pub use curvelp4pricefeed_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvelp4pricefeed_mod {
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
    #[doc = "CurveLP4PriceFeed was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVELP4PRICEFEED_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_curvePool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed1\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed2\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed3\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed4\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_description\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"curvePool\",\"outputs\":[{\"internalType\":\"contract ICurvePool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed1\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed2\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed3\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed4\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVELP4PRICEFEED_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101206040523480156200001257600080fd5b5060405162000c0538038062000c058339810160408190526200003591620001cc565b6001600160a01b03861615806200005357506001600160a01b038516155b806200006657506001600160a01b038416155b806200007957506001600160a01b038316155b806200008c57506001600160a01b038216155b15620000ab57604051635919af9760e11b815260040160405180910390fd5b6001600160a01b0380871660805285811660a05284811660c05283811660e0528216610100528051620000e6906000906020840190620000f3565b5050505050505062000344565b828054620001019062000307565b90600052602060002090601f01602090048101928262000125576000855562000170565b82601f106200014057805160ff191683800117855562000170565b8280016001018555821562000170579182015b828111156200017057825182559160200191906001019062000153565b506200017e92915062000182565b5090565b5b808211156200017e576000815560010162000183565b80516001600160a01b0381168114620001b157600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b60008060008060008060c08789031215620001e657600080fd5b620001f18762000199565b955060206200020281890162000199565b9550620002126040890162000199565b9450620002226060890162000199565b9350620002326080890162000199565b60a08901519093506001600160401b03808211156200025057600080fd5b818a0191508a601f8301126200026557600080fd5b8151818111156200027a576200027a620001b6565b604051601f8201601f19908116603f01168101908382118183101715620002a557620002a5620001b6565b816040528281528d86848701011115620002be57600080fd5b600093505b82841015620002e25784840186015181850187015292850192620002c3565b82841115620002f45760008684830101525b8096505050505050509295509295509295565b600181811c908216806200031c57607f821691505b602082108114156200033e57634e487b7160e01b600052602260045260246000fd5b50919050565b60805160a05160c05160e0516101005161085a620003ab6000396000818161015801526104bc01526000818161010601526104160152600081816101f001526103700152600081816101c901526102da01526000818160a8015261056b015261085a6000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c80637aac1c48116100665780637aac1c48146101535780639a6fc8f51461017a578063ab0ca0e1146101c4578063e5693f41146101eb578063feaf968c1461021257600080fd5b8063218751b2146100a3578063313ce567146100e7578063427cb6fe1461010157806354fd4d50146101285780637284e4161461013e575b600080fd5b6100ca7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100ef600881565b60405160ff90911681526020016100de565b6100ca7f000000000000000000000000000000000000000000000000000000000000000081565b610130600181565b6040519081526020016100de565b61014661021a565b6040516100de919061060d565b6100ca7f000000000000000000000000000000000000000000000000000000000000000081565b61018d61018836600461067d565b6102a8565b6040805169ffffffffffffffffffff968716815260208101959095528401929092526060830152909116608082015260a0016100de565b6100ca7f000000000000000000000000000000000000000000000000000000000000000081565b6100ca7f000000000000000000000000000000000000000000000000000000000000000081565b61018d6102c9565b60008054610227906106a1565b80601f0160208091040260200160405190810160405280929190818152602001828054610253906106a1565b80156102a05780601f10610275576101008083540402835291602001916102a0565b820191906000526020600020905b81548152906001019060200180831161028357829003601f168201915b505050505081565b600080600080600060405163024e46f760e41b815260040160405180910390fd5b6000806000806000806000806000807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610336573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061035a91906106dc565b809a50819b50829c50839d50849e5050505050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa1580156103cc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103f091906106dc565b93985091965094509250905088841215610414578499508398508297508196508095505b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610472573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061049691906106dc565b939850919650945092509050888412156104ba578499508398508297508196508095505b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa158015610518573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061053c91906106dc565b93985091965094509250905088841215610560578499508398508297508196508095505b670de0b6b3a76400007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bb7b8b806040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105eb9190610734565b6105f5908b610763565b6105ff91906107e8565b985050505050509091929394565b600060208083528351808285015260005b8181101561063a5785810183015185820160400152820161061e565b8181111561064c576000604083870101525b50601f01601f1916929092016040019392505050565b69ffffffffffffffffffff8116811461067a57600080fd5b50565b60006020828403121561068f57600080fd5b813561069a81610662565b9392505050565b600181811c908216806106b557607f821691505b602082108114156106d657634e487b7160e01b600052602260045260246000fd5b50919050565b600080600080600060a086880312156106f457600080fd5b85516106ff81610662565b80955050602086015193506040860151925060608601519150608086015161072681610662565b809150509295509295909350565b60006020828403121561074657600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b60006001600160ff1b03818413828413808216868404861116156107895761078961074d565b600160ff1b60008712828116878305891216156107a8576107a861074d565b600087129250878205871284841616156107c4576107c461074d565b878505871281841616156107da576107da61074d565b505050929093029392505050565b60008261080557634e487b7160e01b600052601260045260246000fd5b600160ff1b82146000198414161561081f5761081f61074d565b50059056fea26469706673582212209ba5b3f55941af3099bcabbf87dd46f1d03fb8883545cebbe91c734dc3ea774064736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CurveLP4PriceFeed<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveLP4PriceFeed<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveLP4PriceFeed<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveLP4PriceFeed))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveLP4PriceFeed<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CURVELP4PRICEFEED_ABI.clone(), client)
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
                CURVELP4PRICEFEED_ABI.clone(),
                CURVELP4PRICEFEED_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `priceFeed4` (0x7aac1c48) function"]
        pub fn price_feed_4(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([122, 172, 28, 72], ())
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
        for CurveLP4PriceFeed<M>
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
    #[doc = "Container type for all input parameters for the `priceFeed4`function with signature `priceFeed4()` and selector `[122, 172, 28, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeed4", abi = "priceFeed4()")]
    pub struct PriceFeed4Call;
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
    pub enum CurveLP4PriceFeedCalls {
        CurvePool(CurvePoolCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        PriceFeed1(PriceFeed1Call),
        PriceFeed2(PriceFeed2Call),
        PriceFeed3(PriceFeed3Call),
        PriceFeed4(PriceFeed4Call),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for CurveLP4PriceFeedCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CurvePoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::CurvePool(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::PriceFeed1(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::PriceFeed2(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::PriceFeed3(decoded));
            }
            if let Ok(decoded) =
                <PriceFeed4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::PriceFeed4(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveLP4PriceFeedCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveLP4PriceFeedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveLP4PriceFeedCalls::CurvePool(element) => element.encode(),
                CurveLP4PriceFeedCalls::Decimals(element) => element.encode(),
                CurveLP4PriceFeedCalls::Description(element) => element.encode(),
                CurveLP4PriceFeedCalls::GetRoundData(element) => element.encode(),
                CurveLP4PriceFeedCalls::LatestRoundData(element) => element.encode(),
                CurveLP4PriceFeedCalls::PriceFeed1(element) => element.encode(),
                CurveLP4PriceFeedCalls::PriceFeed2(element) => element.encode(),
                CurveLP4PriceFeedCalls::PriceFeed3(element) => element.encode(),
                CurveLP4PriceFeedCalls::PriceFeed4(element) => element.encode(),
                CurveLP4PriceFeedCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveLP4PriceFeedCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveLP4PriceFeedCalls::CurvePool(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::Decimals(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::Description(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::GetRoundData(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::LatestRoundData(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::PriceFeed1(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::PriceFeed2(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::PriceFeed3(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::PriceFeed4(element) => element.fmt(f),
                CurveLP4PriceFeedCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CurvePoolCall> for CurveLP4PriceFeedCalls {
        fn from(var: CurvePoolCall) -> Self {
            CurveLP4PriceFeedCalls::CurvePool(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CurveLP4PriceFeedCalls {
        fn from(var: DecimalsCall) -> Self {
            CurveLP4PriceFeedCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for CurveLP4PriceFeedCalls {
        fn from(var: DescriptionCall) -> Self {
            CurveLP4PriceFeedCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for CurveLP4PriceFeedCalls {
        fn from(var: GetRoundDataCall) -> Self {
            CurveLP4PriceFeedCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for CurveLP4PriceFeedCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            CurveLP4PriceFeedCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<PriceFeed1Call> for CurveLP4PriceFeedCalls {
        fn from(var: PriceFeed1Call) -> Self {
            CurveLP4PriceFeedCalls::PriceFeed1(var)
        }
    }
    impl ::std::convert::From<PriceFeed2Call> for CurveLP4PriceFeedCalls {
        fn from(var: PriceFeed2Call) -> Self {
            CurveLP4PriceFeedCalls::PriceFeed2(var)
        }
    }
    impl ::std::convert::From<PriceFeed3Call> for CurveLP4PriceFeedCalls {
        fn from(var: PriceFeed3Call) -> Self {
            CurveLP4PriceFeedCalls::PriceFeed3(var)
        }
    }
    impl ::std::convert::From<PriceFeed4Call> for CurveLP4PriceFeedCalls {
        fn from(var: PriceFeed4Call) -> Self {
            CurveLP4PriceFeedCalls::PriceFeed4(var)
        }
    }
    impl ::std::convert::From<VersionCall> for CurveLP4PriceFeedCalls {
        fn from(var: VersionCall) -> Self {
            CurveLP4PriceFeedCalls::Version(var)
        }
    }
}
