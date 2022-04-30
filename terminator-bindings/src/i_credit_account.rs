pub use icreditaccount_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditaccount_mod {
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
    #[doc = "ICreditAccount was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITACCOUNT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"swapContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowedAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelAllowance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"connectTo\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cumulativeIndexAtOpen\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransfer\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"since\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateParameters\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICREDITACCOUNT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICreditAccount<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditAccount<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICreditAccount<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditAccount))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICreditAccount<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ICREDITACCOUNT_ABI.clone(), client)
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
                ICREDITACCOUNT_ABI.clone(),
                ICREDITACCOUNT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `approveToken` (0x03105b04) function"]
        pub fn approve_token(
            &self,
            token: ethers::core::types::Address,
            swap_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 16, 91, 4], (token, swap_contract))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowedAmount` (0x1afbb7a4) function"]
        pub fn borrowed_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([26, 251, 183, 164], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelAllowance` (0x19a16039) function"]
        pub fn cancel_allowance(
            &self,
            token: ethers::core::types::Address,
            target_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 161, 96, 57], (token, target_contract))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `connectTo` (0xc75b5a71) function"]
        pub fn connect_to(
            &self,
            credit_manager: ethers::core::types::Address,
            borrowed_amount: ethers::core::types::U256,
            cumulative_index_at_open: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [199, 91, 90, 113],
                    (credit_manager, borrowed_amount, cumulative_index_at_open),
                )
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
        #[doc = "Calls the contract's `cumulativeIndexAtOpen` (0x17d11a15) function"]
        pub fn cumulative_index_at_open(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([23, 209, 26, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x1cff79cd) function"]
        pub fn execute(
            &self,
            destination: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([28, 255, 121, 205], (destination, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x8129fc1c) function"]
        pub fn initialize(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransfer` (0xd1660f99) function"]
        pub fn safe_transfer(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 102, 15, 153], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `since` (0x3dc54b40) function"]
        pub fn since(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([61, 197, 75, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateParameters` (0x16128211) function"]
        pub fn update_parameters(
            &self,
            borrowed_amount: ethers::core::types::U256,
            cumulative_index_at_open: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [22, 18, 130, 17],
                    (borrowed_amount, cumulative_index_at_open),
                )
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ICreditAccount<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `approveToken`function with signature `approveToken(address,address)` and selector `[3, 16, 91, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approveToken", abi = "approveToken(address,address)")]
    pub struct ApproveTokenCall {
        pub token: ethers::core::types::Address,
        pub swap_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `borrowedAmount`function with signature `borrowedAmount()` and selector `[26, 251, 183, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "borrowedAmount", abi = "borrowedAmount()")]
    pub struct BorrowedAmountCall;
    #[doc = "Container type for all input parameters for the `cancelAllowance`function with signature `cancelAllowance(address,address)` and selector `[25, 161, 96, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cancelAllowance", abi = "cancelAllowance(address,address)")]
    pub struct CancelAllowanceCall {
        pub token: ethers::core::types::Address,
        pub target_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `connectTo`function with signature `connectTo(address,uint256,uint256)` and selector `[199, 91, 90, 113]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "connectTo", abi = "connectTo(address,uint256,uint256)")]
    pub struct ConnectToCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrowed_amount: ethers::core::types::U256,
        pub cumulative_index_at_open: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `cumulativeIndexAtOpen`function with signature `cumulativeIndexAtOpen()` and selector `[23, 209, 26, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cumulativeIndexAtOpen", abi = "cumulativeIndexAtOpen()")]
    pub struct CumulativeIndexAtOpenCall;
    #[doc = "Container type for all input parameters for the `execute`function with signature `execute(address,bytes)` and selector `[28, 255, 121, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "execute", abi = "execute(address,bytes)")]
    pub struct ExecuteCall {
        pub destination: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize()` and selector `[129, 41, 252, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    #[doc = "Container type for all input parameters for the `safeTransfer`function with signature `safeTransfer(address,address,uint256)` and selector `[209, 102, 15, 153]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "safeTransfer", abi = "safeTransfer(address,address,uint256)")]
    pub struct SafeTransferCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `since`function with signature `since()` and selector `[61, 197, 75, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "since", abi = "since()")]
    pub struct SinceCall;
    #[doc = "Container type for all input parameters for the `updateParameters`function with signature `updateParameters(uint256,uint256)` and selector `[22, 18, 130, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateParameters", abi = "updateParameters(uint256,uint256)")]
    pub struct UpdateParametersCall {
        pub borrowed_amount: ethers::core::types::U256,
        pub cumulative_index_at_open: ethers::core::types::U256,
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
    pub enum ICreditAccountCalls {
        ApproveToken(ApproveTokenCall),
        BorrowedAmount(BorrowedAmountCall),
        CancelAllowance(CancelAllowanceCall),
        ConnectTo(ConnectToCall),
        CreditManager(CreditManagerCall),
        CumulativeIndexAtOpen(CumulativeIndexAtOpenCall),
        Execute(ExecuteCall),
        Factory(FactoryCall),
        Initialize(InitializeCall),
        SafeTransfer(SafeTransferCall),
        Since(SinceCall),
        UpdateParameters(UpdateParametersCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for ICreditAccountCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::ApproveToken(decoded));
            }
            if let Ok(decoded) =
                <BorrowedAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::BorrowedAmount(decoded));
            }
            if let Ok(decoded) =
                <CancelAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::CancelAllowance(decoded));
            }
            if let Ok(decoded) =
                <ConnectToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::ConnectTo(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <CumulativeIndexAtOpenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::CumulativeIndexAtOpen(decoded));
            }
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::SafeTransfer(decoded));
            }
            if let Ok(decoded) = <SinceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::Since(decoded));
            }
            if let Ok(decoded) =
                <UpdateParametersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::UpdateParameters(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICreditAccountCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICreditAccountCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICreditAccountCalls::ApproveToken(element) => element.encode(),
                ICreditAccountCalls::BorrowedAmount(element) => element.encode(),
                ICreditAccountCalls::CancelAllowance(element) => element.encode(),
                ICreditAccountCalls::ConnectTo(element) => element.encode(),
                ICreditAccountCalls::CreditManager(element) => element.encode(),
                ICreditAccountCalls::CumulativeIndexAtOpen(element) => element.encode(),
                ICreditAccountCalls::Execute(element) => element.encode(),
                ICreditAccountCalls::Factory(element) => element.encode(),
                ICreditAccountCalls::Initialize(element) => element.encode(),
                ICreditAccountCalls::SafeTransfer(element) => element.encode(),
                ICreditAccountCalls::Since(element) => element.encode(),
                ICreditAccountCalls::UpdateParameters(element) => element.encode(),
                ICreditAccountCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICreditAccountCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICreditAccountCalls::ApproveToken(element) => element.fmt(f),
                ICreditAccountCalls::BorrowedAmount(element) => element.fmt(f),
                ICreditAccountCalls::CancelAllowance(element) => element.fmt(f),
                ICreditAccountCalls::ConnectTo(element) => element.fmt(f),
                ICreditAccountCalls::CreditManager(element) => element.fmt(f),
                ICreditAccountCalls::CumulativeIndexAtOpen(element) => element.fmt(f),
                ICreditAccountCalls::Execute(element) => element.fmt(f),
                ICreditAccountCalls::Factory(element) => element.fmt(f),
                ICreditAccountCalls::Initialize(element) => element.fmt(f),
                ICreditAccountCalls::SafeTransfer(element) => element.fmt(f),
                ICreditAccountCalls::Since(element) => element.fmt(f),
                ICreditAccountCalls::UpdateParameters(element) => element.fmt(f),
                ICreditAccountCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveTokenCall> for ICreditAccountCalls {
        fn from(var: ApproveTokenCall) -> Self {
            ICreditAccountCalls::ApproveToken(var)
        }
    }
    impl ::std::convert::From<BorrowedAmountCall> for ICreditAccountCalls {
        fn from(var: BorrowedAmountCall) -> Self {
            ICreditAccountCalls::BorrowedAmount(var)
        }
    }
    impl ::std::convert::From<CancelAllowanceCall> for ICreditAccountCalls {
        fn from(var: CancelAllowanceCall) -> Self {
            ICreditAccountCalls::CancelAllowance(var)
        }
    }
    impl ::std::convert::From<ConnectToCall> for ICreditAccountCalls {
        fn from(var: ConnectToCall) -> Self {
            ICreditAccountCalls::ConnectTo(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for ICreditAccountCalls {
        fn from(var: CreditManagerCall) -> Self {
            ICreditAccountCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CumulativeIndexAtOpenCall> for ICreditAccountCalls {
        fn from(var: CumulativeIndexAtOpenCall) -> Self {
            ICreditAccountCalls::CumulativeIndexAtOpen(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for ICreditAccountCalls {
        fn from(var: ExecuteCall) -> Self {
            ICreditAccountCalls::Execute(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for ICreditAccountCalls {
        fn from(var: FactoryCall) -> Self {
            ICreditAccountCalls::Factory(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for ICreditAccountCalls {
        fn from(var: InitializeCall) -> Self {
            ICreditAccountCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<SafeTransferCall> for ICreditAccountCalls {
        fn from(var: SafeTransferCall) -> Self {
            ICreditAccountCalls::SafeTransfer(var)
        }
    }
    impl ::std::convert::From<SinceCall> for ICreditAccountCalls {
        fn from(var: SinceCall) -> Self {
            ICreditAccountCalls::Since(var)
        }
    }
    impl ::std::convert::From<UpdateParametersCall> for ICreditAccountCalls {
        fn from(var: UpdateParametersCall) -> Self {
            ICreditAccountCalls::UpdateParameters(var)
        }
    }
    impl ::std::convert::From<VersionCall> for ICreditAccountCalls {
        fn from(var: VersionCall) -> Self {
            ICreditAccountCalls::Version(var)
        }
    }
}
