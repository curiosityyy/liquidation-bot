pub use lidov1adapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod lidov1adapter_mod {
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
    #[doc = "LidoV1Adapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LIDOV1ADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_stETH\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"LimitIsOverException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewLimit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"limit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLimit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"treasury\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"weth\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LIDOV1ADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101406040523480156200001257600080fd5b5060405162001a3038038062001a30833981810160405260408110156200003857600080fd5b50805160209182015160408051632b853d7960e11b81529051929391926001600160a01b0385169263570a7af292600480820193918290030181865afa15801562000087573d6000803e3d6000fd5b505050506040513d60208110156200009e57600080fd5b505160408051630a55006360e21b815290516001600160a01b0390921691632954018c916004808201926020929091908290030181865afa158015620000e8573d6000803e3d6000fd5b505050506040513d6020811015620000ff57600080fd5b505160408051632b853d7960e11b8152905184916001600160a01b0383169163570a7af2916004808201926020929091908290030181865afa1580156200014a573d6000803e3d6000fd5b505050506040513d60208110156200016157600080fd5b505160408051630a55006360e21b815290516001600160a01b0390921691632954018c916004808201926020929091908290030181865afa158015620001ab573d6000803e3d6000fd5b505050506040513d6020811015620001c257600080fd5b505160408051634c252f9160e01b815290516001600160a01b0390921691634c252f91916004808201926020929091908290030181865afa1580156200020c573d6000803e3d6000fd5b505050506040513d60208110156200022357600080fd5b505160405184906200023590620005bb565b6001600160a01b03928316815291166020820152604080519182900301906000f08015801562000269573d6000803e3d6000fd5b506001600160a01b03821615806200028857506001600160a01b038116155b15620002a757604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa158015620002f2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003189190620005c9565b6001600160a01b0390811660a052600080546001600160a81b0319169282169290921790915560408051808201909152600281526105a360f41b602082015291508216620003845760405162461bcd60e51b81526004016200037b9190620005fb565b60405180910390fd5b50806001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa158015620003c4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003ea9190620005c9565b6001600160a01b031660c0816001600160a01b03168152505050600180819055506000826001600160a01b031663570a7af26040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200044c573d6000803e3d6000fd5b505050506040513d60208110156200046357600080fd5b505160408051630a55006360e21b815290516001600160a01b0390921691632954018c916004808201926020929091908290030181865afa158015620004ad573d6000803e3d6000fd5b505050506040513d6020811015620004c457600080fd5b50516001600160a01b0380841660e05260408051634c252f9160e01b8152905192935090831691634c252f91916004808201926020929091908290030181865afa15801562000517573d6000803e3d6000fd5b505050506040513d60208110156200052e57600080fd5b50516001600160a01b0390811661010052604080516326c74fc360e01b81529051918316916326c74fc3916004808201926020929091908290030181865afa1580156200057f573d6000803e3d6000fd5b505050506040513d60208110156200059657600080fd5b50516001600160a01b03166101205250506801158e460913d000006002555062000653565b61049080620015a083390190565b600060208284031215620005dc57600080fd5b81516001600160a01b0381168114620005f457600080fd5b9392505050565b600060208083528351808285015260005b818110156200062a578581018301518582016040015282016200060c565b818111156200063d576000604083870101525b50601f01601f1916929092016040019392505050565b60805160a05160c05160e0516101005161012051610ea9620006f7600039600081816101a601526105aa01526000818161015e015261060501526000818161024901526106270152600081816102cc015281816103d401526104a20152600081816101130152818161096c0152610b14015260008181610222015281816107b2015281816108ff01528181610a8601528181610b760152610bfe0152610ea96000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c80638456cb591161008c578063c12c21c011610066578063c12c21c01461021d578063c1fe3e4814610244578063ce30bbdb1461026b578063ea99c2a61461029a57600080fd5b80638456cb59146101e7578063a4d66daf146101ef578063bd90df701461020a57600080fd5b80633fc8cef3116100c85780633fc8cef3146101595780635c975abb1461018057806361d027b3146101a157806378aa73a4146101c857600080fd5b806327ea6f2b146100ef5780632f7a18811461010e5780633f4ba83a14610151575b600080fd5b61010c6004803603602081101561010557600080fd5b50356102b7565b005b6101357f000000000000000000000000000000000000000000000000000000000000000081565b604080516001600160a01b039092168252519081900360200190f35b61010c6103bf565b6101357f000000000000000000000000000000000000000000000000000000000000000081565b600054600160a01b900460ff16604080519115158252519081900360200190f35b6101357f000000000000000000000000000000000000000000000000000000000000000081565b6101d0600181565b6040805161ffff9092168252519081900360200190f35b61010c61048d565b6101f860025481565b60408051918252519081900360200190f35b600054610135906001600160a01b031681565b6101357f000000000000000000000000000000000000000000000000000000000000000081565b6101357f000000000000000000000000000000000000000000000000000000000000000081565b610273600d81565b6040518082600d81111561028957610289610c66565b815260200191505060405180910390f35b6101f8600480360360208110156102b057600080fd5b5035610559565b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa15801561031b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061033f9190610c7c565b6040518060400160405280600481526020016320a1a61960e11b815250906103835760405162461bcd60e51b815260040161037a9190610d01565b60405180910390fd5b5060028190556040805182815290517fe1e1c8251499b303aefb01cf84a5ce22a95911c20ce2f3f5ae670441a6353d829181900360200190a150565b604051630d4eb5db60e41b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d4eb5db090602401602060405180830381865afa158015610423573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104479190610c7c565b6040518060400160405280600481526020016341434c3160e01b815250906104825760405162461bcd60e51b815260040161037a9190610d01565b5061048b61066a565b565b604051630e907b1960e21b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633a41ec6490602401602060405180830381865afa1580156104f1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105159190610c7c565b6040518060400160405280600481526020016341434c3160e01b815250906105505760405162461bcd60e51b815260040161037a9190610d01565b5061048b610707565b600060025482111561057e5760405163057f9b9d60e31b815260040160405180910390fd5b81600260008282546105909190610d14565b909155505060408051602481018490526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166044808301919091528251808303909101815260649091019091526020810180516001600160e01b0316637a99743560e11b17905261064e907f0000000000000000000000000000000000000000000000000000000000000000907f000000000000000000000000000000000000000000000000000000000000000090600161078f565b806020019051602081101561066257600080fd5b505192915050565b600054600160a01b900460ff166106ba5760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b604482015260640161037a565b6000805460ff60a01b191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b600054600160a01b900460ff16156107545760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b604482015260640161037a565b6000805460ff60a01b1916600160a01b1790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586106ea3390565b604051633a562dc160e21b81523360048201526060906000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e958b70490602401602060405180830381865afa1580156107f9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061081d9190610d39565b905061082c8187878787610836565b9695505050505050565b6060811561095e57600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa15801561089d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108c19190610d62565b101561095e576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b15801561094557600080fd5b505af1158015610959573d6000803e3d6000fd5b505050505b600080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a6c576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa1580156109d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109fb9190610d62565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa158015610a45573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a699190610d62565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a92610ac092339216908a90600401610d7b565b6000604051808303816000875af1158015610adf573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610b079190810190610dc6565b9250336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610bd7576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b158015610bba57600080fd5b505af1158015610bce573d6000803e3d6000fd5b50505050610c5b565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b158015610c4257600080fd5b505af1158015610c56573d6000803e3d6000fd5b505050505b505095945050505050565b634e487b7160e01b600052602160045260246000fd5b600060208284031215610c8e57600080fd5b81518015158114610c9e57600080fd5b9392505050565b60005b83811015610cc0578181015183820152602001610ca8565b83811115610ccf576000848401525b50505050565b60008151808452610ced816020860160208601610ca5565b601f01601f19169290920160200192915050565b602081526000610c9e6020830184610cd5565b600082821015610d3457634e487b7160e01b600052601160045260246000fd5b500390565b600060208284031215610d4b57600080fd5b81516001600160a01b0381168114610c9e57600080fd5b600060208284031215610d7457600080fd5b5051919050565b6001600160a01b03848116825283166020820152606060408201819052600090610da790830184610cd5565b95945050505050565b634e487b7160e01b600052604160045260246000fd5b600060208284031215610dd857600080fd5b815167ffffffffffffffff80821115610df057600080fd5b818401915084601f830112610e0457600080fd5b815181811115610e1657610e16610db0565b604051601f8201601f19908116603f01168101908382118183101715610e3e57610e3e610db0565b81604052828152876020848701011115610e5757600080fd5b610e68836020830160208801610ca5565b97965050505050505056fea26469706673582212204882d6a1a4bd8e85fbc25ac5e57b5fd9f41964ecb9a933135dd6bc5681bbd3eb64736f6c634300080a003360c060405234801561001057600080fd5b506040516104903803806104908339818101604052604081101561003357600080fd5b5080516020909101516001600160a01b038216158061005957506001600160a01b038116155b1561007757604051635919af9760e11b815260040160405180910390fd5b6001600160a01b039081166080521660a05260805160a0516103c86100c860003960008181604b0152818161011e01526101a4015260008181608e0152818161020501526102b201526103c86000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80633fc8cef314610046578063c1fe3e4814610089578063f532e86a146100b0575b600080fd5b61006d7f000000000000000000000000000000000000000000000000000000000000000081565b604080516001600160a01b039092168252519081900360200190f35b61006d7f000000000000000000000000000000000000000000000000000000000000000081565b6100dc600480360360408110156100c657600080fd5b50803590602001356001600160a01b03166100ee565b60408051918252519081900360200190f35b604080516323b872dd60e01b81523360048201523060248201526044810184905290516000916001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016916323b872dd91606480820192602092909190829003018187875af115801561016b573d6000803e3d6000fd5b505050506040513d602081101561018157600080fd5b505060408051632e1a7d4d60e01b81526004810185905290516001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691632e1a7d4d91602480830192600092919082900301818387803b1580156101eb57600080fd5b505af11580156101ff573d6000803e3d6000fd5b505050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663a1903eab836040518263ffffffff1660e01b815260040180826001600160a01b031681526020019150506020604051808303816000875af1158015610277573d6000803e3d6000fd5b505050506040513d602081101561028d57600080fd5b5051604080516370a0823160e01b815230600482015290519192506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163a9059cbb91339184916370a082319160248083019260209291908290030181865afa158015610307573d6000803e3d6000fd5b505050506040513d602081101561031d57600080fd5b5051604080516001600160e01b031960e086901b1681526001600160a01b039093166004840152602483019190915251604480830192602092919082900301816000875af1158015610373573d6000803e3d6000fd5b505050506040513d602081101561038957600080fd5b5090939250505056fea2646970667358221220192ad739d7cc7e73ed23d7c3922268a99616d7c25fc7a3c8f32371a694e50e6e64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct LidoV1Adapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for LidoV1Adapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LidoV1Adapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LidoV1Adapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> LidoV1Adapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LIDOV1ADAPTER_ABI.clone(), client)
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
                LIDOV1ADAPTER_ABI.clone(),
                LIDOV1ADAPTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_gearboxAdapterType` (0xce30bbdb) function"]
        pub fn gearbox_adapter_type(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([206, 48, 187, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_gearboxAdapterVersion` (0x78aa73a4) function"]
        pub fn gearbox_adapter_version(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([120, 170, 115, 164], ())
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
        #[doc = "Calls the contract's `creditManager` (0xc12c21c0) function"]
        pub fn credit_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 44, 33, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `limit` (0xa4d66daf) function"]
        pub fn limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([164, 214, 109, 175], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLimit` (0x27ea6f2b) function"]
        pub fn set_limit(
            &self,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 234, 111, 43], limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stETH` (0xc1fe3e48) function"]
        pub fn st_eth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 254, 62, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submit` (0xea99c2a6) function"]
        pub fn submit(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([234, 153, 194, 166], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetContract` (0xbd90df70) function"]
        pub fn target_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([189, 144, 223, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `treasury` (0x61d027b3) function"]
        pub fn treasury(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([97, 208, 39, 179], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `weth` (0x3fc8cef3) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewLimit` event"]
        pub fn new_limit_filter(&self) -> ethers::contract::builders::Event<M, NewLimitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, LidoV1AdapterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LidoV1Adapter<M> {
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
    #[ethevent(name = "NewLimit", abi = "NewLimit(uint256)")]
    pub struct NewLimitFilter {
        pub limit: ethers::core::types::U256,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LidoV1AdapterEvents {
        NewLimitFilter(NewLimitFilter),
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for LidoV1AdapterEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewLimitFilter::decode_log(log) {
                return Ok(LidoV1AdapterEvents::NewLimitFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(LidoV1AdapterEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(LidoV1AdapterEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for LidoV1AdapterEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LidoV1AdapterEvents::NewLimitFilter(element) => element.fmt(f),
                LidoV1AdapterEvents::PausedFilter(element) => element.fmt(f),
                LidoV1AdapterEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `_gearboxAdapterType`function with signature `_gearboxAdapterType()` and selector `[206, 48, 187, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_gearboxAdapterType", abi = "_gearboxAdapterType()")]
    pub struct GearboxAdapterTypeCall;
    #[doc = "Container type for all input parameters for the `_gearboxAdapterVersion`function with signature `_gearboxAdapterVersion()` and selector `[120, 170, 115, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_gearboxAdapterVersion", abi = "_gearboxAdapterVersion()")]
    pub struct GearboxAdapterVersionCall;
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
    #[doc = "Container type for all input parameters for the `limit`function with signature `limit()` and selector `[164, 214, 109, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "limit", abi = "limit()")]
    pub struct LimitCall;
    #[doc = "Container type for all input parameters for the `pause`function with signature `pause()` and selector `[132, 86, 203, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused`function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `setLimit`function with signature `setLimit(uint256)` and selector `[39, 234, 111, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLimit", abi = "setLimit(uint256)")]
    pub struct SetLimitCall {
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `stETH`function with signature `stETH()` and selector `[193, 254, 62, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stETH", abi = "stETH()")]
    pub struct StETHCall;
    #[doc = "Container type for all input parameters for the `submit`function with signature `submit(uint256)` and selector `[234, 153, 194, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "submit", abi = "submit(uint256)")]
    pub struct SubmitCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `targetContract`function with signature `targetContract()` and selector `[189, 144, 223, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "targetContract", abi = "targetContract()")]
    pub struct TargetContractCall;
    #[doc = "Container type for all input parameters for the `treasury`function with signature `treasury()` and selector `[97, 208, 39, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "treasury", abi = "treasury()")]
    pub struct TreasuryCall;
    #[doc = "Container type for all input parameters for the `unpause`function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[doc = "Container type for all input parameters for the `weth`function with signature `weth()` and selector `[63, 200, 206, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LidoV1AdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Limit(LimitCall),
        Pause(PauseCall),
        Paused(PausedCall),
        SetLimit(SetLimitCall),
        StETH(StETHCall),
        Submit(SubmitCall),
        TargetContract(TargetContractCall),
        Treasury(TreasuryCall),
        Unpause(UnpauseCall),
        Weth(WethCall),
    }
    impl ethers::core::abi::AbiDecode for LidoV1AdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) = <LimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::Limit(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <SetLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::SetLimit(decoded));
            }
            if let Ok(decoded) = <StETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::StETH(decoded));
            }
            if let Ok(decoded) = <SubmitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::Submit(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::TargetContract(decoded));
            }
            if let Ok(decoded) =
                <TreasuryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::Treasury(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LidoV1AdapterCalls::Unpause(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LidoV1AdapterCalls::Weth(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LidoV1AdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LidoV1AdapterCalls::GearboxAdapterType(element) => element.encode(),
                LidoV1AdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                LidoV1AdapterCalls::CreditFacade(element) => element.encode(),
                LidoV1AdapterCalls::CreditManager(element) => element.encode(),
                LidoV1AdapterCalls::Limit(element) => element.encode(),
                LidoV1AdapterCalls::Pause(element) => element.encode(),
                LidoV1AdapterCalls::Paused(element) => element.encode(),
                LidoV1AdapterCalls::SetLimit(element) => element.encode(),
                LidoV1AdapterCalls::StETH(element) => element.encode(),
                LidoV1AdapterCalls::Submit(element) => element.encode(),
                LidoV1AdapterCalls::TargetContract(element) => element.encode(),
                LidoV1AdapterCalls::Treasury(element) => element.encode(),
                LidoV1AdapterCalls::Unpause(element) => element.encode(),
                LidoV1AdapterCalls::Weth(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LidoV1AdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LidoV1AdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                LidoV1AdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                LidoV1AdapterCalls::CreditFacade(element) => element.fmt(f),
                LidoV1AdapterCalls::CreditManager(element) => element.fmt(f),
                LidoV1AdapterCalls::Limit(element) => element.fmt(f),
                LidoV1AdapterCalls::Pause(element) => element.fmt(f),
                LidoV1AdapterCalls::Paused(element) => element.fmt(f),
                LidoV1AdapterCalls::SetLimit(element) => element.fmt(f),
                LidoV1AdapterCalls::StETH(element) => element.fmt(f),
                LidoV1AdapterCalls::Submit(element) => element.fmt(f),
                LidoV1AdapterCalls::TargetContract(element) => element.fmt(f),
                LidoV1AdapterCalls::Treasury(element) => element.fmt(f),
                LidoV1AdapterCalls::Unpause(element) => element.fmt(f),
                LidoV1AdapterCalls::Weth(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for LidoV1AdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            LidoV1AdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for LidoV1AdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            LidoV1AdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for LidoV1AdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            LidoV1AdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for LidoV1AdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            LidoV1AdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<LimitCall> for LidoV1AdapterCalls {
        fn from(var: LimitCall) -> Self {
            LidoV1AdapterCalls::Limit(var)
        }
    }
    impl ::std::convert::From<PauseCall> for LidoV1AdapterCalls {
        fn from(var: PauseCall) -> Self {
            LidoV1AdapterCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for LidoV1AdapterCalls {
        fn from(var: PausedCall) -> Self {
            LidoV1AdapterCalls::Paused(var)
        }
    }
    impl ::std::convert::From<SetLimitCall> for LidoV1AdapterCalls {
        fn from(var: SetLimitCall) -> Self {
            LidoV1AdapterCalls::SetLimit(var)
        }
    }
    impl ::std::convert::From<StETHCall> for LidoV1AdapterCalls {
        fn from(var: StETHCall) -> Self {
            LidoV1AdapterCalls::StETH(var)
        }
    }
    impl ::std::convert::From<SubmitCall> for LidoV1AdapterCalls {
        fn from(var: SubmitCall) -> Self {
            LidoV1AdapterCalls::Submit(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for LidoV1AdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            LidoV1AdapterCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<TreasuryCall> for LidoV1AdapterCalls {
        fn from(var: TreasuryCall) -> Self {
            LidoV1AdapterCalls::Treasury(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for LidoV1AdapterCalls {
        fn from(var: UnpauseCall) -> Self {
            LidoV1AdapterCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<WethCall> for LidoV1AdapterCalls {
        fn from(var: WethCall) -> Self {
            LidoV1AdapterCalls::Weth(var)
        }
    }
}
