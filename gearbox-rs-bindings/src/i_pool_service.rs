pub use ipoolservice_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipoolservice_mod {
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
    #[doc = "IPoolService was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPOOLSERVICE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddLiquidity\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Borrow\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"BorrowForbidden\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewCreditManagerConnected\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newLimit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewExpectedLiquidityLimit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newInterestRateModel\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewInterestRateModel\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewWithdrawFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RemoveLiquidity\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"profit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"loss\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Repay\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"loss\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UncoveredLoss\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_cumulativeIndex_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_timestampLU\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addLiquidity\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"addressProvider\",\"outputs\":[{\"internalType\":\"contract AddressProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"availableLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowAPY_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcLinearCumulative_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManagers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"id\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManagersCanBorrow\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManagersCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dieselToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"expectedLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"expectedLiquidityLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fromDiesel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDieselRate_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lendCreditAccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"profit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"loss\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"repayCreditAccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"toDiesel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalBorrowed\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"withdrawFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IPOOLSERVICE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IPoolService<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IPoolService<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IPoolService<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPoolService))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IPoolService<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IPOOLSERVICE_ABI.clone(), client).into()
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
                IPOOLSERVICE_ABI.clone(),
                IPOOLSERVICE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_cumulativeIndex_RAY` (0xdbcb313b) function"]
        pub fn cumulative_index_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 203, 49, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_timestampLU` (0x609ae317) function"]
        pub fn timestamp_lu(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([96, 154, 227, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidity` (0x9aa5d462) function"]
        pub fn add_liquidity(
            &self,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            referral_code: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 165, 212, 98], (amount, on_behalf_of, referral_code))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addressProvider` (0x2954018c) function"]
        pub fn address_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 84, 1, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `availableLiquidity` (0x74375359) function"]
        pub fn available_liquidity(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([116, 55, 83, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowAPY_RAY` (0x45d31f9d) function"]
        pub fn borrow_apy_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([69, 211, 31, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcLinearCumulative_RAY` (0x0fce70fb) function"]
        pub fn calc_linear_cumulative_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 206, 112, 251], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagers` (0x1e16e4fc) function"]
        pub fn credit_managers(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([30, 22, 228, 252], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagersCanBorrow` (0x2e97ca21) function"]
        pub fn credit_managers_can_borrow(
            &self,
            id: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([46, 151, 202, 33], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagersCount` (0xa4e8273e) function"]
        pub fn credit_managers_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([164, 232, 39, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dieselToken` (0x36dda7d5) function"]
        pub fn diesel_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([54, 221, 167, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectedLiquidity` (0xfe14112d) function"]
        pub fn expected_liquidity(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([254, 20, 17, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectedLiquidityLimit` (0xef8d9603) function"]
        pub fn expected_liquidity_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([239, 141, 150, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fromDiesel` (0x5427c938) function"]
        pub fn from_diesel(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 39, 201, 56], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDieselRate_RAY` (0x788c6bfe) function"]
        pub fn get_diesel_rate_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 140, 107, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lendCreditAccount` (0xbf28068b) function"]
        pub fn lend_credit_account(
            &self,
            borrowed_amount: ethers::core::types::U256,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 40, 6, 139], (borrowed_amount, credit_account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidity` (0x05fe138b) function"]
        pub fn remove_liquidity(
            &self,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([5, 254, 19, 139], (amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayCreditAccount` (0xca9505e4) function"]
        pub fn repay_credit_account(
            &self,
            borrowed_amount: ethers::core::types::U256,
            profit: ethers::core::types::U256,
            loss: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 149, 5, 228], (borrowed_amount, profit, loss))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `toDiesel` (0x4d778ad1) function"]
        pub fn to_diesel(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 119, 138, 209], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalBorrowed` (0x4c19386c) function"]
        pub fn total_borrowed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([76, 25, 56, 108], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFee` (0xe941fa78) function"]
        pub fn withdraw_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([233, 65, 250, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddLiquidity` event"]
        pub fn add_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers::contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowForbidden` event"]
        pub fn borrow_forbidden_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, BorrowForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCreditManagerConnected` event"]
        pub fn new_credit_manager_connected_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCreditManagerConnectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewExpectedLiquidityLimit` event"]
        pub fn new_expected_liquidity_limit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewExpectedLiquidityLimitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewInterestRateModel` event"]
        pub fn new_interest_rate_model_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewInterestRateModelFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewWithdrawFee` event"]
        pub fn new_withdraw_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewWithdrawFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemoveLiquidity` event"]
        pub fn remove_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RemoveLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repay` event"]
        pub fn repay_filter(&self) -> ethers::contract::builders::Event<M, RepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UncoveredLoss` event"]
        pub fn uncovered_loss_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UncoveredLossFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IPoolServiceEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IPoolService<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "AddLiquidity",
        abi = "AddLiquidity(address,address,uint256,uint256)"
    )]
    pub struct AddLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub referral_code: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Borrow", abi = "Borrow(address,address,uint256)")]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub credit_account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "BorrowForbidden", abi = "BorrowForbidden(address)")]
    pub struct BorrowForbiddenFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "NewCreditManagerConnected",
        abi = "NewCreditManagerConnected(address)"
    )]
    pub struct NewCreditManagerConnectedFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "NewExpectedLiquidityLimit",
        abi = "NewExpectedLiquidityLimit(uint256)"
    )]
    pub struct NewExpectedLiquidityLimitFilter {
        pub new_limit: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "NewInterestRateModel", abi = "NewInterestRateModel(address)")]
    pub struct NewInterestRateModelFilter {
        #[ethevent(indexed)]
        pub new_interest_rate_model: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "NewWithdrawFee", abi = "NewWithdrawFee(uint256)")]
    pub struct NewWithdrawFeeFilter {
        pub fee: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "RemoveLiquidity",
        abi = "RemoveLiquidity(address,address,uint256)"
    )]
    pub struct RemoveLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Repay", abi = "Repay(address,uint256,uint256,uint256)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
        pub borrowed_amount: ethers::core::types::U256,
        pub profit: ethers::core::types::U256,
        pub loss: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "UncoveredLoss", abi = "UncoveredLoss(address,uint256)")]
    pub struct UncoveredLossFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
        pub loss: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolServiceEvents {
        AddLiquidityFilter(AddLiquidityFilter),
        BorrowFilter(BorrowFilter),
        BorrowForbiddenFilter(BorrowForbiddenFilter),
        NewCreditManagerConnectedFilter(NewCreditManagerConnectedFilter),
        NewExpectedLiquidityLimitFilter(NewExpectedLiquidityLimitFilter),
        NewInterestRateModelFilter(NewInterestRateModelFilter),
        NewWithdrawFeeFilter(NewWithdrawFeeFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        RepayFilter(RepayFilter),
        UncoveredLossFilter(UncoveredLossFilter),
    }
    impl ethers::contract::EthLogDecode for IPoolServiceEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = BorrowForbiddenFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::BorrowForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewCreditManagerConnectedFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::NewCreditManagerConnectedFilter(decoded));
            }
            if let Ok(decoded) = NewExpectedLiquidityLimitFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::NewExpectedLiquidityLimitFilter(decoded));
            }
            if let Ok(decoded) = NewInterestRateModelFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::NewInterestRateModelFilter(decoded));
            }
            if let Ok(decoded) = NewWithdrawFeeFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::NewWithdrawFeeFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = UncoveredLossFilter::decode_log(log) {
                return Ok(IPoolServiceEvents::UncoveredLossFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IPoolServiceEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolServiceEvents::AddLiquidityFilter(element) => element.fmt(f),
                IPoolServiceEvents::BorrowFilter(element) => element.fmt(f),
                IPoolServiceEvents::BorrowForbiddenFilter(element) => element.fmt(f),
                IPoolServiceEvents::NewCreditManagerConnectedFilter(element) => element.fmt(f),
                IPoolServiceEvents::NewExpectedLiquidityLimitFilter(element) => element.fmt(f),
                IPoolServiceEvents::NewInterestRateModelFilter(element) => element.fmt(f),
                IPoolServiceEvents::NewWithdrawFeeFilter(element) => element.fmt(f),
                IPoolServiceEvents::RemoveLiquidityFilter(element) => element.fmt(f),
                IPoolServiceEvents::RepayFilter(element) => element.fmt(f),
                IPoolServiceEvents::UncoveredLossFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_cumulativeIndex_RAY`function with signature `_cumulativeIndex_RAY()` and selector `[219, 203, 49, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_cumulativeIndex_RAY", abi = "_cumulativeIndex_RAY()")]
    pub struct CumulativeIndexRAYCall;
    #[doc = "Container type for all input parameters for the `_timestampLU`function with signature `_timestampLU()` and selector `[96, 154, 227, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_timestampLU", abi = "_timestampLU()")]
    pub struct TimestampLUCall;
    #[doc = "Container type for all input parameters for the `addLiquidity`function with signature `addLiquidity(uint256,address,uint256)` and selector `[154, 165, 212, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addLiquidity", abi = "addLiquidity(uint256,address,uint256)")]
    pub struct AddLiquidityCall {
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `addressProvider`function with signature `addressProvider()` and selector `[41, 84, 1, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addressProvider", abi = "addressProvider()")]
    pub struct AddressProviderCall;
    #[doc = "Container type for all input parameters for the `availableLiquidity`function with signature `availableLiquidity()` and selector `[116, 55, 83, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "availableLiquidity", abi = "availableLiquidity()")]
    pub struct AvailableLiquidityCall;
    #[doc = "Container type for all input parameters for the `borrowAPY_RAY`function with signature `borrowAPY_RAY()` and selector `[69, 211, 31, 157]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowAPY_RAY", abi = "borrowAPY_RAY()")]
    pub struct BorrowAPYRAYCall;
    #[doc = "Container type for all input parameters for the `calcLinearCumulative_RAY`function with signature `calcLinearCumulative_RAY()` and selector `[15, 206, 112, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "calcLinearCumulative_RAY", abi = "calcLinearCumulative_RAY()")]
    pub struct CalcLinearCumulativeRAYCall;
    #[doc = "Container type for all input parameters for the `creditManagers`function with signature `creditManagers(uint256)` and selector `[30, 22, 228, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManagers", abi = "creditManagers(uint256)")]
    pub struct CreditManagersCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `creditManagersCanBorrow`function with signature `creditManagersCanBorrow(address)` and selector `[46, 151, 202, 33]`"]
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
        name = "creditManagersCanBorrow",
        abi = "creditManagersCanBorrow(address)"
    )]
    pub struct CreditManagersCanBorrowCall {
        pub id: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `creditManagersCount`function with signature `creditManagersCount()` and selector `[164, 232, 39, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManagersCount", abi = "creditManagersCount()")]
    pub struct CreditManagersCountCall;
    #[doc = "Container type for all input parameters for the `dieselToken`function with signature `dieselToken()` and selector `[54, 221, 167, 213]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "dieselToken", abi = "dieselToken()")]
    pub struct DieselTokenCall;
    #[doc = "Container type for all input parameters for the `expectedLiquidity`function with signature `expectedLiquidity()` and selector `[254, 20, 17, 45]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectedLiquidity", abi = "expectedLiquidity()")]
    pub struct ExpectedLiquidityCall;
    #[doc = "Container type for all input parameters for the `expectedLiquidityLimit`function with signature `expectedLiquidityLimit()` and selector `[239, 141, 150, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectedLiquidityLimit", abi = "expectedLiquidityLimit()")]
    pub struct ExpectedLiquidityLimitCall;
    #[doc = "Container type for all input parameters for the `fromDiesel`function with signature `fromDiesel(uint256)` and selector `[84, 39, 201, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fromDiesel", abi = "fromDiesel(uint256)")]
    pub struct FromDieselCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getDieselRate_RAY`function with signature `getDieselRate_RAY()` and selector `[120, 140, 107, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDieselRate_RAY", abi = "getDieselRate_RAY()")]
    pub struct GetDieselRateRAYCall;
    #[doc = "Container type for all input parameters for the `lendCreditAccount`function with signature `lendCreditAccount(uint256,address)` and selector `[191, 40, 6, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lendCreditAccount", abi = "lendCreditAccount(uint256,address)")]
    pub struct LendCreditAccountCall {
        pub borrowed_amount: ethers::core::types::U256,
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidity`function with signature `removeLiquidity(uint256,address)` and selector `[5, 254, 19, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeLiquidity", abi = "removeLiquidity(uint256,address)")]
    pub struct RemoveLiquidityCall {
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `repayCreditAccount`function with signature `repayCreditAccount(uint256,uint256,uint256)` and selector `[202, 149, 5, 228]`"]
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
        name = "repayCreditAccount",
        abi = "repayCreditAccount(uint256,uint256,uint256)"
    )]
    pub struct RepayCreditAccountCall {
        pub borrowed_amount: ethers::core::types::U256,
        pub profit: ethers::core::types::U256,
        pub loss: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `toDiesel`function with signature `toDiesel(uint256)` and selector `[77, 119, 138, 209]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "toDiesel", abi = "toDiesel(uint256)")]
    pub struct ToDieselCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalBorrowed`function with signature `totalBorrowed()` and selector `[76, 25, 56, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalBorrowed", abi = "totalBorrowed()")]
    pub struct TotalBorrowedCall;
    #[doc = "Container type for all input parameters for the `underlying`function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    #[doc = "Container type for all input parameters for the `version`function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[doc = "Container type for all input parameters for the `withdrawFee`function with signature `withdrawFee()` and selector `[233, 65, 250, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawFee", abi = "withdrawFee()")]
    pub struct WithdrawFeeCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IPoolServiceCalls {
        CumulativeIndexRAY(CumulativeIndexRAYCall),
        TimestampLU(TimestampLUCall),
        AddLiquidity(AddLiquidityCall),
        AddressProvider(AddressProviderCall),
        AvailableLiquidity(AvailableLiquidityCall),
        BorrowAPYRAY(BorrowAPYRAYCall),
        CalcLinearCumulativeRAY(CalcLinearCumulativeRAYCall),
        CreditManagers(CreditManagersCall),
        CreditManagersCanBorrow(CreditManagersCanBorrowCall),
        CreditManagersCount(CreditManagersCountCall),
        DieselToken(DieselTokenCall),
        ExpectedLiquidity(ExpectedLiquidityCall),
        ExpectedLiquidityLimit(ExpectedLiquidityLimitCall),
        FromDiesel(FromDieselCall),
        GetDieselRateRAY(GetDieselRateRAYCall),
        LendCreditAccount(LendCreditAccountCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RepayCreditAccount(RepayCreditAccountCall),
        ToDiesel(ToDieselCall),
        TotalBorrowed(TotalBorrowedCall),
        Underlying(UnderlyingCall),
        Version(VersionCall),
        WithdrawFee(WithdrawFeeCall),
    }
    impl ethers::core::abi::AbiDecode for IPoolServiceCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CumulativeIndexRAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::CumulativeIndexRAY(decoded));
            }
            if let Ok(decoded) =
                <TimestampLUCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::TimestampLU(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddressProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::AddressProvider(decoded));
            }
            if let Ok(decoded) =
                <AvailableLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::AvailableLiquidity(decoded));
            }
            if let Ok(decoded) =
                <BorrowAPYRAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::BorrowAPYRAY(decoded));
            }
            if let Ok(decoded) =
                <CalcLinearCumulativeRAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::CalcLinearCumulativeRAY(decoded));
            }
            if let Ok(decoded) =
                <CreditManagersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::CreditManagers(decoded));
            }
            if let Ok(decoded) =
                <CreditManagersCanBorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::CreditManagersCanBorrow(decoded));
            }
            if let Ok(decoded) =
                <CreditManagersCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::CreditManagersCount(decoded));
            }
            if let Ok(decoded) =
                <DieselTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::DieselToken(decoded));
            }
            if let Ok(decoded) =
                <ExpectedLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::ExpectedLiquidity(decoded));
            }
            if let Ok(decoded) =
                <ExpectedLiquidityLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::ExpectedLiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <FromDieselCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::FromDiesel(decoded));
            }
            if let Ok(decoded) =
                <GetDieselRateRAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::GetDieselRateRAY(decoded));
            }
            if let Ok(decoded) =
                <LendCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::LendCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RepayCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::RepayCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <ToDieselCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::ToDiesel(decoded));
            }
            if let Ok(decoded) =
                <TotalBorrowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::TotalBorrowed(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IPoolServiceCalls::WithdrawFee(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IPoolServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IPoolServiceCalls::CumulativeIndexRAY(element) => element.encode(),
                IPoolServiceCalls::TimestampLU(element) => element.encode(),
                IPoolServiceCalls::AddLiquidity(element) => element.encode(),
                IPoolServiceCalls::AddressProvider(element) => element.encode(),
                IPoolServiceCalls::AvailableLiquidity(element) => element.encode(),
                IPoolServiceCalls::BorrowAPYRAY(element) => element.encode(),
                IPoolServiceCalls::CalcLinearCumulativeRAY(element) => element.encode(),
                IPoolServiceCalls::CreditManagers(element) => element.encode(),
                IPoolServiceCalls::CreditManagersCanBorrow(element) => element.encode(),
                IPoolServiceCalls::CreditManagersCount(element) => element.encode(),
                IPoolServiceCalls::DieselToken(element) => element.encode(),
                IPoolServiceCalls::ExpectedLiquidity(element) => element.encode(),
                IPoolServiceCalls::ExpectedLiquidityLimit(element) => element.encode(),
                IPoolServiceCalls::FromDiesel(element) => element.encode(),
                IPoolServiceCalls::GetDieselRateRAY(element) => element.encode(),
                IPoolServiceCalls::LendCreditAccount(element) => element.encode(),
                IPoolServiceCalls::RemoveLiquidity(element) => element.encode(),
                IPoolServiceCalls::RepayCreditAccount(element) => element.encode(),
                IPoolServiceCalls::ToDiesel(element) => element.encode(),
                IPoolServiceCalls::TotalBorrowed(element) => element.encode(),
                IPoolServiceCalls::Underlying(element) => element.encode(),
                IPoolServiceCalls::Version(element) => element.encode(),
                IPoolServiceCalls::WithdrawFee(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IPoolServiceCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IPoolServiceCalls::CumulativeIndexRAY(element) => element.fmt(f),
                IPoolServiceCalls::TimestampLU(element) => element.fmt(f),
                IPoolServiceCalls::AddLiquidity(element) => element.fmt(f),
                IPoolServiceCalls::AddressProvider(element) => element.fmt(f),
                IPoolServiceCalls::AvailableLiquidity(element) => element.fmt(f),
                IPoolServiceCalls::BorrowAPYRAY(element) => element.fmt(f),
                IPoolServiceCalls::CalcLinearCumulativeRAY(element) => element.fmt(f),
                IPoolServiceCalls::CreditManagers(element) => element.fmt(f),
                IPoolServiceCalls::CreditManagersCanBorrow(element) => element.fmt(f),
                IPoolServiceCalls::CreditManagersCount(element) => element.fmt(f),
                IPoolServiceCalls::DieselToken(element) => element.fmt(f),
                IPoolServiceCalls::ExpectedLiquidity(element) => element.fmt(f),
                IPoolServiceCalls::ExpectedLiquidityLimit(element) => element.fmt(f),
                IPoolServiceCalls::FromDiesel(element) => element.fmt(f),
                IPoolServiceCalls::GetDieselRateRAY(element) => element.fmt(f),
                IPoolServiceCalls::LendCreditAccount(element) => element.fmt(f),
                IPoolServiceCalls::RemoveLiquidity(element) => element.fmt(f),
                IPoolServiceCalls::RepayCreditAccount(element) => element.fmt(f),
                IPoolServiceCalls::ToDiesel(element) => element.fmt(f),
                IPoolServiceCalls::TotalBorrowed(element) => element.fmt(f),
                IPoolServiceCalls::Underlying(element) => element.fmt(f),
                IPoolServiceCalls::Version(element) => element.fmt(f),
                IPoolServiceCalls::WithdrawFee(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CumulativeIndexRAYCall> for IPoolServiceCalls {
        fn from(var: CumulativeIndexRAYCall) -> Self {
            IPoolServiceCalls::CumulativeIndexRAY(var)
        }
    }
    impl ::std::convert::From<TimestampLUCall> for IPoolServiceCalls {
        fn from(var: TimestampLUCall) -> Self {
            IPoolServiceCalls::TimestampLU(var)
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for IPoolServiceCalls {
        fn from(var: AddLiquidityCall) -> Self {
            IPoolServiceCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<AddressProviderCall> for IPoolServiceCalls {
        fn from(var: AddressProviderCall) -> Self {
            IPoolServiceCalls::AddressProvider(var)
        }
    }
    impl ::std::convert::From<AvailableLiquidityCall> for IPoolServiceCalls {
        fn from(var: AvailableLiquidityCall) -> Self {
            IPoolServiceCalls::AvailableLiquidity(var)
        }
    }
    impl ::std::convert::From<BorrowAPYRAYCall> for IPoolServiceCalls {
        fn from(var: BorrowAPYRAYCall) -> Self {
            IPoolServiceCalls::BorrowAPYRAY(var)
        }
    }
    impl ::std::convert::From<CalcLinearCumulativeRAYCall> for IPoolServiceCalls {
        fn from(var: CalcLinearCumulativeRAYCall) -> Self {
            IPoolServiceCalls::CalcLinearCumulativeRAY(var)
        }
    }
    impl ::std::convert::From<CreditManagersCall> for IPoolServiceCalls {
        fn from(var: CreditManagersCall) -> Self {
            IPoolServiceCalls::CreditManagers(var)
        }
    }
    impl ::std::convert::From<CreditManagersCanBorrowCall> for IPoolServiceCalls {
        fn from(var: CreditManagersCanBorrowCall) -> Self {
            IPoolServiceCalls::CreditManagersCanBorrow(var)
        }
    }
    impl ::std::convert::From<CreditManagersCountCall> for IPoolServiceCalls {
        fn from(var: CreditManagersCountCall) -> Self {
            IPoolServiceCalls::CreditManagersCount(var)
        }
    }
    impl ::std::convert::From<DieselTokenCall> for IPoolServiceCalls {
        fn from(var: DieselTokenCall) -> Self {
            IPoolServiceCalls::DieselToken(var)
        }
    }
    impl ::std::convert::From<ExpectedLiquidityCall> for IPoolServiceCalls {
        fn from(var: ExpectedLiquidityCall) -> Self {
            IPoolServiceCalls::ExpectedLiquidity(var)
        }
    }
    impl ::std::convert::From<ExpectedLiquidityLimitCall> for IPoolServiceCalls {
        fn from(var: ExpectedLiquidityLimitCall) -> Self {
            IPoolServiceCalls::ExpectedLiquidityLimit(var)
        }
    }
    impl ::std::convert::From<FromDieselCall> for IPoolServiceCalls {
        fn from(var: FromDieselCall) -> Self {
            IPoolServiceCalls::FromDiesel(var)
        }
    }
    impl ::std::convert::From<GetDieselRateRAYCall> for IPoolServiceCalls {
        fn from(var: GetDieselRateRAYCall) -> Self {
            IPoolServiceCalls::GetDieselRateRAY(var)
        }
    }
    impl ::std::convert::From<LendCreditAccountCall> for IPoolServiceCalls {
        fn from(var: LendCreditAccountCall) -> Self {
            IPoolServiceCalls::LendCreditAccount(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for IPoolServiceCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            IPoolServiceCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RepayCreditAccountCall> for IPoolServiceCalls {
        fn from(var: RepayCreditAccountCall) -> Self {
            IPoolServiceCalls::RepayCreditAccount(var)
        }
    }
    impl ::std::convert::From<ToDieselCall> for IPoolServiceCalls {
        fn from(var: ToDieselCall) -> Self {
            IPoolServiceCalls::ToDiesel(var)
        }
    }
    impl ::std::convert::From<TotalBorrowedCall> for IPoolServiceCalls {
        fn from(var: TotalBorrowedCall) -> Self {
            IPoolServiceCalls::TotalBorrowed(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for IPoolServiceCalls {
        fn from(var: UnderlyingCall) -> Self {
            IPoolServiceCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VersionCall> for IPoolServiceCalls {
        fn from(var: VersionCall) -> Self {
            IPoolServiceCalls::Version(var)
        }
    }
    impl ::std::convert::From<WithdrawFeeCall> for IPoolServiceCalls {
        fn from(var: WithdrawFeeCall) -> Self {
            IPoolServiceCalls::WithdrawFee(var)
        }
    }
}
