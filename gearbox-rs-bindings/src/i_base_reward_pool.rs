pub use ibaserewardpool_mod::*;
#[allow(clippy::too_many_arguments)]
mod ibaserewardpool_mod {
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
    #[doc = "IBaseRewardPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IBASEREWARDPOOL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"donate\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"duration\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"earned\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraRewards\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraRewardsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_claimExtras\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"historicalRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastTimeRewardApplicable\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastUpdateTime\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"newRewardRatio\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"operator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"periodFinish\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pid\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"queuedRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardPerToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardPerTokenStored\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardToken\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stake\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stakeAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_for\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stakeFor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stakingToken\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"userRewardPerTokenPaid\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAllAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"claim\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAndUnwrap\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IBASEREWARDPOOL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IBaseRewardPool<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IBaseRewardPool<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IBaseRewardPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IBaseRewardPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IBaseRewardPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IBASEREWARDPOOL_ABI.clone(), client)
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
                IBASEREWARDPOOL_ABI.clone(),
                IBASEREWARDPOOL_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `currentRewards` (0x901a7d53) function"]
        pub fn current_rewards(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([144, 26, 125, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `donate` (0xf14faf6f) function"]
        pub fn donate(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([241, 79, 175, 111], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `duration` (0x0fb5a6b4) function"]
        pub fn duration(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 181, 166, 180], ())
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
        #[doc = "Calls the contract's `extraRewards` (0x40c35446) function"]
        pub fn extra_rewards(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([64, 195, 84, 70], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extraRewardsLength` (0xd55a23f4) function"]
        pub fn extra_rewards_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([213, 90, 35, 244], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReward` (0x3d18b912) function"]
        pub fn get_reward(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([61, 24, 185, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReward` (0x7050ccd9) function"]
        pub fn get_reward_with_account_and_claim_extras(
            &self,
            account: ethers::core::types::Address,
            claim_extras: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([112, 80, 204, 217], (account, claim_extras))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `historicalRewards` (0x262d3d6d) function"]
        pub fn historical_rewards(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([38, 45, 61, 109], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastTimeRewardApplicable` (0x80faa57d) function"]
        pub fn last_time_reward_applicable(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([128, 250, 165, 125], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastUpdateTime` (0xc8f33c91) function"]
        pub fn last_update_time(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([200, 243, 60, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `newRewardRatio` (0x6c8bcee8) function"]
        pub fn new_reward_ratio(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 139, 206, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `operator` (0x570ca735) function"]
        pub fn operator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([87, 12, 167, 53], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `periodFinish` (0xebe2b12b) function"]
        pub fn period_finish(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([235, 226, 177, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pid` (0xf1068454) function"]
        pub fn pid(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([241, 6, 132, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `queuedRewards` (0x63d38c3b) function"]
        pub fn queued_rewards(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([99, 211, 140, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardManager` (0x0f4ef8a6) function"]
        pub fn reward_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([15, 78, 248, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardPerToken` (0xcd3daf9d) function"]
        pub fn reward_per_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([205, 61, 175, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardPerTokenStored` (0xdf136d65) function"]
        pub fn reward_per_token_stored(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 19, 109, 101], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewardRate` (0x7b0a47ee) function"]
        pub fn reward_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([123, 10, 71, 238], ())
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
        #[doc = "Calls the contract's `rewards` (0x0700037d) function"]
        pub fn rewards(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 0, 3, 125], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stake` (0xa694fc3a) function"]
        pub fn stake(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 148, 252, 58], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakeAll` (0x8dcb4061) function"]
        pub fn stake_all(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([141, 203, 64, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakeFor` (0x2ee40908) function"]
        pub fn stake_for(
            &self,
            for_: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([46, 228, 9, 8], (for_, amount))
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
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `userRewardPerTokenPaid` (0x8b876347) function"]
        pub fn user_reward_per_token_paid(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 135, 99, 71], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x38d07436) function"]
        pub fn withdraw(
            &self,
            amount: ethers::core::types::U256,
            claim: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([56, 208, 116, 54], (amount, claim))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAll` (0x1c1c6fe5) function"]
        pub fn withdraw_all(&self, claim: bool) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 28, 111, 229], claim)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAllAndUnwrap` (0x49f039a2) function"]
        pub fn withdraw_all_and_unwrap(
            &self,
            claim: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 240, 57, 162], claim)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAndUnwrap` (0xc32e7202) function"]
        pub fn withdraw_and_unwrap(
            &self,
            amount: ethers::core::types::U256,
            claim: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 46, 114, 2], (amount, claim))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IBaseRewardPool<M> {
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
    #[doc = "Container type for all input parameters for the `currentRewards`function with signature `currentRewards()` and selector `[144, 26, 125, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "currentRewards", abi = "currentRewards()")]
    pub struct CurrentRewardsCall;
    #[doc = "Container type for all input parameters for the `donate`function with signature `donate(uint256)` and selector `[241, 79, 175, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "donate", abi = "donate(uint256)")]
    pub struct DonateCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `duration`function with signature `duration()` and selector `[15, 181, 166, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "duration", abi = "duration()")]
    pub struct DurationCall;
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
    #[doc = "Container type for all input parameters for the `extraRewards`function with signature `extraRewards(uint256)` and selector `[64, 195, 84, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "extraRewards", abi = "extraRewards(uint256)")]
    pub struct ExtraRewardsCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `extraRewardsLength`function with signature `extraRewardsLength()` and selector `[213, 90, 35, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "extraRewardsLength", abi = "extraRewardsLength()")]
    pub struct ExtraRewardsLengthCall;
    #[doc = "Container type for all input parameters for the `getReward`function with signature `getReward()` and selector `[61, 24, 185, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReward", abi = "getReward()")]
    pub struct GetRewardCall;
    #[doc = "Container type for all input parameters for the `getReward`function with signature `getReward(address,bool)` and selector `[112, 80, 204, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getReward", abi = "getReward(address,bool)")]
    pub struct GetRewardWithAccountAndClaimExtrasCall {
        pub account: ethers::core::types::Address,
        pub claim_extras: bool,
    }
    #[doc = "Container type for all input parameters for the `historicalRewards`function with signature `historicalRewards()` and selector `[38, 45, 61, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "historicalRewards", abi = "historicalRewards()")]
    pub struct HistoricalRewardsCall;
    #[doc = "Container type for all input parameters for the `lastTimeRewardApplicable`function with signature `lastTimeRewardApplicable()` and selector `[128, 250, 165, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lastTimeRewardApplicable", abi = "lastTimeRewardApplicable()")]
    pub struct LastTimeRewardApplicableCall;
    #[doc = "Container type for all input parameters for the `lastUpdateTime`function with signature `lastUpdateTime()` and selector `[200, 243, 60, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lastUpdateTime", abi = "lastUpdateTime()")]
    pub struct LastUpdateTimeCall;
    #[doc = "Container type for all input parameters for the `newRewardRatio`function with signature `newRewardRatio()` and selector `[108, 139, 206, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "newRewardRatio", abi = "newRewardRatio()")]
    pub struct NewRewardRatioCall;
    #[doc = "Container type for all input parameters for the `operator`function with signature `operator()` and selector `[87, 12, 167, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "operator", abi = "operator()")]
    pub struct OperatorCall;
    #[doc = "Container type for all input parameters for the `periodFinish`function with signature `periodFinish()` and selector `[235, 226, 177, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "periodFinish", abi = "periodFinish()")]
    pub struct PeriodFinishCall;
    #[doc = "Container type for all input parameters for the `pid`function with signature `pid()` and selector `[241, 6, 132, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pid", abi = "pid()")]
    pub struct PidCall;
    #[doc = "Container type for all input parameters for the `queuedRewards`function with signature `queuedRewards()` and selector `[99, 211, 140, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "queuedRewards", abi = "queuedRewards()")]
    pub struct QueuedRewardsCall;
    #[doc = "Container type for all input parameters for the `rewardManager`function with signature `rewardManager()` and selector `[15, 78, 248, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardManager", abi = "rewardManager()")]
    pub struct RewardManagerCall;
    #[doc = "Container type for all input parameters for the `rewardPerToken`function with signature `rewardPerToken()` and selector `[205, 61, 175, 157]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardPerToken", abi = "rewardPerToken()")]
    pub struct RewardPerTokenCall;
    #[doc = "Container type for all input parameters for the `rewardPerTokenStored`function with signature `rewardPerTokenStored()` and selector `[223, 19, 109, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardPerTokenStored", abi = "rewardPerTokenStored()")]
    pub struct RewardPerTokenStoredCall;
    #[doc = "Container type for all input parameters for the `rewardRate`function with signature `rewardRate()` and selector `[123, 10, 71, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewardRate", abi = "rewardRate()")]
    pub struct RewardRateCall;
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
    #[doc = "Container type for all input parameters for the `rewards`function with signature `rewards(address)` and selector `[7, 0, 3, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewards", abi = "rewards(address)")]
    pub struct RewardsCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `stake`function with signature `stake(uint256)` and selector `[166, 148, 252, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stake", abi = "stake(uint256)")]
    pub struct StakeCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `stakeAll`function with signature `stakeAll()` and selector `[141, 203, 64, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stakeAll", abi = "stakeAll()")]
    pub struct StakeAllCall;
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
    pub struct StakeForCall {
        pub for_: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `userRewardPerTokenPaid`function with signature `userRewardPerTokenPaid(address)` and selector `[139, 135, 99, 71]`"]
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
        name = "userRewardPerTokenPaid",
        abi = "userRewardPerTokenPaid(address)"
    )]
    pub struct UserRewardPerTokenPaidCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,bool)` and selector `[56, 208, 116, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,bool)")]
    pub struct WithdrawCall {
        pub amount: ethers::core::types::U256,
        pub claim: bool,
    }
    #[doc = "Container type for all input parameters for the `withdrawAll`function with signature `withdrawAll(bool)` and selector `[28, 28, 111, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawAll", abi = "withdrawAll(bool)")]
    pub struct WithdrawAllCall {
        pub claim: bool,
    }
    #[doc = "Container type for all input parameters for the `withdrawAllAndUnwrap`function with signature `withdrawAllAndUnwrap(bool)` and selector `[73, 240, 57, 162]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawAllAndUnwrap", abi = "withdrawAllAndUnwrap(bool)")]
    pub struct WithdrawAllAndUnwrapCall {
        pub claim: bool,
    }
    #[doc = "Container type for all input parameters for the `withdrawAndUnwrap`function with signature `withdrawAndUnwrap(uint256,bool)` and selector `[195, 46, 114, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawAndUnwrap", abi = "withdrawAndUnwrap(uint256,bool)")]
    pub struct WithdrawAndUnwrapCall {
        pub amount: ethers::core::types::U256,
        pub claim: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IBaseRewardPoolCalls {
        BalanceOf(BalanceOfCall),
        CurrentRewards(CurrentRewardsCall),
        Donate(DonateCall),
        Duration(DurationCall),
        Earned(EarnedCall),
        ExtraRewards(ExtraRewardsCall),
        ExtraRewardsLength(ExtraRewardsLengthCall),
        GetReward(GetRewardCall),
        GetRewardWithAccountAndClaimExtras(GetRewardWithAccountAndClaimExtrasCall),
        HistoricalRewards(HistoricalRewardsCall),
        LastTimeRewardApplicable(LastTimeRewardApplicableCall),
        LastUpdateTime(LastUpdateTimeCall),
        NewRewardRatio(NewRewardRatioCall),
        Operator(OperatorCall),
        PeriodFinish(PeriodFinishCall),
        Pid(PidCall),
        QueuedRewards(QueuedRewardsCall),
        RewardManager(RewardManagerCall),
        RewardPerToken(RewardPerTokenCall),
        RewardPerTokenStored(RewardPerTokenStoredCall),
        RewardRate(RewardRateCall),
        RewardToken(RewardTokenCall),
        Rewards(RewardsCall),
        Stake(StakeCall),
        StakeAll(StakeAllCall),
        StakeFor(StakeForCall),
        StakingToken(StakingTokenCall),
        TotalSupply(TotalSupplyCall),
        UserRewardPerTokenPaid(UserRewardPerTokenPaidCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
        WithdrawAllAndUnwrap(WithdrawAllAndUnwrapCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
    }
    impl ethers::core::abi::AbiDecode for IBaseRewardPoolCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CurrentRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::CurrentRewards(decoded));
            }
            if let Ok(decoded) = <DonateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Donate(decoded));
            }
            if let Ok(decoded) =
                <DurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Duration(decoded));
            }
            if let Ok(decoded) = <EarnedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Earned(decoded));
            }
            if let Ok(decoded) =
                <ExtraRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::ExtraRewards(decoded));
            }
            if let Ok(decoded) =
                <ExtraRewardsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::ExtraRewardsLength(decoded));
            }
            if let Ok(decoded) =
                <GetRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::GetReward(decoded));
            }
            if let Ok(decoded) =
                <GetRewardWithAccountAndClaimExtrasCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HistoricalRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::HistoricalRewards(decoded));
            }
            if let Ok(decoded) =
                <LastTimeRewardApplicableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IBaseRewardPoolCalls::LastTimeRewardApplicable(decoded));
            }
            if let Ok(decoded) =
                <LastUpdateTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::LastUpdateTime(decoded));
            }
            if let Ok(decoded) =
                <NewRewardRatioCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::NewRewardRatio(decoded));
            }
            if let Ok(decoded) =
                <OperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Operator(decoded));
            }
            if let Ok(decoded) =
                <PeriodFinishCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::PeriodFinish(decoded));
            }
            if let Ok(decoded) = <PidCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IBaseRewardPoolCalls::Pid(decoded));
            }
            if let Ok(decoded) =
                <QueuedRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::QueuedRewards(decoded));
            }
            if let Ok(decoded) =
                <RewardManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::RewardManager(decoded));
            }
            if let Ok(decoded) =
                <RewardPerTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::RewardPerToken(decoded));
            }
            if let Ok(decoded) =
                <RewardPerTokenStoredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::RewardPerTokenStored(decoded));
            }
            if let Ok(decoded) =
                <RewardRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::RewardRate(decoded));
            }
            if let Ok(decoded) =
                <RewardTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::RewardToken(decoded));
            }
            if let Ok(decoded) =
                <RewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Rewards(decoded));
            }
            if let Ok(decoded) = <StakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Stake(decoded));
            }
            if let Ok(decoded) =
                <StakeAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::StakeAll(decoded));
            }
            if let Ok(decoded) =
                <StakeForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::StakeFor(decoded));
            }
            if let Ok(decoded) =
                <StakingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::StakingToken(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <UserRewardPerTokenPaidCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::UserRewardPerTokenPaid(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::WithdrawAll(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::WithdrawAllAndUnwrap(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBaseRewardPoolCalls::WithdrawAndUnwrap(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IBaseRewardPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IBaseRewardPoolCalls::BalanceOf(element) => element.encode(),
                IBaseRewardPoolCalls::CurrentRewards(element) => element.encode(),
                IBaseRewardPoolCalls::Donate(element) => element.encode(),
                IBaseRewardPoolCalls::Duration(element) => element.encode(),
                IBaseRewardPoolCalls::Earned(element) => element.encode(),
                IBaseRewardPoolCalls::ExtraRewards(element) => element.encode(),
                IBaseRewardPoolCalls::ExtraRewardsLength(element) => element.encode(),
                IBaseRewardPoolCalls::GetReward(element) => element.encode(),
                IBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(element) => {
                    element.encode()
                }
                IBaseRewardPoolCalls::HistoricalRewards(element) => element.encode(),
                IBaseRewardPoolCalls::LastTimeRewardApplicable(element) => element.encode(),
                IBaseRewardPoolCalls::LastUpdateTime(element) => element.encode(),
                IBaseRewardPoolCalls::NewRewardRatio(element) => element.encode(),
                IBaseRewardPoolCalls::Operator(element) => element.encode(),
                IBaseRewardPoolCalls::PeriodFinish(element) => element.encode(),
                IBaseRewardPoolCalls::Pid(element) => element.encode(),
                IBaseRewardPoolCalls::QueuedRewards(element) => element.encode(),
                IBaseRewardPoolCalls::RewardManager(element) => element.encode(),
                IBaseRewardPoolCalls::RewardPerToken(element) => element.encode(),
                IBaseRewardPoolCalls::RewardPerTokenStored(element) => element.encode(),
                IBaseRewardPoolCalls::RewardRate(element) => element.encode(),
                IBaseRewardPoolCalls::RewardToken(element) => element.encode(),
                IBaseRewardPoolCalls::Rewards(element) => element.encode(),
                IBaseRewardPoolCalls::Stake(element) => element.encode(),
                IBaseRewardPoolCalls::StakeAll(element) => element.encode(),
                IBaseRewardPoolCalls::StakeFor(element) => element.encode(),
                IBaseRewardPoolCalls::StakingToken(element) => element.encode(),
                IBaseRewardPoolCalls::TotalSupply(element) => element.encode(),
                IBaseRewardPoolCalls::UserRewardPerTokenPaid(element) => element.encode(),
                IBaseRewardPoolCalls::Withdraw(element) => element.encode(),
                IBaseRewardPoolCalls::WithdrawAll(element) => element.encode(),
                IBaseRewardPoolCalls::WithdrawAllAndUnwrap(element) => element.encode(),
                IBaseRewardPoolCalls::WithdrawAndUnwrap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IBaseRewardPoolCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IBaseRewardPoolCalls::BalanceOf(element) => element.fmt(f),
                IBaseRewardPoolCalls::CurrentRewards(element) => element.fmt(f),
                IBaseRewardPoolCalls::Donate(element) => element.fmt(f),
                IBaseRewardPoolCalls::Duration(element) => element.fmt(f),
                IBaseRewardPoolCalls::Earned(element) => element.fmt(f),
                IBaseRewardPoolCalls::ExtraRewards(element) => element.fmt(f),
                IBaseRewardPoolCalls::ExtraRewardsLength(element) => element.fmt(f),
                IBaseRewardPoolCalls::GetReward(element) => element.fmt(f),
                IBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(element) => element.fmt(f),
                IBaseRewardPoolCalls::HistoricalRewards(element) => element.fmt(f),
                IBaseRewardPoolCalls::LastTimeRewardApplicable(element) => element.fmt(f),
                IBaseRewardPoolCalls::LastUpdateTime(element) => element.fmt(f),
                IBaseRewardPoolCalls::NewRewardRatio(element) => element.fmt(f),
                IBaseRewardPoolCalls::Operator(element) => element.fmt(f),
                IBaseRewardPoolCalls::PeriodFinish(element) => element.fmt(f),
                IBaseRewardPoolCalls::Pid(element) => element.fmt(f),
                IBaseRewardPoolCalls::QueuedRewards(element) => element.fmt(f),
                IBaseRewardPoolCalls::RewardManager(element) => element.fmt(f),
                IBaseRewardPoolCalls::RewardPerToken(element) => element.fmt(f),
                IBaseRewardPoolCalls::RewardPerTokenStored(element) => element.fmt(f),
                IBaseRewardPoolCalls::RewardRate(element) => element.fmt(f),
                IBaseRewardPoolCalls::RewardToken(element) => element.fmt(f),
                IBaseRewardPoolCalls::Rewards(element) => element.fmt(f),
                IBaseRewardPoolCalls::Stake(element) => element.fmt(f),
                IBaseRewardPoolCalls::StakeAll(element) => element.fmt(f),
                IBaseRewardPoolCalls::StakeFor(element) => element.fmt(f),
                IBaseRewardPoolCalls::StakingToken(element) => element.fmt(f),
                IBaseRewardPoolCalls::TotalSupply(element) => element.fmt(f),
                IBaseRewardPoolCalls::UserRewardPerTokenPaid(element) => element.fmt(f),
                IBaseRewardPoolCalls::Withdraw(element) => element.fmt(f),
                IBaseRewardPoolCalls::WithdrawAll(element) => element.fmt(f),
                IBaseRewardPoolCalls::WithdrawAllAndUnwrap(element) => element.fmt(f),
                IBaseRewardPoolCalls::WithdrawAndUnwrap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IBaseRewardPoolCalls {
        fn from(var: BalanceOfCall) -> Self {
            IBaseRewardPoolCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CurrentRewardsCall> for IBaseRewardPoolCalls {
        fn from(var: CurrentRewardsCall) -> Self {
            IBaseRewardPoolCalls::CurrentRewards(var)
        }
    }
    impl ::std::convert::From<DonateCall> for IBaseRewardPoolCalls {
        fn from(var: DonateCall) -> Self {
            IBaseRewardPoolCalls::Donate(var)
        }
    }
    impl ::std::convert::From<DurationCall> for IBaseRewardPoolCalls {
        fn from(var: DurationCall) -> Self {
            IBaseRewardPoolCalls::Duration(var)
        }
    }
    impl ::std::convert::From<EarnedCall> for IBaseRewardPoolCalls {
        fn from(var: EarnedCall) -> Self {
            IBaseRewardPoolCalls::Earned(var)
        }
    }
    impl ::std::convert::From<ExtraRewardsCall> for IBaseRewardPoolCalls {
        fn from(var: ExtraRewardsCall) -> Self {
            IBaseRewardPoolCalls::ExtraRewards(var)
        }
    }
    impl ::std::convert::From<ExtraRewardsLengthCall> for IBaseRewardPoolCalls {
        fn from(var: ExtraRewardsLengthCall) -> Self {
            IBaseRewardPoolCalls::ExtraRewardsLength(var)
        }
    }
    impl ::std::convert::From<GetRewardCall> for IBaseRewardPoolCalls {
        fn from(var: GetRewardCall) -> Self {
            IBaseRewardPoolCalls::GetReward(var)
        }
    }
    impl ::std::convert::From<GetRewardWithAccountAndClaimExtrasCall> for IBaseRewardPoolCalls {
        fn from(var: GetRewardWithAccountAndClaimExtrasCall) -> Self {
            IBaseRewardPoolCalls::GetRewardWithAccountAndClaimExtras(var)
        }
    }
    impl ::std::convert::From<HistoricalRewardsCall> for IBaseRewardPoolCalls {
        fn from(var: HistoricalRewardsCall) -> Self {
            IBaseRewardPoolCalls::HistoricalRewards(var)
        }
    }
    impl ::std::convert::From<LastTimeRewardApplicableCall> for IBaseRewardPoolCalls {
        fn from(var: LastTimeRewardApplicableCall) -> Self {
            IBaseRewardPoolCalls::LastTimeRewardApplicable(var)
        }
    }
    impl ::std::convert::From<LastUpdateTimeCall> for IBaseRewardPoolCalls {
        fn from(var: LastUpdateTimeCall) -> Self {
            IBaseRewardPoolCalls::LastUpdateTime(var)
        }
    }
    impl ::std::convert::From<NewRewardRatioCall> for IBaseRewardPoolCalls {
        fn from(var: NewRewardRatioCall) -> Self {
            IBaseRewardPoolCalls::NewRewardRatio(var)
        }
    }
    impl ::std::convert::From<OperatorCall> for IBaseRewardPoolCalls {
        fn from(var: OperatorCall) -> Self {
            IBaseRewardPoolCalls::Operator(var)
        }
    }
    impl ::std::convert::From<PeriodFinishCall> for IBaseRewardPoolCalls {
        fn from(var: PeriodFinishCall) -> Self {
            IBaseRewardPoolCalls::PeriodFinish(var)
        }
    }
    impl ::std::convert::From<PidCall> for IBaseRewardPoolCalls {
        fn from(var: PidCall) -> Self {
            IBaseRewardPoolCalls::Pid(var)
        }
    }
    impl ::std::convert::From<QueuedRewardsCall> for IBaseRewardPoolCalls {
        fn from(var: QueuedRewardsCall) -> Self {
            IBaseRewardPoolCalls::QueuedRewards(var)
        }
    }
    impl ::std::convert::From<RewardManagerCall> for IBaseRewardPoolCalls {
        fn from(var: RewardManagerCall) -> Self {
            IBaseRewardPoolCalls::RewardManager(var)
        }
    }
    impl ::std::convert::From<RewardPerTokenCall> for IBaseRewardPoolCalls {
        fn from(var: RewardPerTokenCall) -> Self {
            IBaseRewardPoolCalls::RewardPerToken(var)
        }
    }
    impl ::std::convert::From<RewardPerTokenStoredCall> for IBaseRewardPoolCalls {
        fn from(var: RewardPerTokenStoredCall) -> Self {
            IBaseRewardPoolCalls::RewardPerTokenStored(var)
        }
    }
    impl ::std::convert::From<RewardRateCall> for IBaseRewardPoolCalls {
        fn from(var: RewardRateCall) -> Self {
            IBaseRewardPoolCalls::RewardRate(var)
        }
    }
    impl ::std::convert::From<RewardTokenCall> for IBaseRewardPoolCalls {
        fn from(var: RewardTokenCall) -> Self {
            IBaseRewardPoolCalls::RewardToken(var)
        }
    }
    impl ::std::convert::From<RewardsCall> for IBaseRewardPoolCalls {
        fn from(var: RewardsCall) -> Self {
            IBaseRewardPoolCalls::Rewards(var)
        }
    }
    impl ::std::convert::From<StakeCall> for IBaseRewardPoolCalls {
        fn from(var: StakeCall) -> Self {
            IBaseRewardPoolCalls::Stake(var)
        }
    }
    impl ::std::convert::From<StakeAllCall> for IBaseRewardPoolCalls {
        fn from(var: StakeAllCall) -> Self {
            IBaseRewardPoolCalls::StakeAll(var)
        }
    }
    impl ::std::convert::From<StakeForCall> for IBaseRewardPoolCalls {
        fn from(var: StakeForCall) -> Self {
            IBaseRewardPoolCalls::StakeFor(var)
        }
    }
    impl ::std::convert::From<StakingTokenCall> for IBaseRewardPoolCalls {
        fn from(var: StakingTokenCall) -> Self {
            IBaseRewardPoolCalls::StakingToken(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IBaseRewardPoolCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IBaseRewardPoolCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<UserRewardPerTokenPaidCall> for IBaseRewardPoolCalls {
        fn from(var: UserRewardPerTokenPaidCall) -> Self {
            IBaseRewardPoolCalls::UserRewardPerTokenPaid(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IBaseRewardPoolCalls {
        fn from(var: WithdrawCall) -> Self {
            IBaseRewardPoolCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAllCall> for IBaseRewardPoolCalls {
        fn from(var: WithdrawAllCall) -> Self {
            IBaseRewardPoolCalls::WithdrawAll(var)
        }
    }
    impl ::std::convert::From<WithdrawAllAndUnwrapCall> for IBaseRewardPoolCalls {
        fn from(var: WithdrawAllAndUnwrapCall) -> Self {
            IBaseRewardPoolCalls::WithdrawAllAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for IBaseRewardPoolCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            IBaseRewardPoolCalls::WithdrawAndUnwrap(var)
        }
    }
}
