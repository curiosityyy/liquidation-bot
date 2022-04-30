pub use iquoter_mod::*;
#[allow(clippy::too_many_arguments)]
mod iquoter_mod {
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
    #[doc = "IQuoter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IQUOTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"quoteExactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IQUOTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IQuoter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IQuoter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IQuoter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IQuoter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IQuoter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IQUOTER_ABI.clone(), client).into()
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
                IQUOTER_ABI.clone(),
                IQUOTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `quoteExactInput` (0xcdca1753) function"]
        pub fn quote_exact_input(
            &self,
            path: ethers::core::types::Bytes,
            amount_in: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([205, 202, 23, 83], (path, amount_in))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quoteExactInputSingle` (0xf7729d43) function"]
        pub fn quote_exact_input_single(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            fee: u32,
            amount_in: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [247, 114, 157, 67],
                    (token_in, token_out, fee, amount_in, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quoteExactOutput` (0x2f80bb1d) function"]
        pub fn quote_exact_output(
            &self,
            path: ethers::core::types::Bytes,
            amount_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([47, 128, 187, 29], (path, amount_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quoteExactOutputSingle` (0x30d07f21) function"]
        pub fn quote_exact_output_single(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            fee: u32,
            amount_out: ethers::core::types::U256,
            sqrt_price_limit_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [48, 208, 127, 33],
                    (token_in, token_out, fee, amount_out, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IQuoter<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `quoteExactInput`function with signature `quoteExactInput(bytes,uint256)` and selector `[205, 202, 23, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "quoteExactInput", abi = "quoteExactInput(bytes,uint256)")]
    pub struct QuoteExactInputCall {
        pub path: ethers::core::types::Bytes,
        pub amount_in: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `quoteExactInputSingle`function with signature `quoteExactInputSingle(address,address,uint24,uint256,uint160)` and selector `[247, 114, 157, 67]`"]
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
        name = "quoteExactInputSingle",
        abi = "quoteExactInputSingle(address,address,uint24,uint256,uint160)"
    )]
    pub struct QuoteExactInputSingleCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub fee: u32,
        pub amount_in: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `quoteExactOutput`function with signature `quoteExactOutput(bytes,uint256)` and selector `[47, 128, 187, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "quoteExactOutput", abi = "quoteExactOutput(bytes,uint256)")]
    pub struct QuoteExactOutputCall {
        pub path: ethers::core::types::Bytes,
        pub amount_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `quoteExactOutputSingle`function with signature `quoteExactOutputSingle(address,address,uint24,uint256,uint160)` and selector `[48, 208, 127, 33]`"]
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
        name = "quoteExactOutputSingle",
        abi = "quoteExactOutputSingle(address,address,uint24,uint256,uint160)"
    )]
    pub struct QuoteExactOutputSingleCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub fee: u32,
        pub amount_out: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IQuoterCalls {
        QuoteExactInput(QuoteExactInputCall),
        QuoteExactInputSingle(QuoteExactInputSingleCall),
        QuoteExactOutput(QuoteExactOutputCall),
        QuoteExactOutputSingle(QuoteExactOutputSingleCall),
    }
    impl ethers::core::abi::AbiDecode for IQuoterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <QuoteExactInputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuoterCalls::QuoteExactInput(decoded));
            }
            if let Ok(decoded) =
                <QuoteExactInputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuoterCalls::QuoteExactInputSingle(decoded));
            }
            if let Ok(decoded) =
                <QuoteExactOutputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuoterCalls::QuoteExactOutput(decoded));
            }
            if let Ok(decoded) =
                <QuoteExactOutputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IQuoterCalls::QuoteExactOutputSingle(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IQuoterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IQuoterCalls::QuoteExactInput(element) => element.encode(),
                IQuoterCalls::QuoteExactInputSingle(element) => element.encode(),
                IQuoterCalls::QuoteExactOutput(element) => element.encode(),
                IQuoterCalls::QuoteExactOutputSingle(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IQuoterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IQuoterCalls::QuoteExactInput(element) => element.fmt(f),
                IQuoterCalls::QuoteExactInputSingle(element) => element.fmt(f),
                IQuoterCalls::QuoteExactOutput(element) => element.fmt(f),
                IQuoterCalls::QuoteExactOutputSingle(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<QuoteExactInputCall> for IQuoterCalls {
        fn from(var: QuoteExactInputCall) -> Self {
            IQuoterCalls::QuoteExactInput(var)
        }
    }
    impl ::std::convert::From<QuoteExactInputSingleCall> for IQuoterCalls {
        fn from(var: QuoteExactInputSingleCall) -> Self {
            IQuoterCalls::QuoteExactInputSingle(var)
        }
    }
    impl ::std::convert::From<QuoteExactOutputCall> for IQuoterCalls {
        fn from(var: QuoteExactOutputCall) -> Self {
            IQuoterCalls::QuoteExactOutput(var)
        }
    }
    impl ::std::convert::From<QuoteExactOutputSingleCall> for IQuoterCalls {
        fn from(var: QuoteExactOutputSingleCall) -> Self {
            IQuoterCalls::QuoteExactOutputSingle(var)
        }
    }
}
