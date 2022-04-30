pub use lidov1gateway_mod::*;
#[allow(clippy::too_many_arguments)]
mod lidov1gateway_mod {
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
    #[doc = "LidoV1Gateway was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LIDOV1GATEWAY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_weth\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_stETH\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stETH\",\"outputs\":[{\"internalType\":\"contract IstETH\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_referral\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"weth\",\"outputs\":[{\"internalType\":\"contract IWETH\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LIDOV1GATEWAY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c060405234801561001057600080fd5b506040516104903803806104908339818101604052604081101561003357600080fd5b5080516020909101516001600160a01b038216158061005957506001600160a01b038116155b1561007757604051635919af9760e11b815260040160405180910390fd5b6001600160a01b039081166080521660a05260805160a0516103c86100c860003960008181604b0152818161011e01526101a4015260008181608e0152818161020501526102b201526103c86000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80633fc8cef314610046578063c1fe3e4814610089578063f532e86a146100b0575b600080fd5b61006d7f000000000000000000000000000000000000000000000000000000000000000081565b604080516001600160a01b039092168252519081900360200190f35b61006d7f000000000000000000000000000000000000000000000000000000000000000081565b6100dc600480360360408110156100c657600080fd5b50803590602001356001600160a01b03166100ee565b60408051918252519081900360200190f35b604080516323b872dd60e01b81523360048201523060248201526044810184905290516000916001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016916323b872dd91606480820192602092909190829003018187875af115801561016b573d6000803e3d6000fd5b505050506040513d602081101561018157600080fd5b505060408051632e1a7d4d60e01b81526004810185905290516001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691632e1a7d4d91602480830192600092919082900301818387803b1580156101eb57600080fd5b505af11580156101ff573d6000803e3d6000fd5b505050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a1903eab836040518263ffffffff1660e01b815260040180826001600160a01b031681526020019150506020604051808303816000875af1158015610277573d6000803e3d6000fd5b505050506040513d602081101561028d57600080fd5b5051604080516370a0823160e01b815230600482015290519192506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163a9059cbb91339184916370a082319160248083019260209291908290030181865afa158015610307573d6000803e3d6000fd5b505050506040513d602081101561031d57600080fd5b5051604080516001600160e01b031960e086901b1681526001600160a01b039093166004840152602483019190915251604480830192602092919082900301816000875af1158015610373573d6000803e3d6000fd5b505050506040513d602081101561038957600080fd5b5090939250505056fea2646970667358221220192ad739d7cc7e73ed23d7c3922268a99616d7c25fc7a3c8f32371a694e50e6e64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct LidoV1Gateway<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for LidoV1Gateway<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LidoV1Gateway<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LidoV1Gateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> LidoV1Gateway<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LIDOV1GATEWAY_ABI.clone(), client)
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
                LIDOV1GATEWAY_ABI.clone(),
                LIDOV1GATEWAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `stETH` (0xc1fe3e48) function"]
        pub fn st_eth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 254, 62, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submit` (0xf532e86a) function"]
        pub fn submit(
            &self,
            amount: ethers::core::types::U256,
            referral: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([245, 50, 232, 106], (amount, referral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `weth` (0x3fc8cef3) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LidoV1Gateway<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `stETH`function with signature `stETH()` and selector `[193, 254, 62, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stETH", abi = "stETH()")]
    pub struct StETHCall;
    #[doc = "Container type for all input parameters for the `submit`function with signature `submit(uint256,address)` and selector `[245, 50, 232, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "submit", abi = "submit(uint256,address)")]
    pub struct SubmitCall {
        pub amount: ethers::core::types::U256,
        pub referral: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `weth`function with signature `weth()` and selector `[63, 200, 206, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LidoV1GatewayCalls {
        StETH(StETHCall),
        Submit(SubmitCall),
        Weth(WethCall),
    }
    impl ethers::core::abi::AbiDecode for LidoV1GatewayCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <StETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1GatewayCalls::StETH(decoded));
            }
            if let Ok(decoded) = <SubmitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1GatewayCalls::Submit(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LidoV1GatewayCalls::Weth(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LidoV1GatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LidoV1GatewayCalls::StETH(element) => element.encode(),
                LidoV1GatewayCalls::Submit(element) => element.encode(),
                LidoV1GatewayCalls::Weth(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LidoV1GatewayCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LidoV1GatewayCalls::StETH(element) => element.fmt(f),
                LidoV1GatewayCalls::Submit(element) => element.fmt(f),
                LidoV1GatewayCalls::Weth(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<StETHCall> for LidoV1GatewayCalls {
        fn from(var: StETHCall) -> Self {
            LidoV1GatewayCalls::StETH(var)
        }
    }
    impl ::std::convert::From<SubmitCall> for LidoV1GatewayCalls {
        fn from(var: SubmitCall) -> Self {
            LidoV1GatewayCalls::Submit(var)
        }
    }
    impl ::std::convert::From<WethCall> for LidoV1GatewayCalls {
        fn from(var: WethCall) -> Self {
            LidoV1GatewayCalls::Weth(var)
        }
    }
}
