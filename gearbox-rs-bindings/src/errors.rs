pub use errors_mod::*;
#[allow(clippy::too_many_arguments)]
mod errors_mod {
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
    #[doc = "Errors was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ERRORS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ACL_CALLER_NOT_CONFIGURATOR\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ACL_CALLER_NOT_PAUSABLE_ADMIN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"AF_CANT_CLOSE_CREDIT_ACCOUNT_IN_THE_SAME_BLOCK\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"AF_CREDIT_ACCOUNT_NOT_IN_STOCK\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"AF_EXTERNAL_ACCOUNTS_ARE_FORBIDDEN\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"AF_MINING_IS_FINISHED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"AS_ADDRESS_NOT_FOUND\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CA_CONNECTED_CREDIT_MANAGER_ONLY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CA_FACTORY_ONLY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CR_CREDIT_MANAGER_ALREADY_ADDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CR_POOL_ALREADY_ADDED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INCORRECT_ARRAY_LENGTH\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INCORRECT_PARAMETER\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"INCORRECT_PATH_LENGTH\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MATH_ADDITION_OVERFLOW\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MATH_DIVISION_BY_ZERO\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MATH_MULTIPLICATION_OVERFLOW\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"NOT_IMPLEMENTED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_CANT_ADD_CREDIT_MANAGER_TWICE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_CONNECTED_CREDIT_MANAGERS_ONLY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_INCOMPATIBLE_CREDIT_ACCOUNT_MANAGER\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_INCORRECT_WITHDRAW_FEE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL_MORE_THAN_EXPECTED_LIQUIDITY_LIMIT\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REGISTERED_CREDIT_ACCOUNT_MANAGERS_ONLY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"REGISTERED_POOLS_ONLY\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TD_CONTRIBUTOR_IS_NOT_REGISTERED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TD_INCORRECT_WEIGHTS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TD_NON_ZERO_BALANCE_AFTER_DISTRIBUTION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TD_WALLET_IS_ALREADY_CONNECTED_TO_VC\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WG_DESTINATION_IS_NOT_WETH_COMPATIBLE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WG_NOT_ENOUGH_FUNDS\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WG_RECEIVE_IS_NOT_ALLOWED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_ADDRESS_IS_NOT_ALLOWED\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERRORS_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61070561003a600b82828239805160001a60731461002d57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe73000000000000000000000000000000000000000030146080604052600436106102055760003560e01c806387f88ef411610124578063bdcb2576116100b7578063de63cd4011610086578063de63cd40146105d0578063e7f3be0c146105f2578063ebbd977f14610613578063fe32e65d14610636578063ff2a04e31461065857600080fd5b8063bdcb257614610549578063beea5ec21461056a578063ccbf92781461058c578063d1a65a38146105ae57600080fd5b8063a988ac60116100f3578063a988ac60146104c0578063abc3d254146104e3578063ac75713914610505578063b563b3001461052757600080fd5b806387f88ef41461043957806394391a4a1461045b57806399a98c991461047c578063a27c03701461049e57600080fd5b80633647c9f91161019c57806343f6e4ab1161016b57806343f6e4ab146103b2578063447d8e42146103d357806353278911146103f557806376d9ebb81461041757600080fd5b80633647c9f91461032d5780633df46fe51461034f5780633f3153b2146103705780634349e3d81461039157600080fd5b80630f5ee482116101d85780630f5ee482146102a6578063119427c5146102c75780632357f362146102e957806328432c221461030b57600080fd5b8063029d23441461020a5780630a2b1d3a146102415780630afeee97146102635780630c9409e714610284575b600080fd5b61022b604051806040016040528060028152602001614d3160f01b81525081565b604051610238919061067a565b60405180910390f35b61022b6040518060400160405280600381526020016243523160e81b81525081565b61022b60405180604001604052806002815260200161052560f41b81525081565b61022b6040518060400160405280600381526020016241463160e81b81525081565b61022b60405180604001604052806002815260200161269960f11b81525081565b61022b6040518060400160405280600381526020016228299960e91b81525081565b61022b6040518060400160405280600381526020016221a09960e91b81525081565b61022b6040518060400160405280600381526020016250533360e81b81525081565b61022b6040518060400160405280600381526020016257473160e81b81525081565b61022b60405180604001604052806002815260200161141360f21b81525081565b61022b6040518060400160405280600281526020016105a360f41b81525081565b61022b604051806040016040528060028152602001614d3360f01b81525081565b61022b604051806040016040528060028152602001614e4960f01b81525081565b61022b604051806040016040528060038152602001622ba39960e91b81525081565b61022b6040518060400160405280600381526020016254443160e81b81525081565b61022b6040518060400160405280600381526020016205053360ec1b81525081565b61022b6040518060400160405280600381526020016220a31960e91b81525081565b61022b60405180604001604052806002815260200161043560f41b81525081565b61022b6040518060400160405280600381526020016221a91960e91b81525081565b61022b6040518060400160405280600381526020016250533160e81b81525081565b61022b6040518060400160405280600481526020016341434c3160e01b81525081565b61022b6040518060400160405280600381526020016254443360e81b81525081565b61022b6040518060400160405280600381526020016241463360e81b81525081565b61022b6040518060400160405280600381526020016215110d60ea1b81525081565b61022b60405180604001604052806002815260200161049560f41b81525081565b61022b6040518060400160405280600381526020016257473360e81b81525081565b61022b604051806040016040528060038152602001621414cd60ea1b81525081565b61022b6040518060400160405280600381526020016210518d60ea1b81525081565b61022b6040518060400160405280600381526020016241503160e81b81525081565b61022b6040518060400160405280600281526020016121a960f11b81525081565b61022b6040518060400160405280600481526020016320a1a61960e11b81525081565b61022b604051806040016040528060038152602001622a221960e91b81525081565b61022b6040518060400160405280600381526020016243413160e81b81525081565b600060208083528351808285015260005b818110156106a75785810183015185820160400152820161068b565b818111156106b9576000604083870101525b50601f01601f191692909201604001939250505056fea26469706673582212206570ae8ca7a211d7252fe203ac6c645f57b0fad429d21c086bd278a97539500e64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Errors<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Errors<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Errors<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Errors))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Errors<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERRORS_ABI.clone(), client).into()
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
                ERRORS_ABI.clone(),
                ERRORS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ACL_CALLER_NOT_CONFIGURATOR` (0xebbd977f) function"]
        pub fn acl_caller_not_configurator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([235, 189, 151, 127], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ACL_CALLER_NOT_PAUSABLE_ADMIN` (0xa988ac60) function"]
        pub fn acl_caller_not_pausable_admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([169, 136, 172, 96], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AF_CANT_CLOSE_CREDIT_ACCOUNT_IN_THE_SAME_BLOCK` (0x0c9409e7) function"]
        pub fn af_cant_close_credit_account_in_the_same_block(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([12, 148, 9, 231], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AF_CREDIT_ACCOUNT_NOT_IN_STOCK` (0xac757139) function"]
        pub fn af_credit_account_not_in_stock(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([172, 117, 113, 57], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AF_EXTERNAL_ACCOUNTS_ARE_FORBIDDEN` (0xd1a65a38) function"]
        pub fn af_external_accounts_are_forbidden(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([209, 166, 90, 56], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AF_MINING_IS_FINISHED` (0x87f88ef4) function"]
        pub fn af_mining_is_finished(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([135, 248, 142, 244], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AS_ADDRESS_NOT_FOUND` (0xde63cd40) function"]
        pub fn as_address_not_found(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([222, 99, 205, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CA_CONNECTED_CREDIT_MANAGER_ONLY` (0xff2a04e3) function"]
        pub fn ca_connected_credit_manager_only(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([255, 42, 4, 227], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CA_FACTORY_ONLY` (0x2357f362) function"]
        pub fn ca_factory_only(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([35, 87, 243, 98], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CR_CREDIT_MANAGER_ALREADY_ADDED` (0x99a98c99) function"]
        pub fn cr_credit_manager_already_added(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([153, 169, 140, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CR_POOL_ALREADY_ADDED` (0x0a2b1d3a) function"]
        pub fn cr_pool_already_added(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([10, 43, 29, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INCORRECT_ARRAY_LENGTH` (0xe7f3be0c) function"]
        pub fn incorrect_array_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([231, 243, 190, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INCORRECT_PARAMETER` (0xbdcb2576) function"]
        pub fn incorrect_parameter(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([189, 203, 37, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `INCORRECT_PATH_LENGTH` (0x3df46fe5) function"]
        pub fn incorrect_path_length(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([61, 244, 111, 229], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MATH_ADDITION_OVERFLOW` (0x0f5ee482) function"]
        pub fn math_addition_overflow(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([15, 94, 228, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MATH_DIVISION_BY_ZERO` (0x4349e3d8) function"]
        pub fn math_division_by_zero(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([67, 73, 227, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MATH_MULTIPLICATION_OVERFLOW` (0x029d2344) function"]
        pub fn math_multiplication_overflow(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([2, 157, 35, 68], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NOT_IMPLEMENTED` (0x43f6e4ab) function"]
        pub fn not_implemented(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([67, 246, 228, 171], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_CANT_ADD_CREDIT_MANAGER_TWICE` (0xccbf9278) function"]
        pub fn pool_cant_add_credit_manager_twice(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([204, 191, 146, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_CONNECTED_CREDIT_MANAGERS_ONLY` (0x76d9ebb8) function"]
        pub fn pool_connected_credit_managers_only(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([118, 217, 235, 184], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_INCOMPATIBLE_CREDIT_ACCOUNT_MANAGER` (0xa27c0370) function"]
        pub fn pool_incompatible_credit_account_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([162, 124, 3, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_INCORRECT_WITHDRAW_FEE` (0x28432c22) function"]
        pub fn pool_incorrect_withdraw_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([40, 67, 44, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_MORE_THAN_EXPECTED_LIQUIDITY_LIMIT` (0x119427c5) function"]
        pub fn pool_more_than_expected_liquidity_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([17, 148, 39, 197], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `REGISTERED_CREDIT_ACCOUNT_MANAGERS_ONLY` (0x94391a4a) function"]
        pub fn registered_credit_account_managers_only(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([148, 57, 26, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `REGISTERED_POOLS_ONLY` (0x0afeee97) function"]
        pub fn registered_pools_only(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([10, 254, 238, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TD_CONTRIBUTOR_IS_NOT_REGISTERED` (0xb563b300) function"]
        pub fn td_contributor_is_not_registered(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([181, 99, 179, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TD_INCORRECT_WEIGHTS` (0xfe32e65d) function"]
        pub fn td_incorrect_weights(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([254, 50, 230, 93], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TD_NON_ZERO_BALANCE_AFTER_DISTRIBUTION` (0xabc3d254) function"]
        pub fn td_non_zero_balance_after_distribution(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([171, 195, 210, 84], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TD_WALLET_IS_ALREADY_CONNECTED_TO_VC` (0x53278911) function"]
        pub fn td_wallet_is_already_connected_to_vc(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([83, 39, 137, 17], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WG_DESTINATION_IS_NOT_WETH_COMPATIBLE` (0x3647c9f9) function"]
        pub fn wg_destination_is_not_weth_compatible(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([54, 71, 201, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WG_NOT_ENOUGH_FUNDS` (0xbeea5ec2) function"]
        pub fn wg_not_enough_funds(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([190, 234, 94, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WG_RECEIVE_IS_NOT_ALLOWED` (0x447d8e42) function"]
        pub fn wg_receive_is_not_allowed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([68, 125, 142, 66], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ZERO_ADDRESS_IS_NOT_ALLOWED` (0x3f3153b2) function"]
        pub fn zero_address_is_not_allowed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([63, 49, 83, 178], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Errors<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ACL_CALLER_NOT_CONFIGURATOR`function with signature `ACL_CALLER_NOT_CONFIGURATOR()` and selector `[235, 189, 151, 127]`"]
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
        name = "ACL_CALLER_NOT_CONFIGURATOR",
        abi = "ACL_CALLER_NOT_CONFIGURATOR()"
    )]
    pub struct AclCallerNotConfiguratorCall;
    #[doc = "Container type for all input parameters for the `ACL_CALLER_NOT_PAUSABLE_ADMIN`function with signature `ACL_CALLER_NOT_PAUSABLE_ADMIN()` and selector `[169, 136, 172, 96]`"]
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
        name = "ACL_CALLER_NOT_PAUSABLE_ADMIN",
        abi = "ACL_CALLER_NOT_PAUSABLE_ADMIN()"
    )]
    pub struct AclCallerNotPausableAdminCall;
    #[doc = "Container type for all input parameters for the `AF_CANT_CLOSE_CREDIT_ACCOUNT_IN_THE_SAME_BLOCK`function with signature `AF_CANT_CLOSE_CREDIT_ACCOUNT_IN_THE_SAME_BLOCK()` and selector `[12, 148, 9, 231]`"]
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
        name = "AF_CANT_CLOSE_CREDIT_ACCOUNT_IN_THE_SAME_BLOCK",
        abi = "AF_CANT_CLOSE_CREDIT_ACCOUNT_IN_THE_SAME_BLOCK()"
    )]
    pub struct AfCantCloseCreditAccountInTheSameBlockCall;
    #[doc = "Container type for all input parameters for the `AF_CREDIT_ACCOUNT_NOT_IN_STOCK`function with signature `AF_CREDIT_ACCOUNT_NOT_IN_STOCK()` and selector `[172, 117, 113, 57]`"]
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
        name = "AF_CREDIT_ACCOUNT_NOT_IN_STOCK",
        abi = "AF_CREDIT_ACCOUNT_NOT_IN_STOCK()"
    )]
    pub struct AfCreditAccountNotInStockCall;
    #[doc = "Container type for all input parameters for the `AF_EXTERNAL_ACCOUNTS_ARE_FORBIDDEN`function with signature `AF_EXTERNAL_ACCOUNTS_ARE_FORBIDDEN()` and selector `[209, 166, 90, 56]`"]
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
        name = "AF_EXTERNAL_ACCOUNTS_ARE_FORBIDDEN",
        abi = "AF_EXTERNAL_ACCOUNTS_ARE_FORBIDDEN()"
    )]
    pub struct AfExternalAccountsAreForbiddenCall;
    #[doc = "Container type for all input parameters for the `AF_MINING_IS_FINISHED`function with signature `AF_MINING_IS_FINISHED()` and selector `[135, 248, 142, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "AF_MINING_IS_FINISHED", abi = "AF_MINING_IS_FINISHED()")]
    pub struct AfMiningIsFinishedCall;
    #[doc = "Container type for all input parameters for the `AS_ADDRESS_NOT_FOUND`function with signature `AS_ADDRESS_NOT_FOUND()` and selector `[222, 99, 205, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "AS_ADDRESS_NOT_FOUND", abi = "AS_ADDRESS_NOT_FOUND()")]
    pub struct AsAddressNotFoundCall;
    #[doc = "Container type for all input parameters for the `CA_CONNECTED_CREDIT_MANAGER_ONLY`function with signature `CA_CONNECTED_CREDIT_MANAGER_ONLY()` and selector `[255, 42, 4, 227]`"]
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
        name = "CA_CONNECTED_CREDIT_MANAGER_ONLY",
        abi = "CA_CONNECTED_CREDIT_MANAGER_ONLY()"
    )]
    pub struct CaConnectedCreditManagerOnlyCall;
    #[doc = "Container type for all input parameters for the `CA_FACTORY_ONLY`function with signature `CA_FACTORY_ONLY()` and selector `[35, 87, 243, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CA_FACTORY_ONLY", abi = "CA_FACTORY_ONLY()")]
    pub struct CaFactoryOnlyCall;
    #[doc = "Container type for all input parameters for the `CR_CREDIT_MANAGER_ALREADY_ADDED`function with signature `CR_CREDIT_MANAGER_ALREADY_ADDED()` and selector `[153, 169, 140, 153]`"]
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
        name = "CR_CREDIT_MANAGER_ALREADY_ADDED",
        abi = "CR_CREDIT_MANAGER_ALREADY_ADDED()"
    )]
    pub struct CrCreditManagerAlreadyAddedCall;
    #[doc = "Container type for all input parameters for the `CR_POOL_ALREADY_ADDED`function with signature `CR_POOL_ALREADY_ADDED()` and selector `[10, 43, 29, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CR_POOL_ALREADY_ADDED", abi = "CR_POOL_ALREADY_ADDED()")]
    pub struct CrPoolAlreadyAddedCall;
    #[doc = "Container type for all input parameters for the `INCORRECT_ARRAY_LENGTH`function with signature `INCORRECT_ARRAY_LENGTH()` and selector `[231, 243, 190, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INCORRECT_ARRAY_LENGTH", abi = "INCORRECT_ARRAY_LENGTH()")]
    pub struct IncorrectArrayLengthCall;
    #[doc = "Container type for all input parameters for the `INCORRECT_PARAMETER`function with signature `INCORRECT_PARAMETER()` and selector `[189, 203, 37, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INCORRECT_PARAMETER", abi = "INCORRECT_PARAMETER()")]
    pub struct IncorrectParameterCall;
    #[doc = "Container type for all input parameters for the `INCORRECT_PATH_LENGTH`function with signature `INCORRECT_PATH_LENGTH()` and selector `[61, 244, 111, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "INCORRECT_PATH_LENGTH", abi = "INCORRECT_PATH_LENGTH()")]
    pub struct IncorrectPathLengthCall;
    #[doc = "Container type for all input parameters for the `MATH_ADDITION_OVERFLOW`function with signature `MATH_ADDITION_OVERFLOW()` and selector `[15, 94, 228, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MATH_ADDITION_OVERFLOW", abi = "MATH_ADDITION_OVERFLOW()")]
    pub struct MathAdditionOverflowCall;
    #[doc = "Container type for all input parameters for the `MATH_DIVISION_BY_ZERO`function with signature `MATH_DIVISION_BY_ZERO()` and selector `[67, 73, 227, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MATH_DIVISION_BY_ZERO", abi = "MATH_DIVISION_BY_ZERO()")]
    pub struct MathDivisionByZeroCall;
    #[doc = "Container type for all input parameters for the `MATH_MULTIPLICATION_OVERFLOW`function with signature `MATH_MULTIPLICATION_OVERFLOW()` and selector `[2, 157, 35, 68]`"]
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
        name = "MATH_MULTIPLICATION_OVERFLOW",
        abi = "MATH_MULTIPLICATION_OVERFLOW()"
    )]
    pub struct MathMultiplicationOverflowCall;
    #[doc = "Container type for all input parameters for the `NOT_IMPLEMENTED`function with signature `NOT_IMPLEMENTED()` and selector `[67, 246, 228, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "NOT_IMPLEMENTED", abi = "NOT_IMPLEMENTED()")]
    pub struct NotImplementedCall;
    #[doc = "Container type for all input parameters for the `POOL_CANT_ADD_CREDIT_MANAGER_TWICE`function with signature `POOL_CANT_ADD_CREDIT_MANAGER_TWICE()` and selector `[204, 191, 146, 120]`"]
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
        name = "POOL_CANT_ADD_CREDIT_MANAGER_TWICE",
        abi = "POOL_CANT_ADD_CREDIT_MANAGER_TWICE()"
    )]
    pub struct PoolCantAddCreditManagerTwiceCall;
    #[doc = "Container type for all input parameters for the `POOL_CONNECTED_CREDIT_MANAGERS_ONLY`function with signature `POOL_CONNECTED_CREDIT_MANAGERS_ONLY()` and selector `[118, 217, 235, 184]`"]
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
        name = "POOL_CONNECTED_CREDIT_MANAGERS_ONLY",
        abi = "POOL_CONNECTED_CREDIT_MANAGERS_ONLY()"
    )]
    pub struct PoolConnectedCreditManagersOnlyCall;
    #[doc = "Container type for all input parameters for the `POOL_INCOMPATIBLE_CREDIT_ACCOUNT_MANAGER`function with signature `POOL_INCOMPATIBLE_CREDIT_ACCOUNT_MANAGER()` and selector `[162, 124, 3, 112]`"]
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
        name = "POOL_INCOMPATIBLE_CREDIT_ACCOUNT_MANAGER",
        abi = "POOL_INCOMPATIBLE_CREDIT_ACCOUNT_MANAGER()"
    )]
    pub struct PoolIncompatibleCreditAccountManagerCall;
    #[doc = "Container type for all input parameters for the `POOL_INCORRECT_WITHDRAW_FEE`function with signature `POOL_INCORRECT_WITHDRAW_FEE()` and selector `[40, 67, 44, 34]`"]
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
        name = "POOL_INCORRECT_WITHDRAW_FEE",
        abi = "POOL_INCORRECT_WITHDRAW_FEE()"
    )]
    pub struct PoolIncorrectWithdrawFeeCall;
    #[doc = "Container type for all input parameters for the `POOL_MORE_THAN_EXPECTED_LIQUIDITY_LIMIT`function with signature `POOL_MORE_THAN_EXPECTED_LIQUIDITY_LIMIT()` and selector `[17, 148, 39, 197]`"]
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
        name = "POOL_MORE_THAN_EXPECTED_LIQUIDITY_LIMIT",
        abi = "POOL_MORE_THAN_EXPECTED_LIQUIDITY_LIMIT()"
    )]
    pub struct PoolMoreThanExpectedLiquidityLimitCall;
    #[doc = "Container type for all input parameters for the `REGISTERED_CREDIT_ACCOUNT_MANAGERS_ONLY`function with signature `REGISTERED_CREDIT_ACCOUNT_MANAGERS_ONLY()` and selector `[148, 57, 26, 74]`"]
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
        name = "REGISTERED_CREDIT_ACCOUNT_MANAGERS_ONLY",
        abi = "REGISTERED_CREDIT_ACCOUNT_MANAGERS_ONLY()"
    )]
    pub struct RegisteredCreditAccountManagersOnlyCall;
    #[doc = "Container type for all input parameters for the `REGISTERED_POOLS_ONLY`function with signature `REGISTERED_POOLS_ONLY()` and selector `[10, 254, 238, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "REGISTERED_POOLS_ONLY", abi = "REGISTERED_POOLS_ONLY()")]
    pub struct RegisteredPoolsOnlyCall;
    #[doc = "Container type for all input parameters for the `TD_CONTRIBUTOR_IS_NOT_REGISTERED`function with signature `TD_CONTRIBUTOR_IS_NOT_REGISTERED()` and selector `[181, 99, 179, 0]`"]
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
        name = "TD_CONTRIBUTOR_IS_NOT_REGISTERED",
        abi = "TD_CONTRIBUTOR_IS_NOT_REGISTERED()"
    )]
    pub struct TdContributorIsNotRegisteredCall;
    #[doc = "Container type for all input parameters for the `TD_INCORRECT_WEIGHTS`function with signature `TD_INCORRECT_WEIGHTS()` and selector `[254, 50, 230, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "TD_INCORRECT_WEIGHTS", abi = "TD_INCORRECT_WEIGHTS()")]
    pub struct TdIncorrectWeightsCall;
    #[doc = "Container type for all input parameters for the `TD_NON_ZERO_BALANCE_AFTER_DISTRIBUTION`function with signature `TD_NON_ZERO_BALANCE_AFTER_DISTRIBUTION()` and selector `[171, 195, 210, 84]`"]
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
        name = "TD_NON_ZERO_BALANCE_AFTER_DISTRIBUTION",
        abi = "TD_NON_ZERO_BALANCE_AFTER_DISTRIBUTION()"
    )]
    pub struct TdNonZeroBalanceAfterDistributionCall;
    #[doc = "Container type for all input parameters for the `TD_WALLET_IS_ALREADY_CONNECTED_TO_VC`function with signature `TD_WALLET_IS_ALREADY_CONNECTED_TO_VC()` and selector `[83, 39, 137, 17]`"]
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
        name = "TD_WALLET_IS_ALREADY_CONNECTED_TO_VC",
        abi = "TD_WALLET_IS_ALREADY_CONNECTED_TO_VC()"
    )]
    pub struct TdWalletIsAlreadyConnectedToVcCall;
    #[doc = "Container type for all input parameters for the `WG_DESTINATION_IS_NOT_WETH_COMPATIBLE`function with signature `WG_DESTINATION_IS_NOT_WETH_COMPATIBLE()` and selector `[54, 71, 201, 249]`"]
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
        name = "WG_DESTINATION_IS_NOT_WETH_COMPATIBLE",
        abi = "WG_DESTINATION_IS_NOT_WETH_COMPATIBLE()"
    )]
    pub struct WgDestinationIsNotWethCompatibleCall;
    #[doc = "Container type for all input parameters for the `WG_NOT_ENOUGH_FUNDS`function with signature `WG_NOT_ENOUGH_FUNDS()` and selector `[190, 234, 94, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "WG_NOT_ENOUGH_FUNDS", abi = "WG_NOT_ENOUGH_FUNDS()")]
    pub struct WgNotEnoughFundsCall;
    #[doc = "Container type for all input parameters for the `WG_RECEIVE_IS_NOT_ALLOWED`function with signature `WG_RECEIVE_IS_NOT_ALLOWED()` and selector `[68, 125, 142, 66]`"]
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
        name = "WG_RECEIVE_IS_NOT_ALLOWED",
        abi = "WG_RECEIVE_IS_NOT_ALLOWED()"
    )]
    pub struct WgReceiveIsNotAllowedCall;
    #[doc = "Container type for all input parameters for the `ZERO_ADDRESS_IS_NOT_ALLOWED`function with signature `ZERO_ADDRESS_IS_NOT_ALLOWED()` and selector `[63, 49, 83, 178]`"]
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
        name = "ZERO_ADDRESS_IS_NOT_ALLOWED",
        abi = "ZERO_ADDRESS_IS_NOT_ALLOWED()"
    )]
    pub struct ZeroAddressIsNotAllowedCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ErrorsCalls {
        AclCallerNotConfigurator(AclCallerNotConfiguratorCall),
        AclCallerNotPausableAdmin(AclCallerNotPausableAdminCall),
        AfCantCloseCreditAccountInTheSameBlock(AfCantCloseCreditAccountInTheSameBlockCall),
        AfCreditAccountNotInStock(AfCreditAccountNotInStockCall),
        AfExternalAccountsAreForbidden(AfExternalAccountsAreForbiddenCall),
        AfMiningIsFinished(AfMiningIsFinishedCall),
        AsAddressNotFound(AsAddressNotFoundCall),
        CaConnectedCreditManagerOnly(CaConnectedCreditManagerOnlyCall),
        CaFactoryOnly(CaFactoryOnlyCall),
        CrCreditManagerAlreadyAdded(CrCreditManagerAlreadyAddedCall),
        CrPoolAlreadyAdded(CrPoolAlreadyAddedCall),
        IncorrectArrayLength(IncorrectArrayLengthCall),
        IncorrectParameter(IncorrectParameterCall),
        IncorrectPathLength(IncorrectPathLengthCall),
        MathAdditionOverflow(MathAdditionOverflowCall),
        MathDivisionByZero(MathDivisionByZeroCall),
        MathMultiplicationOverflow(MathMultiplicationOverflowCall),
        NotImplemented(NotImplementedCall),
        PoolCantAddCreditManagerTwice(PoolCantAddCreditManagerTwiceCall),
        PoolConnectedCreditManagersOnly(PoolConnectedCreditManagersOnlyCall),
        PoolIncompatibleCreditAccountManager(PoolIncompatibleCreditAccountManagerCall),
        PoolIncorrectWithdrawFee(PoolIncorrectWithdrawFeeCall),
        PoolMoreThanExpectedLiquidityLimit(PoolMoreThanExpectedLiquidityLimitCall),
        RegisteredCreditAccountManagersOnly(RegisteredCreditAccountManagersOnlyCall),
        RegisteredPoolsOnly(RegisteredPoolsOnlyCall),
        TdContributorIsNotRegistered(TdContributorIsNotRegisteredCall),
        TdIncorrectWeights(TdIncorrectWeightsCall),
        TdNonZeroBalanceAfterDistribution(TdNonZeroBalanceAfterDistributionCall),
        TdWalletIsAlreadyConnectedToVc(TdWalletIsAlreadyConnectedToVcCall),
        WgDestinationIsNotWethCompatible(WgDestinationIsNotWethCompatibleCall),
        WgNotEnoughFunds(WgNotEnoughFundsCall),
        WgReceiveIsNotAllowed(WgReceiveIsNotAllowedCall),
        ZeroAddressIsNotAllowed(ZeroAddressIsNotAllowedCall),
    }
    impl ethers::core::abi::AbiDecode for ErrorsCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AclCallerNotConfiguratorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AclCallerNotConfigurator(decoded));
            }
            if let Ok(decoded) =
                <AclCallerNotPausableAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AclCallerNotPausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <AfCantCloseCreditAccountInTheSameBlockCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AfCantCloseCreditAccountInTheSameBlock(decoded));
            }
            if let Ok(decoded) =
                <AfCreditAccountNotInStockCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AfCreditAccountNotInStock(decoded));
            }
            if let Ok(decoded) =
                <AfExternalAccountsAreForbiddenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::AfExternalAccountsAreForbidden(decoded));
            }
            if let Ok(decoded) =
                <AfMiningIsFinishedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::AfMiningIsFinished(decoded));
            }
            if let Ok(decoded) =
                <AsAddressNotFoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::AsAddressNotFound(decoded));
            }
            if let Ok(decoded) =
                <CaConnectedCreditManagerOnlyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CaConnectedCreditManagerOnly(decoded));
            }
            if let Ok(decoded) =
                <CaFactoryOnlyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CaFactoryOnly(decoded));
            }
            if let Ok(decoded) =
                <CrCreditManagerAlreadyAddedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::CrCreditManagerAlreadyAdded(decoded));
            }
            if let Ok(decoded) =
                <CrPoolAlreadyAddedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::CrPoolAlreadyAdded(decoded));
            }
            if let Ok(decoded) =
                <IncorrectArrayLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::IncorrectArrayLength(decoded));
            }
            if let Ok(decoded) =
                <IncorrectParameterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::IncorrectParameter(decoded));
            }
            if let Ok(decoded) =
                <IncorrectPathLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::IncorrectPathLength(decoded));
            }
            if let Ok(decoded) =
                <MathAdditionOverflowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::MathAdditionOverflow(decoded));
            }
            if let Ok(decoded) =
                <MathDivisionByZeroCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::MathDivisionByZero(decoded));
            }
            if let Ok(decoded) =
                <MathMultiplicationOverflowCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::MathMultiplicationOverflow(decoded));
            }
            if let Ok(decoded) =
                <NotImplementedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::NotImplemented(decoded));
            }
            if let Ok(decoded) =
                <PoolCantAddCreditManagerTwiceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::PoolCantAddCreditManagerTwice(decoded));
            }
            if let Ok(decoded) =
                <PoolConnectedCreditManagersOnlyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::PoolConnectedCreditManagersOnly(decoded));
            }
            if let Ok(decoded) =
                <PoolIncompatibleCreditAccountManagerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::PoolIncompatibleCreditAccountManager(decoded));
            }
            if let Ok(decoded) =
                <PoolIncorrectWithdrawFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::PoolIncorrectWithdrawFee(decoded));
            }
            if let Ok(decoded) =
                <PoolMoreThanExpectedLiquidityLimitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::PoolMoreThanExpectedLiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <RegisteredCreditAccountManagersOnlyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::RegisteredCreditAccountManagersOnly(decoded));
            }
            if let Ok(decoded) =
                <RegisteredPoolsOnlyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::RegisteredPoolsOnly(decoded));
            }
            if let Ok(decoded) =
                <TdContributorIsNotRegisteredCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::TdContributorIsNotRegistered(decoded));
            }
            if let Ok(decoded) =
                <TdIncorrectWeightsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::TdIncorrectWeights(decoded));
            }
            if let Ok(decoded) =
                <TdNonZeroBalanceAfterDistributionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::TdNonZeroBalanceAfterDistribution(decoded));
            }
            if let Ok(decoded) =
                <TdWalletIsAlreadyConnectedToVcCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::TdWalletIsAlreadyConnectedToVc(decoded));
            }
            if let Ok(decoded) =
                <WgDestinationIsNotWethCompatibleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ErrorsCalls::WgDestinationIsNotWethCompatible(decoded));
            }
            if let Ok(decoded) =
                <WgNotEnoughFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::WgNotEnoughFunds(decoded));
            }
            if let Ok(decoded) =
                <WgReceiveIsNotAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::WgReceiveIsNotAllowed(decoded));
            }
            if let Ok(decoded) =
                <ZeroAddressIsNotAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ErrorsCalls::ZeroAddressIsNotAllowed(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ErrorsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ErrorsCalls::AclCallerNotConfigurator(element) => element.encode(),
                ErrorsCalls::AclCallerNotPausableAdmin(element) => element.encode(),
                ErrorsCalls::AfCantCloseCreditAccountInTheSameBlock(element) => element.encode(),
                ErrorsCalls::AfCreditAccountNotInStock(element) => element.encode(),
                ErrorsCalls::AfExternalAccountsAreForbidden(element) => element.encode(),
                ErrorsCalls::AfMiningIsFinished(element) => element.encode(),
                ErrorsCalls::AsAddressNotFound(element) => element.encode(),
                ErrorsCalls::CaConnectedCreditManagerOnly(element) => element.encode(),
                ErrorsCalls::CaFactoryOnly(element) => element.encode(),
                ErrorsCalls::CrCreditManagerAlreadyAdded(element) => element.encode(),
                ErrorsCalls::CrPoolAlreadyAdded(element) => element.encode(),
                ErrorsCalls::IncorrectArrayLength(element) => element.encode(),
                ErrorsCalls::IncorrectParameter(element) => element.encode(),
                ErrorsCalls::IncorrectPathLength(element) => element.encode(),
                ErrorsCalls::MathAdditionOverflow(element) => element.encode(),
                ErrorsCalls::MathDivisionByZero(element) => element.encode(),
                ErrorsCalls::MathMultiplicationOverflow(element) => element.encode(),
                ErrorsCalls::NotImplemented(element) => element.encode(),
                ErrorsCalls::PoolCantAddCreditManagerTwice(element) => element.encode(),
                ErrorsCalls::PoolConnectedCreditManagersOnly(element) => element.encode(),
                ErrorsCalls::PoolIncompatibleCreditAccountManager(element) => element.encode(),
                ErrorsCalls::PoolIncorrectWithdrawFee(element) => element.encode(),
                ErrorsCalls::PoolMoreThanExpectedLiquidityLimit(element) => element.encode(),
                ErrorsCalls::RegisteredCreditAccountManagersOnly(element) => element.encode(),
                ErrorsCalls::RegisteredPoolsOnly(element) => element.encode(),
                ErrorsCalls::TdContributorIsNotRegistered(element) => element.encode(),
                ErrorsCalls::TdIncorrectWeights(element) => element.encode(),
                ErrorsCalls::TdNonZeroBalanceAfterDistribution(element) => element.encode(),
                ErrorsCalls::TdWalletIsAlreadyConnectedToVc(element) => element.encode(),
                ErrorsCalls::WgDestinationIsNotWethCompatible(element) => element.encode(),
                ErrorsCalls::WgNotEnoughFunds(element) => element.encode(),
                ErrorsCalls::WgReceiveIsNotAllowed(element) => element.encode(),
                ErrorsCalls::ZeroAddressIsNotAllowed(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ErrorsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ErrorsCalls::AclCallerNotConfigurator(element) => element.fmt(f),
                ErrorsCalls::AclCallerNotPausableAdmin(element) => element.fmt(f),
                ErrorsCalls::AfCantCloseCreditAccountInTheSameBlock(element) => element.fmt(f),
                ErrorsCalls::AfCreditAccountNotInStock(element) => element.fmt(f),
                ErrorsCalls::AfExternalAccountsAreForbidden(element) => element.fmt(f),
                ErrorsCalls::AfMiningIsFinished(element) => element.fmt(f),
                ErrorsCalls::AsAddressNotFound(element) => element.fmt(f),
                ErrorsCalls::CaConnectedCreditManagerOnly(element) => element.fmt(f),
                ErrorsCalls::CaFactoryOnly(element) => element.fmt(f),
                ErrorsCalls::CrCreditManagerAlreadyAdded(element) => element.fmt(f),
                ErrorsCalls::CrPoolAlreadyAdded(element) => element.fmt(f),
                ErrorsCalls::IncorrectArrayLength(element) => element.fmt(f),
                ErrorsCalls::IncorrectParameter(element) => element.fmt(f),
                ErrorsCalls::IncorrectPathLength(element) => element.fmt(f),
                ErrorsCalls::MathAdditionOverflow(element) => element.fmt(f),
                ErrorsCalls::MathDivisionByZero(element) => element.fmt(f),
                ErrorsCalls::MathMultiplicationOverflow(element) => element.fmt(f),
                ErrorsCalls::NotImplemented(element) => element.fmt(f),
                ErrorsCalls::PoolCantAddCreditManagerTwice(element) => element.fmt(f),
                ErrorsCalls::PoolConnectedCreditManagersOnly(element) => element.fmt(f),
                ErrorsCalls::PoolIncompatibleCreditAccountManager(element) => element.fmt(f),
                ErrorsCalls::PoolIncorrectWithdrawFee(element) => element.fmt(f),
                ErrorsCalls::PoolMoreThanExpectedLiquidityLimit(element) => element.fmt(f),
                ErrorsCalls::RegisteredCreditAccountManagersOnly(element) => element.fmt(f),
                ErrorsCalls::RegisteredPoolsOnly(element) => element.fmt(f),
                ErrorsCalls::TdContributorIsNotRegistered(element) => element.fmt(f),
                ErrorsCalls::TdIncorrectWeights(element) => element.fmt(f),
                ErrorsCalls::TdNonZeroBalanceAfterDistribution(element) => element.fmt(f),
                ErrorsCalls::TdWalletIsAlreadyConnectedToVc(element) => element.fmt(f),
                ErrorsCalls::WgDestinationIsNotWethCompatible(element) => element.fmt(f),
                ErrorsCalls::WgNotEnoughFunds(element) => element.fmt(f),
                ErrorsCalls::WgReceiveIsNotAllowed(element) => element.fmt(f),
                ErrorsCalls::ZeroAddressIsNotAllowed(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AclCallerNotConfiguratorCall> for ErrorsCalls {
        fn from(var: AclCallerNotConfiguratorCall) -> Self {
            ErrorsCalls::AclCallerNotConfigurator(var)
        }
    }
    impl ::std::convert::From<AclCallerNotPausableAdminCall> for ErrorsCalls {
        fn from(var: AclCallerNotPausableAdminCall) -> Self {
            ErrorsCalls::AclCallerNotPausableAdmin(var)
        }
    }
    impl ::std::convert::From<AfCantCloseCreditAccountInTheSameBlockCall> for ErrorsCalls {
        fn from(var: AfCantCloseCreditAccountInTheSameBlockCall) -> Self {
            ErrorsCalls::AfCantCloseCreditAccountInTheSameBlock(var)
        }
    }
    impl ::std::convert::From<AfCreditAccountNotInStockCall> for ErrorsCalls {
        fn from(var: AfCreditAccountNotInStockCall) -> Self {
            ErrorsCalls::AfCreditAccountNotInStock(var)
        }
    }
    impl ::std::convert::From<AfExternalAccountsAreForbiddenCall> for ErrorsCalls {
        fn from(var: AfExternalAccountsAreForbiddenCall) -> Self {
            ErrorsCalls::AfExternalAccountsAreForbidden(var)
        }
    }
    impl ::std::convert::From<AfMiningIsFinishedCall> for ErrorsCalls {
        fn from(var: AfMiningIsFinishedCall) -> Self {
            ErrorsCalls::AfMiningIsFinished(var)
        }
    }
    impl ::std::convert::From<AsAddressNotFoundCall> for ErrorsCalls {
        fn from(var: AsAddressNotFoundCall) -> Self {
            ErrorsCalls::AsAddressNotFound(var)
        }
    }
    impl ::std::convert::From<CaConnectedCreditManagerOnlyCall> for ErrorsCalls {
        fn from(var: CaConnectedCreditManagerOnlyCall) -> Self {
            ErrorsCalls::CaConnectedCreditManagerOnly(var)
        }
    }
    impl ::std::convert::From<CaFactoryOnlyCall> for ErrorsCalls {
        fn from(var: CaFactoryOnlyCall) -> Self {
            ErrorsCalls::CaFactoryOnly(var)
        }
    }
    impl ::std::convert::From<CrCreditManagerAlreadyAddedCall> for ErrorsCalls {
        fn from(var: CrCreditManagerAlreadyAddedCall) -> Self {
            ErrorsCalls::CrCreditManagerAlreadyAdded(var)
        }
    }
    impl ::std::convert::From<CrPoolAlreadyAddedCall> for ErrorsCalls {
        fn from(var: CrPoolAlreadyAddedCall) -> Self {
            ErrorsCalls::CrPoolAlreadyAdded(var)
        }
    }
    impl ::std::convert::From<IncorrectArrayLengthCall> for ErrorsCalls {
        fn from(var: IncorrectArrayLengthCall) -> Self {
            ErrorsCalls::IncorrectArrayLength(var)
        }
    }
    impl ::std::convert::From<IncorrectParameterCall> for ErrorsCalls {
        fn from(var: IncorrectParameterCall) -> Self {
            ErrorsCalls::IncorrectParameter(var)
        }
    }
    impl ::std::convert::From<IncorrectPathLengthCall> for ErrorsCalls {
        fn from(var: IncorrectPathLengthCall) -> Self {
            ErrorsCalls::IncorrectPathLength(var)
        }
    }
    impl ::std::convert::From<MathAdditionOverflowCall> for ErrorsCalls {
        fn from(var: MathAdditionOverflowCall) -> Self {
            ErrorsCalls::MathAdditionOverflow(var)
        }
    }
    impl ::std::convert::From<MathDivisionByZeroCall> for ErrorsCalls {
        fn from(var: MathDivisionByZeroCall) -> Self {
            ErrorsCalls::MathDivisionByZero(var)
        }
    }
    impl ::std::convert::From<MathMultiplicationOverflowCall> for ErrorsCalls {
        fn from(var: MathMultiplicationOverflowCall) -> Self {
            ErrorsCalls::MathMultiplicationOverflow(var)
        }
    }
    impl ::std::convert::From<NotImplementedCall> for ErrorsCalls {
        fn from(var: NotImplementedCall) -> Self {
            ErrorsCalls::NotImplemented(var)
        }
    }
    impl ::std::convert::From<PoolCantAddCreditManagerTwiceCall> for ErrorsCalls {
        fn from(var: PoolCantAddCreditManagerTwiceCall) -> Self {
            ErrorsCalls::PoolCantAddCreditManagerTwice(var)
        }
    }
    impl ::std::convert::From<PoolConnectedCreditManagersOnlyCall> for ErrorsCalls {
        fn from(var: PoolConnectedCreditManagersOnlyCall) -> Self {
            ErrorsCalls::PoolConnectedCreditManagersOnly(var)
        }
    }
    impl ::std::convert::From<PoolIncompatibleCreditAccountManagerCall> for ErrorsCalls {
        fn from(var: PoolIncompatibleCreditAccountManagerCall) -> Self {
            ErrorsCalls::PoolIncompatibleCreditAccountManager(var)
        }
    }
    impl ::std::convert::From<PoolIncorrectWithdrawFeeCall> for ErrorsCalls {
        fn from(var: PoolIncorrectWithdrawFeeCall) -> Self {
            ErrorsCalls::PoolIncorrectWithdrawFee(var)
        }
    }
    impl ::std::convert::From<PoolMoreThanExpectedLiquidityLimitCall> for ErrorsCalls {
        fn from(var: PoolMoreThanExpectedLiquidityLimitCall) -> Self {
            ErrorsCalls::PoolMoreThanExpectedLiquidityLimit(var)
        }
    }
    impl ::std::convert::From<RegisteredCreditAccountManagersOnlyCall> for ErrorsCalls {
        fn from(var: RegisteredCreditAccountManagersOnlyCall) -> Self {
            ErrorsCalls::RegisteredCreditAccountManagersOnly(var)
        }
    }
    impl ::std::convert::From<RegisteredPoolsOnlyCall> for ErrorsCalls {
        fn from(var: RegisteredPoolsOnlyCall) -> Self {
            ErrorsCalls::RegisteredPoolsOnly(var)
        }
    }
    impl ::std::convert::From<TdContributorIsNotRegisteredCall> for ErrorsCalls {
        fn from(var: TdContributorIsNotRegisteredCall) -> Self {
            ErrorsCalls::TdContributorIsNotRegistered(var)
        }
    }
    impl ::std::convert::From<TdIncorrectWeightsCall> for ErrorsCalls {
        fn from(var: TdIncorrectWeightsCall) -> Self {
            ErrorsCalls::TdIncorrectWeights(var)
        }
    }
    impl ::std::convert::From<TdNonZeroBalanceAfterDistributionCall> for ErrorsCalls {
        fn from(var: TdNonZeroBalanceAfterDistributionCall) -> Self {
            ErrorsCalls::TdNonZeroBalanceAfterDistribution(var)
        }
    }
    impl ::std::convert::From<TdWalletIsAlreadyConnectedToVcCall> for ErrorsCalls {
        fn from(var: TdWalletIsAlreadyConnectedToVcCall) -> Self {
            ErrorsCalls::TdWalletIsAlreadyConnectedToVc(var)
        }
    }
    impl ::std::convert::From<WgDestinationIsNotWethCompatibleCall> for ErrorsCalls {
        fn from(var: WgDestinationIsNotWethCompatibleCall) -> Self {
            ErrorsCalls::WgDestinationIsNotWethCompatible(var)
        }
    }
    impl ::std::convert::From<WgNotEnoughFundsCall> for ErrorsCalls {
        fn from(var: WgNotEnoughFundsCall) -> Self {
            ErrorsCalls::WgNotEnoughFunds(var)
        }
    }
    impl ::std::convert::From<WgReceiveIsNotAllowedCall> for ErrorsCalls {
        fn from(var: WgReceiveIsNotAllowedCall) -> Self {
            ErrorsCalls::WgReceiveIsNotAllowed(var)
        }
    }
    impl ::std::convert::From<ZeroAddressIsNotAllowedCall> for ErrorsCalls {
        fn from(var: ZeroAddressIsNotAllowedCall) -> Self {
            ErrorsCalls::ZeroAddressIsNotAllowed(var)
        }
    }
}
