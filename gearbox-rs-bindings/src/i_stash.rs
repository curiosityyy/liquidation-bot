pub use istash_mod::*;
#[allow(clippy::too_many_arguments)]
mod istash_mod {
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
    #[doc = "IStash was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ISTASH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_staker\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_gauge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_rewardFactory\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"processStash\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stashRewards\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ISTASH_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IStash<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IStash<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IStash<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IStash))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IStash<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISTASH_ABI.clone(), client).into()
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
                ISTASH_ABI.clone(),
                ISTASH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `claimRewards` (0x372500ab) function"]
        pub fn claim_rewards(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 37, 0, 171], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x3641e083) function"]
        pub fn initialize(
            &self,
            pid: ethers::core::types::U256,
            operator: ethers::core::types::Address,
            staker: ethers::core::types::Address,
            gauge: ethers::core::types::Address,
            reward_factory: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [54, 65, 224, 131],
                    (pid, operator, staker, gauge, reward_factory),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `processStash` (0xca8b0176) function"]
        pub fn process_stash(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 139, 1, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stashRewards` (0xb87bd481) function"]
        pub fn stash_rewards(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([184, 123, 212, 129], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IStash<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards()` and selector `[55, 37, 0, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards()")]
    pub struct ClaimRewardsCall;
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(uint256,address,address,address,address)` and selector `[54, 65, 224, 131]`"]
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
        name = "initialize",
        abi = "initialize(uint256,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub pid: ethers::core::types::U256,
        pub operator: ethers::core::types::Address,
        pub staker: ethers::core::types::Address,
        pub gauge: ethers::core::types::Address,
        pub reward_factory: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `processStash`function with signature `processStash()` and selector `[202, 139, 1, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "processStash", abi = "processStash()")]
    pub struct ProcessStashCall;
    #[doc = "Container type for all input parameters for the `stashRewards`function with signature `stashRewards()` and selector `[184, 123, 212, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stashRewards", abi = "stashRewards()")]
    pub struct StashRewardsCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IStashCalls {
        ClaimRewards(ClaimRewardsCall),
        Initialize(InitializeCall),
        ProcessStash(ProcessStashCall),
        StashRewards(StashRewardsCall),
    }
    impl ethers::core::abi::AbiDecode for IStashCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStashCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStashCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <ProcessStashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStashCalls::ProcessStash(decoded));
            }
            if let Ok(decoded) =
                <StashRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStashCalls::StashRewards(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IStashCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IStashCalls::ClaimRewards(element) => element.encode(),
                IStashCalls::Initialize(element) => element.encode(),
                IStashCalls::ProcessStash(element) => element.encode(),
                IStashCalls::StashRewards(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IStashCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IStashCalls::ClaimRewards(element) => element.fmt(f),
                IStashCalls::Initialize(element) => element.fmt(f),
                IStashCalls::ProcessStash(element) => element.fmt(f),
                IStashCalls::StashRewards(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for IStashCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            IStashCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IStashCalls {
        fn from(var: InitializeCall) -> Self {
            IStashCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<ProcessStashCall> for IStashCalls {
        fn from(var: ProcessStashCall) -> Self {
            IStashCalls::ProcessStash(var)
        }
    }
    impl ::std::convert::From<StashRewardsCall> for IStashCalls {
        fn from(var: StashRewardsCall) -> Self {
            IStashCalls::StashRewards(var)
        }
    }
}
