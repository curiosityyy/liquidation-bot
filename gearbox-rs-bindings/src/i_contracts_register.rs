pub use icontractsregister_mod::*;
#[allow(clippy::too_many_arguments)]
mod icontractsregister_mod {
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
    #[doc = "IContractsRegister was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICONTRACTSREGISTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewCreditManagerAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewPoolAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"creditManagers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditManagers\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditManagersCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPools\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolsCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCreditManager\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPool\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pools\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICONTRACTSREGISTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IContractsRegister<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IContractsRegister<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IContractsRegister<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IContractsRegister))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IContractsRegister<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICONTRACTSREGISTER_ABI.clone(), client)
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
                ICONTRACTSREGISTER_ABI.clone(),
                ICONTRACTSREGISTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `creditManagers` (0x1e16e4fc) function"]
        pub fn credit_managers(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([30, 22, 228, 252], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagers` (0x94144856) function"]
        pub fn get_credit_managers(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([148, 20, 72, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagersCount` (0xc29277cd) function"]
        pub fn get_credit_managers_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([194, 146, 119, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPools` (0x673a2a1f) function"]
        pub fn get_pools(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([103, 58, 42, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolsCount` (0xb4ac6860) function"]
        pub fn get_pools_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([180, 172, 104, 96], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCreditManager` (0x6fbc6f6b) function"]
        pub fn is_credit_manager(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([111, 188, 111, 107], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPool` (0x5b16ebb7) function"]
        pub fn is_pool(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([91, 22, 235, 183], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pools` (0xac4afa38) function"]
        pub fn pools(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([172, 74, 250, 56], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewCreditManagerAdded` event"]
        pub fn new_credit_manager_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCreditManagerAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPoolAdded` event"]
        pub fn new_pool_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPoolAddedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IContractsRegisterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IContractsRegister<M>
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
    #[ethevent(name = "NewCreditManagerAdded", abi = "NewCreditManagerAdded(address)")]
    pub struct NewCreditManagerAddedFilter {
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
    #[ethevent(name = "NewPoolAdded", abi = "NewPoolAdded(address)")]
    pub struct NewPoolAddedFilter {
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IContractsRegisterEvents {
        NewCreditManagerAddedFilter(NewCreditManagerAddedFilter),
        NewPoolAddedFilter(NewPoolAddedFilter),
    }
    impl ethers::contract::EthLogDecode for IContractsRegisterEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewCreditManagerAddedFilter::decode_log(log) {
                return Ok(IContractsRegisterEvents::NewCreditManagerAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPoolAddedFilter::decode_log(log) {
                return Ok(IContractsRegisterEvents::NewPoolAddedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IContractsRegisterEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IContractsRegisterEvents::NewCreditManagerAddedFilter(element) => element.fmt(f),
                IContractsRegisterEvents::NewPoolAddedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `creditManagers`function with signature `creditManagers(uint256)` and selector `[30, 22, 228, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManagers", abi = "creditManagers(uint256)")]
    pub struct CreditManagersCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getCreditManagers`function with signature `getCreditManagers()` and selector `[148, 20, 72, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCreditManagers", abi = "getCreditManagers()")]
    pub struct GetCreditManagersCall;
    #[doc = "Container type for all input parameters for the `getCreditManagersCount`function with signature `getCreditManagersCount()` and selector `[194, 146, 119, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCreditManagersCount", abi = "getCreditManagersCount()")]
    pub struct GetCreditManagersCountCall;
    #[doc = "Container type for all input parameters for the `getPools`function with signature `getPools()` and selector `[103, 58, 42, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPools", abi = "getPools()")]
    pub struct GetPoolsCall;
    #[doc = "Container type for all input parameters for the `getPoolsCount`function with signature `getPoolsCount()` and selector `[180, 172, 104, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolsCount", abi = "getPoolsCount()")]
    pub struct GetPoolsCountCall;
    #[doc = "Container type for all input parameters for the `isCreditManager`function with signature `isCreditManager(address)` and selector `[111, 188, 111, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isCreditManager", abi = "isCreditManager(address)")]
    pub struct IsCreditManagerCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `isPool`function with signature `isPool(address)` and selector `[91, 22, 235, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPool", abi = "isPool(address)")]
    pub struct IsPoolCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `pools`function with signature `pools(uint256)` and selector `[172, 74, 250, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall {
        pub i: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IContractsRegisterCalls {
        CreditManagers(CreditManagersCall),
        GetCreditManagers(GetCreditManagersCall),
        GetCreditManagersCount(GetCreditManagersCountCall),
        GetPools(GetPoolsCall),
        GetPoolsCount(GetPoolsCountCall),
        IsCreditManager(IsCreditManagerCall),
        IsPool(IsPoolCall),
        Pools(PoolsCall),
    }
    impl ethers::core::abi::AbiDecode for IContractsRegisterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreditManagersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::CreditManagers(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::GetCreditManagers(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::GetCreditManagersCount(decoded));
            }
            if let Ok(decoded) =
                <GetPoolsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::GetPools(decoded));
            }
            if let Ok(decoded) =
                <GetPoolsCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::GetPoolsCount(decoded));
            }
            if let Ok(decoded) =
                <IsCreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::IsCreditManager(decoded));
            }
            if let Ok(decoded) = <IsPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::IsPool(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IContractsRegisterCalls::Pools(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IContractsRegisterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IContractsRegisterCalls::CreditManagers(element) => element.encode(),
                IContractsRegisterCalls::GetCreditManagers(element) => element.encode(),
                IContractsRegisterCalls::GetCreditManagersCount(element) => element.encode(),
                IContractsRegisterCalls::GetPools(element) => element.encode(),
                IContractsRegisterCalls::GetPoolsCount(element) => element.encode(),
                IContractsRegisterCalls::IsCreditManager(element) => element.encode(),
                IContractsRegisterCalls::IsPool(element) => element.encode(),
                IContractsRegisterCalls::Pools(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IContractsRegisterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IContractsRegisterCalls::CreditManagers(element) => element.fmt(f),
                IContractsRegisterCalls::GetCreditManagers(element) => element.fmt(f),
                IContractsRegisterCalls::GetCreditManagersCount(element) => element.fmt(f),
                IContractsRegisterCalls::GetPools(element) => element.fmt(f),
                IContractsRegisterCalls::GetPoolsCount(element) => element.fmt(f),
                IContractsRegisterCalls::IsCreditManager(element) => element.fmt(f),
                IContractsRegisterCalls::IsPool(element) => element.fmt(f),
                IContractsRegisterCalls::Pools(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreditManagersCall> for IContractsRegisterCalls {
        fn from(var: CreditManagersCall) -> Self {
            IContractsRegisterCalls::CreditManagers(var)
        }
    }
    impl ::std::convert::From<GetCreditManagersCall> for IContractsRegisterCalls {
        fn from(var: GetCreditManagersCall) -> Self {
            IContractsRegisterCalls::GetCreditManagers(var)
        }
    }
    impl ::std::convert::From<GetCreditManagersCountCall> for IContractsRegisterCalls {
        fn from(var: GetCreditManagersCountCall) -> Self {
            IContractsRegisterCalls::GetCreditManagersCount(var)
        }
    }
    impl ::std::convert::From<GetPoolsCall> for IContractsRegisterCalls {
        fn from(var: GetPoolsCall) -> Self {
            IContractsRegisterCalls::GetPools(var)
        }
    }
    impl ::std::convert::From<GetPoolsCountCall> for IContractsRegisterCalls {
        fn from(var: GetPoolsCountCall) -> Self {
            IContractsRegisterCalls::GetPoolsCount(var)
        }
    }
    impl ::std::convert::From<IsCreditManagerCall> for IContractsRegisterCalls {
        fn from(var: IsCreditManagerCall) -> Self {
            IContractsRegisterCalls::IsCreditManager(var)
        }
    }
    impl ::std::convert::From<IsPoolCall> for IContractsRegisterCalls {
        fn from(var: IsPoolCall) -> Self {
            IContractsRegisterCalls::IsPool(var)
        }
    }
    impl ::std::convert::From<PoolsCall> for IContractsRegisterCalls {
        fn from(var: PoolsCall) -> Self {
            IContractsRegisterCalls::Pools(var)
        }
    }
}
