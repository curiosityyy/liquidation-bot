pub use creditaccount_mod::*;
#[allow(clippy::too_many_arguments)]
mod creditaccount_mod {
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
    #[doc = "CreditAccount was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CREDITACCOUNT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"swapContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveToken\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"borrowedAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelAllowance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"connectTo\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cumulativeIndexAtOpen\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransfer\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"since\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_borrowedAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cumulativeIndexAtOpen\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateParameters\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CREDITACCOUNT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610cfe806100206000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c80633dc54b401161008c578063c12c21c011610066578063c12c21c01461016d578063c45a015514610198578063c75b5a71146101b1578063d1660f99146101c457600080fd5b80633dc54b401461015457806354fd4d501461015d5780638129fc1c1461016557600080fd5b806303105b04146100d457806316128211146100e957806317d11a15146100fc57806319a16039146101185780631afbb7a41461012b5780631cff79cd14610134575b600080fd5b6100e76100e2366004610a66565b6101d7565b005b6100e76100f7366004610a99565b610388565b61010560035481565b6040519081526020015b60405180910390f35b6100e7610126366004610a66565b6103da565b61010560025481565b610147610142366004610ad1565b61043c565b60405161010f9190610bef565b61010560045481565b610105600181565b6100e761049f565b600154610180906001600160a01b031681565b6040516001600160a01b03909116815260200161010f565b600054610180906201000090046001600160a01b031681565b6100e76101bf366004610c02565b610571565b6100e76101d2366004610c35565b6105eb565b60015460408051808201909152600381526243413160e81b6020820152906001600160a01b031633146102265760405162461bcd60e51b815260040161021d9190610bef565b60405180910390fd5b5060405163095ea7b360e01b81526001600160a01b038281166004830152600019602483015283169063095ea7b3906044016020604051808303816000875af1925050508015610293575060408051601f3d908101601f1916820190925261029091810190610c71565b60015b61037f5760405163095ea7b360e01b81526001600160a01b0382811660048301526000602483015283169063095ea7b3906044016020604051808303816000875af11580156102e6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061030a9190610c71565b5060405163095ea7b360e01b81526001600160a01b038281166004830152600019602483015283169063095ea7b3906044016020604051808303816000875af115801561035b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061037f9190610c71565b505050565b5050565b60015460408051808201909152600381526243413160e81b6020820152906001600160a01b031633146103ce5760405162461bcd60e51b815260040161021d9190610bef565b50600291909155600355565b60005460408051808201909152600381526221a09960e91b6020820152906201000090046001600160a01b031633146104265760405162461bcd60e51b815260040161021d9190610bef565b506103846001600160a01b038316826000610646565b60015460408051808201909152600381526243413160e81b60208201526060916001600160a01b031633146104845760405162461bcd60e51b815260040161021d9190610bef565b506104986001600160a01b0384168361078e565b9392505050565b600054610100900460ff166104ba5760005460ff16156104be565b303b155b6105215760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b606482015260840161021d565b600054610100900460ff16158015610543576000805461ffff19166101011790555b6000805462010000600160b01b031916336201000002179055801561056e576000805461ff00191690555b50565b60005460408051808201909152600381526221a09960e91b6020820152906201000090046001600160a01b031633146105bd5760405162461bcd60e51b815260040161021d9190610bef565b50600180546001600160a01b0319166001600160a01b03949094169390931790925560025560035543600455565b60015460408051808201909152600381526243413160e81b6020820152906001600160a01b031633146106315760405162461bcd60e51b815260040161021d9190610bef565b5061037f6001600160a01b03841683836107d0565b8015806106c05750604051636eb1769f60e11b81523060048201526001600160a01b03838116602483015284169063dd62ed3e90604401602060405180830381865afa15801561069a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106be9190610c93565b155b61072b5760405162461bcd60e51b815260206004820152603660248201527f5361666545524332303a20617070726f76652066726f6d206e6f6e2d7a65726f60448201527520746f206e6f6e2d7a65726f20616c6c6f77616e636560501b606482015260840161021d565b6040516001600160a01b03831660248201526044810182905261037f90849063095ea7b360e01b906064015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152610800565b606061049883836040518060400160405280601e81526020017f416464726573733a206c6f772d6c6576656c2063616c6c206661696c656400008152506108d2565b6040516001600160a01b03831660248201526044810182905261037f90849063a9059cbb60e01b90606401610757565b6000610855826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166108d29092919063ffffffff16565b80519091501561037f57808060200190518101906108739190610c71565b61037f5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b606482015260840161021d565b60606108e184846000856108e9565b949350505050565b60608247101561094a5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b606482015260840161021d565b843b6109985760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161021d565b600080866001600160a01b031685876040516109b49190610cac565b60006040518083038185875af1925050503d80600081146109f1576040519150601f19603f3d011682016040523d82523d6000602084013e6109f6565b606091505b5091509150610a06828286610a11565b979650505050505050565b60608315610a20575081610498565b825115610a305782518084602001fd5b8160405162461bcd60e51b815260040161021d9190610bef565b80356001600160a01b0381168114610a6157600080fd5b919050565b60008060408385031215610a7957600080fd5b610a8283610a4a565b9150610a9060208401610a4a565b90509250929050565b60008060408385031215610aac57600080fd5b50508035926020909101359150565b634e487b7160e01b600052604160045260246000fd5b60008060408385031215610ae457600080fd5b610aed83610a4a565b9150602083013567ffffffffffffffff80821115610b0a57600080fd5b818501915085601f830112610b1e57600080fd5b813581811115610b3057610b30610abb565b604051601f8201601f19908116603f01168101908382118183101715610b5857610b58610abb565b81604052828152886020848701011115610b7157600080fd5b8260208601602083013760006020848301015280955050505050509250929050565b60005b83811015610bae578181015183820152602001610b96565b83811115610bbd576000848401525b50505050565b60008151808452610bdb816020860160208601610b93565b601f01601f19169290920160200192915050565b6020815260006104986020830184610bc3565b600080600060608486031215610c1757600080fd5b610c2084610a4a565b95602085013595506040909401359392505050565b600080600060608486031215610c4a57600080fd5b610c5384610a4a565b9250610c6160208501610a4a565b9150604084013590509250925092565b600060208284031215610c8357600080fd5b8151801515811461049857600080fd5b600060208284031215610ca557600080fd5b5051919050565b60008251610cbe818460208701610b93565b919091019291505056fea2646970667358221220767f262ec46278aa06105a9ddf34249141a9a891d586d7b2a24f4f675222692264736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CreditAccount<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CreditAccount<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CreditAccount<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CreditAccount))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CreditAccount<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CREDITACCOUNT_ABI.clone(), client)
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
                CREDITACCOUNT_ABI.clone(),
                CREDITACCOUNT_BYTECODE.clone().into(),
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CreditAccount<M> {
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
    pub enum CreditAccountCalls {
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
    impl ethers::core::abi::AbiDecode for CreditAccountCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::ApproveToken(decoded));
            }
            if let Ok(decoded) =
                <BorrowedAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::BorrowedAmount(decoded));
            }
            if let Ok(decoded) =
                <CancelAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::CancelAllowance(decoded));
            }
            if let Ok(decoded) =
                <ConnectToCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::ConnectTo(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <CumulativeIndexAtOpenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::CumulativeIndexAtOpen(decoded));
            }
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::SafeTransfer(decoded));
            }
            if let Ok(decoded) = <SinceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::Since(decoded));
            }
            if let Ok(decoded) =
                <UpdateParametersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::UpdateParameters(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CreditAccountCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CreditAccountCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CreditAccountCalls::ApproveToken(element) => element.encode(),
                CreditAccountCalls::BorrowedAmount(element) => element.encode(),
                CreditAccountCalls::CancelAllowance(element) => element.encode(),
                CreditAccountCalls::ConnectTo(element) => element.encode(),
                CreditAccountCalls::CreditManager(element) => element.encode(),
                CreditAccountCalls::CumulativeIndexAtOpen(element) => element.encode(),
                CreditAccountCalls::Execute(element) => element.encode(),
                CreditAccountCalls::Factory(element) => element.encode(),
                CreditAccountCalls::Initialize(element) => element.encode(),
                CreditAccountCalls::SafeTransfer(element) => element.encode(),
                CreditAccountCalls::Since(element) => element.encode(),
                CreditAccountCalls::UpdateParameters(element) => element.encode(),
                CreditAccountCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CreditAccountCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CreditAccountCalls::ApproveToken(element) => element.fmt(f),
                CreditAccountCalls::BorrowedAmount(element) => element.fmt(f),
                CreditAccountCalls::CancelAllowance(element) => element.fmt(f),
                CreditAccountCalls::ConnectTo(element) => element.fmt(f),
                CreditAccountCalls::CreditManager(element) => element.fmt(f),
                CreditAccountCalls::CumulativeIndexAtOpen(element) => element.fmt(f),
                CreditAccountCalls::Execute(element) => element.fmt(f),
                CreditAccountCalls::Factory(element) => element.fmt(f),
                CreditAccountCalls::Initialize(element) => element.fmt(f),
                CreditAccountCalls::SafeTransfer(element) => element.fmt(f),
                CreditAccountCalls::Since(element) => element.fmt(f),
                CreditAccountCalls::UpdateParameters(element) => element.fmt(f),
                CreditAccountCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveTokenCall> for CreditAccountCalls {
        fn from(var: ApproveTokenCall) -> Self {
            CreditAccountCalls::ApproveToken(var)
        }
    }
    impl ::std::convert::From<BorrowedAmountCall> for CreditAccountCalls {
        fn from(var: BorrowedAmountCall) -> Self {
            CreditAccountCalls::BorrowedAmount(var)
        }
    }
    impl ::std::convert::From<CancelAllowanceCall> for CreditAccountCalls {
        fn from(var: CancelAllowanceCall) -> Self {
            CreditAccountCalls::CancelAllowance(var)
        }
    }
    impl ::std::convert::From<ConnectToCall> for CreditAccountCalls {
        fn from(var: ConnectToCall) -> Self {
            CreditAccountCalls::ConnectTo(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for CreditAccountCalls {
        fn from(var: CreditManagerCall) -> Self {
            CreditAccountCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CumulativeIndexAtOpenCall> for CreditAccountCalls {
        fn from(var: CumulativeIndexAtOpenCall) -> Self {
            CreditAccountCalls::CumulativeIndexAtOpen(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for CreditAccountCalls {
        fn from(var: ExecuteCall) -> Self {
            CreditAccountCalls::Execute(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for CreditAccountCalls {
        fn from(var: FactoryCall) -> Self {
            CreditAccountCalls::Factory(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CreditAccountCalls {
        fn from(var: InitializeCall) -> Self {
            CreditAccountCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<SafeTransferCall> for CreditAccountCalls {
        fn from(var: SafeTransferCall) -> Self {
            CreditAccountCalls::SafeTransfer(var)
        }
    }
    impl ::std::convert::From<SinceCall> for CreditAccountCalls {
        fn from(var: SinceCall) -> Self {
            CreditAccountCalls::Since(var)
        }
    }
    impl ::std::convert::From<UpdateParametersCall> for CreditAccountCalls {
        fn from(var: UpdateParametersCall) -> Self {
            CreditAccountCalls::UpdateParameters(var)
        }
    }
    impl ::std::convert::From<VersionCall> for CreditAccountCalls {
        fn from(var: VersionCall) -> Self {
            CreditAccountCalls::Version(var)
        }
    }
}
