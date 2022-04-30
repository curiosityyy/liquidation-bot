pub use idatacompressor_mod::*;
#[allow(clippy::too_many_arguments)]
mod idatacompressor_mod {
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
    #[doc = "IDataCompressor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IDATACOMPRESSOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcExpectedAtOpenHf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"balances\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcExpectedHf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditAccountData\",\"outputs\":[{\"internalType\":\"struct CreditAccountData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"inUse\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmountPlusInterest\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"healthFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct TokenBalance[]\",\"name\":\"balances\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isAllowed\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"canBeClosed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"since\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditAccountList\",\"outputs\":[{\"internalType\":\"struct CreditAccountData[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"inUse\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmountPlusInterest\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"healthFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct TokenBalance[]\",\"name\":\"balances\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isAllowed\",\"type\":\"bool\",\"components\":[]}]},{\"internalType\":\"uint256\",\"name\":\"repayAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"liquidationAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"canBeClosed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"since\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditManagerData\",\"outputs\":[{\"internalType\":\"struct CreditManagerData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"hasAccount\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"canBorrow\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxLeverageFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"availableLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"allowedTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"struct ContractAdapter[]\",\"name\":\"adapters\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"allowedContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"adapter\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditManagersList\",\"outputs\":[{\"internalType\":\"struct CreditManagerData[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"hasAccount\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"canBorrow\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxLeverageFactor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"availableLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"allowedTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"struct ContractAdapter[]\",\"name\":\"adapters\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"allowedContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"adapter\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolData\",\"outputs\":[{\"internalType\":\"struct PoolData\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dieselToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"linearCumulativeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"availableLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expectedLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expectedLiquidityLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalBorrowed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAPY_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAPY_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dieselRate_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"withdrawFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeIndex_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestampLU\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolsList\",\"outputs\":[{\"internalType\":\"struct PoolData[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"underlying\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dieselToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"linearCumulativeIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"availableLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expectedLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expectedLiquidityLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalBorrowed\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAPY_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"borrowAPY_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dieselRate_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"withdrawFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"cumulativeIndex_RAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestampLU\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"addr\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenData\",\"outputs\":[{\"internalType\":\"struct TokenInfo[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"decimals\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasOpenedCreditAccount\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IDATACOMPRESSOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IDataCompressor<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IDataCompressor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IDataCompressor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IDataCompressor))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IDataCompressor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IDATACOMPRESSOR_ABI.clone(), client)
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
                IDATACOMPRESSOR_ABI.clone(),
                IDATACOMPRESSOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `calcExpectedAtOpenHf` (0x39595cf8) function"]
        pub fn calc_expected_at_open_hf(
            &self,
            credit_manager: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            borrowed_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [57, 89, 92, 248],
                    (credit_manager, token, amount, borrowed_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcExpectedHf` (0xba3b7345) function"]
        pub fn calc_expected_hf(
            &self,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            balances: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 59, 115, 69], (credit_manager, borrower, balances))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountData` (0x0dbd616d) function"]
        pub fn get_credit_account_data(
            &self,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, CreditAccountData> {
            self.0
                .method_hash([13, 189, 97, 109], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountList` (0xa80deda3) function"]
        pub fn get_credit_account_list(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CreditAccountData>>
        {
            self.0
                .method_hash([168, 13, 237, 163], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagerData` (0xb10b074e) function"]
        pub fn get_credit_manager_data(
            &self,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, CreditManagerData> {
            self.0
                .method_hash([177, 11, 7, 78], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagersList` (0xb8169039) function"]
        pub fn get_credit_managers_list(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CreditManagerData>>
        {
            self.0
                .method_hash([184, 22, 144, 57], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolData` (0x13d21cdf) function"]
        pub fn get_pool_data(
            &self,
            pool: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, PoolData> {
            self.0
                .method_hash([19, 210, 28, 223], pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolsList` (0x1bcd8fc0) function"]
        pub fn get_pools_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PoolData>> {
            self.0
                .method_hash([27, 205, 143, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokenData` (0xbf2eb19e) function"]
        pub fn get_token_data(
            &self,
            addr: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenInfo>> {
            self.0
                .method_hash([191, 46, 177, 158], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasOpenedCreditAccount` (0xfc9914cb) function"]
        pub fn has_opened_credit_account(
            &self,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 153, 20, 203], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IDataCompressor<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `calcExpectedAtOpenHf`function with signature `calcExpectedAtOpenHf(address,address,uint256,uint256)` and selector `[57, 89, 92, 248]`"]
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
        name = "calcExpectedAtOpenHf",
        abi = "calcExpectedAtOpenHf(address,address,uint256,uint256)"
    )]
    pub struct CalcExpectedAtOpenHfCall {
        pub credit_manager: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub borrowed_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calcExpectedHf`function with signature `calcExpectedHf(address,address,uint256[])` and selector `[186, 59, 115, 69]`"]
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
        name = "calcExpectedHf",
        abi = "calcExpectedHf(address,address,uint256[])"
    )]
    pub struct CalcExpectedHfCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub balances: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `getCreditAccountData`function with signature `getCreditAccountData(address,address)` and selector `[13, 189, 97, 109]`"]
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
        name = "getCreditAccountData",
        abi = "getCreditAccountData(address,address)"
    )]
    pub struct GetCreditAccountDataCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditAccountList`function with signature `getCreditAccountList(address)` and selector `[168, 13, 237, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCreditAccountList", abi = "getCreditAccountList(address)")]
    pub struct GetCreditAccountListCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditManagerData`function with signature `getCreditManagerData(address,address)` and selector `[177, 11, 7, 78]`"]
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
        name = "getCreditManagerData",
        abi = "getCreditManagerData(address,address)"
    )]
    pub struct GetCreditManagerDataCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditManagersList`function with signature `getCreditManagersList(address)` and selector `[184, 22, 144, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCreditManagersList", abi = "getCreditManagersList(address)")]
    pub struct GetCreditManagersListCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPoolData`function with signature `getPoolData(address)` and selector `[19, 210, 28, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolData", abi = "getPoolData(address)")]
    pub struct GetPoolDataCall {
        pub pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPoolsList`function with signature `getPoolsList()` and selector `[27, 205, 143, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolsList", abi = "getPoolsList()")]
    pub struct GetPoolsListCall;
    #[doc = "Container type for all input parameters for the `getTokenData`function with signature `getTokenData(address[])` and selector `[191, 46, 177, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTokenData", abi = "getTokenData(address[])")]
    pub struct GetTokenDataCall {
        pub addr: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `hasOpenedCreditAccount`function with signature `hasOpenedCreditAccount(address,address)` and selector `[252, 153, 20, 203]`"]
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
        abi = "hasOpenedCreditAccount(address,address)"
    )]
    pub struct HasOpenedCreditAccountCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IDataCompressorCalls {
        CalcExpectedAtOpenHf(CalcExpectedAtOpenHfCall),
        CalcExpectedHf(CalcExpectedHfCall),
        GetCreditAccountData(GetCreditAccountDataCall),
        GetCreditAccountList(GetCreditAccountListCall),
        GetCreditManagerData(GetCreditManagerDataCall),
        GetCreditManagersList(GetCreditManagersListCall),
        GetPoolData(GetPoolDataCall),
        GetPoolsList(GetPoolsListCall),
        GetTokenData(GetTokenDataCall),
        HasOpenedCreditAccount(HasOpenedCreditAccountCall),
    }
    impl ethers::core::abi::AbiDecode for IDataCompressorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CalcExpectedAtOpenHfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::CalcExpectedAtOpenHf(decoded));
            }
            if let Ok(decoded) =
                <CalcExpectedHfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::CalcExpectedHf(decoded));
            }
            if let Ok(decoded) =
                <GetCreditAccountDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditAccountData(decoded));
            }
            if let Ok(decoded) =
                <GetCreditAccountListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditAccountList(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagerDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditManagerData(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditManagersList(decoded));
            }
            if let Ok(decoded) =
                <GetPoolDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetPoolData(decoded));
            }
            if let Ok(decoded) =
                <GetPoolsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetPoolsList(decoded));
            }
            if let Ok(decoded) =
                <GetTokenDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetTokenData(decoded));
            }
            if let Ok(decoded) =
                <HasOpenedCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::HasOpenedCreditAccount(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IDataCompressorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IDataCompressorCalls::CalcExpectedAtOpenHf(element) => element.encode(),
                IDataCompressorCalls::CalcExpectedHf(element) => element.encode(),
                IDataCompressorCalls::GetCreditAccountData(element) => element.encode(),
                IDataCompressorCalls::GetCreditAccountList(element) => element.encode(),
                IDataCompressorCalls::GetCreditManagerData(element) => element.encode(),
                IDataCompressorCalls::GetCreditManagersList(element) => element.encode(),
                IDataCompressorCalls::GetPoolData(element) => element.encode(),
                IDataCompressorCalls::GetPoolsList(element) => element.encode(),
                IDataCompressorCalls::GetTokenData(element) => element.encode(),
                IDataCompressorCalls::HasOpenedCreditAccount(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IDataCompressorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IDataCompressorCalls::CalcExpectedAtOpenHf(element) => element.fmt(f),
                IDataCompressorCalls::CalcExpectedHf(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditAccountData(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditAccountList(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditManagerData(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditManagersList(element) => element.fmt(f),
                IDataCompressorCalls::GetPoolData(element) => element.fmt(f),
                IDataCompressorCalls::GetPoolsList(element) => element.fmt(f),
                IDataCompressorCalls::GetTokenData(element) => element.fmt(f),
                IDataCompressorCalls::HasOpenedCreditAccount(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CalcExpectedAtOpenHfCall> for IDataCompressorCalls {
        fn from(var: CalcExpectedAtOpenHfCall) -> Self {
            IDataCompressorCalls::CalcExpectedAtOpenHf(var)
        }
    }
    impl ::std::convert::From<CalcExpectedHfCall> for IDataCompressorCalls {
        fn from(var: CalcExpectedHfCall) -> Self {
            IDataCompressorCalls::CalcExpectedHf(var)
        }
    }
    impl ::std::convert::From<GetCreditAccountDataCall> for IDataCompressorCalls {
        fn from(var: GetCreditAccountDataCall) -> Self {
            IDataCompressorCalls::GetCreditAccountData(var)
        }
    }
    impl ::std::convert::From<GetCreditAccountListCall> for IDataCompressorCalls {
        fn from(var: GetCreditAccountListCall) -> Self {
            IDataCompressorCalls::GetCreditAccountList(var)
        }
    }
    impl ::std::convert::From<GetCreditManagerDataCall> for IDataCompressorCalls {
        fn from(var: GetCreditManagerDataCall) -> Self {
            IDataCompressorCalls::GetCreditManagerData(var)
        }
    }
    impl ::std::convert::From<GetCreditManagersListCall> for IDataCompressorCalls {
        fn from(var: GetCreditManagersListCall) -> Self {
            IDataCompressorCalls::GetCreditManagersList(var)
        }
    }
    impl ::std::convert::From<GetPoolDataCall> for IDataCompressorCalls {
        fn from(var: GetPoolDataCall) -> Self {
            IDataCompressorCalls::GetPoolData(var)
        }
    }
    impl ::std::convert::From<GetPoolsListCall> for IDataCompressorCalls {
        fn from(var: GetPoolsListCall) -> Self {
            IDataCompressorCalls::GetPoolsList(var)
        }
    }
    impl ::std::convert::From<GetTokenDataCall> for IDataCompressorCalls {
        fn from(var: GetTokenDataCall) -> Self {
            IDataCompressorCalls::GetTokenData(var)
        }
    }
    impl ::std::convert::From<HasOpenedCreditAccountCall> for IDataCompressorCalls {
        fn from(var: HasOpenedCreditAccountCall) -> Self {
            IDataCompressorCalls::HasOpenedCreditAccount(var)
        }
    }
}