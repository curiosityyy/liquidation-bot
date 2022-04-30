pub use istaker_mod::*;
#[allow(clippy::too_many_arguments)]
mod istaker_mod {
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
    #[doc = "IStaker was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ISTAKER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfPool\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimCrv\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createLock\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseTime\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"operator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"release\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStashAccess\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"vote\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"voteGaugeWeight\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAll\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ISTAKER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IStaker<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IStaker<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IStaker<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IStaker))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IStaker<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISTAKER_ABI.clone(), client).into()
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
                ISTAKER_ABI.clone(),
                ISTAKER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balanceOfPool` (0xb0f63794) function"]
        pub fn balance_of_pool(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([176, 246, 55, 148], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimCrv` (0x3fe9bc06) function"]
        pub fn claim_crv(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([63, 233, 188, 6], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimFees` (0x2dbfa735) function"]
        pub fn claim_fees(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 191, 167, 53], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewards` (0xef5cfb8c) function"]
        pub fn claim_rewards(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 92, 251, 140], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createLock` (0xb52c05fe) function"]
        pub fn create_lock(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 44, 5, 254], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xf9609f08) function"]
        pub fn deposit(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 96, 159, 8], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0xb61d27f6) function"]
        pub fn execute(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::Bytes)>
        {
            self.0
                .method_hash([182, 29, 39, 246], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAmount` (0x15456eba) function"]
        pub fn increase_amount(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 69, 110, 186], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseTime` (0x3c9a2a1a) function"]
        pub fn increase_time(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 154, 42, 26], p0)
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
        #[doc = "Calls the contract's `release` (0x86d1a69f) function"]
        pub fn release(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 209, 166, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStashAccess` (0xfa3964b2) function"]
        pub fn set_stash_access(
            &self,
            p0: ethers::core::types::Address,
            p1: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 57, 100, 178], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vote` (0xe2cdd42a) function"]
        pub fn vote(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Address,
            p2: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 205, 212, 42], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `voteGaugeWeight` (0x5d7e9bcb) function"]
        pub fn vote_gauge_weight(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 126, 155, 203], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x51cff8d9) function"]
        pub fn withdraw_0(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 207, 248, 217], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xd9caed12) function"]
        pub fn withdraw_1(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 202, 237, 18], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAll` (0x09cae2c8) function"]
        pub fn withdraw_all(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 202, 226, 200], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IStaker<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `balanceOfPool`function with signature `balanceOfPool(address)` and selector `[176, 246, 55, 148]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOfPool", abi = "balanceOfPool(address)")]
    pub struct BalanceOfPoolCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `claimCrv`function with signature `claimCrv(address)` and selector `[63, 233, 188, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimCrv", abi = "claimCrv(address)")]
    pub struct ClaimCrvCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `claimFees`function with signature `claimFees(address,address)` and selector `[45, 191, 167, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimFees", abi = "claimFees(address,address)")]
    pub struct ClaimFeesCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(address)` and selector `[239, 92, 251, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(address)")]
    pub struct ClaimRewardsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `createLock`function with signature `createLock(uint256,uint256)` and selector `[181, 44, 5, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createLock", abi = "createLock(uint256,uint256)")]
    pub struct CreateLockCall(pub ethers::core::types::U256, pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(address,address)` and selector `[249, 96, 159, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,address)")]
    pub struct DepositCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `execute`function with signature `execute(address,uint256,bytes)` and selector `[182, 29, 39, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "execute", abi = "execute(address,uint256,bytes)")]
    pub struct ExecuteCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `increaseAmount`function with signature `increaseAmount(uint256)` and selector `[21, 69, 110, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseAmount", abi = "increaseAmount(uint256)")]
    pub struct IncreaseAmountCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `increaseTime`function with signature `increaseTime(uint256)` and selector `[60, 154, 42, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseTime", abi = "increaseTime(uint256)")]
    pub struct IncreaseTimeCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `release`function with signature `release()` and selector `[134, 209, 166, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "release", abi = "release()")]
    pub struct ReleaseCall;
    #[doc = "Container type for all input parameters for the `setStashAccess`function with signature `setStashAccess(address,bool)` and selector `[250, 57, 100, 178]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setStashAccess", abi = "setStashAccess(address,bool)")]
    pub struct SetStashAccessCall(pub ethers::core::types::Address, pub bool);
    #[doc = "Container type for all input parameters for the `vote`function with signature `vote(uint256,address,bool)` and selector `[226, 205, 212, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vote", abi = "vote(uint256,address,bool)")]
    pub struct VoteCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub bool,
    );
    #[doc = "Container type for all input parameters for the `voteGaugeWeight`function with signature `voteGaugeWeight(address,uint256)` and selector `[93, 126, 155, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "voteGaugeWeight", abi = "voteGaugeWeight(address,uint256)")]
    pub struct VoteGaugeWeightCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(address)` and selector `[81, 207, 248, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address)")]
    pub struct Withdraw0Call(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(address,address,uint256)` and selector `[217, 202, 237, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,address,uint256)")]
    pub struct Withdraw1Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `withdrawAll`function with signature `withdrawAll(address,address)` and selector `[9, 202, 226, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawAll", abi = "withdrawAll(address,address)")]
    pub struct WithdrawAllCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IStakerCalls {
        BalanceOfPool(BalanceOfPoolCall),
        ClaimCrv(ClaimCrvCall),
        ClaimFees(ClaimFeesCall),
        ClaimRewards(ClaimRewardsCall),
        CreateLock(CreateLockCall),
        Deposit(DepositCall),
        Execute(ExecuteCall),
        IncreaseAmount(IncreaseAmountCall),
        IncreaseTime(IncreaseTimeCall),
        Operator(OperatorCall),
        Release(ReleaseCall),
        SetStashAccess(SetStashAccessCall),
        Vote(VoteCall),
        VoteGaugeWeight(VoteGaugeWeightCall),
        Withdraw0(Withdraw0Call),
        Withdraw1(Withdraw1Call),
        WithdrawAll(WithdrawAllCall),
    }
    impl ethers::core::abi::AbiDecode for IStakerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceOfPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::BalanceOfPool(decoded));
            }
            if let Ok(decoded) =
                <ClaimCrvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::ClaimCrv(decoded));
            }
            if let Ok(decoded) =
                <ClaimFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::ClaimFees(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <CreateLockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::CreateLock(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::IncreaseAmount(decoded));
            }
            if let Ok(decoded) =
                <IncreaseTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::IncreaseTime(decoded));
            }
            if let Ok(decoded) =
                <OperatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::Operator(decoded));
            }
            if let Ok(decoded) =
                <ReleaseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::Release(decoded));
            }
            if let Ok(decoded) =
                <SetStashAccessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::SetStashAccess(decoded));
            }
            if let Ok(decoded) = <VoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IStakerCalls::Vote(decoded));
            }
            if let Ok(decoded) =
                <VoteGaugeWeightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::VoteGaugeWeight(decoded));
            }
            if let Ok(decoded) =
                <Withdraw0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::Withdraw0(decoded));
            }
            if let Ok(decoded) =
                <Withdraw1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::Withdraw1(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IStakerCalls::WithdrawAll(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IStakerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IStakerCalls::BalanceOfPool(element) => element.encode(),
                IStakerCalls::ClaimCrv(element) => element.encode(),
                IStakerCalls::ClaimFees(element) => element.encode(),
                IStakerCalls::ClaimRewards(element) => element.encode(),
                IStakerCalls::CreateLock(element) => element.encode(),
                IStakerCalls::Deposit(element) => element.encode(),
                IStakerCalls::Execute(element) => element.encode(),
                IStakerCalls::IncreaseAmount(element) => element.encode(),
                IStakerCalls::IncreaseTime(element) => element.encode(),
                IStakerCalls::Operator(element) => element.encode(),
                IStakerCalls::Release(element) => element.encode(),
                IStakerCalls::SetStashAccess(element) => element.encode(),
                IStakerCalls::Vote(element) => element.encode(),
                IStakerCalls::VoteGaugeWeight(element) => element.encode(),
                IStakerCalls::Withdraw0(element) => element.encode(),
                IStakerCalls::Withdraw1(element) => element.encode(),
                IStakerCalls::WithdrawAll(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IStakerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IStakerCalls::BalanceOfPool(element) => element.fmt(f),
                IStakerCalls::ClaimCrv(element) => element.fmt(f),
                IStakerCalls::ClaimFees(element) => element.fmt(f),
                IStakerCalls::ClaimRewards(element) => element.fmt(f),
                IStakerCalls::CreateLock(element) => element.fmt(f),
                IStakerCalls::Deposit(element) => element.fmt(f),
                IStakerCalls::Execute(element) => element.fmt(f),
                IStakerCalls::IncreaseAmount(element) => element.fmt(f),
                IStakerCalls::IncreaseTime(element) => element.fmt(f),
                IStakerCalls::Operator(element) => element.fmt(f),
                IStakerCalls::Release(element) => element.fmt(f),
                IStakerCalls::SetStashAccess(element) => element.fmt(f),
                IStakerCalls::Vote(element) => element.fmt(f),
                IStakerCalls::VoteGaugeWeight(element) => element.fmt(f),
                IStakerCalls::Withdraw0(element) => element.fmt(f),
                IStakerCalls::Withdraw1(element) => element.fmt(f),
                IStakerCalls::WithdrawAll(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceOfPoolCall> for IStakerCalls {
        fn from(var: BalanceOfPoolCall) -> Self {
            IStakerCalls::BalanceOfPool(var)
        }
    }
    impl ::std::convert::From<ClaimCrvCall> for IStakerCalls {
        fn from(var: ClaimCrvCall) -> Self {
            IStakerCalls::ClaimCrv(var)
        }
    }
    impl ::std::convert::From<ClaimFeesCall> for IStakerCalls {
        fn from(var: ClaimFeesCall) -> Self {
            IStakerCalls::ClaimFees(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for IStakerCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            IStakerCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<CreateLockCall> for IStakerCalls {
        fn from(var: CreateLockCall) -> Self {
            IStakerCalls::CreateLock(var)
        }
    }
    impl ::std::convert::From<DepositCall> for IStakerCalls {
        fn from(var: DepositCall) -> Self {
            IStakerCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for IStakerCalls {
        fn from(var: ExecuteCall) -> Self {
            IStakerCalls::Execute(var)
        }
    }
    impl ::std::convert::From<IncreaseAmountCall> for IStakerCalls {
        fn from(var: IncreaseAmountCall) -> Self {
            IStakerCalls::IncreaseAmount(var)
        }
    }
    impl ::std::convert::From<IncreaseTimeCall> for IStakerCalls {
        fn from(var: IncreaseTimeCall) -> Self {
            IStakerCalls::IncreaseTime(var)
        }
    }
    impl ::std::convert::From<OperatorCall> for IStakerCalls {
        fn from(var: OperatorCall) -> Self {
            IStakerCalls::Operator(var)
        }
    }
    impl ::std::convert::From<ReleaseCall> for IStakerCalls {
        fn from(var: ReleaseCall) -> Self {
            IStakerCalls::Release(var)
        }
    }
    impl ::std::convert::From<SetStashAccessCall> for IStakerCalls {
        fn from(var: SetStashAccessCall) -> Self {
            IStakerCalls::SetStashAccess(var)
        }
    }
    impl ::std::convert::From<VoteCall> for IStakerCalls {
        fn from(var: VoteCall) -> Self {
            IStakerCalls::Vote(var)
        }
    }
    impl ::std::convert::From<VoteGaugeWeightCall> for IStakerCalls {
        fn from(var: VoteGaugeWeightCall) -> Self {
            IStakerCalls::VoteGaugeWeight(var)
        }
    }
    impl ::std::convert::From<Withdraw0Call> for IStakerCalls {
        fn from(var: Withdraw0Call) -> Self {
            IStakerCalls::Withdraw0(var)
        }
    }
    impl ::std::convert::From<Withdraw1Call> for IStakerCalls {
        fn from(var: Withdraw1Call) -> Self {
            IStakerCalls::Withdraw1(var)
        }
    }
    impl ::std::convert::From<WithdrawAllCall> for IStakerCalls {
        fn from(var: WithdrawAllCall) -> Self {
            IStakerCalls::WithdrawAll(var)
        }
    }
}
