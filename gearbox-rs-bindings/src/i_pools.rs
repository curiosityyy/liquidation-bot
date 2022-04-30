pub use ipools_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipools_mod {
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
    #[doc = "IPools was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOLS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_lptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_gauge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_stashVersion\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPool\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_lptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_gauge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_stashVersion\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceAddPool\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gaugeMap\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolInfo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_poolM\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPoolManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"shutdownPool\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IPOOLS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IPools<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IPools<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPools<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPools))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IPools<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOOLS_ABI.clone(), client).into()
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
                IPOOLS_ABI.clone(),
                IPOOLS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addPool` (0x7e29d6c2) function"]
        pub fn add_pool(
            &self,
            lptoken: ethers::core::types::Address,
            gauge: ethers::core::types::Address,
            stash_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([126, 41, 214, 194], (lptoken, gauge, stash_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forceAddPool` (0x5744887d) function"]
        pub fn force_add_pool(
            &self,
            lptoken: ethers::core::types::Address,
            gauge: ethers::core::types::Address,
            stash_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([87, 68, 136, 125], (lptoken, gauge, stash_version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gaugeMap` (0xcb0d5b52) function"]
        pub fn gauge_map(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([203, 13, 91, 82], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolInfo` (0x1526fe27) function"]
        pub fn pool_info(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                bool,
            ),
        > {
            self.0
                .method_hash([21, 38, 254, 39], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolLength` (0x081e3eda) function"]
        pub fn pool_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([8, 30, 62, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolManager` (0x7aef6715) function"]
        pub fn set_pool_manager(
            &self,
            pool_m: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 239, 103, 21], pool_m)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shutdownPool` (0x60cafe84) function"]
        pub fn shutdown_pool(
            &self,
            pid: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 202, 254, 132], pid)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPools<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addPool`function with signature `addPool(address,address,uint256)` and selector `[126, 41, 214, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addPool", abi = "addPool(address,address,uint256)")]
    pub struct AddPoolCall {
        pub lptoken: ethers::core::types::Address,
        pub gauge: ethers::core::types::Address,
        pub stash_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `forceAddPool`function with signature `forceAddPool(address,address,uint256)` and selector `[87, 68, 136, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "forceAddPool", abi = "forceAddPool(address,address,uint256)")]
    pub struct ForceAddPoolCall {
        pub lptoken: ethers::core::types::Address,
        pub gauge: ethers::core::types::Address,
        pub stash_version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `gaugeMap`function with signature `gaugeMap(address)` and selector `[203, 13, 91, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gaugeMap", abi = "gaugeMap(address)")]
    pub struct GaugeMapCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `poolInfo`function with signature `poolInfo(uint256)` and selector `[21, 38, 254, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolInfo", abi = "poolInfo(uint256)")]
    pub struct PoolInfoCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `poolLength`function with signature `poolLength()` and selector `[8, 30, 62, 218]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolLength", abi = "poolLength()")]
    pub struct PoolLengthCall;
    #[doc = "Container type for all input parameters for the `setPoolManager`function with signature `setPoolManager(address)` and selector `[122, 239, 103, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPoolManager", abi = "setPoolManager(address)")]
    pub struct SetPoolManagerCall {
        pub pool_m: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `shutdownPool`function with signature `shutdownPool(uint256)` and selector `[96, 202, 254, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "shutdownPool", abi = "shutdownPool(uint256)")]
    pub struct ShutdownPoolCall {
        pub pid: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolsCalls {
        AddPool(AddPoolCall),
        ForceAddPool(ForceAddPoolCall),
        GaugeMap(GaugeMapCall),
        PoolInfo(PoolInfoCall),
        PoolLength(PoolLengthCall),
        SetPoolManager(SetPoolManagerCall),
        ShutdownPool(ShutdownPoolCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::AddPool(decoded));
            }
            if let Ok(decoded) =
                <ForceAddPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::ForceAddPool(decoded));
            }
            if let Ok(decoded) =
                <GaugeMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::GaugeMap(decoded));
            }
            if let Ok(decoded) =
                <PoolInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::PoolInfo(decoded));
            }
            if let Ok(decoded) =
                <PoolLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::PoolLength(decoded));
            }
            if let Ok(decoded) =
                <SetPoolManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::SetPoolManager(decoded));
            }
            if let Ok(decoded) =
                <ShutdownPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolsCalls::ShutdownPool(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolsCalls::AddPool(element) => element.encode(),
                IPoolsCalls::ForceAddPool(element) => element.encode(),
                IPoolsCalls::GaugeMap(element) => element.encode(),
                IPoolsCalls::PoolInfo(element) => element.encode(),
                IPoolsCalls::PoolLength(element) => element.encode(),
                IPoolsCalls::SetPoolManager(element) => element.encode(),
                IPoolsCalls::ShutdownPool(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPoolsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolsCalls::AddPool(element) => element.fmt(f),
                IPoolsCalls::ForceAddPool(element) => element.fmt(f),
                IPoolsCalls::GaugeMap(element) => element.fmt(f),
                IPoolsCalls::PoolInfo(element) => element.fmt(f),
                IPoolsCalls::PoolLength(element) => element.fmt(f),
                IPoolsCalls::SetPoolManager(element) => element.fmt(f),
                IPoolsCalls::ShutdownPool(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddPoolCall> for IPoolsCalls {
        fn from(var: AddPoolCall) -> Self {
            IPoolsCalls::AddPool(var)
        }
    }
    impl ::std::convert::From<ForceAddPoolCall> for IPoolsCalls {
        fn from(var: ForceAddPoolCall) -> Self {
            IPoolsCalls::ForceAddPool(var)
        }
    }
    impl ::std::convert::From<GaugeMapCall> for IPoolsCalls {
        fn from(var: GaugeMapCall) -> Self {
            IPoolsCalls::GaugeMap(var)
        }
    }
    impl ::std::convert::From<PoolInfoCall> for IPoolsCalls {
        fn from(var: PoolInfoCall) -> Self {
            IPoolsCalls::PoolInfo(var)
        }
    }
    impl ::std::convert::From<PoolLengthCall> for IPoolsCalls {
        fn from(var: PoolLengthCall) -> Self {
            IPoolsCalls::PoolLength(var)
        }
    }
    impl ::std::convert::From<SetPoolManagerCall> for IPoolsCalls {
        fn from(var: SetPoolManagerCall) -> Self {
            IPoolsCalls::SetPoolManager(var)
        }
    }
    impl ::std::convert::From<ShutdownPoolCall> for IPoolsCalls {
        fn from(var: ShutdownPoolCall) -> Self {
            IPoolsCalls::ShutdownPool(var)
        }
    }
}
