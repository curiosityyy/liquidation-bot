pub use iaddressprovider_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaddressprovider_mod {
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
    #[doc = "IAddressProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IADDRESSPROVIDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"service\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getACL\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getContractsRegister\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDataCompressor\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGearToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTreasuryContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getWETHGateway\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getWethToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IADDRESSPROVIDER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAddressProvider<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAddressProvider<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAddressProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAddressProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAddressProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IADDRESSPROVIDER_ABI.clone(), client)
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
                IADDRESSPROVIDER_ABI.clone(),
                IADDRESSPROVIDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getACL` (0x08737695) function"]
        pub fn get_acl(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 115, 118, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountFactory` (0x9068a868) function"]
        pub fn get_account_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([144, 104, 168, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getContractsRegister` (0xc513c9bb) function"]
        pub fn get_contracts_register(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([197, 19, 201, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDataCompressor` (0x060678c2) function"]
        pub fn get_data_compressor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([6, 6, 120, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getGearToken` (0xaffd9243) function"]
        pub fn get_gear_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([175, 253, 146, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracle` (0xfca513a8) function"]
        pub fn get_price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 165, 19, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTreasuryContract` (0x26c74fc3) function"]
        pub fn get_treasury_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 199, 79, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWETHGateway` (0x77532ed9) function"]
        pub fn get_weth_gateway(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([119, 83, 46, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWethToken` (0x4c252f91) function"]
        pub fn get_weth_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([76, 37, 47, 145], ())
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
        #[doc = "Gets the contract's `AddressSet` event"]
        pub fn address_set_filter(&self) -> ethers::contract::builders::Event<M, AddressSetFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AddressSetFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAddressProvider<M> {
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
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address)")]
    pub struct AddressSetFilter {
        #[ethevent(indexed)]
        pub service: [u8; 32],
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getACL`function with signature `getACL()` and selector `[8, 115, 118, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getACL", abi = "getACL()")]
    pub struct GetACLCall;
    #[doc = "Container type for all input parameters for the `getAccountFactory`function with signature `getAccountFactory()` and selector `[144, 104, 168, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAccountFactory", abi = "getAccountFactory()")]
    pub struct GetAccountFactoryCall;
    #[doc = "Container type for all input parameters for the `getContractsRegister`function with signature `getContractsRegister()` and selector `[197, 19, 201, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getContractsRegister", abi = "getContractsRegister()")]
    pub struct GetContractsRegisterCall;
    #[doc = "Container type for all input parameters for the `getDataCompressor`function with signature `getDataCompressor()` and selector `[6, 6, 120, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDataCompressor", abi = "getDataCompressor()")]
    pub struct GetDataCompressorCall;
    #[doc = "Container type for all input parameters for the `getGearToken`function with signature `getGearToken()` and selector `[175, 253, 146, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getGearToken", abi = "getGearToken()")]
    pub struct GetGearTokenCall;
    #[doc = "Container type for all input parameters for the `getPriceOracle`function with signature `getPriceOracle()` and selector `[252, 165, 19, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriceOracle", abi = "getPriceOracle()")]
    pub struct GetPriceOracleCall;
    #[doc = "Container type for all input parameters for the `getTreasuryContract`function with signature `getTreasuryContract()` and selector `[38, 199, 79, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTreasuryContract", abi = "getTreasuryContract()")]
    pub struct GetTreasuryContractCall;
    #[doc = "Container type for all input parameters for the `getWETHGateway`function with signature `getWETHGateway()` and selector `[119, 83, 46, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getWETHGateway", abi = "getWETHGateway()")]
    pub struct GetWETHGatewayCall;
    #[doc = "Container type for all input parameters for the `getWethToken`function with signature `getWethToken()` and selector `[76, 37, 47, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getWethToken", abi = "getWethToken()")]
    pub struct GetWethTokenCall;
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
    pub enum IAddressProviderCalls {
        GetACL(GetACLCall),
        GetAccountFactory(GetAccountFactoryCall),
        GetContractsRegister(GetContractsRegisterCall),
        GetDataCompressor(GetDataCompressorCall),
        GetGearToken(GetGearTokenCall),
        GetPriceOracle(GetPriceOracleCall),
        GetTreasuryContract(GetTreasuryContractCall),
        GetWETHGateway(GetWETHGatewayCall),
        GetWethToken(GetWethTokenCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for IAddressProviderCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <GetACLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetACL(decoded));
            }
            if let Ok(decoded) =
                <GetAccountFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetAccountFactory(decoded));
            }
            if let Ok(decoded) =
                <GetContractsRegisterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetContractsRegister(decoded));
            }
            if let Ok(decoded) =
                <GetDataCompressorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetDataCompressor(decoded));
            }
            if let Ok(decoded) =
                <GetGearTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetGearToken(decoded));
            }
            if let Ok(decoded) =
                <GetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <GetTreasuryContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetTreasuryContract(decoded));
            }
            if let Ok(decoded) =
                <GetWETHGatewayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetWETHGateway(decoded));
            }
            if let Ok(decoded) =
                <GetWethTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::GetWethToken(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAddressProviderCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAddressProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAddressProviderCalls::GetACL(element) => element.encode(),
                IAddressProviderCalls::GetAccountFactory(element) => element.encode(),
                IAddressProviderCalls::GetContractsRegister(element) => element.encode(),
                IAddressProviderCalls::GetDataCompressor(element) => element.encode(),
                IAddressProviderCalls::GetGearToken(element) => element.encode(),
                IAddressProviderCalls::GetPriceOracle(element) => element.encode(),
                IAddressProviderCalls::GetTreasuryContract(element) => element.encode(),
                IAddressProviderCalls::GetWETHGateway(element) => element.encode(),
                IAddressProviderCalls::GetWethToken(element) => element.encode(),
                IAddressProviderCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAddressProviderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAddressProviderCalls::GetACL(element) => element.fmt(f),
                IAddressProviderCalls::GetAccountFactory(element) => element.fmt(f),
                IAddressProviderCalls::GetContractsRegister(element) => element.fmt(f),
                IAddressProviderCalls::GetDataCompressor(element) => element.fmt(f),
                IAddressProviderCalls::GetGearToken(element) => element.fmt(f),
                IAddressProviderCalls::GetPriceOracle(element) => element.fmt(f),
                IAddressProviderCalls::GetTreasuryContract(element) => element.fmt(f),
                IAddressProviderCalls::GetWETHGateway(element) => element.fmt(f),
                IAddressProviderCalls::GetWethToken(element) => element.fmt(f),
                IAddressProviderCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetACLCall> for IAddressProviderCalls {
        fn from(var: GetACLCall) -> Self {
            IAddressProviderCalls::GetACL(var)
        }
    }
    impl ::std::convert::From<GetAccountFactoryCall> for IAddressProviderCalls {
        fn from(var: GetAccountFactoryCall) -> Self {
            IAddressProviderCalls::GetAccountFactory(var)
        }
    }
    impl ::std::convert::From<GetContractsRegisterCall> for IAddressProviderCalls {
        fn from(var: GetContractsRegisterCall) -> Self {
            IAddressProviderCalls::GetContractsRegister(var)
        }
    }
    impl ::std::convert::From<GetDataCompressorCall> for IAddressProviderCalls {
        fn from(var: GetDataCompressorCall) -> Self {
            IAddressProviderCalls::GetDataCompressor(var)
        }
    }
    impl ::std::convert::From<GetGearTokenCall> for IAddressProviderCalls {
        fn from(var: GetGearTokenCall) -> Self {
            IAddressProviderCalls::GetGearToken(var)
        }
    }
    impl ::std::convert::From<GetPriceOracleCall> for IAddressProviderCalls {
        fn from(var: GetPriceOracleCall) -> Self {
            IAddressProviderCalls::GetPriceOracle(var)
        }
    }
    impl ::std::convert::From<GetTreasuryContractCall> for IAddressProviderCalls {
        fn from(var: GetTreasuryContractCall) -> Self {
            IAddressProviderCalls::GetTreasuryContract(var)
        }
    }
    impl ::std::convert::From<GetWETHGatewayCall> for IAddressProviderCalls {
        fn from(var: GetWETHGatewayCall) -> Self {
            IAddressProviderCalls::GetWETHGateway(var)
        }
    }
    impl ::std::convert::From<GetWethTokenCall> for IAddressProviderCalls {
        fn from(var: GetWethTokenCall) -> Self {
            IAddressProviderCalls::GetWethToken(var)
        }
    }
    impl ::std::convert::From<VersionCall> for IAddressProviderCalls {
        fn from(var: VersionCall) -> Self {
            IAddressProviderCalls::Version(var)
        }
    }
}
