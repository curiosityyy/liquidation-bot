pub use icurvepool2assets_mod::*;
#[allow(clippy::too_many_arguments)]
mod icurvepool2assets_mod {
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
    #[doc = "ICurvePool2Assets was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICURVEPOOL2ASSETS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"amounts\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_mint_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add_liquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"coins\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange_underlying\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy_underlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_virtual_price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"min_amounts\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"amounts\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"max_burn_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_imbalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_token_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_one_coin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICURVEPOOL2ASSETS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICurvePool2Assets<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICurvePool2Assets<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICurvePool2Assets<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICurvePool2Assets))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICurvePool2Assets<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICURVEPOOL2ASSETS_ABI.clone(), client)
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
                ICURVEPOOL2ASSETS_ABI.clone(),
                ICURVEPOOL2ASSETS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `add_liquidity` (0x0b4c7e4d) function"]
        pub fn add_liquidity(
            &self,
            amounts: [ethers::core::types::U256; 2usize],
            min_mint_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 76, 126, 77], (amounts, min_mint_amount))
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `remove_liquidity` (0x5b36389c) function"]
        pub fn remove_liquidity(
            &self,
            amount: ethers::core::types::U256,
            min_amounts: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 54, 56, 156], (amount, min_amounts))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove_liquidity_imbalance` (0xe3103273) function"]
        pub fn remove_liquidity_imbalance(
            &self,
            amounts: [ethers::core::types::U256; 2usize],
            max_burn_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 16, 50, 115], (amounts, max_burn_amount))
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICurvePool2Assets<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `add_liquidity`function with signature `add_liquidity(uint256[2],uint256)` and selector `[11, 76, 126, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "add_liquidity", abi = "add_liquidity(uint256[2],uint256)")]
    pub struct AddLiquidityCall {
        pub amounts: [ethers::core::types::U256; 2usize],
        pub min_mint_amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `remove_liquidity`function with signature `remove_liquidity(uint256,uint256[2])` and selector `[91, 54, 56, 156]`"]
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
        name = "remove_liquidity",
        abi = "remove_liquidity(uint256,uint256[2])"
    )]
    pub struct RemoveLiquidityCall {
        pub amount: ethers::core::types::U256,
        pub min_amounts: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `remove_liquidity_imbalance`function with signature `remove_liquidity_imbalance(uint256[2],uint256)` and selector `[227, 16, 50, 115]`"]
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
        name = "remove_liquidity_imbalance",
        abi = "remove_liquidity_imbalance(uint256[2],uint256)"
    )]
    pub struct RemoveLiquidityImbalanceCall {
        pub amounts: [ethers::core::types::U256; 2usize],
        pub max_burn_amount: ethers::core::types::U256,
    }
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
    pub enum ICurvePool2AssetsCalls {
        AddLiquidity(AddLiquidityCall),
        Coins(CoinsCall),
        Exchange(ExchangeCall),
        ExchangeUnderlying(ExchangeUnderlyingCall),
        GetDy(GetDyCall),
        GetDyUnderlying(GetDyUnderlyingCall),
        GetVirtualPrice(GetVirtualPriceCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityImbalance(RemoveLiquidityImbalanceCall),
        RemoveLiquidityOneCoin(RemoveLiquidityOneCoinCall),
        Token(TokenCall),
    }
    impl ethers::core::abi::AbiDecode for ICurvePool2AssetsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <CoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::Coins(decoded));
            }
            if let Ok(decoded) =
                <ExchangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::Exchange(decoded));
            }
            if let Ok(decoded) =
                <ExchangeUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::ExchangeUnderlying(decoded));
            }
            if let Ok(decoded) = <GetDyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::GetDy(decoded));
            }
            if let Ok(decoded) =
                <GetDyUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::GetDyUnderlying(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::GetVirtualPrice(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityImbalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICurvePool2AssetsCalls::RemoveLiquidityImbalance(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::RemoveLiquidityOneCoin(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePool2AssetsCalls::Token(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICurvePool2AssetsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICurvePool2AssetsCalls::AddLiquidity(element) => element.encode(),
                ICurvePool2AssetsCalls::Coins(element) => element.encode(),
                ICurvePool2AssetsCalls::Exchange(element) => element.encode(),
                ICurvePool2AssetsCalls::ExchangeUnderlying(element) => element.encode(),
                ICurvePool2AssetsCalls::GetDy(element) => element.encode(),
                ICurvePool2AssetsCalls::GetDyUnderlying(element) => element.encode(),
                ICurvePool2AssetsCalls::GetVirtualPrice(element) => element.encode(),
                ICurvePool2AssetsCalls::RemoveLiquidity(element) => element.encode(),
                ICurvePool2AssetsCalls::RemoveLiquidityImbalance(element) => element.encode(),
                ICurvePool2AssetsCalls::RemoveLiquidityOneCoin(element) => element.encode(),
                ICurvePool2AssetsCalls::Token(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICurvePool2AssetsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICurvePool2AssetsCalls::AddLiquidity(element) => element.fmt(f),
                ICurvePool2AssetsCalls::Coins(element) => element.fmt(f),
                ICurvePool2AssetsCalls::Exchange(element) => element.fmt(f),
                ICurvePool2AssetsCalls::ExchangeUnderlying(element) => element.fmt(f),
                ICurvePool2AssetsCalls::GetDy(element) => element.fmt(f),
                ICurvePool2AssetsCalls::GetDyUnderlying(element) => element.fmt(f),
                ICurvePool2AssetsCalls::GetVirtualPrice(element) => element.fmt(f),
                ICurvePool2AssetsCalls::RemoveLiquidity(element) => element.fmt(f),
                ICurvePool2AssetsCalls::RemoveLiquidityImbalance(element) => element.fmt(f),
                ICurvePool2AssetsCalls::RemoveLiquidityOneCoin(element) => element.fmt(f),
                ICurvePool2AssetsCalls::Token(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for ICurvePool2AssetsCalls {
        fn from(var: AddLiquidityCall) -> Self {
            ICurvePool2AssetsCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<CoinsCall> for ICurvePool2AssetsCalls {
        fn from(var: CoinsCall) -> Self {
            ICurvePool2AssetsCalls::Coins(var)
        }
    }
    impl ::std::convert::From<ExchangeCall> for ICurvePool2AssetsCalls {
        fn from(var: ExchangeCall) -> Self {
            ICurvePool2AssetsCalls::Exchange(var)
        }
    }
    impl ::std::convert::From<ExchangeUnderlyingCall> for ICurvePool2AssetsCalls {
        fn from(var: ExchangeUnderlyingCall) -> Self {
            ICurvePool2AssetsCalls::ExchangeUnderlying(var)
        }
    }
    impl ::std::convert::From<GetDyCall> for ICurvePool2AssetsCalls {
        fn from(var: GetDyCall) -> Self {
            ICurvePool2AssetsCalls::GetDy(var)
        }
    }
    impl ::std::convert::From<GetDyUnderlyingCall> for ICurvePool2AssetsCalls {
        fn from(var: GetDyUnderlyingCall) -> Self {
            ICurvePool2AssetsCalls::GetDyUnderlying(var)
        }
    }
    impl ::std::convert::From<GetVirtualPriceCall> for ICurvePool2AssetsCalls {
        fn from(var: GetVirtualPriceCall) -> Self {
            ICurvePool2AssetsCalls::GetVirtualPrice(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for ICurvePool2AssetsCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            ICurvePool2AssetsCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityImbalanceCall> for ICurvePool2AssetsCalls {
        fn from(var: RemoveLiquidityImbalanceCall) -> Self {
            ICurvePool2AssetsCalls::RemoveLiquidityImbalance(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityOneCoinCall> for ICurvePool2AssetsCalls {
        fn from(var: RemoveLiquidityOneCoinCall) -> Self {
            ICurvePool2AssetsCalls::RemoveLiquidityOneCoin(var)
        }
    }
    impl ::std::convert::From<TokenCall> for ICurvePool2AssetsCalls {
        fn from(var: TokenCall) -> Self {
            ICurvePool2AssetsCalls::Token(var)
        }
    }
}
