pub use icreditmanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditmanager_mod {
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
    #[doc = "ICreditManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITMANAGER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AdaptersOrFacadeOnlyException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BorrowAmountOutOfLimitsException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HasNoOpenedAccountException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectLimitsException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotCreditConfiguratorException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotCreditFacadeException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotEnoughCollateralException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TargetContractNotAllowedExpcetion\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TokenAlreadyAddedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TokenNotAllowedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TooMuchTokensException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressOrUserAlreadyHasAccountException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExecuteOrder\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newConfigurator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewConfigurator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"adapter\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"adapterToContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"payer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedTokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedTokensCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveCreditAccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"totalValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isLiquidated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmountWithInterest\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcClosePayments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountToPool\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"remainingFunds\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"profit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"loss\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcCreditAccountAccruedInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmountWithInterest\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"checkAndEnableToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isLiquidated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"skipTokenMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"convertWETH\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"closeCreditAccount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"remainingFunds\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditAccounts\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditConfigurator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"enabledTokensMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeOrder\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balanceInBefore\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balanceOutBefore\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fastCollateralCheck\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeInterest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeLiquidation\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"forbidenTokenMask\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fullCollateralCheck\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditAccountOrRevert\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationDiscount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"liquidationThresholds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"increase\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"manageDebt\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"newBorrowedAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxBorrowedAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minBorrowedAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"openCreditAccount\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolService\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceOracle\",\"outputs\":[{\"internalType\":\"contract IPriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenMasksMap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAccountOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wethAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITMANAGER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditManager<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditManager<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICREDITMANAGER_ABI.clone(), client)
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
                ICREDITMANAGER_ABI.clone(),
                ICREDITMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `adapterToContract` (0xff687543) function"]
        pub fn adapter_to_contract(
            &self,
            adapter: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([255, 104, 117, 67], adapter)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addCollateral` (0x830aa745) function"]
        pub fn add_collateral(
            &self,
            payer: ethers::core::types::Address,
            on_behalf_of: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 10, 167, 69], (payer, on_behalf_of, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedTokens` (0x5e5f2e26) function"]
        pub fn allowed_tokens(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 95, 46, 38], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedTokensCount` (0x20a05ff7) function"]
        pub fn allowed_tokens_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([32, 160, 95, 247], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveCreditAccount` (0x46fb371d) function"]
        pub fn approve_credit_account(
            &self,
            borrower: ethers::core::types::Address,
            target_contract: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [70, 251, 55, 29],
                    (borrower, target_contract, token, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcClosePayments` (0x76f54966) function"]
        pub fn calc_close_payments(
            &self,
            total_value: ethers::core::types::U256,
            is_liquidated: bool,
            borrowed_amount: ethers::core::types::U256,
            borrowed_amount_with_interest: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [118, 245, 73, 102],
                    (
                        total_value,
                        is_liquidated,
                        borrowed_amount,
                        borrowed_amount_with_interest,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcCreditAccountAccruedInterest` (0x3192195c) function"]
        pub fn calc_credit_account_accrued_interest(
            &self,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([49, 146, 25, 92], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkAndEnableToken` (0x51e3f160) function"]
        pub fn check_and_enable_token(
            &self,
            credit_account: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 227, 241, 96], (credit_account, token_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeCreditAccount` (0x8c5c9fe1) function"]
        pub fn close_credit_account(
            &self,
            borrower: ethers::core::types::Address,
            is_liquidated: bool,
            total_value: ethers::core::types::U256,
            caller: ethers::core::types::Address,
            to: ethers::core::types::Address,
            skip_token_mask: ethers::core::types::U256,
            convert_weth: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [140, 92, 159, 225],
                    (
                        borrower,
                        is_liquidated,
                        total_value,
                        caller,
                        to,
                        skip_token_mask,
                        convert_weth,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditAccounts` (0x055ee9b5) function"]
        pub fn credit_accounts(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 94, 233, 181], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditConfigurator` (0xf9aa028a) function"]
        pub fn credit_configurator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([249, 170, 2, 138], ())
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
        #[doc = "Calls the contract's `enabledTokensMap` (0x8991b2f1) function"]
        pub fn enabled_tokens_map(
            &self,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([137, 145, 178, 241], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOrder` (0x6ce4074a) function"]
        pub fn execute_order(
            &self,
            borrower: ethers::core::types::Address,
            target: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([108, 228, 7, 74], (borrower, target, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fastCollateralCheck` (0x654a9eda) function"]
        pub fn fast_collateral_check(
            &self,
            credit_account: ethers::core::types::Address,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            balance_in_before: ethers::core::types::U256,
            balance_out_before: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [101, 74, 158, 218],
                    (
                        credit_account,
                        token_in,
                        token_out,
                        balance_in_before,
                        balance_out_before,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeInterest` (0x5e0b63d3) function"]
        pub fn fee_interest(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 11, 99, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeLiquidation` (0x3915ffaa) function"]
        pub fn fee_liquidation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([57, 21, 255, 170], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidenTokenMask` (0xe46c95bc) function"]
        pub fn forbiden_token_mask(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([228, 108, 149, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fullCollateralCheck` (0x95373018) function"]
        pub fn full_collateral_check(
            &self,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 55, 48, 24], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountOrRevert` (0xe958b704) function"]
        pub fn get_credit_account_or_revert(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([233, 88, 183, 4], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationDiscount` (0x8053fcbe) function"]
        pub fn liquidation_discount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([128, 83, 252, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationThresholds` (0x78327438) function"]
        pub fn liquidation_thresholds(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 50, 116, 56], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manageDebt` (0x94cf073a) function"]
        pub fn manage_debt(
            &self,
            borrower: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            increase: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 207, 7, 58], (borrower, amount, increase))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxBorrowedAmount` (0x62186905) function"]
        pub fn max_borrowed_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([98, 24, 105, 5], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minBorrowedAmount` (0x0bc772da) function"]
        pub fn min_borrowed_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([11, 199, 114, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openCreditAccount` (0x8fe3f93f) function"]
        pub fn open_credit_account(
            &self,
            borrowed_amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([143, 227, 249, 63], (borrowed_amount, on_behalf_of))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolService` (0x570a7af2) function"]
        pub fn pool_service(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([87, 10, 122, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceOracle` (0x2630c12f) function"]
        pub fn price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 48, 193, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenMasksMap` (0xf67c5bd0) function"]
        pub fn token_masks_map(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([246, 124, 91, 208], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferAccountOwnership` (0xe1998cf9) function"]
        pub fn transfer_account_ownership(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 153, 140, 249], (from, to))
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
        #[doc = "Calls the contract's `wethAddress` (0x4f0e0ef3) function"]
        pub fn weth_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([79, 14, 14, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ExecuteOrder` event"]
        pub fn execute_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewConfigurator` event"]
        pub fn new_configurator_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewConfiguratorFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ICreditManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICreditManager<M> {
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
    #[ethevent(name = "ExecuteOrder", abi = "ExecuteOrder(address,address)")]
    pub struct ExecuteOrderFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
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
    #[ethevent(name = "NewConfigurator", abi = "NewConfigurator(address)")]
    pub struct NewConfiguratorFilter {
        #[ethevent(indexed)]
        pub new_configurator: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditManagerEvents {
        ExecuteOrderFilter(ExecuteOrderFilter),
        NewConfiguratorFilter(NewConfiguratorFilter),
    }
    impl ethers::contract::EthLogDecode for ICreditManagerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ExecuteOrderFilter::decode_log(log) {
                return Ok(ICreditManagerEvents::ExecuteOrderFilter(decoded));
            }
            if let Ok(decoded) = NewConfiguratorFilter::decode_log(log) {
                return Ok(ICreditManagerEvents::NewConfiguratorFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ICreditManagerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditManagerEvents::ExecuteOrderFilter(element) => element.fmt(f),
                ICreditManagerEvents::NewConfiguratorFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `adapterToContract`function with signature `adapterToContract(address)` and selector `[255, 104, 117, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "adapterToContract", abi = "adapterToContract(address)")]
    pub struct AdapterToContractCall {
        pub adapter: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addCollateral`function with signature `addCollateral(address,address,address,uint256)` and selector `[131, 10, 167, 69]`"]
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
        name = "addCollateral",
        abi = "addCollateral(address,address,address,uint256)"
    )]
    pub struct AddCollateralCall {
        pub payer: ethers::core::types::Address,
        pub on_behalf_of: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allowedTokens`function with signature `allowedTokens(uint256)` and selector `[94, 95, 46, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowedTokens", abi = "allowedTokens(uint256)")]
    pub struct AllowedTokensCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allowedTokensCount`function with signature `allowedTokensCount()` and selector `[32, 160, 95, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowedTokensCount", abi = "allowedTokensCount()")]
    pub struct AllowedTokensCountCall;
    #[doc = "Container type for all input parameters for the `approveCreditAccount`function with signature `approveCreditAccount(address,address,address,uint256)` and selector `[70, 251, 55, 29]`"]
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
        name = "approveCreditAccount",
        abi = "approveCreditAccount(address,address,address,uint256)"
    )]
    pub struct ApproveCreditAccountCall {
        pub borrower: ethers::core::types::Address,
        pub target_contract: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calcClosePayments`function with signature `calcClosePayments(uint256,bool,uint256,uint256)` and selector `[118, 245, 73, 102]`"]
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
        name = "calcClosePayments",
        abi = "calcClosePayments(uint256,bool,uint256,uint256)"
    )]
    pub struct CalcClosePaymentsCall {
        pub total_value: ethers::core::types::U256,
        pub is_liquidated: bool,
        pub borrowed_amount: ethers::core::types::U256,
        pub borrowed_amount_with_interest: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calcCreditAccountAccruedInterest`function with signature `calcCreditAccountAccruedInterest(address)` and selector `[49, 146, 25, 92]`"]
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
        name = "calcCreditAccountAccruedInterest",
        abi = "calcCreditAccountAccruedInterest(address)"
    )]
    pub struct CalcCreditAccountAccruedInterestCall {
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `checkAndEnableToken`function with signature `checkAndEnableToken(address,address)` and selector `[81, 227, 241, 96]`"]
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
        name = "checkAndEnableToken",
        abi = "checkAndEnableToken(address,address)"
    )]
    pub struct CheckAndEnableTokenCall {
        pub credit_account: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `closeCreditAccount`function with signature `closeCreditAccount(address,bool,uint256,address,address,uint256,bool)` and selector `[140, 92, 159, 225]`"]
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
        name = "closeCreditAccount",
        abi = "closeCreditAccount(address,bool,uint256,address,address,uint256,bool)"
    )]
    pub struct CloseCreditAccountCall {
        pub borrower: ethers::core::types::Address,
        pub is_liquidated: bool,
        pub total_value: ethers::core::types::U256,
        pub caller: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub skip_token_mask: ethers::core::types::U256,
        pub convert_weth: bool,
    }
    #[doc = "Container type for all input parameters for the `creditAccounts`function with signature `creditAccounts(address)` and selector `[5, 94, 233, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditAccounts", abi = "creditAccounts(address)")]
    pub struct CreditAccountsCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `creditConfigurator`function with signature `creditConfigurator()` and selector `[249, 170, 2, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditConfigurator", abi = "creditConfigurator()")]
    pub struct CreditConfiguratorCall;
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
    #[doc = "Container type for all input parameters for the `enabledTokensMap`function with signature `enabledTokensMap(address)` and selector `[137, 145, 178, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "enabledTokensMap", abi = "enabledTokensMap(address)")]
    pub struct EnabledTokensMapCall {
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executeOrder`function with signature `executeOrder(address,address,bytes)` and selector `[108, 228, 7, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "executeOrder", abi = "executeOrder(address,address,bytes)")]
    pub struct ExecuteOrderCall {
        pub borrower: ethers::core::types::Address,
        pub target: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `fastCollateralCheck`function with signature `fastCollateralCheck(address,address,address,uint256,uint256)` and selector `[101, 74, 158, 218]`"]
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
        name = "fastCollateralCheck",
        abi = "fastCollateralCheck(address,address,address,uint256,uint256)"
    )]
    pub struct FastCollateralCheckCall {
        pub credit_account: ethers::core::types::Address,
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub balance_in_before: ethers::core::types::U256,
        pub balance_out_before: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `feeInterest`function with signature `feeInterest()` and selector `[94, 11, 99, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "feeInterest", abi = "feeInterest()")]
    pub struct FeeInterestCall;
    #[doc = "Container type for all input parameters for the `feeLiquidation`function with signature `feeLiquidation()` and selector `[57, 21, 255, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "feeLiquidation", abi = "feeLiquidation()")]
    pub struct FeeLiquidationCall;
    #[doc = "Container type for all input parameters for the `forbidenTokenMask`function with signature `forbidenTokenMask()` and selector `[228, 108, 149, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "forbidenTokenMask", abi = "forbidenTokenMask()")]
    pub struct ForbidenTokenMaskCall;
    #[doc = "Container type for all input parameters for the `fullCollateralCheck`function with signature `fullCollateralCheck(address)` and selector `[149, 55, 48, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fullCollateralCheck", abi = "fullCollateralCheck(address)")]
    pub struct FullCollateralCheckCall {
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditAccountOrRevert`function with signature `getCreditAccountOrRevert(address)` and selector `[233, 88, 183, 4]`"]
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
        name = "getCreditAccountOrRevert",
        abi = "getCreditAccountOrRevert(address)"
    )]
    pub struct GetCreditAccountOrRevertCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `liquidationDiscount`function with signature `liquidationDiscount()` and selector `[128, 83, 252, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "liquidationDiscount", abi = "liquidationDiscount()")]
    pub struct LiquidationDiscountCall;
    #[doc = "Container type for all input parameters for the `liquidationThresholds`function with signature `liquidationThresholds(address)` and selector `[120, 50, 116, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "liquidationThresholds", abi = "liquidationThresholds(address)")]
    pub struct LiquidationThresholdsCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `manageDebt`function with signature `manageDebt(address,uint256,bool)` and selector `[148, 207, 7, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "manageDebt", abi = "manageDebt(address,uint256,bool)")]
    pub struct ManageDebtCall {
        pub borrower: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub increase: bool,
    }
    #[doc = "Container type for all input parameters for the `maxBorrowedAmount`function with signature `maxBorrowedAmount()` and selector `[98, 24, 105, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "maxBorrowedAmount", abi = "maxBorrowedAmount()")]
    pub struct MaxBorrowedAmountCall;
    #[doc = "Container type for all input parameters for the `minBorrowedAmount`function with signature `minBorrowedAmount()` and selector `[11, 199, 114, 218]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "minBorrowedAmount", abi = "minBorrowedAmount()")]
    pub struct MinBorrowedAmountCall;
    #[doc = "Container type for all input parameters for the `openCreditAccount`function with signature `openCreditAccount(uint256,address)` and selector `[143, 227, 249, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "openCreditAccount", abi = "openCreditAccount(uint256,address)")]
    pub struct OpenCreditAccountCall {
        pub borrowed_amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `poolService`function with signature `poolService()` and selector `[87, 10, 122, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolService", abi = "poolService()")]
    pub struct PoolServiceCall;
    #[doc = "Container type for all input parameters for the `priceOracle`function with signature `priceOracle()` and selector `[38, 48, 193, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceOracle", abi = "priceOracle()")]
    pub struct PriceOracleCall;
    #[doc = "Container type for all input parameters for the `tokenMasksMap`function with signature `tokenMasksMap(address)` and selector `[246, 124, 91, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenMasksMap", abi = "tokenMasksMap(address)")]
    pub struct TokenMasksMapCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferAccountOwnership`function with signature `transferAccountOwnership(address,address)` and selector `[225, 153, 140, 249]`"]
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
        name = "transferAccountOwnership",
        abi = "transferAccountOwnership(address,address)"
    )]
    pub struct TransferAccountOwnershipCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `wethAddress`function with signature `wethAddress()` and selector `[79, 14, 14, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wethAddress", abi = "wethAddress()")]
    pub struct WethAddressCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditManagerCalls {
        AdapterToContract(AdapterToContractCall),
        AddCollateral(AddCollateralCall),
        AllowedTokens(AllowedTokensCall),
        AllowedTokensCount(AllowedTokensCountCall),
        ApproveCreditAccount(ApproveCreditAccountCall),
        CalcClosePayments(CalcClosePaymentsCall),
        CalcCreditAccountAccruedInterest(CalcCreditAccountAccruedInterestCall),
        CheckAndEnableToken(CheckAndEnableTokenCall),
        CloseCreditAccount(CloseCreditAccountCall),
        CreditAccounts(CreditAccountsCall),
        CreditConfigurator(CreditConfiguratorCall),
        CreditFacade(CreditFacadeCall),
        EnabledTokensMap(EnabledTokensMapCall),
        ExecuteOrder(ExecuteOrderCall),
        FastCollateralCheck(FastCollateralCheckCall),
        FeeInterest(FeeInterestCall),
        FeeLiquidation(FeeLiquidationCall),
        ForbidenTokenMask(ForbidenTokenMaskCall),
        FullCollateralCheck(FullCollateralCheckCall),
        GetCreditAccountOrRevert(GetCreditAccountOrRevertCall),
        LiquidationDiscount(LiquidationDiscountCall),
        LiquidationThresholds(LiquidationThresholdsCall),
        ManageDebt(ManageDebtCall),
        MaxBorrowedAmount(MaxBorrowedAmountCall),
        MinBorrowedAmount(MinBorrowedAmountCall),
        OpenCreditAccount(OpenCreditAccountCall),
        PoolService(PoolServiceCall),
        PriceOracle(PriceOracleCall),
        TokenMasksMap(TokenMasksMapCall),
        TransferAccountOwnership(TransferAccountOwnershipCall),
        Underlying(UnderlyingCall),
        Version(VersionCall),
        WethAddress(WethAddressCall),
    }
    impl ethers::core::abi::AbiDecode for ICreditManagerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AdapterToContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::AdapterToContract(decoded));
            }
            if let Ok(decoded) =
                <AddCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::AddCollateral(decoded));
            }
            if let Ok(decoded) =
                <AllowedTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::AllowedTokens(decoded));
            }
            if let Ok(decoded) =
                <AllowedTokensCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::AllowedTokensCount(decoded));
            }
            if let Ok(decoded) =
                <ApproveCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::ApproveCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <CalcClosePaymentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::CalcClosePayments(decoded));
            }
            if let Ok(decoded) =
                <CalcCreditAccountAccruedInterestCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditManagerCalls::CalcCreditAccountAccruedInterest(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CheckAndEnableTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::CheckAndEnableToken(decoded));
            }
            if let Ok(decoded) =
                <CloseCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::CloseCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <CreditAccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::CreditAccounts(decoded));
            }
            if let Ok(decoded) =
                <CreditConfiguratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::CreditConfigurator(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <EnabledTokensMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::EnabledTokensMap(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::ExecuteOrder(decoded));
            }
            if let Ok(decoded) =
                <FastCollateralCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::FastCollateralCheck(decoded));
            }
            if let Ok(decoded) =
                <FeeInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::FeeInterest(decoded));
            }
            if let Ok(decoded) =
                <FeeLiquidationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::FeeLiquidation(decoded));
            }
            if let Ok(decoded) =
                <ForbidenTokenMaskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::ForbidenTokenMask(decoded));
            }
            if let Ok(decoded) =
                <FullCollateralCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::FullCollateralCheck(decoded));
            }
            if let Ok(decoded) =
                <GetCreditAccountOrRevertCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditManagerCalls::GetCreditAccountOrRevert(decoded));
            }
            if let Ok(decoded) =
                <LiquidationDiscountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::LiquidationDiscount(decoded));
            }
            if let Ok(decoded) =
                <LiquidationThresholdsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::LiquidationThresholds(decoded));
            }
            if let Ok(decoded) =
                <ManageDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::ManageDebt(decoded));
            }
            if let Ok(decoded) =
                <MaxBorrowedAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::MaxBorrowedAmount(decoded));
            }
            if let Ok(decoded) =
                <MinBorrowedAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::MinBorrowedAmount(decoded));
            }
            if let Ok(decoded) =
                <OpenCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::OpenCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <PoolServiceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::PoolService(decoded));
            }
            if let Ok(decoded) =
                <PriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::PriceOracle(decoded));
            }
            if let Ok(decoded) =
                <TokenMasksMapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::TokenMasksMap(decoded));
            }
            if let Ok(decoded) =
                <TransferAccountOwnershipCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditManagerCalls::TransferAccountOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::Underlying(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <WethAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditManagerCalls::WethAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICreditManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICreditManagerCalls::AdapterToContract(element) => element.encode(),
                ICreditManagerCalls::AddCollateral(element) => element.encode(),
                ICreditManagerCalls::AllowedTokens(element) => element.encode(),
                ICreditManagerCalls::AllowedTokensCount(element) => element.encode(),
                ICreditManagerCalls::ApproveCreditAccount(element) => element.encode(),
                ICreditManagerCalls::CalcClosePayments(element) => element.encode(),
                ICreditManagerCalls::CalcCreditAccountAccruedInterest(element) => element.encode(),
                ICreditManagerCalls::CheckAndEnableToken(element) => element.encode(),
                ICreditManagerCalls::CloseCreditAccount(element) => element.encode(),
                ICreditManagerCalls::CreditAccounts(element) => element.encode(),
                ICreditManagerCalls::CreditConfigurator(element) => element.encode(),
                ICreditManagerCalls::CreditFacade(element) => element.encode(),
                ICreditManagerCalls::EnabledTokensMap(element) => element.encode(),
                ICreditManagerCalls::ExecuteOrder(element) => element.encode(),
                ICreditManagerCalls::FastCollateralCheck(element) => element.encode(),
                ICreditManagerCalls::FeeInterest(element) => element.encode(),
                ICreditManagerCalls::FeeLiquidation(element) => element.encode(),
                ICreditManagerCalls::ForbidenTokenMask(element) => element.encode(),
                ICreditManagerCalls::FullCollateralCheck(element) => element.encode(),
                ICreditManagerCalls::GetCreditAccountOrRevert(element) => element.encode(),
                ICreditManagerCalls::LiquidationDiscount(element) => element.encode(),
                ICreditManagerCalls::LiquidationThresholds(element) => element.encode(),
                ICreditManagerCalls::ManageDebt(element) => element.encode(),
                ICreditManagerCalls::MaxBorrowedAmount(element) => element.encode(),
                ICreditManagerCalls::MinBorrowedAmount(element) => element.encode(),
                ICreditManagerCalls::OpenCreditAccount(element) => element.encode(),
                ICreditManagerCalls::PoolService(element) => element.encode(),
                ICreditManagerCalls::PriceOracle(element) => element.encode(),
                ICreditManagerCalls::TokenMasksMap(element) => element.encode(),
                ICreditManagerCalls::TransferAccountOwnership(element) => element.encode(),
                ICreditManagerCalls::Underlying(element) => element.encode(),
                ICreditManagerCalls::Version(element) => element.encode(),
                ICreditManagerCalls::WethAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICreditManagerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditManagerCalls::AdapterToContract(element) => element.fmt(f),
                ICreditManagerCalls::AddCollateral(element) => element.fmt(f),
                ICreditManagerCalls::AllowedTokens(element) => element.fmt(f),
                ICreditManagerCalls::AllowedTokensCount(element) => element.fmt(f),
                ICreditManagerCalls::ApproveCreditAccount(element) => element.fmt(f),
                ICreditManagerCalls::CalcClosePayments(element) => element.fmt(f),
                ICreditManagerCalls::CalcCreditAccountAccruedInterest(element) => element.fmt(f),
                ICreditManagerCalls::CheckAndEnableToken(element) => element.fmt(f),
                ICreditManagerCalls::CloseCreditAccount(element) => element.fmt(f),
                ICreditManagerCalls::CreditAccounts(element) => element.fmt(f),
                ICreditManagerCalls::CreditConfigurator(element) => element.fmt(f),
                ICreditManagerCalls::CreditFacade(element) => element.fmt(f),
                ICreditManagerCalls::EnabledTokensMap(element) => element.fmt(f),
                ICreditManagerCalls::ExecuteOrder(element) => element.fmt(f),
                ICreditManagerCalls::FastCollateralCheck(element) => element.fmt(f),
                ICreditManagerCalls::FeeInterest(element) => element.fmt(f),
                ICreditManagerCalls::FeeLiquidation(element) => element.fmt(f),
                ICreditManagerCalls::ForbidenTokenMask(element) => element.fmt(f),
                ICreditManagerCalls::FullCollateralCheck(element) => element.fmt(f),
                ICreditManagerCalls::GetCreditAccountOrRevert(element) => element.fmt(f),
                ICreditManagerCalls::LiquidationDiscount(element) => element.fmt(f),
                ICreditManagerCalls::LiquidationThresholds(element) => element.fmt(f),
                ICreditManagerCalls::ManageDebt(element) => element.fmt(f),
                ICreditManagerCalls::MaxBorrowedAmount(element) => element.fmt(f),
                ICreditManagerCalls::MinBorrowedAmount(element) => element.fmt(f),
                ICreditManagerCalls::OpenCreditAccount(element) => element.fmt(f),
                ICreditManagerCalls::PoolService(element) => element.fmt(f),
                ICreditManagerCalls::PriceOracle(element) => element.fmt(f),
                ICreditManagerCalls::TokenMasksMap(element) => element.fmt(f),
                ICreditManagerCalls::TransferAccountOwnership(element) => element.fmt(f),
                ICreditManagerCalls::Underlying(element) => element.fmt(f),
                ICreditManagerCalls::Version(element) => element.fmt(f),
                ICreditManagerCalls::WethAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AdapterToContractCall> for ICreditManagerCalls {
        fn from(var: AdapterToContractCall) -> Self {
            ICreditManagerCalls::AdapterToContract(var)
        }
    }
    impl ::std::convert::From<AddCollateralCall> for ICreditManagerCalls {
        fn from(var: AddCollateralCall) -> Self {
            ICreditManagerCalls::AddCollateral(var)
        }
    }
    impl ::std::convert::From<AllowedTokensCall> for ICreditManagerCalls {
        fn from(var: AllowedTokensCall) -> Self {
            ICreditManagerCalls::AllowedTokens(var)
        }
    }
    impl ::std::convert::From<AllowedTokensCountCall> for ICreditManagerCalls {
        fn from(var: AllowedTokensCountCall) -> Self {
            ICreditManagerCalls::AllowedTokensCount(var)
        }
    }
    impl ::std::convert::From<ApproveCreditAccountCall> for ICreditManagerCalls {
        fn from(var: ApproveCreditAccountCall) -> Self {
            ICreditManagerCalls::ApproveCreditAccount(var)
        }
    }
    impl ::std::convert::From<CalcClosePaymentsCall> for ICreditManagerCalls {
        fn from(var: CalcClosePaymentsCall) -> Self {
            ICreditManagerCalls::CalcClosePayments(var)
        }
    }
    impl ::std::convert::From<CalcCreditAccountAccruedInterestCall> for ICreditManagerCalls {
        fn from(var: CalcCreditAccountAccruedInterestCall) -> Self {
            ICreditManagerCalls::CalcCreditAccountAccruedInterest(var)
        }
    }
    impl ::std::convert::From<CheckAndEnableTokenCall> for ICreditManagerCalls {
        fn from(var: CheckAndEnableTokenCall) -> Self {
            ICreditManagerCalls::CheckAndEnableToken(var)
        }
    }
    impl ::std::convert::From<CloseCreditAccountCall> for ICreditManagerCalls {
        fn from(var: CloseCreditAccountCall) -> Self {
            ICreditManagerCalls::CloseCreditAccount(var)
        }
    }
    impl ::std::convert::From<CreditAccountsCall> for ICreditManagerCalls {
        fn from(var: CreditAccountsCall) -> Self {
            ICreditManagerCalls::CreditAccounts(var)
        }
    }
    impl ::std::convert::From<CreditConfiguratorCall> for ICreditManagerCalls {
        fn from(var: CreditConfiguratorCall) -> Self {
            ICreditManagerCalls::CreditConfigurator(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for ICreditManagerCalls {
        fn from(var: CreditFacadeCall) -> Self {
            ICreditManagerCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<EnabledTokensMapCall> for ICreditManagerCalls {
        fn from(var: EnabledTokensMapCall) -> Self {
            ICreditManagerCalls::EnabledTokensMap(var)
        }
    }
    impl ::std::convert::From<ExecuteOrderCall> for ICreditManagerCalls {
        fn from(var: ExecuteOrderCall) -> Self {
            ICreditManagerCalls::ExecuteOrder(var)
        }
    }
    impl ::std::convert::From<FastCollateralCheckCall> for ICreditManagerCalls {
        fn from(var: FastCollateralCheckCall) -> Self {
            ICreditManagerCalls::FastCollateralCheck(var)
        }
    }
    impl ::std::convert::From<FeeInterestCall> for ICreditManagerCalls {
        fn from(var: FeeInterestCall) -> Self {
            ICreditManagerCalls::FeeInterest(var)
        }
    }
    impl ::std::convert::From<FeeLiquidationCall> for ICreditManagerCalls {
        fn from(var: FeeLiquidationCall) -> Self {
            ICreditManagerCalls::FeeLiquidation(var)
        }
    }
    impl ::std::convert::From<ForbidenTokenMaskCall> for ICreditManagerCalls {
        fn from(var: ForbidenTokenMaskCall) -> Self {
            ICreditManagerCalls::ForbidenTokenMask(var)
        }
    }
    impl ::std::convert::From<FullCollateralCheckCall> for ICreditManagerCalls {
        fn from(var: FullCollateralCheckCall) -> Self {
            ICreditManagerCalls::FullCollateralCheck(var)
        }
    }
    impl ::std::convert::From<GetCreditAccountOrRevertCall> for ICreditManagerCalls {
        fn from(var: GetCreditAccountOrRevertCall) -> Self {
            ICreditManagerCalls::GetCreditAccountOrRevert(var)
        }
    }
    impl ::std::convert::From<LiquidationDiscountCall> for ICreditManagerCalls {
        fn from(var: LiquidationDiscountCall) -> Self {
            ICreditManagerCalls::LiquidationDiscount(var)
        }
    }
    impl ::std::convert::From<LiquidationThresholdsCall> for ICreditManagerCalls {
        fn from(var: LiquidationThresholdsCall) -> Self {
            ICreditManagerCalls::LiquidationThresholds(var)
        }
    }
    impl ::std::convert::From<ManageDebtCall> for ICreditManagerCalls {
        fn from(var: ManageDebtCall) -> Self {
            ICreditManagerCalls::ManageDebt(var)
        }
    }
    impl ::std::convert::From<MaxBorrowedAmountCall> for ICreditManagerCalls {
        fn from(var: MaxBorrowedAmountCall) -> Self {
            ICreditManagerCalls::MaxBorrowedAmount(var)
        }
    }
    impl ::std::convert::From<MinBorrowedAmountCall> for ICreditManagerCalls {
        fn from(var: MinBorrowedAmountCall) -> Self {
            ICreditManagerCalls::MinBorrowedAmount(var)
        }
    }
    impl ::std::convert::From<OpenCreditAccountCall> for ICreditManagerCalls {
        fn from(var: OpenCreditAccountCall) -> Self {
            ICreditManagerCalls::OpenCreditAccount(var)
        }
    }
    impl ::std::convert::From<PoolServiceCall> for ICreditManagerCalls {
        fn from(var: PoolServiceCall) -> Self {
            ICreditManagerCalls::PoolService(var)
        }
    }
    impl ::std::convert::From<PriceOracleCall> for ICreditManagerCalls {
        fn from(var: PriceOracleCall) -> Self {
            ICreditManagerCalls::PriceOracle(var)
        }
    }
    impl ::std::convert::From<TokenMasksMapCall> for ICreditManagerCalls {
        fn from(var: TokenMasksMapCall) -> Self {
            ICreditManagerCalls::TokenMasksMap(var)
        }
    }
    impl ::std::convert::From<TransferAccountOwnershipCall> for ICreditManagerCalls {
        fn from(var: TransferAccountOwnershipCall) -> Self {
            ICreditManagerCalls::TransferAccountOwnership(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for ICreditManagerCalls {
        fn from(var: UnderlyingCall) -> Self {
            ICreditManagerCalls::Underlying(var)
        }
    }
    impl ::std::convert::From<VersionCall> for ICreditManagerCalls {
        fn from(var: VersionCall) -> Self {
            ICreditManagerCalls::Version(var)
        }
    }
    impl ::std::convert::From<WethAddressCall> for ICreditManagerCalls {
        fn from(var: WethAddressCall) -> Self {
            ICreditManagerCalls::WethAddress(var)
        }
    }
}
