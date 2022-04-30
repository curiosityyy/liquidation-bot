pub use iaccountfactory_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaccountfactory_mod {
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
    #[doc = "IAccountFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IACCOUNTFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AccountMinerChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"InitializeCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ReturnCreditAccount\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TakeForever\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"countCreditAccounts\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"countCreditAccountsInStock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditAccounts\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNext\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"head\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"usedAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"returnCreditAccount\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tail\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"takeCreditAccount\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IACCOUNTFACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAccountFactory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAccountFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAccountFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAccountFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAccountFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IACCOUNTFACTORY_ABI.clone(), client)
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
                IACCOUNTFACTORY_ABI.clone(),
                IACCOUNTFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `countCreditAccounts` (0xb60e8518) function"]
        pub fn count_credit_accounts(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 14, 133, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `countCreditAccountsInStock` (0xb1939763) function"]
        pub fn count_credit_accounts_in_stock(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([177, 147, 151, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditAccounts` (0xe3ba9ace) function"]
        pub fn credit_accounts(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([227, 186, 154, 206], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNext` (0x765e0159) function"]
        pub fn get_next(
            &self,
            credit_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([118, 94, 1, 89], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `head` (0x8f7dcfa3) function"]
        pub fn head(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([143, 125, 207, 163], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `returnCreditAccount` (0x89b77b3e) function"]
        pub fn return_credit_account(
            &self,
            used_account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 183, 123, 62], used_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tail` (0x13d8c840) function"]
        pub fn tail(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([19, 216, 200, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `takeCreditAccount` (0x21d18456) function"]
        pub fn take_credit_account(
            &self,
            borrowed_amount: ethers::core::types::U256,
            cumulative_index_at_open: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash(
                    [33, 209, 132, 86],
                    (borrowed_amount, cumulative_index_at_open),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AccountMinerChanged` event"]
        pub fn account_miner_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AccountMinerChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `InitializeCreditAccount` event"]
        pub fn initialize_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializeCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCreditAccount` event"]
        pub fn new_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReturnCreditAccount` event"]
        pub fn return_credit_account_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReturnCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TakeForever` event"]
        pub fn take_forever_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TakeForeverFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IAccountFactoryEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IAccountFactory<M> {
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
    #[ethevent(name = "AccountMinerChanged", abi = "AccountMinerChanged(address)")]
    pub struct AccountMinerChangedFilter {
        #[ethevent(indexed)]
        pub miner: ethers::core::types::Address,
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
        name = "InitializeCreditAccount",
        abi = "InitializeCreditAccount(address,address)"
    )]
    pub struct InitializeCreditAccountFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "NewCreditAccount", abi = "NewCreditAccount(address)")]
    pub struct NewCreditAccountFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "ReturnCreditAccount", abi = "ReturnCreditAccount(address)")]
    pub struct ReturnCreditAccountFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "TakeForever", abi = "TakeForever(address,address)")]
    pub struct TakeForeverFilter {
        #[ethevent(indexed)]
        pub credit_account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAccountFactoryEvents {
        AccountMinerChangedFilter(AccountMinerChangedFilter),
        InitializeCreditAccountFilter(InitializeCreditAccountFilter),
        NewCreditAccountFilter(NewCreditAccountFilter),
        ReturnCreditAccountFilter(ReturnCreditAccountFilter),
        TakeForeverFilter(TakeForeverFilter),
    }
    impl ethers::contract::EthLogDecode for IAccountFactoryEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccountMinerChangedFilter::decode_log(log) {
                return Ok(IAccountFactoryEvents::AccountMinerChangedFilter(decoded));
            }
            if let Ok(decoded) = InitializeCreditAccountFilter::decode_log(log) {
                return Ok(IAccountFactoryEvents::InitializeCreditAccountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewCreditAccountFilter::decode_log(log) {
                return Ok(IAccountFactoryEvents::NewCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = ReturnCreditAccountFilter::decode_log(log) {
                return Ok(IAccountFactoryEvents::ReturnCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = TakeForeverFilter::decode_log(log) {
                return Ok(IAccountFactoryEvents::TakeForeverFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IAccountFactoryEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAccountFactoryEvents::AccountMinerChangedFilter(element) => element.fmt(f),
                IAccountFactoryEvents::InitializeCreditAccountFilter(element) => element.fmt(f),
                IAccountFactoryEvents::NewCreditAccountFilter(element) => element.fmt(f),
                IAccountFactoryEvents::ReturnCreditAccountFilter(element) => element.fmt(f),
                IAccountFactoryEvents::TakeForeverFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `countCreditAccounts`function with signature `countCreditAccounts()` and selector `[182, 14, 133, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "countCreditAccounts", abi = "countCreditAccounts()")]
    pub struct CountCreditAccountsCall;
    #[doc = "Container type for all input parameters for the `countCreditAccountsInStock`function with signature `countCreditAccountsInStock()` and selector `[177, 147, 151, 99]`"]
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
        name = "countCreditAccountsInStock",
        abi = "countCreditAccountsInStock()"
    )]
    pub struct CountCreditAccountsInStockCall;
    #[doc = "Container type for all input parameters for the `creditAccounts`function with signature `creditAccounts(uint256)` and selector `[227, 186, 154, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditAccounts", abi = "creditAccounts(uint256)")]
    pub struct CreditAccountsCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNext`function with signature `getNext(address)` and selector `[118, 94, 1, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNext", abi = "getNext(address)")]
    pub struct GetNextCall {
        pub credit_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `head`function with signature `head()` and selector `[143, 125, 207, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "head", abi = "head()")]
    pub struct HeadCall;
    #[doc = "Container type for all input parameters for the `returnCreditAccount`function with signature `returnCreditAccount(address)` and selector `[137, 183, 123, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "returnCreditAccount", abi = "returnCreditAccount(address)")]
    pub struct ReturnCreditAccountCall {
        pub used_account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `tail`function with signature `tail()` and selector `[19, 216, 200, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tail", abi = "tail()")]
    pub struct TailCall;
    #[doc = "Container type for all input parameters for the `takeCreditAccount`function with signature `takeCreditAccount(uint256,uint256)` and selector `[33, 209, 132, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "takeCreditAccount", abi = "takeCreditAccount(uint256,uint256)")]
    pub struct TakeCreditAccountCall {
        pub borrowed_amount: ethers::core::types::U256,
        pub cumulative_index_at_open: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAccountFactoryCalls {
        CountCreditAccounts(CountCreditAccountsCall),
        CountCreditAccountsInStock(CountCreditAccountsInStockCall),
        CreditAccounts(CreditAccountsCall),
        GetNext(GetNextCall),
        Head(HeadCall),
        ReturnCreditAccount(ReturnCreditAccountCall),
        Tail(TailCall),
        TakeCreditAccount(TakeCreditAccountCall),
    }
    impl ethers::core::abi::AbiDecode for IAccountFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CountCreditAccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryCalls::CountCreditAccounts(decoded));
            }
            if let Ok(decoded) =
                <CountCreditAccountsInStockCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAccountFactoryCalls::CountCreditAccountsInStock(decoded));
            }
            if let Ok(decoded) =
                <CreditAccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryCalls::CreditAccounts(decoded));
            }
            if let Ok(decoded) =
                <GetNextCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryCalls::GetNext(decoded));
            }
            if let Ok(decoded) = <HeadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAccountFactoryCalls::Head(decoded));
            }
            if let Ok(decoded) =
                <ReturnCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryCalls::ReturnCreditAccount(decoded));
            }
            if let Ok(decoded) = <TailCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAccountFactoryCalls::Tail(decoded));
            }
            if let Ok(decoded) =
                <TakeCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryCalls::TakeCreditAccount(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAccountFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAccountFactoryCalls::CountCreditAccounts(element) => element.encode(),
                IAccountFactoryCalls::CountCreditAccountsInStock(element) => element.encode(),
                IAccountFactoryCalls::CreditAccounts(element) => element.encode(),
                IAccountFactoryCalls::GetNext(element) => element.encode(),
                IAccountFactoryCalls::Head(element) => element.encode(),
                IAccountFactoryCalls::ReturnCreditAccount(element) => element.encode(),
                IAccountFactoryCalls::Tail(element) => element.encode(),
                IAccountFactoryCalls::TakeCreditAccount(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAccountFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAccountFactoryCalls::CountCreditAccounts(element) => element.fmt(f),
                IAccountFactoryCalls::CountCreditAccountsInStock(element) => element.fmt(f),
                IAccountFactoryCalls::CreditAccounts(element) => element.fmt(f),
                IAccountFactoryCalls::GetNext(element) => element.fmt(f),
                IAccountFactoryCalls::Head(element) => element.fmt(f),
                IAccountFactoryCalls::ReturnCreditAccount(element) => element.fmt(f),
                IAccountFactoryCalls::Tail(element) => element.fmt(f),
                IAccountFactoryCalls::TakeCreditAccount(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CountCreditAccountsCall> for IAccountFactoryCalls {
        fn from(var: CountCreditAccountsCall) -> Self {
            IAccountFactoryCalls::CountCreditAccounts(var)
        }
    }
    impl ::std::convert::From<CountCreditAccountsInStockCall> for IAccountFactoryCalls {
        fn from(var: CountCreditAccountsInStockCall) -> Self {
            IAccountFactoryCalls::CountCreditAccountsInStock(var)
        }
    }
    impl ::std::convert::From<CreditAccountsCall> for IAccountFactoryCalls {
        fn from(var: CreditAccountsCall) -> Self {
            IAccountFactoryCalls::CreditAccounts(var)
        }
    }
    impl ::std::convert::From<GetNextCall> for IAccountFactoryCalls {
        fn from(var: GetNextCall) -> Self {
            IAccountFactoryCalls::GetNext(var)
        }
    }
    impl ::std::convert::From<HeadCall> for IAccountFactoryCalls {
        fn from(var: HeadCall) -> Self {
            IAccountFactoryCalls::Head(var)
        }
    }
    impl ::std::convert::From<ReturnCreditAccountCall> for IAccountFactoryCalls {
        fn from(var: ReturnCreditAccountCall) -> Self {
            IAccountFactoryCalls::ReturnCreditAccount(var)
        }
    }
    impl ::std::convert::From<TailCall> for IAccountFactoryCalls {
        fn from(var: TailCall) -> Self {
            IAccountFactoryCalls::Tail(var)
        }
    }
    impl ::std::convert::From<TakeCreditAccountCall> for IAccountFactoryCalls {
        fn from(var: TakeCreditAccountCall) -> Self {
            IAccountFactoryCalls::TakeCreditAccount(var)
        }
    }
}
