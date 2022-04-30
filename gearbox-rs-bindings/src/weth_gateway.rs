pub use wethgateway_mod::*;
#[allow(clippy::too_many_arguments)]
mod wethgateway_mod {
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
    #[doc = "WETHGateway was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static WETHGATEWAY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"WithdrawETH\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"onBehalfOf\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"referralCode\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"addLiquidityETH\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeLiquidityETH\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapWETH\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wethAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static WETHGATEWAY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c06040523480156200001157600080fd5b506040516200118438038062001184833981016040819052620000349162000177565b60408051808201909152600281526105a360f41b60208201526001600160a01b038216620000805760405162461bcd60e51b8152600401620000779190620001a9565b60405180910390fd5b50806001600160a01b0316634c252f916040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000c0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e6919062000177565b6001600160a01b03166080816001600160a01b031681525050806001600160a01b031663c513c9bb6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200013e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000164919062000177565b6001600160a01b031660a0525062000201565b6000602082840312156200018a57600080fd5b81516001600160a01b0381168114620001a257600080fd5b9392505050565b600060208083528351808285015260005b81811015620001d857858101830151858201604001528201620001ba565b81811115620001eb576000604083870101525b50601f01601f1916929092016040019392505050565b60805160a051610f2062000264600039600081816101a801528181610285015261050d01526000818160780152818160d90152818161032e015281816103ff015281816105b6015281816107db0152818161087e01526109180152610f206000f3fe60806040526004361061004e5760003560e01c80634f0e0ef3146100c757806354fd4d50146101185780635869dba81461013b578063deecfbc91461015d578063e79a40891461017057600080fd5b366100c2576040805180820190915260038152622ba39960e91b6020820152336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146100bf5760405162461bcd60e51b81526004016100b69190610d76565b60405180910390fd5b50005b600080fd5b3480156100d357600080fd5b506100fb7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561012457600080fd5b5061012d600181565b60405190815260200161010f565b34801561014757600080fd5b5061015b610156366004610dc1565b610190565b005b61015b61016b366004610ded565b610264565b34801561017c57600080fd5b5061015b61018b366004610e3f565b6104ec565b604051636fbc6f6b60e01b81523360048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636fbc6f6b90602401602060405180830381865afa1580156101f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061021b9190610e76565b60405180604001604052806002815260200161043560f41b815250906102545760405162461bcd60e51b81526004016100b69190610d76565b5061025f83836107c5565b505050565b604051635b16ebb760e01b81526001600160a01b03808516600483015284917f000000000000000000000000000000000000000000000000000000000000000090911690635b16ebb790602401602060405180830381865afa1580156102ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102f29190610e76565b60405180604001604052806002815260200161052560f41b8152509061032b5760405162461bcd60e51b81526004016100b69190610d76565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316816001600160a01b0316636f307dc36040518163ffffffff1660e01b8152600401602060405180830381865afa158015610394573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103b89190610e98565b6001600160a01b0316146040518060400160405280600381526020016257473160e81b815250906103fc5760405162461bcd60e51b81526004016100b69190610d76565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0346040518263ffffffff1660e01b81526004016000604051808303818588803b15801561045857600080fd5b505af115801561046c573d6000803e3d6000fd5b505050505061047b8434610857565b604051634d52ea3160e11b81523460048201526001600160a01b03848116602483015261ffff84166044830152851690639aa5d46290606401600060405180830381600087803b1580156104ce57600080fd5b505af11580156104e2573d6000803e3d6000fd5b5050505050505050565b604051635b16ebb760e01b81526001600160a01b03808516600483015284917f000000000000000000000000000000000000000000000000000000000000000090911690635b16ebb790602401602060405180830381865afa158015610556573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061057a9190610e76565b60405180604001604052806002815260200161052560f41b815250906105b35760405162461bcd60e51b81526004016100b69190610d76565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316816001600160a01b0316636f307dc36040518163ffffffff1660e01b8152600401602060405180830381865afa15801561061c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106409190610e98565b6001600160a01b0316146040518060400160405280600381526020016257473160e81b815250906106845760405162461bcd60e51b81526004016100b69190610d76565b506106fe333085876001600160a01b03166336dda7d56040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ed9190610e98565b6001600160a01b0316929190610985565b6040516305fe138b60e01b8152600481018490523060248201526000906001600160a01b038616906305fe138b906044016020604051808303816000875af115801561074e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107729190610eb5565b905061077e83826107c5565b826001600160a01b0316856001600160a01b03167f6de63bb986f2779478e384365c03cc2e62f06b453856acca87d5a519ce02664960405160405180910390a35050505050565b604051632e1a7d4d60e01b8152600481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690632e1a7d4d90602401600060405180830381600087803b15801561082757600080fd5b505af115801561083b573d6000803e3d6000fd5b50610853925050506001600160a01b038316826109e5565b5050565b604051636eb1769f60e11b81523060048201526001600160a01b03838116602483015282917f00000000000000000000000000000000000000000000000000000000000000009091169063dd62ed3e90604401602060405180830381865afa1580156108c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108eb9190610eb5565b10156108535760405163095ea7b360e01b81526001600160a01b03838116600483015260001960248301527f0000000000000000000000000000000000000000000000000000000000000000169063095ea7b3906044016020604051808303816000875af1158015610961573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061025f9190610e76565b604080516001600160a01b0385811660248301528416604482015260648082018490528251808303909101815260849091019091526020810180516001600160e01b03166323b872dd60e01b1790526109df908590610afe565b50505050565b80471015610a355760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016100b6565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114610a82576040519150601f19603f3d011682016040523d82523d6000602084013e610a87565b606091505b505090508061025f5760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016100b6565b6000610b53826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316610bd09092919063ffffffff16565b80519091501561025f5780806020019051810190610b719190610e76565b61025f5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016100b6565b6060610bdf8484600085610be9565b90505b9392505050565b606082471015610c4a5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016100b6565b843b610c985760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016100b6565b600080866001600160a01b03168587604051610cb49190610ece565b60006040518083038185875af1925050503d8060008114610cf1576040519150601f19603f3d011682016040523d82523d6000602084013e610cf6565b606091505b5091509150610d06828286610d11565b979650505050505050565b60608315610d20575081610be2565b825115610d305782518084602001fd5b8160405162461bcd60e51b81526004016100b69190610d76565b60005b83811015610d65578181015183820152602001610d4d565b838111156109df5750506000910152565b6020815260008251806020840152610d95816040850160208701610d4a565b601f01601f19169190910160400192915050565b6001600160a01b0381168114610dbe57600080fd5b50565b60008060408385031215610dd457600080fd5b8235610ddf81610da9565b946020939093013593505050565b600080600060608486031215610e0257600080fd5b8335610e0d81610da9565b92506020840135610e1d81610da9565b9150604084013561ffff81168114610e3457600080fd5b809150509250925092565b600080600060608486031215610e5457600080fd5b8335610e5f81610da9565b9250602084013591506040840135610e3481610da9565b600060208284031215610e8857600080fd5b81518015158114610be257600080fd5b600060208284031215610eaa57600080fd5b8151610be281610da9565b600060208284031215610ec757600080fd5b5051919050565b60008251610ee0818460208701610d4a565b919091019291505056fea2646970667358221220933abbffb9c99157b843a86bae90eadb5af523da99fb35023bbdb6b2db658a2d64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct WETHGateway<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for WETHGateway<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for WETHGateway<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(WETHGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> WETHGateway<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), WETHGATEWAY_ABI.clone(), client).into()
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
                WETHGATEWAY_ABI.clone(),
                WETHGATEWAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xdeecfbc9) function"]
        pub fn add_liquidity_eth(
            &self,
            pool: ethers::core::types::Address,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 236, 251, 201], (pool, on_behalf_of, referral_code))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0xe79a4089) function"]
        pub fn remove_liquidity_eth(
            &self,
            pool: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 154, 64, 137], (pool, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH` (0x5869dba8) function"]
        pub fn unwrap_weth(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 105, 219, 168], (to, amount))
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
        #[doc = "Gets the contract's `WithdrawETH` event"]
        pub fn withdraw_eth_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawETHFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, WithdrawETHFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for WETHGateway<M> {
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
    #[ethevent(name = "WithdrawETH", abi = "WithdrawETH(address,address)")]
    pub struct WithdrawETHFilter {
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addLiquidityETH`function with signature `addLiquidityETH(address,address,uint16)` and selector `[222, 236, 251, 201]`"]
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
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,address,uint16)"
    )]
    pub struct AddLiquidityETHCall {
        pub pool: ethers::core::types::Address,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETH`function with signature `removeLiquidityETH(address,uint256,address)` and selector `[231, 154, 64, 137]`"]
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
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,uint256,address)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub pool: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unwrapWETH`function with signature `unwrapWETH(address,uint256)` and selector `[88, 105, 219, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unwrapWETH", abi = "unwrapWETH(address,uint256)")]
    pub struct UnwrapWETHCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    pub enum WETHGatewayCalls {
        AddLiquidityETH(AddLiquidityETHCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        UnwrapWETH(UnwrapWETHCall),
        Version(VersionCall),
        WethAddress(WethAddressCall),
    }
    impl ethers::core::abi::AbiDecode for WETHGatewayCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WETHGatewayCalls::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WETHGatewayCalls::RemoveLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WETHGatewayCalls::UnwrapWETH(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WETHGatewayCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <WethAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(WETHGatewayCalls::WethAddress(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for WETHGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                WETHGatewayCalls::AddLiquidityETH(element) => element.encode(),
                WETHGatewayCalls::RemoveLiquidityETH(element) => element.encode(),
                WETHGatewayCalls::UnwrapWETH(element) => element.encode(),
                WETHGatewayCalls::Version(element) => element.encode(),
                WETHGatewayCalls::WethAddress(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for WETHGatewayCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                WETHGatewayCalls::AddLiquidityETH(element) => element.fmt(f),
                WETHGatewayCalls::RemoveLiquidityETH(element) => element.fmt(f),
                WETHGatewayCalls::UnwrapWETH(element) => element.fmt(f),
                WETHGatewayCalls::Version(element) => element.fmt(f),
                WETHGatewayCalls::WethAddress(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityETHCall> for WETHGatewayCalls {
        fn from(var: AddLiquidityETHCall) -> Self {
            WETHGatewayCalls::AddLiquidityETH(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHCall> for WETHGatewayCalls {
        fn from(var: RemoveLiquidityETHCall) -> Self {
            WETHGatewayCalls::RemoveLiquidityETH(var)
        }
    }
    impl ::std::convert::From<UnwrapWETHCall> for WETHGatewayCalls {
        fn from(var: UnwrapWETHCall) -> Self {
            WETHGatewayCalls::UnwrapWETH(var)
        }
    }
    impl ::std::convert::From<VersionCall> for WETHGatewayCalls {
        fn from(var: VersionCall) -> Self {
            WETHGatewayCalls::Version(var)
        }
    }
    impl ::std::convert::From<WethAddressCall> for WETHGatewayCalls {
        fn from(var: WethAddressCall) -> Self {
            WETHGatewayCalls::WethAddress(var)
        }
    }
}
