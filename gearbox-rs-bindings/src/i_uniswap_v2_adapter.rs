pub use iuniswapv2adapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod iuniswapv2adapter_mod {
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
    #[doc = "IUniswapV2Adapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IUNISWAPV2ADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountADesired\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountBDesired\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountAMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountBMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountB\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTokenDesired\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTokenMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETHMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"addLiquidityETH\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountToken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETH\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountIn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOut\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountsIn\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountsOut\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveB\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"quote\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountB\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountAMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountBMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountB\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTokenMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETHMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityETH\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountToken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETH\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTokenMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETHMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityETHSupportingFeeOnTransferTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountETH\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTokenMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETHMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approveMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityETHWithPermit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountToken\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETH\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTokenMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountETHMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approveMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityETHWithPermitSupportingFeeOnTransferTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountETH\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountAMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountBMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approveMax\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityWithPermit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountB\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rateMinRAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapAllTokensForTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapETHForExactTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapExactETHForTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapExactETHForTokensSupportingFeeOnTransferTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapExactTokensForETH\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapExactTokensForETHSupportingFeeOnTransferTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapExactTokensForTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapExactTokensForTokensSupportingFeeOnTransferTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapTokensForExactETH\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapTokensForExactTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IUNISWAPV2ADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IUniswapV2Adapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IUniswapV2Adapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IUniswapV2Adapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV2Adapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IUniswapV2Adapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IUNISWAPV2ADAPTER_ABI.clone(), client)
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
                IUNISWAPV2ADAPTER_ABI.clone(),
                IUNISWAPV2ADAPTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_gearboxAdapterType` (0xce30bbdb) function"]
        pub fn gearbox_adapter_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([206, 48, 187, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_gearboxAdapterVersion` (0x78aa73a4) function"]
        pub fn gearbox_adapter_version(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([120, 170, 115, 164], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidity` (0xe8e33700) function"]
        pub fn add_liquidity(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
            amount_a_desired: ethers::core::types::U256,
            amount_b_desired: ethers::core::types::U256,
            amount_a_min: ethers::core::types::U256,
            amount_b_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [232, 227, 55, 0],
                    (
                        token_a,
                        token_b,
                        amount_a_desired,
                        amount_b_desired,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xf305d719) function"]
        pub fn add_liquidity_eth(
            &self,
            token: ethers::core::types::Address,
            amount_token_desired: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [243, 5, 215, 25],
                    (
                        token,
                        amount_token_desired,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditFacade` (0x2f7a1881) function"]
        pub fn credit_facade(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([47, 122, 24, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManager` (0xc12c21c0) function"]
        pub fn credit_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 44, 33, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountIn` (0x85f8c259) function"]
        pub fn get_amount_in(
            &self,
            amount_out: ethers::core::types::U256,
            reserve_in: ethers::core::types::U256,
            reserve_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 248, 194, 89], (amount_out, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountOut` (0x054d50d4) function"]
        pub fn get_amount_out(
            &self,
            amount_in: ethers::core::types::U256,
            reserve_in: ethers::core::types::U256,
            reserve_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([5, 77, 80, 212], (amount_in, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsIn` (0x1f00ca74) function"]
        pub fn get_amounts_in(
            &self,
            amount_out: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([31, 0, 202, 116], (amount_out, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsOut` (0xd06ca61f) function"]
        pub fn get_amounts_out(
            &self,
            amount_in: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([208, 108, 166, 31], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quote` (0xad615dec) function"]
        pub fn quote(
            &self,
            amount_a: ethers::core::types::U256,
            reserve_a: ethers::core::types::U256,
            reserve_b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([173, 97, 93, 236], (amount_a, reserve_a, reserve_b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidity` (0xbaa2abde) function"]
        pub fn remove_liquidity(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_a_min: ethers::core::types::U256,
            amount_b_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [186, 162, 171, 222],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0x02751cec) function"]
        pub fn remove_liquidity_eth(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [2, 117, 28, 236],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHSupportingFeeOnTransferTokens` (0xaf2979eb) function"]
        pub fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [175, 41, 121, 235],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermit` (0xded9382a) function"]
        pub fn remove_liquidity_eth_with_permit(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [222, 217, 56, 42],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` (0x5b0d5984) function"]
        pub fn remove_liquidity_eth_with_permit_supporting_fee_on_transfer_tokens(
            &self,
            token: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_token_min: ethers::core::types::U256,
            amount_eth_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [91, 13, 89, 132],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityWithPermit` (0x2195995c) function"]
        pub fn remove_liquidity_with_permit(
            &self,
            token_a: ethers::core::types::Address,
            token_b: ethers::core::types::Address,
            liquidity: ethers::core::types::U256,
            amount_a_min: ethers::core::types::U256,
            amount_b_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [33, 149, 153, 92],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapAllTokensForTokens` (0xbdbeaa31) function"]
        pub fn swap_all_tokens_for_tokens(
            &self,
            rate_min_ray: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([189, 190, 170, 49], (rate_min_ray, path, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapETHForExactTokens` (0xfb3bdb41) function"]
        pub fn swap_eth_for_exact_tokens(
            &self,
            amount_out: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([251, 59, 219, 65], (amount_out, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokens` (0x7ff36ab5) function"]
        pub fn swap_exact_eth_for_tokens(
            &self,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([127, 243, 106, 181], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokensSupportingFeeOnTransferTokens` (0xb6f9de95) function"]
        pub fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 249, 222, 149], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETH` (0x18cbafe5) function"]
        pub fn swap_exact_tokens_for_eth(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [24, 203, 175, 229],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETHSupportingFeeOnTransferTokens` (0x791ac947) function"]
        pub fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [121, 26, 201, 71],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokens` (0x38ed1739) function"]
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [56, 237, 23, 57],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokensSupportingFeeOnTransferTokens` (0x5c11d795) function"]
        pub fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [92, 17, 215, 149],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactETH` (0x4a25d94a) function"]
        pub fn swap_tokens_for_exact_eth(
            &self,
            amount_out: ethers::core::types::U256,
            amount_in_max: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [74, 37, 217, 74],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactTokens` (0x8803dbee) function"]
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ethers::core::types::U256,
            amount_in_max: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            to: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [136, 3, 219, 238],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetContract` (0xbd90df70) function"]
        pub fn target_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([189, 144, 223, 112], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IUniswapV2Adapter<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `WETH`function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    #[doc = "Container type for all input parameters for the `_gearboxAdapterType`function with signature `_gearboxAdapterType()` and selector `[206, 48, 187, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_gearboxAdapterType", abi = "_gearboxAdapterType()")]
    pub struct GearboxAdapterTypeCall;
    #[doc = "Container type for all input parameters for the `_gearboxAdapterVersion`function with signature `_gearboxAdapterVersion()` and selector `[120, 170, 115, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_gearboxAdapterVersion", abi = "_gearboxAdapterVersion()")]
    pub struct GearboxAdapterVersionCall;
    #[doc = "Container type for all input parameters for the `addLiquidity`function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `[232, 227, 55, 0]`"]
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
        name = "addLiquidity",
        abi = "addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
        pub amount_a_desired: ethers::core::types::U256,
        pub amount_b_desired: ethers::core::types::U256,
        pub amount_a_min: ethers::core::types::U256,
        pub amount_b_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `addLiquidityETH`function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[243, 5, 215, 25]`"]
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
        abi = "addLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityETHCall {
        pub token: ethers::core::types::Address,
        pub amount_token_desired: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `creditFacade`function with signature `creditFacade()` and selector `[47, 122, 24, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditFacade", abi = "creditFacade()")]
    pub struct CreditFacadeCall;
    #[doc = "Container type for all input parameters for the `creditManager`function with signature `creditManager()` and selector `[193, 44, 33, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManager", abi = "creditManager()")]
    pub struct CreditManagerCall;
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `getAmountIn`function with signature `getAmountIn(uint256,uint256,uint256)` and selector `[133, 248, 194, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountIn", abi = "getAmountIn(uint256,uint256,uint256)")]
    pub struct GetAmountInCall {
        pub amount_out: ethers::core::types::U256,
        pub reserve_in: ethers::core::types::U256,
        pub reserve_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAmountOut`function with signature `getAmountOut(uint256,uint256,uint256)` and selector `[5, 77, 80, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint256,uint256,uint256)")]
    pub struct GetAmountOutCall {
        pub amount_in: ethers::core::types::U256,
        pub reserve_in: ethers::core::types::U256,
        pub reserve_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAmountsIn`function with signature `getAmountsIn(uint256,address[])` and selector `[31, 0, 202, 116]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountsIn", abi = "getAmountsIn(uint256,address[])")]
    pub struct GetAmountsInCall {
        pub amount_out: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getAmountsOut`function with signature `getAmountsOut(uint256,address[])` and selector `[208, 108, 166, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountsOut", abi = "getAmountsOut(uint256,address[])")]
    pub struct GetAmountsOutCall {
        pub amount_in: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `quote`function with signature `quote(uint256,uint256,uint256)` and selector `[173, 97, 93, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "quote", abi = "quote(uint256,uint256,uint256)")]
    pub struct QuoteCall {
        pub amount_a: ethers::core::types::U256,
        pub reserve_a: ethers::core::types::U256,
        pub reserve_b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidity`function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `[186, 162, 171, 222]`"]
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
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_a_min: ethers::core::types::U256,
        pub amount_b_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETH`function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[2, 117, 28, 236]`"]
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
        abi = "removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETHSupportingFeeOnTransferTokens`function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)` and selector `[175, 41, 121, 235]`"]
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
        name = "removeLiquidityETHSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETHWithPermit`function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[222, 217, 56, 42]`"]
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
        name = "removeLiquidityETHWithPermit",
        abi = "removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens`function with signature `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[91, 13, 89, 132]`"]
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
        name = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall {
        pub token: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_token_min: ethers::core::types::U256,
        pub amount_eth_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityWithPermit`function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[33, 149, 153, 92]`"]
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
        name = "removeLiquidityWithPermit",
        abi = "removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityWithPermitCall {
        pub token_a: ethers::core::types::Address,
        pub token_b: ethers::core::types::Address,
        pub liquidity: ethers::core::types::U256,
        pub amount_a_min: ethers::core::types::U256,
        pub amount_b_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub approve_max: bool,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `swapAllTokensForTokens`function with signature `swapAllTokensForTokens(uint256,address[],uint256)` and selector `[189, 190, 170, 49]`"]
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
        name = "swapAllTokensForTokens",
        abi = "swapAllTokensForTokens(uint256,address[],uint256)"
    )]
    pub struct SwapAllTokensForTokensCall {
        pub rate_min_ray: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapETHForExactTokens`function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `[251, 59, 219, 65]`"]
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
        name = "swapETHForExactTokens",
        abi = "swapETHForExactTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapETHForExactTokensCall {
        pub amount_out: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactETHForTokens`function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `[127, 243, 106, 181]`"]
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
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall {
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactETHForTokensSupportingFeeOnTransferTokens`function with signature `swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)` and selector `[182, 249, 222, 149]`"]
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
        name = "swapExactETHForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensSupportingFeeOnTransferTokensCall {
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForETH`function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `[24, 203, 175, 229]`"]
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
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForETHSupportingFeeOnTransferTokens`function with signature `swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `[121, 26, 201, 71]`"]
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
        name = "swapExactTokensForETHSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHSupportingFeeOnTransferTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForTokens`function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `[56, 237, 23, 57]`"]
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForTokensSupportingFeeOnTransferTokens`function with signature `swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `[92, 17, 215, 149]`"]
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
        name = "swapExactTokensForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensSupportingFeeOnTransferTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapTokensForExactETH`function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `[74, 37, 217, 74]`"]
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
        name = "swapTokensForExactETH",
        abi = "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactETHCall {
        pub amount_out: ethers::core::types::U256,
        pub amount_in_max: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapTokensForExactTokens`function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `[136, 3, 219, 238]`"]
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
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub amount_out: ethers::core::types::U256,
        pub amount_in_max: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub to: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `targetContract`function with signature `targetContract()` and selector `[189, 144, 223, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "targetContract", abi = "targetContract()")]
    pub struct TargetContractCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IUniswapV2AdapterCalls {
        Weth(WethCall),
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Factory(FactoryCall),
        GetAmountIn(GetAmountInCall),
        GetAmountOut(GetAmountOutCall),
        GetAmountsIn(GetAmountsInCall),
        GetAmountsOut(GetAmountsOutCall),
        Quote(QuoteCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHSupportingFeeOnTransferTokens(
            RemoveLiquidityETHSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityETHWithPermit(RemoveLiquidityETHWithPermitCall),
        RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
            RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityWithPermit(RemoveLiquidityWithPermitCall),
        SwapAllTokensForTokens(SwapAllTokensForTokensCall),
        SwapETHForExactTokens(SwapETHForExactTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactETHForTokensSupportingFeeOnTransferTokens(
            SwapExactETHForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForETHSupportingFeeOnTransferTokens(
            SwapExactTokensForETHSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapExactTokensForTokensSupportingFeeOnTransferTokens(
            SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapTokensForExactETH(SwapTokensForExactETHCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
        TargetContract(TargetContractCall),
    }
    impl ethers::core::abi::AbiDecode for IUniswapV2AdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IUniswapV2AdapterCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetAmountInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::GetAmountIn(decoded));
            }
            if let Ok(decoded) =
                <GetAmountOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::GetAmountOut(decoded));
            }
            if let Ok(decoded) =
                <GetAmountsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::GetAmountsIn(decoded));
            }
            if let Ok(decoded) =
                <GetAmountsOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::GetAmountsOut(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::Quote(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::RemoveLiquidityETH(decoded));
            }
            if let Ok (decoded) = < RemoveLiquidityETHSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IUniswapV2AdapterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <RemoveLiquidityETHWithPermitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV2AdapterCalls::RemoveLiquidityETHWithPermit(
                    decoded,
                ));
            }
            if let Ok (decoded) = < RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IUniswapV2AdapterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <RemoveLiquidityWithPermitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV2AdapterCalls::RemoveLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SwapAllTokensForTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::SwapAllTokensForTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapETHForExactTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::SwapETHForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapExactETHForTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::SwapExactETHForTokens(decoded));
            }
            if let Ok (decoded) = < SwapExactETHForTokensSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IUniswapV2AdapterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapExactTokensForETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::SwapExactTokensForETH(decoded));
            }
            if let Ok (decoded) = < SwapExactTokensForETHSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IUniswapV2AdapterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapExactTokensForTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV2AdapterCalls::SwapExactTokensForTokens(decoded));
            }
            if let Ok (decoded) = < SwapExactTokensForTokensSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (IUniswapV2AdapterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapTokensForExactETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::SwapTokensForExactETH(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensForExactTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IUniswapV2AdapterCalls::SwapTokensForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2AdapterCalls::TargetContract(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IUniswapV2AdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self { IUniswapV2AdapterCalls :: Weth (element) => element . encode () , IUniswapV2AdapterCalls :: GearboxAdapterType (element) => element . encode () , IUniswapV2AdapterCalls :: GearboxAdapterVersion (element) => element . encode () , IUniswapV2AdapterCalls :: AddLiquidity (element) => element . encode () , IUniswapV2AdapterCalls :: AddLiquidityETH (element) => element . encode () , IUniswapV2AdapterCalls :: CreditFacade (element) => element . encode () , IUniswapV2AdapterCalls :: CreditManager (element) => element . encode () , IUniswapV2AdapterCalls :: Factory (element) => element . encode () , IUniswapV2AdapterCalls :: GetAmountIn (element) => element . encode () , IUniswapV2AdapterCalls :: GetAmountOut (element) => element . encode () , IUniswapV2AdapterCalls :: GetAmountsIn (element) => element . encode () , IUniswapV2AdapterCalls :: GetAmountsOut (element) => element . encode () , IUniswapV2AdapterCalls :: Quote (element) => element . encode () , IUniswapV2AdapterCalls :: RemoveLiquidity (element) => element . encode () , IUniswapV2AdapterCalls :: RemoveLiquidityETH (element) => element . encode () , IUniswapV2AdapterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (element) => element . encode () , IUniswapV2AdapterCalls :: RemoveLiquidityETHWithPermit (element) => element . encode () , IUniswapV2AdapterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (element) => element . encode () , IUniswapV2AdapterCalls :: RemoveLiquidityWithPermit (element) => element . encode () , IUniswapV2AdapterCalls :: SwapAllTokensForTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapETHForExactTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapExactETHForTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapExactTokensForETH (element) => element . encode () , IUniswapV2AdapterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapExactTokensForTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (element) => element . encode () , IUniswapV2AdapterCalls :: SwapTokensForExactETH (element) => element . encode () , IUniswapV2AdapterCalls :: SwapTokensForExactTokens (element) => element . encode () , IUniswapV2AdapterCalls :: TargetContract (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for IUniswapV2AdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { IUniswapV2AdapterCalls :: Weth (element) => element . fmt (f) , IUniswapV2AdapterCalls :: GearboxAdapterType (element) => element . fmt (f) , IUniswapV2AdapterCalls :: GearboxAdapterVersion (element) => element . fmt (f) , IUniswapV2AdapterCalls :: AddLiquidity (element) => element . fmt (f) , IUniswapV2AdapterCalls :: AddLiquidityETH (element) => element . fmt (f) , IUniswapV2AdapterCalls :: CreditFacade (element) => element . fmt (f) , IUniswapV2AdapterCalls :: CreditManager (element) => element . fmt (f) , IUniswapV2AdapterCalls :: Factory (element) => element . fmt (f) , IUniswapV2AdapterCalls :: GetAmountIn (element) => element . fmt (f) , IUniswapV2AdapterCalls :: GetAmountOut (element) => element . fmt (f) , IUniswapV2AdapterCalls :: GetAmountsIn (element) => element . fmt (f) , IUniswapV2AdapterCalls :: GetAmountsOut (element) => element . fmt (f) , IUniswapV2AdapterCalls :: Quote (element) => element . fmt (f) , IUniswapV2AdapterCalls :: RemoveLiquidity (element) => element . fmt (f) , IUniswapV2AdapterCalls :: RemoveLiquidityETH (element) => element . fmt (f) , IUniswapV2AdapterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: RemoveLiquidityETHWithPermit (element) => element . fmt (f) , IUniswapV2AdapterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: RemoveLiquidityWithPermit (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapAllTokensForTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapETHForExactTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapExactETHForTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapExactTokensForETH (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapExactTokensForTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapTokensForExactETH (element) => element . fmt (f) , IUniswapV2AdapterCalls :: SwapTokensForExactTokens (element) => element . fmt (f) , IUniswapV2AdapterCalls :: TargetContract (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<WethCall> for IUniswapV2AdapterCalls {
        fn from(var: WethCall) -> Self {
            IUniswapV2AdapterCalls::Weth(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for IUniswapV2AdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            IUniswapV2AdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for IUniswapV2AdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            IUniswapV2AdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for IUniswapV2AdapterCalls {
        fn from(var: AddLiquidityCall) -> Self {
            IUniswapV2AdapterCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<AddLiquidityETHCall> for IUniswapV2AdapterCalls {
        fn from(var: AddLiquidityETHCall) -> Self {
            IUniswapV2AdapterCalls::AddLiquidityETH(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for IUniswapV2AdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            IUniswapV2AdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for IUniswapV2AdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            IUniswapV2AdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for IUniswapV2AdapterCalls {
        fn from(var: FactoryCall) -> Self {
            IUniswapV2AdapterCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetAmountInCall> for IUniswapV2AdapterCalls {
        fn from(var: GetAmountInCall) -> Self {
            IUniswapV2AdapterCalls::GetAmountIn(var)
        }
    }
    impl ::std::convert::From<GetAmountOutCall> for IUniswapV2AdapterCalls {
        fn from(var: GetAmountOutCall) -> Self {
            IUniswapV2AdapterCalls::GetAmountOut(var)
        }
    }
    impl ::std::convert::From<GetAmountsInCall> for IUniswapV2AdapterCalls {
        fn from(var: GetAmountsInCall) -> Self {
            IUniswapV2AdapterCalls::GetAmountsIn(var)
        }
    }
    impl ::std::convert::From<GetAmountsOutCall> for IUniswapV2AdapterCalls {
        fn from(var: GetAmountsOutCall) -> Self {
            IUniswapV2AdapterCalls::GetAmountsOut(var)
        }
    }
    impl ::std::convert::From<QuoteCall> for IUniswapV2AdapterCalls {
        fn from(var: QuoteCall) -> Self {
            IUniswapV2AdapterCalls::Quote(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for IUniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            IUniswapV2AdapterCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHCall> for IUniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityETHCall) -> Self {
            IUniswapV2AdapterCalls::RemoveLiquidityETH(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHSupportingFeeOnTransferTokensCall>
        for IUniswapV2AdapterCalls
    {
        fn from(var: RemoveLiquidityETHSupportingFeeOnTransferTokensCall) -> Self {
            IUniswapV2AdapterCalls::RemoveLiquidityETHSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHWithPermitCall> for IUniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityETHWithPermitCall) -> Self {
            IUniswapV2AdapterCalls::RemoveLiquidityETHWithPermit(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall>
        for IUniswapV2AdapterCalls
    {
        fn from(var: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall) -> Self {
            IUniswapV2AdapterCalls::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityWithPermitCall> for IUniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityWithPermitCall) -> Self {
            IUniswapV2AdapterCalls::RemoveLiquidityWithPermit(var)
        }
    }
    impl ::std::convert::From<SwapAllTokensForTokensCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapAllTokensForTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapAllTokensForTokens(var)
        }
    }
    impl ::std::convert::From<SwapETHForExactTokensCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapETHForExactTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapETHForExactTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactETHForTokensCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapExactETHForTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapExactETHForTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactETHForTokensSupportingFeeOnTransferTokensCall>
        for IUniswapV2AdapterCalls
    {
        fn from(var: SwapExactETHForTokensSupportingFeeOnTransferTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapExactETHForTokensSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForETHCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapExactTokensForETHCall) -> Self {
            IUniswapV2AdapterCalls::SwapExactTokensForETH(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForETHSupportingFeeOnTransferTokensCall>
        for IUniswapV2AdapterCalls
    {
        fn from(var: SwapExactTokensForETHSupportingFeeOnTransferTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapExactTokensForETHSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapExactTokensForTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapExactTokensForTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensSupportingFeeOnTransferTokensCall>
        for IUniswapV2AdapterCalls
    {
        fn from(var: SwapExactTokensForTokensSupportingFeeOnTransferTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapExactTokensForTokensSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactETHCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapTokensForExactETHCall) -> Self {
            IUniswapV2AdapterCalls::SwapTokensForExactETH(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactTokensCall> for IUniswapV2AdapterCalls {
        fn from(var: SwapTokensForExactTokensCall) -> Self {
            IUniswapV2AdapterCalls::SwapTokensForExactTokens(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for IUniswapV2AdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            IUniswapV2AdapterCalls::TargetContract(var)
        }
    }
}