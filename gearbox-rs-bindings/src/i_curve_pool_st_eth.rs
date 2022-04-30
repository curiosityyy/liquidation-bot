pub use icurvepoolsteth_mod::*;
#[allow(clippy::too_many_arguments)]
mod icurvepoolsteth_mod {
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
    #[doc = "ICurvePoolStETH was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICURVEPOOLSTETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"amounts\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_mint_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"add_liquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"coins\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exchange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange_underlying\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy_underlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_virtual_price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"min_amounts\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"amounts\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"max_burn_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_imbalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_token_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_one_coin\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICURVEPOOLSTETH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICurvePoolStETH<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICurvePoolStETH<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICurvePoolStETH<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICurvePoolStETH))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICurvePoolStETH<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICURVEPOOLSTETH_ABI.clone(), client)
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
                ICURVEPOOLSTETH_ABI.clone(),
                ICURVEPOOLSTETH_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `lp_token` (0x82c63066) function"]
        pub fn lp_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([130, 198, 48, 102], ())
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICurvePoolStETH<M> {
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
    #[doc = "Container type for all input parameters for the `lp_token`function with signature `lp_token()` and selector `[130, 198, 48, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lp_token", abi = "lp_token()")]
    pub struct LpTokenCall;
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICurvePoolStETHCalls {
        AddLiquidity(AddLiquidityCall),
        Coins(CoinsCall),
        Exchange(ExchangeCall),
        ExchangeUnderlying(ExchangeUnderlyingCall),
        GetDy(GetDyCall),
        GetDyUnderlying(GetDyUnderlyingCall),
        GetVirtualPrice(GetVirtualPriceCall),
        LpToken(LpTokenCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityImbalance(RemoveLiquidityImbalanceCall),
        RemoveLiquidityOneCoin(RemoveLiquidityOneCoinCall),
    }
    impl ethers::core::abi::AbiDecode for ICurvePoolStETHCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <CoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::Coins(decoded));
            }
            if let Ok(decoded) =
                <ExchangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::Exchange(decoded));
            }
            if let Ok(decoded) =
                <ExchangeUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::ExchangeUnderlying(decoded));
            }
            if let Ok(decoded) = <GetDyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::GetDy(decoded));
            }
            if let Ok(decoded) =
                <GetDyUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::GetDyUnderlying(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::GetVirtualPrice(decoded));
            }
            if let Ok(decoded) =
                <LpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::LpToken(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityImbalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICurvePoolStETHCalls::RemoveLiquidityImbalance(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurvePoolStETHCalls::RemoveLiquidityOneCoin(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICurvePoolStETHCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICurvePoolStETHCalls::AddLiquidity(element) => element.encode(),
                ICurvePoolStETHCalls::Coins(element) => element.encode(),
                ICurvePoolStETHCalls::Exchange(element) => element.encode(),
                ICurvePoolStETHCalls::ExchangeUnderlying(element) => element.encode(),
                ICurvePoolStETHCalls::GetDy(element) => element.encode(),
                ICurvePoolStETHCalls::GetDyUnderlying(element) => element.encode(),
                ICurvePoolStETHCalls::GetVirtualPrice(element) => element.encode(),
                ICurvePoolStETHCalls::LpToken(element) => element.encode(),
                ICurvePoolStETHCalls::RemoveLiquidity(element) => element.encode(),
                ICurvePoolStETHCalls::RemoveLiquidityImbalance(element) => element.encode(),
                ICurvePoolStETHCalls::RemoveLiquidityOneCoin(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICurvePoolStETHCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICurvePoolStETHCalls::AddLiquidity(element) => element.fmt(f),
                ICurvePoolStETHCalls::Coins(element) => element.fmt(f),
                ICurvePoolStETHCalls::Exchange(element) => element.fmt(f),
                ICurvePoolStETHCalls::ExchangeUnderlying(element) => element.fmt(f),
                ICurvePoolStETHCalls::GetDy(element) => element.fmt(f),
                ICurvePoolStETHCalls::GetDyUnderlying(element) => element.fmt(f),
                ICurvePoolStETHCalls::GetVirtualPrice(element) => element.fmt(f),
                ICurvePoolStETHCalls::LpToken(element) => element.fmt(f),
                ICurvePoolStETHCalls::RemoveLiquidity(element) => element.fmt(f),
                ICurvePoolStETHCalls::RemoveLiquidityImbalance(element) => element.fmt(f),
                ICurvePoolStETHCalls::RemoveLiquidityOneCoin(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for ICurvePoolStETHCalls {
        fn from(var: AddLiquidityCall) -> Self {
            ICurvePoolStETHCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<CoinsCall> for ICurvePoolStETHCalls {
        fn from(var: CoinsCall) -> Self {
            ICurvePoolStETHCalls::Coins(var)
        }
    }
    impl ::std::convert::From<ExchangeCall> for ICurvePoolStETHCalls {
        fn from(var: ExchangeCall) -> Self {
            ICurvePoolStETHCalls::Exchange(var)
        }
    }
    impl ::std::convert::From<ExchangeUnderlyingCall> for ICurvePoolStETHCalls {
        fn from(var: ExchangeUnderlyingCall) -> Self {
            ICurvePoolStETHCalls::ExchangeUnderlying(var)
        }
    }
    impl ::std::convert::From<GetDyCall> for ICurvePoolStETHCalls {
        fn from(var: GetDyCall) -> Self {
            ICurvePoolStETHCalls::GetDy(var)
        }
    }
    impl ::std::convert::From<GetDyUnderlyingCall> for ICurvePoolStETHCalls {
        fn from(var: GetDyUnderlyingCall) -> Self {
            ICurvePoolStETHCalls::GetDyUnderlying(var)
        }
    }
    impl ::std::convert::From<GetVirtualPriceCall> for ICurvePoolStETHCalls {
        fn from(var: GetVirtualPriceCall) -> Self {
            ICurvePoolStETHCalls::GetVirtualPrice(var)
        }
    }
    impl ::std::convert::From<LpTokenCall> for ICurvePoolStETHCalls {
        fn from(var: LpTokenCall) -> Self {
            ICurvePoolStETHCalls::LpToken(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for ICurvePoolStETHCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            ICurvePoolStETHCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityImbalanceCall> for ICurvePoolStETHCalls {
        fn from(var: RemoveLiquidityImbalanceCall) -> Self {
            ICurvePoolStETHCalls::RemoveLiquidityImbalance(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityOneCoinCall> for ICurvePoolStETHCalls {
        fn from(var: RemoveLiquidityOneCoinCall) -> Self {
            ICurvePoolStETHCalls::RemoveLiquidityOneCoin(var)
        }
    }
}
