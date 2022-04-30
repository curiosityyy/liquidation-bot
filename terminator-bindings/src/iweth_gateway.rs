pub use iwethgateway_mod::*;
#[allow(clippy::too_many_arguments)]
mod iwethgateway_mod {
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
    #[doc = "IWETHGateway was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IWETHGATEWAY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"addLiquidityETH\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityETH\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapWETH\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IWETHGATEWAY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IWETHGateway<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IWETHGateway<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IWETHGateway<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IWETHGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IWETHGateway<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IWETHGATEWAY_ABI.clone(), client).into()
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
                IWETHGATEWAY_ABI.clone(),
                IWETHGATEWAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xdeecfbc9) function"]
        pub fn add_liquidity_eth(
            &self,
            pool: ethers::core::types::Address,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 236, 251, 201], (pool, on_behalf_of, referral_code))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0xe79a4089) function"]
        pub fn remove_liquidity_eth(
            &self,
            pool: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 154, 64, 137], (pool, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH` (0x5869dba8) function"]
        pub fn unwrap_weth(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 105, 219, 168], (to, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IWETHGateway<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addLiquidityETH`function with signature `addLiquidityETH(address,address,uint16)` and selector `[222, 236, 251, 201]`"]
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
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,address,uint16)"
    )]
    pub struct AddLiquidityETHCall {
        pub pool: ethers::core::types::Address,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETH`function with signature `removeLiquidityETH(address,uint256,address)` and selector `[231, 154, 64, 137]`"]
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
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,uint256,address)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub pool: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unwrapWETH`function with signature `unwrapWETH(address,uint256)` and selector `[88, 105, 219, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unwrapWETH", abi = "unwrapWETH(address,uint256)")]
    pub struct UnwrapWETHCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IWETHGatewayCalls {
        AddLiquidityETH(AddLiquidityETHCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        UnwrapWETH(UnwrapWETHCall),
    }
    impl ethers::core::abi::AbiDecode for IWETHGatewayCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHGatewayCalls::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHGatewayCalls::RemoveLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IWETHGatewayCalls::UnwrapWETH(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IWETHGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IWETHGatewayCalls::AddLiquidityETH(element) => element.encode(),
                IWETHGatewayCalls::RemoveLiquidityETH(element) => element.encode(),
                IWETHGatewayCalls::UnwrapWETH(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IWETHGatewayCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IWETHGatewayCalls::AddLiquidityETH(element) => element.fmt(f),
                IWETHGatewayCalls::RemoveLiquidityETH(element) => element.fmt(f),
                IWETHGatewayCalls::UnwrapWETH(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityETHCall> for IWETHGatewayCalls {
        fn from(var: AddLiquidityETHCall) -> Self {
            IWETHGatewayCalls::AddLiquidityETH(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHCall> for IWETHGatewayCalls {
        fn from(var: RemoveLiquidityETHCall) -> Self {
            IWETHGatewayCalls::RemoveLiquidityETH(var)
        }
    }
    impl ::std::convert::From<UnwrapWETHCall> for IWETHGatewayCalls {
        fn from(var: UnwrapWETHCall) -> Self {
            IWETHGatewayCalls::UnwrapWETH(var)
        }
    }
}
