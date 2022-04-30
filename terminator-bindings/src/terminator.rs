pub use terminator_mod::*;
#[allow(clippy::too_many_arguments)]
mod terminator_mod {
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
    #[doc = "Terminator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TERMINATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_wethToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_yearn\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addYearn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"allowExecutor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"executors\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forbidExecutor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditFacade\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"skipTokenMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"convertWETH\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct MultiCall[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"_router\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct Terminator.UniV2Params[]\",\"name\":\"_paths\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"liquidateAndSellOnV2\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferToOwner\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wethToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"yearn\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TERMINATOR_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060405162001b6438038062001b64833981016040819052610031916100af565b61003a3361005f565b600280546001600160a01b0319166001600160a01b03929092169190911790556100df565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6000602082840312156100c157600080fd5b81516001600160a01b03811681146100d857600080fd5b9392505050565b611a7580620000ef6000396000f3fe6080604052600436106100a05760003560e01c8063715018a611610064578063715018a61461016b5780638568523a146101805780638da5cb5b146101a05780639ac2a011146101be578063b1b05f2a146101fe578063f2fde38b1461021e57600080fd5b806305f0275b146100ac5780631c282440146100ce5780634b57b0be1461010b578063548fa5a21461012b5780636e9d59871461014b57600080fd5b366100a757005b600080fd5b3480156100b857600080fd5b506100cc6100c7366004611401565b61023e565b005b3480156100da57600080fd5b506100ee6100e936600461141e565b6102c2565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561011757600080fd5b506002546100ee906001600160a01b031681565b34801561013757600080fd5b506100cc610146366004611491565b6102ec565b34801561015757600080fd5b506100cc610166366004611401565b6104ab565b34801561017757600080fd5b506100cc6104f6565b34801561018c57600080fd5b506100cc61019b366004611559565b61052c565b3480156101ac57600080fd5b506000546001600160a01b03166100ee565b3480156101ca57600080fd5b506101ee6101d9366004611401565b60036020526000908152604090205460ff1681565b6040519015158152602001610102565b34801561020a57600080fd5b506100cc610219366004611401565b61056e565b34801561022a57600080fd5b506100cc610239366004611401565b6105bc565b6000546001600160a01b031633146102715760405162461bcd60e51b815260040161026890611585565b60405180910390fd5b6001805480820182556000919091527fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf60180546001600160a01b0319166001600160a01b0392909216919091179055565b600181815481106102d257600080fd5b6000918252602090912001546001600160a01b0316905081565b3360009081526003602052604090205460ff166103405760405162461bcd60e51b8152602060048201526012602482015271466f72206578656375746f7273206f6e6c7960701b6044820152606401610268565b60008990506000816001600160a01b031663c12c21c06040518163ffffffff1660e01b8152600401602060405180830381865afa158015610385573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103a991906115ba565b905060006103b9828c8c87610657565b905061042682836001600160a01b0316636f307dc36040518163ffffffff1660e01b8152600401602060405180830381865afa1580156103fd573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061042191906115ba565b610959565b6040516302ec8d0760e51b81526001600160a01b03841690635d91a0e09061045c908e9030908f908f908f908f906004016115d7565b600060405180830381600087803b15801561047657600080fd5b505af115801561048a573d6000803e3d6000fd5b5050505061049d828c8c848a8a8a610a52565b505050505050505050505050565b6000546001600160a01b031633146104d55760405162461bcd60e51b815260040161026890611585565b6001600160a01b03166000908152600360205260409020805460ff19169055565b6000546001600160a01b031633146105205760405162461bcd60e51b815260040161026890611585565b61052a6000610f18565b565b6000546001600160a01b031633146105565760405162461bcd60e51b815260040161026890611585565b61056a6001600160a01b0383163383610f68565b5050565b6000546001600160a01b031633146105985760405162461bcd60e51b815260040161026890611585565b6001600160a01b03166000908152600360205260409020805460ff19166001179055565b6000546001600160a01b031633146105e65760405162461bcd60e51b815260040161026890611585565b6001600160a01b03811661064b5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610268565b61065481610f18565b50565b604051633a562dc160e21b81526001600160a01b03848116600483015260609160009187169063e958b70490602401602060405180830381865afa1580156106a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106c791906115ba565b90506000866001600160a01b03166320a05ff76040518163ffffffff1660e01b8152600401602060405180830381865afa158015610709573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061072d91906116e5565b905060008167ffffffffffffffff81111561074a5761074a6116fe565b604051908082528060200260200182016040528015610773578160200160208202803683370190505b50604051638991b2f160e01b81526001600160a01b0385811660048301529192506000918a1690638991b2f190602401602060405180830381865afa1580156107c0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107e491906116e5565b871916905060008684146108325760405162461bcd60e51b8152602060048201526015602482015274092dcc6dee4e4cac6e840e0c2e8d040d8cadccee8d605b1b6044820152606401610268565b60015b8481101561094a576001811b91508282161561094257604051632f2f971360e11b8152600481018290526000906001600160a01b038d1690635e5f2e2690602401602060405180830381865afa158015610893573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b791906115ba565b6040516370a0823160e01b81523060048201529091506001600160a01b038216906370a0823190602401602060405180830381865afa1580156108fe573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061092291906116e5565b85838151811061093457610934611714565b602002602001018181525050505b600101610835565b50919998505050505050505050565b604051636eb1769f60e11b81523060048201526001600160a01b0383811660248301526001600160fe1b03919083169063dd62ed3e90604401602060405180830381865afa1580156109af573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109d391906116e5565b101561056a5760405163095ea7b360e01b81526001600160a01b038381166004830152600019602483015282169063095ea7b3906044016020604051808303816000875af1158015610a29573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a4d919061172a565b505050565b604051633a562dc160e21b81526001600160a01b0387811660048301526000919089169063e958b70490602401602060405180830381865afa158015610a9c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ac091906115ba565b90506000886001600160a01b03166320a05ff76040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b02573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b2691906116e5565b604051638991b2f160e01b81526001600160a01b0384811660048301529192506000918b1690638991b2f190602401602060405180830381865afa158015610b72573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b9691906116e5565b88191690506000805b600154811015610ce0576001808281548110610bbd57610bbd611714565b6000918252602090912001546040516370a0823160e01b81523060048201526001600160a01b03909116906370a0823190602401602060405180830381865afa158015610c0e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c3291906116e5565b1115610cce5760018181548110610c4b57610c4b611714565b9060005260206000200160009054906101000a90046001600160a01b03166001600160a01b0316633ccfd60b6040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610ca8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ccc91906116e5565b505b80610cd88161175d565b915050610b9f565b5060015b8381101561049d576001811b915082821615610f1057604051632f2f971360e11b8152600481018290526000906001600160a01b038e1690635e5f2e2690602401602060405180830381865afa158015610d42573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d6691906115ba565b6040516370a0823160e01b81523060048201529091506000906001600160a01b038316906370a0823190602401602060405180830381865afa158015610db0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610dd491906116e5565b90508a8381518110610de857610de8611714565b60200260200101516001610dfc9190611778565b811115610f0d5760018b8481518110610e1757610e17611714565b602002602001015182610e2a9190611790565b610e349190611790565b9050888884818110610e4857610e48611714565b9050602002810190610e5a91906117a7565b610e689060208101906117c7565b6000818110610e7957610e79611714565b9050602002016020810190610e8e9190611401565b6001600160a01b0316826001600160a01b031614610edf5760405162461bcd60e51b815260206004820152600e60248201526d0d2dcc6dee4e4cac6e840e0c2e8d60931b6044820152606401610268565b610f0d8a828b8b87818110610ef657610ef6611714565b9050602002810190610f0891906117a7565b610fba565b50505b600101610ce4565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052610a4d90849061117a565b610ff183610fcb60208401846117c7565b6000818110610fdc57610fdc611714565b90506020020160208101906104219190611401565b600061100c823561100660408501358661124c565b9061125f565b6002549091506001600160a01b031661102860208401846117c7565b600081811061103957611039611714565b905060200201602081019061104e9190611401565b6001600160a01b031614156110ea576001600160a01b038416637ff36ab5848361107b60208701876117c7565b30426040518763ffffffff1660e01b815260040161109d95949392919061185a565b60006040518083038185885af11580156110bb573d6000803e3d6000fd5b50505050506040513d6000823e601f3d908101601f191682016040526110e49190810190611891565b50611174565b6001600160a01b0384166338ed1739848361110860208701876117c7565b30426040518763ffffffff1660e01b815260040161112b9695949392919061194f565b6000604051808303816000875af115801561114a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111729190810190611891565b505b50505050565b60006111cf826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661126b9092919063ffffffff16565b805190915015610a4d57808060200190518101906111ed919061172a565b610a4d5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610268565b6000611258828461198d565b9392505050565b600061125882846119ac565b606061127a8484600085611282565b949350505050565b6060824710156112e35760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610268565b6001600160a01b0385163b61133a5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610268565b600080866001600160a01b0316858760405161135691906119fa565b60006040518083038185875af1925050503d8060008114611393576040519150601f19603f3d011682016040523d82523d6000602084013e611398565b606091505b50915091506113a88282866113b3565b979650505050505050565b606083156113c2575081611258565b8251156113d25782518084602001fd5b8160405162461bcd60e51b81526004016102689190611a0c565b6001600160a01b038116811461065457600080fd5b60006020828403121561141357600080fd5b8135611258816113ec565b60006020828403121561143057600080fd5b5035919050565b801515811461065457600080fd5b60008083601f84011261145757600080fd5b50813567ffffffffffffffff81111561146f57600080fd5b6020830191508360208260051b850101111561148a57600080fd5b9250929050565b600080600080600080600080600060e08a8c0312156114af57600080fd5b89356114ba816113ec565b985060208a01356114ca816113ec565b975060408a0135965060608a01356114e181611437565b955060808a013567ffffffffffffffff808211156114fe57600080fd5b61150a8d838e01611445565b909750955060a08c0135915061151f826113ec565b90935060c08b0135908082111561153557600080fd5b506115428c828d01611445565b915080935050809150509295985092959850929598565b6000806040838503121561156c57600080fd5b8235611577816113ec565b946020939093013593505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b6000602082840312156115cc57600080fd5b8151611258816113ec565b600060a0820160018060a01b03808a1684526020818a16818601526040898187015260608915158188015260a060808801528488865260c08801905060c08960051b89010195508960005b8a8110156116d15789880360bf190183528135368d9003603e1901811261164857600080fd5b8c018035611655816113ec565b881689528087013536829003601e1901811261167057600080fd5b8101803567ffffffffffffffff81111561168957600080fd5b80360383131561169857600080fd5b87898c015280888c015280898301888d013760008b8201880152601f01601f191690990185019850509185019190850190600101611622565b50959e9d5050505050505050505050505050565b6000602082840312156116f757600080fd5b5051919050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b60006020828403121561173c57600080fd5b815161125881611437565b634e487b7160e01b600052601160045260246000fd5b600060001982141561177157611771611747565b5060010190565b6000821982111561178b5761178b611747565b500190565b6000828210156117a2576117a2611747565b500390565b60008235605e198336030181126117bd57600080fd5b9190910192915050565b6000808335601e198436030181126117de57600080fd5b83018035915067ffffffffffffffff8211156117f957600080fd5b6020019150600581901b360382131561148a57600080fd5b8183526000602080850194508260005b8581101561184f578135611834816113ec565b6001600160a01b031687529582019590820190600101611821565b509495945050505050565b858152608060208201526000611874608083018688611811565b6001600160a01b0394909416604083015250606001529392505050565b600060208083850312156118a457600080fd5b825167ffffffffffffffff808211156118bc57600080fd5b818501915085601f8301126118d057600080fd5b8151818111156118e2576118e26116fe565b8060051b604051601f19603f83011681018181108582111715611907576119076116fe565b60405291825284820192508381018501918883111561192557600080fd5b938501935b828510156119435784518452938501939285019261192a565b98975050505050505050565b86815285602082015260a06040820152600061196f60a083018688611811565b6001600160a01b039490941660608301525060800152949350505050565b60008160001904831182151516156119a7576119a7611747565b500290565b6000826119c957634e487b7160e01b600052601260045260246000fd5b500490565b60005b838110156119e95781810151838201526020016119d1565b838111156111745750506000910152565b600082516117bd8184602087016119ce565b6020815260008251806020840152611a2b8160408501602087016119ce565b601f01601f1916919091016040019291505056fea26469706673582212200558ee2ec96d09d194e95e373afe4bd6bcb8e1abf00c80ebfdbed0f475a1e5cc64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Terminator<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Terminator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Terminator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Terminator))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Terminator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), TERMINATOR_ABI.clone(), client).into()
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
                TERMINATOR_ABI.clone(),
                TERMINATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addYearn` (0x05f0275b) function"]
        pub fn add_yearn(
            &self,
            yearn: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 240, 39, 91], yearn)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowExecutor` (0xb1b05f2a) function"]
        pub fn allow_executor(
            &self,
            executor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 176, 95, 42], executor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executors` (0x9ac2a011) function"]
        pub fn executors(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([154, 194, 160, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidExecutor` (0x6e9d5987) function"]
        pub fn forbid_executor(
            &self,
            executor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 157, 89, 135], executor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateAndSellOnV2` (0x548fa5a2) function"]
        pub fn liquidate_and_sell_on_v2(
            &self,
            credit_facade: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            skip_token_mask: ethers::core::types::U256,
            convert_weth: bool,
            calls: ::std::vec::Vec<MultiCall>,
            router: ethers::core::types::Address,
            paths: ::std::vec::Vec<UniV2Params>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [84, 143, 165, 162],
                    (
                        credit_facade,
                        borrower,
                        skip_token_mask,
                        convert_weth,
                        calls,
                        router,
                        paths,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferToOwner` (0x8568523a) function"]
        pub fn transfer_to_owner(
            &self,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 104, 82, 58], (token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wethToken` (0x4b57b0be) function"]
        pub fn weth_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([75, 87, 176, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `yearn` (0x1c282440) function"]
        pub fn yearn(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([28, 40, 36, 64], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Terminator<M> {
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addYearn`function with signature `addYearn(address)` and selector `[5, 240, 39, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addYearn", abi = "addYearn(address)")]
    pub struct AddYearnCall {
        pub yearn: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `allowExecutor`function with signature `allowExecutor(address)` and selector `[177, 176, 95, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowExecutor", abi = "allowExecutor(address)")]
    pub struct AllowExecutorCall {
        pub executor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executors`function with signature `executors(address)` and selector `[154, 194, 160, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "executors", abi = "executors(address)")]
    pub struct ExecutorsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `forbidExecutor`function with signature `forbidExecutor(address)` and selector `[110, 157, 89, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "forbidExecutor", abi = "forbidExecutor(address)")]
    pub struct ForbidExecutorCall {
        pub executor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `liquidateAndSellOnV2`function with signature `liquidateAndSellOnV2(address,address,uint256,bool,(address,bytes)[],address,(uint256,address[],uint256)[])` and selector `[84, 143, 165, 162]`"]
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
        name = "liquidateAndSellOnV2",
        abi = "liquidateAndSellOnV2(address,address,uint256,bool,(address,bytes)[],address,(uint256,address[],uint256)[])"
    )]
    pub struct LiquidateAndSellOnV2Call {
        pub credit_facade: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub skip_token_mask: ethers::core::types::U256,
        pub convert_weth: bool,
        pub calls: ::std::vec::Vec<MultiCall>,
        pub router: ethers::core::types::Address,
        pub paths: ::std::vec::Vec<UniV2Params>,
    }
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferToOwner`function with signature `transferToOwner(address,uint256)` and selector `[133, 104, 82, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferToOwner", abi = "transferToOwner(address,uint256)")]
    pub struct TransferToOwnerCall {
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wethToken`function with signature `wethToken()` and selector `[75, 87, 176, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wethToken", abi = "wethToken()")]
    pub struct WethTokenCall;
    #[doc = "Container type for all input parameters for the `yearn`function with signature `yearn(uint256)` and selector `[28, 40, 36, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "yearn", abi = "yearn(uint256)")]
    pub struct YearnCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TerminatorCalls {
        AddYearn(AddYearnCall),
        AllowExecutor(AllowExecutorCall),
        Executors(ExecutorsCall),
        ForbidExecutor(ForbidExecutorCall),
        LiquidateAndSellOnV2(LiquidateAndSellOnV2Call),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        TransferToOwner(TransferToOwnerCall),
        WethToken(WethTokenCall),
        Yearn(YearnCall),
    }
    impl ethers::core::abi::AbiDecode for TerminatorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddYearnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::AddYearn(decoded));
            }
            if let Ok(decoded) =
                <AllowExecutorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::AllowExecutor(decoded));
            }
            if let Ok(decoded) =
                <ExecutorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::Executors(decoded));
            }
            if let Ok(decoded) =
                <ForbidExecutorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::ForbidExecutor(decoded));
            }
            if let Ok(decoded) =
                <LiquidateAndSellOnV2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::LiquidateAndSellOnV2(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferToOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::TransferToOwner(decoded));
            }
            if let Ok(decoded) =
                <WethTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::WethToken(decoded));
            }
            if let Ok(decoded) = <YearnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TerminatorCalls::Yearn(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TerminatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TerminatorCalls::AddYearn(element) => element.encode(),
                TerminatorCalls::AllowExecutor(element) => element.encode(),
                TerminatorCalls::Executors(element) => element.encode(),
                TerminatorCalls::ForbidExecutor(element) => element.encode(),
                TerminatorCalls::LiquidateAndSellOnV2(element) => element.encode(),
                TerminatorCalls::Owner(element) => element.encode(),
                TerminatorCalls::RenounceOwnership(element) => element.encode(),
                TerminatorCalls::TransferOwnership(element) => element.encode(),
                TerminatorCalls::TransferToOwner(element) => element.encode(),
                TerminatorCalls::WethToken(element) => element.encode(),
                TerminatorCalls::Yearn(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TerminatorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TerminatorCalls::AddYearn(element) => element.fmt(f),
                TerminatorCalls::AllowExecutor(element) => element.fmt(f),
                TerminatorCalls::Executors(element) => element.fmt(f),
                TerminatorCalls::ForbidExecutor(element) => element.fmt(f),
                TerminatorCalls::LiquidateAndSellOnV2(element) => element.fmt(f),
                TerminatorCalls::Owner(element) => element.fmt(f),
                TerminatorCalls::RenounceOwnership(element) => element.fmt(f),
                TerminatorCalls::TransferOwnership(element) => element.fmt(f),
                TerminatorCalls::TransferToOwner(element) => element.fmt(f),
                TerminatorCalls::WethToken(element) => element.fmt(f),
                TerminatorCalls::Yearn(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddYearnCall> for TerminatorCalls {
        fn from(var: AddYearnCall) -> Self {
            TerminatorCalls::AddYearn(var)
        }
    }
    impl ::std::convert::From<AllowExecutorCall> for TerminatorCalls {
        fn from(var: AllowExecutorCall) -> Self {
            TerminatorCalls::AllowExecutor(var)
        }
    }
    impl ::std::convert::From<ExecutorsCall> for TerminatorCalls {
        fn from(var: ExecutorsCall) -> Self {
            TerminatorCalls::Executors(var)
        }
    }
    impl ::std::convert::From<ForbidExecutorCall> for TerminatorCalls {
        fn from(var: ForbidExecutorCall) -> Self {
            TerminatorCalls::ForbidExecutor(var)
        }
    }
    impl ::std::convert::From<LiquidateAndSellOnV2Call> for TerminatorCalls {
        fn from(var: LiquidateAndSellOnV2Call) -> Self {
            TerminatorCalls::LiquidateAndSellOnV2(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for TerminatorCalls {
        fn from(var: OwnerCall) -> Self {
            TerminatorCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for TerminatorCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            TerminatorCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for TerminatorCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            TerminatorCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TransferToOwnerCall> for TerminatorCalls {
        fn from(var: TransferToOwnerCall) -> Self {
            TerminatorCalls::TransferToOwner(var)
        }
    }
    impl ::std::convert::From<WethTokenCall> for TerminatorCalls {
        fn from(var: WethTokenCall) -> Self {
            TerminatorCalls::WethToken(var)
        }
    }
    impl ::std::convert::From<YearnCall> for TerminatorCalls {
        fn from(var: YearnCall) -> Self {
            TerminatorCalls::Yearn(var)
        }
    }
    #[doc = "`UniV2Params(uint256,address[],uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct UniV2Params {
        pub amount_in: ethers::core::types::U256,
        pub path: Vec<ethers::core::types::Address>,
        pub amount_out_min: ethers::core::types::U256,
    }
}
