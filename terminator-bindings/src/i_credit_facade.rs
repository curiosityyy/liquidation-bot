pub use icreditfacade_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditfacade_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ICreditFacade was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITFACADE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"AccountTransferNotAllowedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CantLiquidateWithSuchHealthFactorException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CantTransferLiquidatableAccountException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ContractNotAllowedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CreditConfiguratorOnlyException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CreditManagerCallsForbiddenException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"HasAlreadyOpenedCreditAccountInDegenMode\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectCallDataLengthException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectOpenCreditAccountAmountException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncreaseAndDecreaseForbiddenInOneCallException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncreaseDebtForbiddenException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IntlCallsDuringClosureForbiddenException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoDegenNFTInDegenModeException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TargetIsNotAdapterException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TokenNotAllowedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnknownMethodException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddCollateral\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CloseCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DecreaseBorrowedAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"IncreaseBorrowedAmount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"liquidator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"remainingFunds\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LiquidateCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"MultiCallFinished\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MultiCallStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"borrowAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OpenCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"oldOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TransferAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TransferAccountAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"addCollateral\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedContractsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"state\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveAccountTransfers\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcCreditAccountHealthFactor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"hf\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcTotalValue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"total\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"twv\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"skipTokenMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"convertWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct MultiCall[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"closeCreditAccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"contractToAdapter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManagerV2\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseDebt\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"degenMode\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"degenNFT\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasOpenedCreditAccount\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseDebt\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isIncreaseDebtForbidden\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTokenAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"skipTokenMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"convertWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct MultiCall[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"liquidateCreditAccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct MultiCall[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"leverageFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"openCreditAccount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct MultiCall[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"referralCode\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"openCreditAccountMulticall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalOpenedAccountsDegenMode\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferAccountOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transfersAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITFACADE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditFacade<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditFacade<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditFacade<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditFacade))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditFacade<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICREDITFACADE_ABI.clone(), client)
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
                ICREDITFACADE_ABI.clone(),
                ICREDITFACADE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addCollateral` (0x59781034) function"]
        pub fn add_collateral(
            &self,
            on_behalf_of: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 120, 16, 52], (on_behalf_of, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContract` (0x4b7942b1) function"]
        pub fn allowed_contract(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([75, 121, 66, 177], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContractsLength` (0x729dbea8) function"]
        pub fn allowed_contracts_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([114, 157, 190, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0xe1f21c67) function"]
        pub fn approve(
            &self,
            target_contract: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 242, 28, 103], (target_contract, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveAccountTransfers` (0x5f27212a) function"]
        pub fn approve_account_transfers(
            &self,
            from: ethers::core::types::Address,
            state: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 39, 33, 42], (from, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcCreditAccountHealthFactor` (0xdfd59465) function"]
        pub fn calc_credit_account_health_factor(
            &self,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([223, 213, 148, 101], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcTotalValue` (0xc7de38a6) function"]
        pub fn calc_total_value(
            &self,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([199, 222, 56, 166], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeCreditAccount` (0x5f73fbec) function"]
        pub fn close_credit_account(
            &self,
            to: ethers::core::types::Address,
            skip_token_mask: ethers::core::types::U256,
            convert_weth: bool,
            calls: ::std::vec::Vec<MultiCall>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [95, 115, 251, 236],
                    (to, skip_token_mask, convert_weth, calls),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `contractToAdapter` (0xfdd57645) function"]
        pub fn contract_to_adapter(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([253, 213, 118, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManager` (0xc12c21c0) function"]
        pub fn credit_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 44, 33, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseDebt` (0x2a7ba1f7) function"]
        pub fn decrease_debt(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 123, 161, 247], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `degenMode` (0xdf1a701c) function"]
        pub fn degen_mode(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([223, 26, 112, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `degenNFT` (0x9408b63f) function"]
        pub fn degen_nft(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([148, 8, 182, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasOpenedCreditAccount` (0x256ac915) function"]
        pub fn has_opened_credit_account(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 106, 201, 21], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseDebt` (0x2b7c7b11) function"]
        pub fn increase_debt(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 124, 123, 17], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isIncreaseDebtForbidden` (0x43ac1819) function"]
        pub fn is_increase_debt_forbidden(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 172, 24, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTokenAllowed` (0xf9eaee0d) function"]
        pub fn is_token_allowed(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 234, 238, 13], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateCreditAccount` (0x5d91a0e0) function"]
        pub fn liquidate_credit_account(
            &self,
            borrower: ethers::core::types::Address,
            to: ethers::core::types::Address,
            skip_token_mask: ethers::core::types::U256,
            convert_weth: bool,
            calls: ::std::vec::Vec<MultiCall>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [93, 145, 160, 224],
                    (borrower, to, skip_token_mask, convert_weth, calls),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0xcaa5c23f) function"]
        pub fn multicall(
            &self,
            calls: ::std::vec::Vec<MultiCall>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 165, 194, 63], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openCreditAccount` (0x5288ba4b) function"]
        pub fn open_credit_account(
            &self,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            leverage_factor: ethers::core::types::U256,
            referral_code: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [82, 136, 186, 75],
                    (amount, on_behalf_of, leverage_factor, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openCreditAccountMulticall` (0x47639fa8) function"]
        pub fn open_credit_account_multicall(
            &self,
            borrowed_amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            calls: ::std::vec::Vec<MultiCall>,
            referral_code: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [71, 99, 159, 168],
                    (borrowed_amount, on_behalf_of, calls, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalOpenedAccountsDegenMode` (0xce03841e) function"]
        pub fn total_opened_accounts_degen_mode(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([206, 3, 132, 30], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferAccountOwnership` (0x5019e20a) function"]
        pub fn transfer_account_ownership(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 25, 226, 10], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfersAllowed` (0xd9ccbec1) function"]
        pub fn transfers_allowed(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 204, 190, 193], (from, to))
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
        #[doc = "Gets the contract's `AddCollateral` event"]
        pub fn add_collateral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CloseCreditAccount` event"]
        pub fn close_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CloseCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DecreaseBorrowedAmount` event"]
        pub fn decrease_borrowed_amount_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DecreaseBorrowedAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseBorrowedAmount` event"]
        pub fn increase_borrowed_amount_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreaseBorrowedAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidateCreditAccount` event"]
        pub fn liquidate_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidateCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MultiCallFinished` event"]
        pub fn multi_call_finished_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MultiCallFinishedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MultiCallStarted` event"]
        pub fn multi_call_started_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MultiCallStartedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OpenCreditAccount` event"]
        pub fn open_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OpenCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAccount` event"]
        pub fn transfer_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAccountAllowed` event"]
        pub fn transfer_account_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferAccountAllowedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ICreditFacadeEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICreditFacade<M> {
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
    #[ethevent(name = "AddCollateral", abi = "AddCollateral(address,address,uint256)")]
    pub struct AddCollateralFilter {
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
        name = "CloseCreditAccount",
        abi = "CloseCreditAccount(address,address)"
    )]
    pub struct CloseCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
        name = "DecreaseBorrowedAmount",
        abi = "DecreaseBorrowedAmount(address,uint256)"
    )]
    pub struct DecreaseBorrowedAmountFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
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
    #[ethevent(
        name = "IncreaseBorrowedAmount",
        abi = "IncreaseBorrowedAmount(address,uint256)"
    )]
    pub struct IncreaseBorrowedAmountFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
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
    #[ethevent(
        name = "LiquidateCreditAccount",
        abi = "LiquidateCreditAccount(address,address,address,uint256)"
    )]
    pub struct LiquidateCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub liquidator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub remaining_funds: ethers::core::types::U256,
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
    #[ethevent(name = "MultiCallFinished", abi = "MultiCallFinished()")]
    pub struct MultiCallFinishedFilter();
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "MultiCallStarted", abi = "MultiCallStarted(address)")]
    pub struct MultiCallStartedFilter {
        #[ethevent(indexed)]
        pub borrower: ethers::core::types::Address,
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
        name = "OpenCreditAccount",
        abi = "OpenCreditAccount(address,address,uint256,uint256)"
    )]
    pub struct OpenCreditAccountFilter {
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub credit_account: ethers::core::types::Address,
        pub borrow_amount: ethers::core::types::U256,
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
    #[ethevent(name = "TransferAccount", abi = "TransferAccount(address,address)")]
    pub struct TransferAccountFilter {
        #[ethevent(indexed)]
        pub old_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        name = "TransferAccountAllowed",
        abi = "TransferAccountAllowed(address,address,bool)"
    )]
    pub struct TransferAccountAllowedFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub state: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditFacadeEvents {
        AddCollateralFilter(AddCollateralFilter),
        CloseCreditAccountFilter(CloseCreditAccountFilter),
        DecreaseBorrowedAmountFilter(DecreaseBorrowedAmountFilter),
        IncreaseBorrowedAmountFilter(IncreaseBorrowedAmountFilter),
        LiquidateCreditAccountFilter(LiquidateCreditAccountFilter),
        MultiCallFinishedFilter(MultiCallFinishedFilter),
        MultiCallStartedFilter(MultiCallStartedFilter),
        OpenCreditAccountFilter(OpenCreditAccountFilter),
        TransferAccountFilter(TransferAccountFilter),
        TransferAccountAllowedFilter(TransferAccountAllowedFilter),
    }
    impl ethers::contract::EthLogDecode for ICreditFacadeEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddCollateralFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::AddCollateralFilter(decoded));
            }
            if let Ok(decoded) = CloseCreditAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::CloseCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = DecreaseBorrowedAmountFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::DecreaseBorrowedAmountFilter(decoded));
            }
            if let Ok(decoded) = IncreaseBorrowedAmountFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::IncreaseBorrowedAmountFilter(decoded));
            }
            if let Ok(decoded) = LiquidateCreditAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::LiquidateCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = MultiCallFinishedFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::MultiCallFinishedFilter(decoded));
            }
            if let Ok(decoded) = MultiCallStartedFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::MultiCallStartedFilter(decoded));
            }
            if let Ok(decoded) = OpenCreditAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::OpenCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::TransferAccountFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountAllowedFilter::decode_log(log) {
                return Ok(ICreditFacadeEvents::TransferAccountAllowedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ICreditFacadeEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditFacadeEvents::AddCollateralFilter(element) => element.fmt(f),
                ICreditFacadeEvents::CloseCreditAccountFilter(element) => element.fmt(f),
                ICreditFacadeEvents::DecreaseBorrowedAmountFilter(element) => element.fmt(f),
                ICreditFacadeEvents::IncreaseBorrowedAmountFilter(element) => element.fmt(f),
                ICreditFacadeEvents::LiquidateCreditAccountFilter(element) => element.fmt(f),
                ICreditFacadeEvents::MultiCallFinishedFilter(element) => element.fmt(f),
                ICreditFacadeEvents::MultiCallStartedFilter(element) => element.fmt(f),
                ICreditFacadeEvents::OpenCreditAccountFilter(element) => element.fmt(f),
                ICreditFacadeEvents::TransferAccountFilter(element) => element.fmt(f),
                ICreditFacadeEvents::TransferAccountAllowedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addCollateral`function with signature `addCollateral(address,address,uint256)` and selector `[89, 120, 16, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addCollateral", abi = "addCollateral(address,address,uint256)")]
    pub struct AddCollateralCall {
        pub on_behalf_of: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allowedContract`function with signature `allowedContract(uint256)` and selector `[75, 121, 66, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowedContract", abi = "allowedContract(uint256)")]
    pub struct AllowedContractCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allowedContractsLength`function with signature `allowedContractsLength()` and selector `[114, 157, 190, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowedContractsLength", abi = "allowedContractsLength()")]
    pub struct AllowedContractsLengthCall;
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,address,uint256)` and selector `[225, 242, 28, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,address,uint256)")]
    pub struct ApproveCall {
        pub target_contract: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `approveAccountTransfers`function with signature `approveAccountTransfers(address,bool)` and selector `[95, 39, 33, 42]`"]
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
        name = "approveAccountTransfers",
        abi = "approveAccountTransfers(address,bool)"
    )]
    pub struct ApproveAccountTransfersCall {
        pub from: ethers::core::types::Address,
        pub state: bool,
    }
    #[doc = "Container type for all input parameters for the `calcCreditAccountHealthFactor`function with signature `calcCreditAccountHealthFactor(address)` and selector `[223, 213, 148, 101]`"]
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
        name = "calcCreditAccountHealthFactor",
        abi = "calcCreditAccountHealthFactor(address)"
    )]
    pub struct CalcCreditAccountHealthFactorCall {
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `calcTotalValue`function with signature `calcTotalValue(address)` and selector `[199, 222, 56, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "calcTotalValue", abi = "calcTotalValue(address)")]
    pub struct CalcTotalValueCall {
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `closeCreditAccount`function with signature `closeCreditAccount(address,uint256,bool,(address,bytes)[])` and selector `[95, 115, 251, 236]`"]
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
        abi = "closeCreditAccount(address,uint256,bool,(address,bytes)[])"
    )]
    pub struct CloseCreditAccountCall {
        pub to: ethers::core::types::Address,
        pub skip_token_mask: ethers::core::types::U256,
        pub convert_weth: bool,
        pub calls: ::std::vec::Vec<MultiCall>,
    }
    #[doc = "Container type for all input parameters for the `contractToAdapter`function with signature `contractToAdapter(address)` and selector `[253, 213, 118, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "contractToAdapter", abi = "contractToAdapter(address)")]
    pub struct ContractToAdapterCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `creditManager`function with signature `creditManager()` and selector `[193, 44, 33, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManager", abi = "creditManager()")]
    pub struct CreditManagerCall;
    #[doc = "Container type for all input parameters for the `decreaseDebt`function with signature `decreaseDebt(uint256)` and selector `[42, 123, 161, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decreaseDebt", abi = "decreaseDebt(uint256)")]
    pub struct DecreaseDebtCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `degenMode`function with signature `degenMode()` and selector `[223, 26, 112, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "degenMode", abi = "degenMode()")]
    pub struct DegenModeCall;
    #[doc = "Container type for all input parameters for the `degenNFT`function with signature `degenNFT()` and selector `[148, 8, 182, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "degenNFT", abi = "degenNFT()")]
    pub struct DegenNFTCall;
    #[doc = "Container type for all input parameters for the `hasOpenedCreditAccount`function with signature `hasOpenedCreditAccount(address)` and selector `[37, 106, 201, 21]`"]
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
        name = "hasOpenedCreditAccount",
        abi = "hasOpenedCreditAccount(address)"
    )]
    pub struct HasOpenedCreditAccountCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `increaseDebt`function with signature `increaseDebt(uint256)` and selector `[43, 124, 123, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "increaseDebt", abi = "increaseDebt(uint256)")]
    pub struct IncreaseDebtCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isIncreaseDebtForbidden`function with signature `isIncreaseDebtForbidden()` and selector `[67, 172, 24, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isIncreaseDebtForbidden", abi = "isIncreaseDebtForbidden()")]
    pub struct IsIncreaseDebtForbiddenCall;
    #[doc = "Container type for all input parameters for the `isTokenAllowed`function with signature `isTokenAllowed(address)` and selector `[249, 234, 238, 13]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isTokenAllowed", abi = "isTokenAllowed(address)")]
    pub struct IsTokenAllowedCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `liquidateCreditAccount`function with signature `liquidateCreditAccount(address,address,uint256,bool,(address,bytes)[])` and selector `[93, 145, 160, 224]`"]
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
        name = "liquidateCreditAccount",
        abi = "liquidateCreditAccount(address,address,uint256,bool,(address,bytes)[])"
    )]
    pub struct LiquidateCreditAccountCall {
        pub borrower: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub skip_token_mask: ethers::core::types::U256,
        pub convert_weth: bool,
        pub calls: ::std::vec::Vec<MultiCall>,
    }
    #[doc = "Container type for all input parameters for the `multicall`function with signature `multicall((address,bytes)[])` and selector `[202, 165, 194, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "multicall", abi = "multicall((address,bytes)[])")]
    pub struct MulticallCall {
        pub calls: ::std::vec::Vec<MultiCall>,
    }
    #[doc = "Container type for all input parameters for the `openCreditAccount`function with signature `openCreditAccount(uint256,address,uint256,uint256)` and selector `[82, 136, 186, 75]`"]
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
        name = "openCreditAccount",
        abi = "openCreditAccount(uint256,address,uint256,uint256)"
    )]
    pub struct OpenCreditAccountCall {
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub leverage_factor: ethers::core::types::U256,
        pub referral_code: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `openCreditAccountMulticall`function with signature `openCreditAccountMulticall(uint256,address,(address,bytes)[],uint256)` and selector `[71, 99, 159, 168]`"]
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
        name = "openCreditAccountMulticall",
        abi = "openCreditAccountMulticall(uint256,address,(address,bytes)[],uint256)"
    )]
    pub struct OpenCreditAccountMulticallCall {
        pub borrowed_amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub calls: ::std::vec::Vec<MultiCall>,
        pub referral_code: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalOpenedAccountsDegenMode`function with signature `totalOpenedAccountsDegenMode(address)` and selector `[206, 3, 132, 30]`"]
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
        name = "totalOpenedAccountsDegenMode",
        abi = "totalOpenedAccountsDegenMode(address)"
    )]
    pub struct TotalOpenedAccountsDegenModeCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `transferAccountOwnership`function with signature `transferAccountOwnership(address)` and selector `[80, 25, 226, 10]`"]
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
        abi = "transferAccountOwnership(address)"
    )]
    pub struct TransferAccountOwnershipCall {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transfersAllowed`function with signature `transfersAllowed(address,address)` and selector `[217, 204, 190, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfersAllowed", abi = "transfersAllowed(address,address)")]
    pub struct TransfersAllowedCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
    }
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICreditFacadeCalls {
        AddCollateral(AddCollateralCall),
        AllowedContract(AllowedContractCall),
        AllowedContractsLength(AllowedContractsLengthCall),
        Approve(ApproveCall),
        ApproveAccountTransfers(ApproveAccountTransfersCall),
        CalcCreditAccountHealthFactor(CalcCreditAccountHealthFactorCall),
        CalcTotalValue(CalcTotalValueCall),
        CloseCreditAccount(CloseCreditAccountCall),
        ContractToAdapter(ContractToAdapterCall),
        CreditManager(CreditManagerCall),
        DecreaseDebt(DecreaseDebtCall),
        DegenMode(DegenModeCall),
        DegenNFT(DegenNFTCall),
        HasOpenedCreditAccount(HasOpenedCreditAccountCall),
        IncreaseDebt(IncreaseDebtCall),
        IsIncreaseDebtForbidden(IsIncreaseDebtForbiddenCall),
        IsTokenAllowed(IsTokenAllowedCall),
        LiquidateCreditAccount(LiquidateCreditAccountCall),
        Multicall(MulticallCall),
        OpenCreditAccount(OpenCreditAccountCall),
        OpenCreditAccountMulticall(OpenCreditAccountMulticallCall),
        TotalOpenedAccountsDegenMode(TotalOpenedAccountsDegenModeCall),
        TransferAccountOwnership(TransferAccountOwnershipCall),
        TransfersAllowed(TransfersAllowedCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for ICreditFacadeCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddCollateralCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::AddCollateral(decoded));
            }
            if let Ok(decoded) =
                <AllowedContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::AllowedContract(decoded));
            }
            if let Ok(decoded) =
                <AllowedContractsLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::AllowedContractsLength(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <ApproveAccountTransfersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::ApproveAccountTransfers(decoded));
            }
            if let Ok(decoded) =
                <CalcCreditAccountHealthFactorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditFacadeCalls::CalcCreditAccountHealthFactor(decoded));
            }
            if let Ok(decoded) =
                <CalcTotalValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::CalcTotalValue(decoded));
            }
            if let Ok(decoded) =
                <CloseCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::CloseCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <ContractToAdapterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::ContractToAdapter(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <DecreaseDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::DecreaseDebt(decoded));
            }
            if let Ok(decoded) =
                <DegenModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::DegenMode(decoded));
            }
            if let Ok(decoded) =
                <DegenNFTCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::DegenNFT(decoded));
            }
            if let Ok(decoded) =
                <HasOpenedCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::HasOpenedCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <IncreaseDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::IncreaseDebt(decoded));
            }
            if let Ok(decoded) =
                <IsIncreaseDebtForbiddenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::IsIncreaseDebtForbidden(decoded));
            }
            if let Ok(decoded) =
                <IsTokenAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::IsTokenAllowed(decoded));
            }
            if let Ok(decoded) =
                <LiquidateCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::LiquidateCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::Multicall(decoded));
            }
            if let Ok(decoded) =
                <OpenCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::OpenCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <OpenCreditAccountMulticallCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditFacadeCalls::OpenCreditAccountMulticall(decoded));
            }
            if let Ok(decoded) =
                <TotalOpenedAccountsDegenModeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditFacadeCalls::TotalOpenedAccountsDegenMode(decoded));
            }
            if let Ok(decoded) =
                <TransferAccountOwnershipCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ICreditFacadeCalls::TransferAccountOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransfersAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::TransfersAllowed(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditFacadeCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICreditFacadeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICreditFacadeCalls::AddCollateral(element) => element.encode(),
                ICreditFacadeCalls::AllowedContract(element) => element.encode(),
                ICreditFacadeCalls::AllowedContractsLength(element) => element.encode(),
                ICreditFacadeCalls::Approve(element) => element.encode(),
                ICreditFacadeCalls::ApproveAccountTransfers(element) => element.encode(),
                ICreditFacadeCalls::CalcCreditAccountHealthFactor(element) => element.encode(),
                ICreditFacadeCalls::CalcTotalValue(element) => element.encode(),
                ICreditFacadeCalls::CloseCreditAccount(element) => element.encode(),
                ICreditFacadeCalls::ContractToAdapter(element) => element.encode(),
                ICreditFacadeCalls::CreditManager(element) => element.encode(),
                ICreditFacadeCalls::DecreaseDebt(element) => element.encode(),
                ICreditFacadeCalls::DegenMode(element) => element.encode(),
                ICreditFacadeCalls::DegenNFT(element) => element.encode(),
                ICreditFacadeCalls::HasOpenedCreditAccount(element) => element.encode(),
                ICreditFacadeCalls::IncreaseDebt(element) => element.encode(),
                ICreditFacadeCalls::IsIncreaseDebtForbidden(element) => element.encode(),
                ICreditFacadeCalls::IsTokenAllowed(element) => element.encode(),
                ICreditFacadeCalls::LiquidateCreditAccount(element) => element.encode(),
                ICreditFacadeCalls::Multicall(element) => element.encode(),
                ICreditFacadeCalls::OpenCreditAccount(element) => element.encode(),
                ICreditFacadeCalls::OpenCreditAccountMulticall(element) => element.encode(),
                ICreditFacadeCalls::TotalOpenedAccountsDegenMode(element) => element.encode(),
                ICreditFacadeCalls::TransferAccountOwnership(element) => element.encode(),
                ICreditFacadeCalls::TransfersAllowed(element) => element.encode(),
                ICreditFacadeCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICreditFacadeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditFacadeCalls::AddCollateral(element) => element.fmt(f),
                ICreditFacadeCalls::AllowedContract(element) => element.fmt(f),
                ICreditFacadeCalls::AllowedContractsLength(element) => element.fmt(f),
                ICreditFacadeCalls::Approve(element) => element.fmt(f),
                ICreditFacadeCalls::ApproveAccountTransfers(element) => element.fmt(f),
                ICreditFacadeCalls::CalcCreditAccountHealthFactor(element) => element.fmt(f),
                ICreditFacadeCalls::CalcTotalValue(element) => element.fmt(f),
                ICreditFacadeCalls::CloseCreditAccount(element) => element.fmt(f),
                ICreditFacadeCalls::ContractToAdapter(element) => element.fmt(f),
                ICreditFacadeCalls::CreditManager(element) => element.fmt(f),
                ICreditFacadeCalls::DecreaseDebt(element) => element.fmt(f),
                ICreditFacadeCalls::DegenMode(element) => element.fmt(f),
                ICreditFacadeCalls::DegenNFT(element) => element.fmt(f),
                ICreditFacadeCalls::HasOpenedCreditAccount(element) => element.fmt(f),
                ICreditFacadeCalls::IncreaseDebt(element) => element.fmt(f),
                ICreditFacadeCalls::IsIncreaseDebtForbidden(element) => element.fmt(f),
                ICreditFacadeCalls::IsTokenAllowed(element) => element.fmt(f),
                ICreditFacadeCalls::LiquidateCreditAccount(element) => element.fmt(f),
                ICreditFacadeCalls::Multicall(element) => element.fmt(f),
                ICreditFacadeCalls::OpenCreditAccount(element) => element.fmt(f),
                ICreditFacadeCalls::OpenCreditAccountMulticall(element) => element.fmt(f),
                ICreditFacadeCalls::TotalOpenedAccountsDegenMode(element) => element.fmt(f),
                ICreditFacadeCalls::TransferAccountOwnership(element) => element.fmt(f),
                ICreditFacadeCalls::TransfersAllowed(element) => element.fmt(f),
                ICreditFacadeCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddCollateralCall> for ICreditFacadeCalls {
        fn from(var: AddCollateralCall) -> Self {
            ICreditFacadeCalls::AddCollateral(var)
        }
    }
    impl ::std::convert::From<AllowedContractCall> for ICreditFacadeCalls {
        fn from(var: AllowedContractCall) -> Self {
            ICreditFacadeCalls::AllowedContract(var)
        }
    }
    impl ::std::convert::From<AllowedContractsLengthCall> for ICreditFacadeCalls {
        fn from(var: AllowedContractsLengthCall) -> Self {
            ICreditFacadeCalls::AllowedContractsLength(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ICreditFacadeCalls {
        fn from(var: ApproveCall) -> Self {
            ICreditFacadeCalls::Approve(var)
        }
    }
    impl ::std::convert::From<ApproveAccountTransfersCall> for ICreditFacadeCalls {
        fn from(var: ApproveAccountTransfersCall) -> Self {
            ICreditFacadeCalls::ApproveAccountTransfers(var)
        }
    }
    impl ::std::convert::From<CalcCreditAccountHealthFactorCall> for ICreditFacadeCalls {
        fn from(var: CalcCreditAccountHealthFactorCall) -> Self {
            ICreditFacadeCalls::CalcCreditAccountHealthFactor(var)
        }
    }
    impl ::std::convert::From<CalcTotalValueCall> for ICreditFacadeCalls {
        fn from(var: CalcTotalValueCall) -> Self {
            ICreditFacadeCalls::CalcTotalValue(var)
        }
    }
    impl ::std::convert::From<CloseCreditAccountCall> for ICreditFacadeCalls {
        fn from(var: CloseCreditAccountCall) -> Self {
            ICreditFacadeCalls::CloseCreditAccount(var)
        }
    }
    impl ::std::convert::From<ContractToAdapterCall> for ICreditFacadeCalls {
        fn from(var: ContractToAdapterCall) -> Self {
            ICreditFacadeCalls::ContractToAdapter(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for ICreditFacadeCalls {
        fn from(var: CreditManagerCall) -> Self {
            ICreditFacadeCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<DecreaseDebtCall> for ICreditFacadeCalls {
        fn from(var: DecreaseDebtCall) -> Self {
            ICreditFacadeCalls::DecreaseDebt(var)
        }
    }
    impl ::std::convert::From<DegenModeCall> for ICreditFacadeCalls {
        fn from(var: DegenModeCall) -> Self {
            ICreditFacadeCalls::DegenMode(var)
        }
    }
    impl ::std::convert::From<DegenNFTCall> for ICreditFacadeCalls {
        fn from(var: DegenNFTCall) -> Self {
            ICreditFacadeCalls::DegenNFT(var)
        }
    }
    impl ::std::convert::From<HasOpenedCreditAccountCall> for ICreditFacadeCalls {
        fn from(var: HasOpenedCreditAccountCall) -> Self {
            ICreditFacadeCalls::HasOpenedCreditAccount(var)
        }
    }
    impl ::std::convert::From<IncreaseDebtCall> for ICreditFacadeCalls {
        fn from(var: IncreaseDebtCall) -> Self {
            ICreditFacadeCalls::IncreaseDebt(var)
        }
    }
    impl ::std::convert::From<IsIncreaseDebtForbiddenCall> for ICreditFacadeCalls {
        fn from(var: IsIncreaseDebtForbiddenCall) -> Self {
            ICreditFacadeCalls::IsIncreaseDebtForbidden(var)
        }
    }
    impl ::std::convert::From<IsTokenAllowedCall> for ICreditFacadeCalls {
        fn from(var: IsTokenAllowedCall) -> Self {
            ICreditFacadeCalls::IsTokenAllowed(var)
        }
    }
    impl ::std::convert::From<LiquidateCreditAccountCall> for ICreditFacadeCalls {
        fn from(var: LiquidateCreditAccountCall) -> Self {
            ICreditFacadeCalls::LiquidateCreditAccount(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for ICreditFacadeCalls {
        fn from(var: MulticallCall) -> Self {
            ICreditFacadeCalls::Multicall(var)
        }
    }
    impl ::std::convert::From<OpenCreditAccountCall> for ICreditFacadeCalls {
        fn from(var: OpenCreditAccountCall) -> Self {
            ICreditFacadeCalls::OpenCreditAccount(var)
        }
    }
    impl ::std::convert::From<OpenCreditAccountMulticallCall> for ICreditFacadeCalls {
        fn from(var: OpenCreditAccountMulticallCall) -> Self {
            ICreditFacadeCalls::OpenCreditAccountMulticall(var)
        }
    }
    impl ::std::convert::From<TotalOpenedAccountsDegenModeCall> for ICreditFacadeCalls {
        fn from(var: TotalOpenedAccountsDegenModeCall) -> Self {
            ICreditFacadeCalls::TotalOpenedAccountsDegenMode(var)
        }
    }
    impl ::std::convert::From<TransferAccountOwnershipCall> for ICreditFacadeCalls {
        fn from(var: TransferAccountOwnershipCall) -> Self {
            ICreditFacadeCalls::TransferAccountOwnership(var)
        }
    }
    impl ::std::convert::From<TransfersAllowedCall> for ICreditFacadeCalls {
        fn from(var: TransfersAllowedCall) -> Self {
            ICreditFacadeCalls::TransfersAllowed(var)
        }
    }
    impl ::std::convert::From<VersionCall> for ICreditFacadeCalls {
        fn from(var: VersionCall) -> Self {
            ICreditFacadeCalls::Version(var)
        }
    }
}
