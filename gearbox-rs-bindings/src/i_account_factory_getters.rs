pub use iaccountfactorygetters_mod::*;
#[allow(clippy::too_many_arguments)]
mod iaccountfactorygetters_mod {
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
    #[doc = "IAccountFactoryGetters was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IACCOUNTFACTORYGETTERS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"countCreditAccounts\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"countCreditAccountsInStock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditAccounts\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditAccount\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNext\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"head\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tail\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IACCOUNTFACTORYGETTERS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IAccountFactoryGetters<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IAccountFactoryGetters<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IAccountFactoryGetters<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAccountFactoryGetters))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IAccountFactoryGetters<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                IACCOUNTFACTORYGETTERS_ABI.clone(),
                client,
            )
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
                IACCOUNTFACTORYGETTERS_ABI.clone(),
                IACCOUNTFACTORYGETTERS_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `tail` (0x13d8c840) function"]
        pub fn tail(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([19, 216, 200, 64], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IAccountFactoryGetters<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IAccountFactoryGettersCalls {
        CountCreditAccounts(CountCreditAccountsCall),
        CountCreditAccountsInStock(CountCreditAccountsInStockCall),
        CreditAccounts(CreditAccountsCall),
        GetNext(GetNextCall),
        Head(HeadCall),
        Tail(TailCall),
    }
    impl ethers::core::abi::AbiDecode for IAccountFactoryGettersCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CountCreditAccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryGettersCalls::CountCreditAccounts(decoded));
            }
            if let Ok(decoded) =
                <CountCreditAccountsInStockCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IAccountFactoryGettersCalls::CountCreditAccountsInStock(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CreditAccountsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryGettersCalls::CreditAccounts(decoded));
            }
            if let Ok(decoded) =
                <GetNextCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IAccountFactoryGettersCalls::GetNext(decoded));
            }
            if let Ok(decoded) = <HeadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAccountFactoryGettersCalls::Head(decoded));
            }
            if let Ok(decoded) = <TailCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IAccountFactoryGettersCalls::Tail(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IAccountFactoryGettersCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IAccountFactoryGettersCalls::CountCreditAccounts(element) => element.encode(),
                IAccountFactoryGettersCalls::CountCreditAccountsInStock(element) => {
                    element.encode()
                }
                IAccountFactoryGettersCalls::CreditAccounts(element) => element.encode(),
                IAccountFactoryGettersCalls::GetNext(element) => element.encode(),
                IAccountFactoryGettersCalls::Head(element) => element.encode(),
                IAccountFactoryGettersCalls::Tail(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IAccountFactoryGettersCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IAccountFactoryGettersCalls::CountCreditAccounts(element) => element.fmt(f),
                IAccountFactoryGettersCalls::CountCreditAccountsInStock(element) => element.fmt(f),
                IAccountFactoryGettersCalls::CreditAccounts(element) => element.fmt(f),
                IAccountFactoryGettersCalls::GetNext(element) => element.fmt(f),
                IAccountFactoryGettersCalls::Head(element) => element.fmt(f),
                IAccountFactoryGettersCalls::Tail(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CountCreditAccountsCall> for IAccountFactoryGettersCalls {
        fn from(var: CountCreditAccountsCall) -> Self {
            IAccountFactoryGettersCalls::CountCreditAccounts(var)
        }
    }
    impl ::std::convert::From<CountCreditAccountsInStockCall> for IAccountFactoryGettersCalls {
        fn from(var: CountCreditAccountsInStockCall) -> Self {
            IAccountFactoryGettersCalls::CountCreditAccountsInStock(var)
        }
    }
    impl ::std::convert::From<CreditAccountsCall> for IAccountFactoryGettersCalls {
        fn from(var: CreditAccountsCall) -> Self {
            IAccountFactoryGettersCalls::CreditAccounts(var)
        }
    }
    impl ::std::convert::From<GetNextCall> for IAccountFactoryGettersCalls {
        fn from(var: GetNextCall) -> Self {
            IAccountFactoryGettersCalls::GetNext(var)
        }
    }
    impl ::std::convert::From<HeadCall> for IAccountFactoryGettersCalls {
        fn from(var: HeadCall) -> Self {
            IAccountFactoryGettersCalls::Head(var)
        }
    }
    impl ::std::convert::From<TailCall> for IAccountFactoryGettersCalls {
        fn from(var: TailCall) -> Self {
            IAccountFactoryGettersCalls::Tail(var)
        }
    }
}
