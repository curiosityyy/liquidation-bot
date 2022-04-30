pub use contractupgrader_mod::*;
#[allow(clippy::too_many_arguments)]
mod contractupgrader_mod {
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
    #[doc = "ContractUpgrader was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONTRACTUPGRADER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"RootSelfDestoyException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"addressProvider\",\"outputs\":[{\"internalType\":\"contract AddressProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configure\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"destoy\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getRootBack\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"root\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONTRACTUPGRADER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ContractUpgrader<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ContractUpgrader<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ContractUpgrader<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ContractUpgrader))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ContractUpgrader<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CONTRACTUPGRADER_ABI.clone(), client)
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
                CONTRACTUPGRADER_ABI.clone(),
                CONTRACTUPGRADER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addressProvider` (0x2954018c) function"]
        pub fn address_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 84, 1, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configure` (0x3e0b1a23) function"]
        pub fn configure(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 11, 26, 35], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `destoy` (0x805abe6a) function"]
        pub fn destoy(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 90, 190, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRootBack` (0x318d0d10) function"]
        pub fn get_root_back(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 141, 13, 16], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `root` (0xebf0c717) function"]
        pub fn root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([235, 240, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ContractUpgrader<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addressProvider`function with signature `addressProvider()` and selector `[41, 84, 1, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addressProvider", abi = "addressProvider()")]
    pub struct AddressProviderCall;
    #[doc = "Container type for all input parameters for the `configure`function with signature `configure()` and selector `[62, 11, 26, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "configure", abi = "configure()")]
    pub struct ConfigureCall;
    #[doc = "Container type for all input parameters for the `destoy`function with signature `destoy()` and selector `[128, 90, 190, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "destoy", abi = "destoy()")]
    pub struct DestoyCall;
    #[doc = "Container type for all input parameters for the `getRootBack`function with signature `getRootBack()` and selector `[49, 141, 13, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRootBack", abi = "getRootBack()")]
    pub struct GetRootBackCall;
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `root`function with signature `root()` and selector `[235, 240, 199, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "root", abi = "root()")]
    pub struct RootCall;
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ContractUpgraderCalls {
        AddressProvider(AddressProviderCall),
        Configure(ConfigureCall),
        Destoy(DestoyCall),
        GetRootBack(GetRootBackCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        Root(RootCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for ContractUpgraderCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::AddressProvider(decoded));
            }
            if let Ok(decoded) =
                <ConfigureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::Configure(decoded));
            }
            if let Ok(decoded) = <DestoyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::Destoy(decoded));
            }
            if let Ok(decoded) =
                <GetRootBackCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::GetRootBack(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ContractUpgraderCalls::Root(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractUpgraderCalls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ContractUpgraderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ContractUpgraderCalls::AddressProvider(element) => element.encode(),
                ContractUpgraderCalls::Configure(element) => element.encode(),
                ContractUpgraderCalls::Destoy(element) => element.encode(),
                ContractUpgraderCalls::GetRootBack(element) => element.encode(),
                ContractUpgraderCalls::Owner(element) => element.encode(),
                ContractUpgraderCalls::RenounceOwnership(element) => element.encode(),
                ContractUpgraderCalls::Root(element) => element.encode(),
                ContractUpgraderCalls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ContractUpgraderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ContractUpgraderCalls::AddressProvider(element) => element.fmt(f),
                ContractUpgraderCalls::Configure(element) => element.fmt(f),
                ContractUpgraderCalls::Destoy(element) => element.fmt(f),
                ContractUpgraderCalls::GetRootBack(element) => element.fmt(f),
                ContractUpgraderCalls::Owner(element) => element.fmt(f),
                ContractUpgraderCalls::RenounceOwnership(element) => element.fmt(f),
                ContractUpgraderCalls::Root(element) => element.fmt(f),
                ContractUpgraderCalls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressProviderCall> for ContractUpgraderCalls {
        fn from(var: AddressProviderCall) -> Self {
            ContractUpgraderCalls::AddressProvider(var)
        }
    }
    impl ::std::convert::From<ConfigureCall> for ContractUpgraderCalls {
        fn from(var: ConfigureCall) -> Self {
            ContractUpgraderCalls::Configure(var)
        }
    }
    impl ::std::convert::From<DestoyCall> for ContractUpgraderCalls {
        fn from(var: DestoyCall) -> Self {
            ContractUpgraderCalls::Destoy(var)
        }
    }
    impl ::std::convert::From<GetRootBackCall> for ContractUpgraderCalls {
        fn from(var: GetRootBackCall) -> Self {
            ContractUpgraderCalls::GetRootBack(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ContractUpgraderCalls {
        fn from(var: OwnerCall) -> Self {
            ContractUpgraderCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ContractUpgraderCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ContractUpgraderCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RootCall> for ContractUpgraderCalls {
        fn from(var: RootCall) -> Self {
            ContractUpgraderCalls::Root(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ContractUpgraderCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ContractUpgraderCalls::TransferOwnership(var)
        }
    }
}
