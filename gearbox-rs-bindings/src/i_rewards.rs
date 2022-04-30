pub use irewards_mod::*;
#[allow(clippy::too_many_arguments)]
mod irewards_mod {
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
    #[doc = "IRewards was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IREWARDS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addExtraReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earned\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"notifyRewardAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queueNewRewards\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stakeFor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stakingToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IREWARDS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IRewards<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IRewards<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRewards<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRewards))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IRewards<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IREWARDS_ABI.clone(), client).into()
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
                IREWARDS_ABI.clone(),
                IREWARDS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addExtraReward` (0x5e43c47b) function"]
        pub fn add_extra_reward(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 67, 196, 123], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `earned` (0x008cc262) function"]
        pub fn earned(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 140, 194, 98], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exit` (0xb42652e9) function"]
        pub fn exit(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 38, 82, 233], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReward` (0xc00007b0) function"]
        pub fn get_reward(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 0, 7, 176], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `notifyRewardAmount` (0x3c6b16ab) function"]
        pub fn notify_reward_amount(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 107, 22, 171], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queueNewRewards` (0x590a41f5) function"]
        pub fn queue_new_rewards(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 10, 65, 245], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardToken` (0xf7c618c1) function"]
        pub fn reward_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stake` (0xadc9772e) function"]
        pub fn stake(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 201, 119, 46], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakeFor` (0x2ee40908) function"]
        pub fn stake_for(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 228, 9, 8], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakingToken` (0x72f702f3) function"]
        pub fn staking_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([114, 247, 2, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xf3fef3a3) function"]
        pub fn withdraw(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 254, 243, 163], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IRewards<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addExtraReward`function with signature `addExtraReward(address)` and selector `[94, 67, 196, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addExtraReward", abi = "addExtraReward(address)")]
    pub struct AddExtraRewardCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `earned`function with signature `earned(address)` and selector `[0, 140, 194, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "earned", abi = "earned(address)")]
    pub struct EarnedCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `exit`function with signature `exit(address)` and selector `[180, 38, 82, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exit", abi = "exit(address)")]
    pub struct ExitCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getReward`function with signature `getReward(address)` and selector `[192, 0, 7, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReward", abi = "getReward(address)")]
    pub struct GetRewardCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `notifyRewardAmount`function with signature `notifyRewardAmount(uint256)` and selector `[60, 107, 22, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(uint256)")]
    pub struct NotifyRewardAmountCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `queueNewRewards`function with signature `queueNewRewards(uint256)` and selector `[89, 10, 65, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "queueNewRewards", abi = "queueNewRewards(uint256)")]
    pub struct QueueNewRewardsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `rewardToken`function with signature `rewardToken()` and selector `[247, 198, 24, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardToken", abi = "rewardToken()")]
    pub struct RewardTokenCall;
    #[doc = "Container type for all input parameters for the `stake`function with signature `stake(address,uint256)` and selector `[173, 201, 119, 46]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stake", abi = "stake(address,uint256)")]
    pub struct StakeCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `stakeFor`function with signature `stakeFor(address,uint256)` and selector `[46, 228, 9, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stakeFor", abi = "stakeFor(address,uint256)")]
    pub struct StakeForCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `stakingToken`function with signature `stakingToken()` and selector `[114, 247, 2, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stakingToken", abi = "stakingToken()")]
    pub struct StakingTokenCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(address,uint256)` and selector `[243, 254, 243, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256)")]
    pub struct WithdrawCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IRewardsCalls {
        AddExtraReward(AddExtraRewardCall),
        Earned(EarnedCall),
        Exit(ExitCall),
        GetReward(GetRewardCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        QueueNewRewards(QueueNewRewardsCall),
        RewardToken(RewardTokenCall),
        Stake(StakeCall),
        StakeFor(StakeForCall),
        StakingToken(StakingTokenCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for IRewardsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddExtraRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::AddExtraReward(decoded));
            }
            if let Ok(decoded) = <EarnedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::Earned(decoded));
            }
            if let Ok(decoded) = <ExitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IRewardsCalls::Exit(decoded));
            }
            if let Ok(decoded) =
                <GetRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::GetReward(decoded));
            }
            if let Ok(decoded) =
                <NotifyRewardAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::NotifyRewardAmount(decoded));
            }
            if let Ok(decoded) =
                <QueueNewRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::QueueNewRewards(decoded));
            }
            if let Ok(decoded) =
                <RewardTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::RewardToken(decoded));
            }
            if let Ok(decoded) = <StakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::Stake(decoded));
            }
            if let Ok(decoded) =
                <StakeForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::StakeFor(decoded));
            }
            if let Ok(decoded) =
                <StakingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::StakingToken(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardsCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IRewardsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IRewardsCalls::AddExtraReward(element) => element.encode(),
                IRewardsCalls::Earned(element) => element.encode(),
                IRewardsCalls::Exit(element) => element.encode(),
                IRewardsCalls::GetReward(element) => element.encode(),
                IRewardsCalls::NotifyRewardAmount(element) => element.encode(),
                IRewardsCalls::QueueNewRewards(element) => element.encode(),
                IRewardsCalls::RewardToken(element) => element.encode(),
                IRewardsCalls::Stake(element) => element.encode(),
                IRewardsCalls::StakeFor(element) => element.encode(),
                IRewardsCalls::StakingToken(element) => element.encode(),
                IRewardsCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRewardsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRewardsCalls::AddExtraReward(element) => element.fmt(f),
                IRewardsCalls::Earned(element) => element.fmt(f),
                IRewardsCalls::Exit(element) => element.fmt(f),
                IRewardsCalls::GetReward(element) => element.fmt(f),
                IRewardsCalls::NotifyRewardAmount(element) => element.fmt(f),
                IRewardsCalls::QueueNewRewards(element) => element.fmt(f),
                IRewardsCalls::RewardToken(element) => element.fmt(f),
                IRewardsCalls::Stake(element) => element.fmt(f),
                IRewardsCalls::StakeFor(element) => element.fmt(f),
                IRewardsCalls::StakingToken(element) => element.fmt(f),
                IRewardsCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddExtraRewardCall> for IRewardsCalls {
        fn from(var: AddExtraRewardCall) -> Self {
            IRewardsCalls::AddExtraReward(var)
        }
    }
    impl ::std::convert::From<EarnedCall> for IRewardsCalls {
        fn from(var: EarnedCall) -> Self {
            IRewardsCalls::Earned(var)
        }
    }
    impl ::std::convert::From<ExitCall> for IRewardsCalls {
        fn from(var: ExitCall) -> Self {
            IRewardsCalls::Exit(var)
        }
    }
    impl ::std::convert::From<GetRewardCall> for IRewardsCalls {
        fn from(var: GetRewardCall) -> Self {
            IRewardsCalls::GetReward(var)
        }
    }
    impl ::std::convert::From<NotifyRewardAmountCall> for IRewardsCalls {
        fn from(var: NotifyRewardAmountCall) -> Self {
            IRewardsCalls::NotifyRewardAmount(var)
        }
    }
    impl ::std::convert::From<QueueNewRewardsCall> for IRewardsCalls {
        fn from(var: QueueNewRewardsCall) -> Self {
            IRewardsCalls::QueueNewRewards(var)
        }
    }
    impl ::std::convert::From<RewardTokenCall> for IRewardsCalls {
        fn from(var: RewardTokenCall) -> Self {
            IRewardsCalls::RewardToken(var)
        }
    }
    impl ::std::convert::From<StakeCall> for IRewardsCalls {
        fn from(var: StakeCall) -> Self {
            IRewardsCalls::Stake(var)
        }
    }
    impl ::std::convert::From<StakeForCall> for IRewardsCalls {
        fn from(var: StakeForCall) -> Self {
            IRewardsCalls::StakeFor(var)
        }
    }
    impl ::std::convert::From<StakingTokenCall> for IRewardsCalls {
        fn from(var: StakingTokenCall) -> Self {
            IRewardsCalls::StakingToken(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IRewardsCalls {
        fn from(var: WithdrawCall) -> Self {
            IRewardsCalls::Withdraw(var)
        }
    }
}
