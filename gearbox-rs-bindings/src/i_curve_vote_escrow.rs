pub use icurvevoteescrow_mod::*;
#[allow(clippy::too_many_arguments)]
mod icurvevoteescrow_mod {
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
    #[doc = "ICurveVoteEscrow was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICURVEVOTEESCROW_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"create_lock\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increase_amount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increase_unlock_time\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"smart_wallet_checker\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICURVEVOTEESCROW_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICurveVoteEscrow<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICurveVoteEscrow<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICurveVoteEscrow<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICurveVoteEscrow))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICurveVoteEscrow<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICURVEVOTEESCROW_ABI.clone(), client)
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
                ICURVEVOTEESCROW_ABI.clone(),
                ICURVEVOTEESCROW_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `create_lock` (0x65fc3873) function"]
        pub fn create_lock(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 252, 56, 115], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increase_amount` (0x4957677c) function"]
        pub fn increase_amount(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 87, 103, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increase_unlock_time` (0xeff7a612) function"]
        pub fn increase_unlock_time(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 247, 166, 18], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `smart_wallet_checker` (0x7175d4f7) function"]
        pub fn smart_wallet_checker(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([113, 117, 212, 247], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x3ccfd60b) function"]
        pub fn withdraw(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICurveVoteEscrow<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `create_lock`function with signature `create_lock(uint256,uint256)` and selector `[101, 252, 56, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "create_lock", abi = "create_lock(uint256,uint256)")]
    pub struct CreateLockCall(pub ethers::core::types::U256, pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `increase_amount`function with signature `increase_amount(uint256)` and selector `[73, 87, 103, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increase_amount", abi = "increase_amount(uint256)")]
    pub struct IncreaseAmountCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `increase_unlock_time`function with signature `increase_unlock_time(uint256)` and selector `[239, 247, 166, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increase_unlock_time", abi = "increase_unlock_time(uint256)")]
    pub struct IncreaseUnlockTimeCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `smart_wallet_checker`function with signature `smart_wallet_checker()` and selector `[113, 117, 212, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "smart_wallet_checker", abi = "smart_wallet_checker()")]
    pub struct SmartWalletCheckerCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw()` and selector `[60, 207, 214, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct WithdrawCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICurveVoteEscrowCalls {
        CreateLock(CreateLockCall),
        IncreaseAmount(IncreaseAmountCall),
        IncreaseUnlockTime(IncreaseUnlockTimeCall),
        SmartWalletChecker(SmartWalletCheckerCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for ICurveVoteEscrowCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateLockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveVoteEscrowCalls::CreateLock(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveVoteEscrowCalls::IncreaseAmount(decoded));
            }
            if let Ok(decoded) =
                <IncreaseUnlockTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveVoteEscrowCalls::IncreaseUnlockTime(decoded));
            }
            if let Ok(decoded) =
                <SmartWalletCheckerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveVoteEscrowCalls::SmartWalletChecker(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveVoteEscrowCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICurveVoteEscrowCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICurveVoteEscrowCalls::CreateLock(element) => element.encode(),
                ICurveVoteEscrowCalls::IncreaseAmount(element) => element.encode(),
                ICurveVoteEscrowCalls::IncreaseUnlockTime(element) => element.encode(),
                ICurveVoteEscrowCalls::SmartWalletChecker(element) => element.encode(),
                ICurveVoteEscrowCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICurveVoteEscrowCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICurveVoteEscrowCalls::CreateLock(element) => element.fmt(f),
                ICurveVoteEscrowCalls::IncreaseAmount(element) => element.fmt(f),
                ICurveVoteEscrowCalls::IncreaseUnlockTime(element) => element.fmt(f),
                ICurveVoteEscrowCalls::SmartWalletChecker(element) => element.fmt(f),
                ICurveVoteEscrowCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateLockCall> for ICurveVoteEscrowCalls {
        fn from(var: CreateLockCall) -> Self {
            ICurveVoteEscrowCalls::CreateLock(var)
        }
    }
    impl ::std::convert::From<IncreaseAmountCall> for ICurveVoteEscrowCalls {
        fn from(var: IncreaseAmountCall) -> Self {
            ICurveVoteEscrowCalls::IncreaseAmount(var)
        }
    }
    impl ::std::convert::From<IncreaseUnlockTimeCall> for ICurveVoteEscrowCalls {
        fn from(var: IncreaseUnlockTimeCall) -> Self {
            ICurveVoteEscrowCalls::IncreaseUnlockTime(var)
        }
    }
    impl ::std::convert::From<SmartWalletCheckerCall> for ICurveVoteEscrowCalls {
        fn from(var: SmartWalletCheckerCall) -> Self {
            ICurveVoteEscrowCalls::SmartWalletChecker(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ICurveVoteEscrowCalls {
        fn from(var: WithdrawCall) -> Self {
            ICurveVoteEscrowCalls::Withdraw(var)
        }
    }
}
