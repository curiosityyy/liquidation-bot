pub use icurvepool_mod::*;
#[allow(clippy::too_many_arguments)]
mod icurvepool_mod {
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
    #[doc = "ICurvePool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICURVEPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"coins\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange_underlying\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy_underlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_virtual_price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_token_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_one_coin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICURVEPOOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICurvePool<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICurvePool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICurvePool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICurvePool))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICurvePool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICURVEPOOL_ABI.clone(), client).into()
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
                ICURVEPOOL_ABI.clone(),
                ICURVEPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `coins` (0xc6610657) function"]
        pub fn coins(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([198, 97, 6, 87], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchange` (0x3df02124) function"]
        pub fn exchange(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
            min_dy: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 240, 33, 36], (i, j, dx, min_dy))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchange_underlying` (0xa6417ed6) function"]
        pub fn exchange_underlying(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
            min_dy: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 65, 126, 214], (i, j, dx, min_dy))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_dy` (0x5e0d443f) function"]
        pub fn get_dy(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 13, 68, 63], (i, j, dx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_dy_underlying` (0x07211ef7) function"]
        pub fn get_dy_underlying(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 33, 30, 247], (i, j, dx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_virtual_price` (0xbb7b8b80) function"]
        pub fn get_virtual_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([187, 123, 139, 128], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove_liquidity_one_coin` (0x1a4d01d2) function"]
        pub fn remove_liquidity_one_coin(
            &self,
            token_amount: ethers::core::types::U256,
            i: i128,
            min_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 77, 1, 210], (token_amount, i, min_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICurvePool<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `coins`function with signature `coins(uint256)` and selector `[198, 97, 6, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "coins", abi = "coins(uint256)")]
    pub struct CoinsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `exchange`function with signature `exchange(int128,int128,uint256,uint256)` and selector `[61, 240, 33, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exchange", abi = "exchange(int128,int128,uint256,uint256)")]
    pub struct ExchangeCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
        pub min_dy: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exchange_underlying`function with signature `exchange_underlying(int128,int128,uint256,uint256)` and selector `[166, 65, 126, 214]`"]
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
        name = "exchange_underlying",
        abi = "exchange_underlying(int128,int128,uint256,uint256)"
    )]
    pub struct ExchangeUnderlyingCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
        pub min_dy: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `get_dy`function with signature `get_dy(int128,int128,uint256)` and selector `[94, 13, 68, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_dy", abi = "get_dy(int128,int128,uint256)")]
    pub struct GetDyCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `get_dy_underlying`function with signature `get_dy_underlying(int128,int128,uint256)` and selector `[7, 33, 30, 247]`"]
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
        name = "get_dy_underlying",
        abi = "get_dy_underlying(int128,int128,uint256)"
    )]
    pub struct GetDyUnderlyingCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `get_virtual_price`function with signature `get_virtual_price()` and selector `[187, 123, 139, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_virtual_price", abi = "get_virtual_price()")]
    pub struct GetVirtualPriceCall;
    #[doc = "Container type for all input parameters for the `remove_liquidity_one_coin`function with signature `remove_liquidity_one_coin(uint256,int128,uint256)` and selector `[26, 77, 1, 210]`"]
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
        name = "remove_liquidity_one_coin",
        abi = "remove_liquidity_one_coin(uint256,int128,uint256)"
    )]
    pub struct RemoveLiquidityOneCoinCall {
        pub token_amount: ethers::core::types::U256,
        pub i: i128,
        pub min_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICurvePoolCalls {
        Coins(CoinsCall),
        Exchange(ExchangeCall),
        ExchangeUnderlying(ExchangeUnderlyingCall),
        GetDy(GetDyCall),
        GetDyUnderlying(GetDyUnderlyingCall),
        GetVirtualPrice(GetVirtualPriceCall),
        RemoveLiquidityOneCoin(RemoveLiquidityOneCoinCall),
        Token(TokenCall),
    }
    impl ethers::core::abi::AbiDecode for ICurvePoolCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::Coins(decoded));
            }
            if let Ok(decoded) =
                <ExchangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::Exchange(decoded));
            }
            if let Ok(decoded) =
                <ExchangeUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::ExchangeUnderlying(decoded));
            }
            if let Ok(decoded) = <GetDyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::GetDy(decoded));
            }
            if let Ok(decoded) =
                <GetDyUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::GetDyUnderlying(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::GetVirtualPrice(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::RemoveLiquidityOneCoin(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolCalls::Token(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICurvePoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICurvePoolCalls::Coins(element) => element.encode(),
                ICurvePoolCalls::Exchange(element) => element.encode(),
                ICurvePoolCalls::ExchangeUnderlying(element) => element.encode(),
                ICurvePoolCalls::GetDy(element) => element.encode(),
                ICurvePoolCalls::GetDyUnderlying(element) => element.encode(),
                ICurvePoolCalls::GetVirtualPrice(element) => element.encode(),
                ICurvePoolCalls::RemoveLiquidityOneCoin(element) => element.encode(),
                ICurvePoolCalls::Token(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICurvePoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICurvePoolCalls::Coins(element) => element.fmt(f),
                ICurvePoolCalls::Exchange(element) => element.fmt(f),
                ICurvePoolCalls::ExchangeUnderlying(element) => element.fmt(f),
                ICurvePoolCalls::GetDy(element) => element.fmt(f),
                ICurvePoolCalls::GetDyUnderlying(element) => element.fmt(f),
                ICurvePoolCalls::GetVirtualPrice(element) => element.fmt(f),
                ICurvePoolCalls::RemoveLiquidityOneCoin(element) => element.fmt(f),
                ICurvePoolCalls::Token(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CoinsCall> for ICurvePoolCalls {
        fn from(var: CoinsCall) -> Self {
            ICurvePoolCalls::Coins(var)
        }
    }
    impl ::std::convert::From<ExchangeCall> for ICurvePoolCalls {
        fn from(var: ExchangeCall) -> Self {
            ICurvePoolCalls::Exchange(var)
        }
    }
    impl ::std::convert::From<ExchangeUnderlyingCall> for ICurvePoolCalls {
        fn from(var: ExchangeUnderlyingCall) -> Self {
            ICurvePoolCalls::ExchangeUnderlying(var)
        }
    }
    impl ::std::convert::From<GetDyCall> for ICurvePoolCalls {
        fn from(var: GetDyCall) -> Self {
            ICurvePoolCalls::GetDy(var)
        }
    }
    impl ::std::convert::From<GetDyUnderlyingCall> for ICurvePoolCalls {
        fn from(var: GetDyUnderlyingCall) -> Self {
            ICurvePoolCalls::GetDyUnderlying(var)
        }
    }
    impl ::std::convert::From<GetVirtualPriceCall> for ICurvePoolCalls {
        fn from(var: GetVirtualPriceCall) -> Self {
            ICurvePoolCalls::GetVirtualPrice(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityOneCoinCall> for ICurvePoolCalls {
        fn from(var: RemoveLiquidityOneCoinCall) -> Self {
            ICurvePoolCalls::RemoveLiquidityOneCoin(var)
        }
    }
    impl ::std::convert::From<TokenCall> for ICurvePoolCalls {
        fn from(var: TokenCall) -> Self {
            ICurvePoolCalls::Token(var)
        }
    }
}
