pub use ideposit_mod::*;
#[allow(clippy::too_many_arguments)]
mod ideposit_mod {
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
    #[doc = "IDeposit was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IDEPOSIT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isShutdown\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolInfo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rewardArbitrator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rewardClaimed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGaugeRedirect\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawTo\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IDEPOSIT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IDeposit<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IDeposit<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IDeposit<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IDeposit))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IDeposit<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IDEPOSIT_ABI.clone(), client).into()
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
                IDEPOSIT_ABI.clone(),
                IDEPOSIT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewards` (0x6c7b69cb) function"]
        pub fn claim_rewards(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 123, 105, 203], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isShutdown` (0xbf86d690) function"]
        pub fn is_shutdown(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolInfo` (0x1526fe27) function"]
        pub fn pool_info(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                bool,
            ),
        > {
            self.0
                .method_hash([21, 38, 254, 39], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardArbitrator` (0x043b684a) function"]
        pub fn reward_arbitrator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([4, 59, 104, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardClaimed` (0x71192b17) function"]
        pub fn reward_claimed(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 25, 43, 23], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGaugeRedirect` (0x9123d404) function"]
        pub fn set_gauge_redirect(
            &self,
            pid: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 35, 212, 4], pid)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawTo` (0x14cd70e4) function"]
        pub fn withdraw_to(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 205, 112, 228], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IDeposit<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(uint256,address)` and selector `[108, 123, 105, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(uint256,address)")]
    pub struct ClaimRewardsCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `isShutdown`function with signature `isShutdown()` and selector `[191, 134, 214, 144]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isShutdown", abi = "isShutdown()")]
    pub struct IsShutdownCall;
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `poolInfo`function with signature `poolInfo(uint256)` and selector `[21, 38, 254, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolInfo", abi = "poolInfo(uint256)")]
    pub struct PoolInfoCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `rewardArbitrator`function with signature `rewardArbitrator()` and selector `[4, 59, 104, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardArbitrator", abi = "rewardArbitrator()")]
    pub struct RewardArbitratorCall;
    #[doc = "Container type for all input parameters for the `rewardClaimed`function with signature `rewardClaimed(uint256,address,uint256)` and selector `[113, 25, 43, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardClaimed", abi = "rewardClaimed(uint256,address,uint256)")]
    pub struct RewardClaimedCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `setGaugeRedirect`function with signature `setGaugeRedirect(uint256)` and selector `[145, 35, 212, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGaugeRedirect", abi = "setGaugeRedirect(uint256)")]
    pub struct SetGaugeRedirectCall {
        pub pid: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `withdrawTo`function with signature `withdrawTo(uint256,uint256,address)` and selector `[20, 205, 112, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(uint256,uint256,address)")]
    pub struct WithdrawToCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IDepositCalls {
        BalanceOf(BalanceOfCall),
        ClaimRewards(ClaimRewardsCall),
        IsShutdown(IsShutdownCall),
        Owner(OwnerCall),
        PoolInfo(PoolInfoCall),
        RewardArbitrator(RewardArbitratorCall),
        RewardClaimed(RewardClaimedCall),
        SetGaugeRedirect(SetGaugeRedirectCall),
        TotalSupply(TotalSupplyCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ethers::core::abi::AbiDecode for IDepositCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::IsShutdown(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PoolInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::PoolInfo(decoded));
            }
            if let Ok(decoded) =
                <RewardArbitratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::RewardArbitrator(decoded));
            }
            if let Ok(decoded) =
                <RewardClaimedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::RewardClaimed(decoded));
            }
            if let Ok(decoded) =
                <SetGaugeRedirectCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::SetGaugeRedirect(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <WithdrawToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDepositCalls::WithdrawTo(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IDepositCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IDepositCalls::BalanceOf(element) => element.encode(),
                IDepositCalls::ClaimRewards(element) => element.encode(),
                IDepositCalls::IsShutdown(element) => element.encode(),
                IDepositCalls::Owner(element) => element.encode(),
                IDepositCalls::PoolInfo(element) => element.encode(),
                IDepositCalls::RewardArbitrator(element) => element.encode(),
                IDepositCalls::RewardClaimed(element) => element.encode(),
                IDepositCalls::SetGaugeRedirect(element) => element.encode(),
                IDepositCalls::TotalSupply(element) => element.encode(),
                IDepositCalls::WithdrawTo(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IDepositCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IDepositCalls::BalanceOf(element) => element.fmt(f),
                IDepositCalls::ClaimRewards(element) => element.fmt(f),
                IDepositCalls::IsShutdown(element) => element.fmt(f),
                IDepositCalls::Owner(element) => element.fmt(f),
                IDepositCalls::PoolInfo(element) => element.fmt(f),
                IDepositCalls::RewardArbitrator(element) => element.fmt(f),
                IDepositCalls::RewardClaimed(element) => element.fmt(f),
                IDepositCalls::SetGaugeRedirect(element) => element.fmt(f),
                IDepositCalls::TotalSupply(element) => element.fmt(f),
                IDepositCalls::WithdrawTo(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IDepositCalls {
        fn from(var: BalanceOfCall) -> Self {
            IDepositCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for IDepositCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            IDepositCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for IDepositCalls {
        fn from(var: IsShutdownCall) -> Self {
            IDepositCalls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for IDepositCalls {
        fn from(var: OwnerCall) -> Self {
            IDepositCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PoolInfoCall> for IDepositCalls {
        fn from(var: PoolInfoCall) -> Self {
            IDepositCalls::PoolInfo(var)
        }
    }
    impl ::std::convert::From<RewardArbitratorCall> for IDepositCalls {
        fn from(var: RewardArbitratorCall) -> Self {
            IDepositCalls::RewardArbitrator(var)
        }
    }
    impl ::std::convert::From<RewardClaimedCall> for IDepositCalls {
        fn from(var: RewardClaimedCall) -> Self {
            IDepositCalls::RewardClaimed(var)
        }
    }
    impl ::std::convert::From<SetGaugeRedirectCall> for IDepositCalls {
        fn from(var: SetGaugeRedirectCall) -> Self {
            IDepositCalls::SetGaugeRedirect(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IDepositCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IDepositCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<WithdrawToCall> for IDepositCalls {
        fn from(var: WithdrawToCall) -> Self {
            IDepositCalls::WithdrawTo(var)
        }
    }
}
