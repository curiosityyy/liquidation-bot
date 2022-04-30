pub use convexv1boosteradapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod convexv1boosteradapter_mod {
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
    #[doc = "ConvexV1BoosterAdapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONVEXV1BOOSTERADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_booster\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crv\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_stake\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_stake\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pidToPhantomToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolInfo\",\"outputs\":[{\"internalType\":\"struct IBooster.PoolInfo\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"lptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gauge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"crvRewards\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stash\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"shutdown\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"staker\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateStakedPhantomTokensMap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONVEXV1BOOSTERADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101206040523480156200001257600080fd5b506040516200208238038062002082833981016040819052620000359162000774565b816001600160a01b031663570a7af26040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000074573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200009a9190620007b3565b6001600160a01b0316632954018c6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000d8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000fe9190620007b3565b82826001600160a01b03821615806200011e57506001600160a01b038116155b156200013d57604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa15801562000188573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001ae9190620007b3565b6001600160a01b0390811660a052600080546001600160a81b0319169282169290921790915560408051808201909152600281526105a360f41b6020820152915082166200021a5760405162461bcd60e51b8152600401620002119190620007da565b60405180910390fd5b50806001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200025a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002809190620007b3565b6001600160a01b0390811660c0526001805560408051636a4874a160e01b815290519184169250636a4874a19160048083019260209291908290030181865afa158015620002d2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002f89190620007b3565b6001600160a01b031660e0816001600160a01b031681525050806001600160a01b031663075461726040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000350573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003769190620007b3565b6001600160a01b0316610100526200038d62000395565b505062000885565b60006080516001600160a01b031663f9aa028a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620003d8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003fe9190620007b3565b905060006080516001600160a01b0316632f7a18816040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000443573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620004699190620007b3565b90506000826001600160a01b03166350e036ff6040518163ffffffff1660e01b8152600401602060405180830381865afa158015620004ac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620004d2919062000832565b905060005b818110156200075557604051635094cb4f60e01b8152600481018290526000906001600160a01b03861690635094cb4f90602401602060405180830381865afa15801562000529573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200054f9190620007b3565b60405163fdd5764560e01b81526001600160a01b03808316600483015291925060009186169063fdd5764590602401602060405180830381865afa1580156200059c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620005c29190620007b3565b90506000816001600160a01b031663ce30bbdb6040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000605573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200062b91906200084c565b9050600a81600d8111156200064457620006446200086f565b141562000746576000826001600160a01b031663f10684546040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200068c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620006b2919062000832565b9050826001600160a01b03166320b2c1516040518163ffffffff1660e01b8152600401602060405180830381865afa158015620006f3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620007199190620007b3565b60009182526002602052604090912080546001600160a01b0319166001600160a01b039092169190911790555b836001019350505050620004d7565b50505050565b6001600160a01b03811681146200077157600080fd5b50565b600080604083850312156200078857600080fd5b825162000795816200075b565b6020840151909250620007a8816200075b565b809150509250929050565b600060208284031215620007c657600080fd5b8151620007d3816200075b565b9392505050565b600060208083528351808285015260005b818110156200080957858101830151858201604001528201620007eb565b818111156200081c576000604083870101525b50601f01601f1916929092016040019392505050565b6000602082840312156200084557600080fd5b5051919050565b6000602082840312156200085f57600080fd5b8151600e8110620007d357600080fd5b634e487b7160e01b600052602160045260246000fd5b60805160a05160c05160e0516101005161176b62000917600039600061012b015260006102b2015260008181610494015281816106bd01526107cc01526000818161021e015281816110b6015261125e01526000818161032a01528181610af901528181610b7f01528181610efc01528181611049015281816111d0015281816112c00152611348015261176b6000f3fe608060405234801561001057600080fd5b50600436106101215760003560e01c80635ebaf1db116100ad578063958e2d3111610071578063958e2d31146102f75780639b51ecd31461030a578063bd90df7014610312578063c12c21c014610325578063ce30bbdb1461034c57600080fd5b80635ebaf1db1461029257806360759fce1461029a5780636a4874a1146102ad57806378aa73a4146102d45780638456cb59146102ef57600080fd5b80632f7a1881116100f45780632f7a1881146102195780633f4ba83a1461024057806343a0d0661461024a578063441a3e701461026d5780635c975abb1461028057600080fd5b80630754617214610126578063081e3eda1461016a5780631526fe2714610180578063251d48c0146101f0575b600080fd5b61014d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b610172610361565b604051908152602001610161565b61019361018e3660046113b0565b6103de565b604051610161919081516001600160a01b0390811682526020808401518216908301526040808401518216908301526060808401518216908301526080808401519091169082015260a09182015115159181019190915260c00190565b61014d6101fe3660046113b0565b6002602052600090815260409020546001600160a01b031681565b61014d7f000000000000000000000000000000000000000000000000000000000000000081565b61024861047f565b005b61025d6102583660046113da565b610556565b6040519015158152602001610161565b61025d61027b366004611413565b6105a2565b600054600160a01b900460ff1661025d565b61014d6105ec565b61025d6102a8366004611435565b610664565b61014d7f000000000000000000000000000000000000000000000000000000000000000081565b6102dc600181565b60405161ffff9091168152602001610161565b6102486106a8565b61025d6103053660046113b0565b610774565b6102486107b7565b60005461014d906001600160a01b031681565b61014d7f000000000000000000000000000000000000000000000000000000000000000081565b610354600b81565b604051610161919061147b565b60008060009054906101000a90046001600160a01b03166001600160a01b031663081e3eda6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156103b5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103d991906114a3565b905090565b6040805160c081018252600080825260208201819052818301819052606082018190526080820181905260a08201819052549151631526fe2760e01b81526004810184905290916001600160a01b031690631526fe279060240160c060405180830381865afa158015610455573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061047991906114ee565b92915050565b604051630d4eb5db60e41b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d4eb5db090602401602060405180830381865afa1580156104e3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610507919061158f565b6040518060400160405280600481526020016341434c3160e01b8152509061054b5760405162461bcd60e51b81526004016105429190611604565b60405180910390fd5b50610554610883565b565b600061059a84836000368080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061092092505050565b949350505050565b60006105e5836000368080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506109da92505050565b9392505050565b60008060009054906101000a90046001600160a01b03166001600160a01b0316635ebaf1db6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610640573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103d99190611617565b60006105e583836000368080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061092092505050565b604051630e907b1960e21b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633a41ec6490602401602060405180830381865afa15801561070c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610730919061158f565b6040518060400160405280600481526020016341434c3160e01b8152509061076b5760405162461bcd60e51b81526004016105429190611604565b50610554610a6d565b6000610479826000368080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152506109da92505050565b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa15801561081b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061083f919061158f565b6040518060400160405280600481526020016320a1a61960e11b8152509061087a5760405162461bcd60e51b81526004016105429190611604565b50610554610af5565b600054600160a01b900460ff166108d35760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b6044820152606401610542565b6000805460ff60a01b191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b60008054604051631526fe2760e01b81526004810186905282916001600160a01b031690631526fe279060240160c060405180830381865afa15801561096a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098e91906114ee565b80519091506000856109a45782602001516109bd565b6000878152600260205260409020546001600160a01b03165b90506109cc8282876001610ed9565b506001979650505050505050565b60008054604051631526fe2760e01b81526004810185905282916001600160a01b031690631526fe279060240160c060405180830381865afa158015610a24573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a4891906114ee565b6020810151815191925090610a608282876000610ed9565b5060019695505050505050565b600054600160a01b900460ff1615610aba5760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b6044820152606401610542565b6000805460ff60a01b1916600160a01b1790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586109033390565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f9aa028a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b55573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b799190611617565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632f7a18816040518163ffffffff1660e01b8152600401602060405180830381865afa158015610bdb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bff9190611617565b90506000826001600160a01b03166350e036ff6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c41573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c6591906114a3565b905060005b81811015610ed357604051635094cb4f60e01b8152600481018290526000906001600160a01b03861690635094cb4f90602401602060405180830381865afa158015610cba573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cde9190611617565b60405163fdd5764560e01b81526001600160a01b03808316600483015291925060009186169063fdd5764590602401602060405180830381865afa158015610d2a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d4e9190611617565b90506000816001600160a01b031663ce30bbdb6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d90573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610db49190611632565b9050600a81600d811115610dca57610dca611465565b1415610ec5576000826001600160a01b031663f10684546040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e10573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e3491906114a3565b9050826001600160a01b03166320b2c1516040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e74573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e989190611617565b60009182526002602052604090912080546001600160a01b0319166001600160a01b039092169190911790555b836001019350505050610c6a565b50505050565b604051633a562dc160e21b81523360048201526060906000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e958b70490602401602060405180830381865afa158015610f43573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f679190611617565b9050610f768187878787610f80565b9695505050505050565b606081156110a857600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa158015610fe7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061100b91906114a3565b10156110a8576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b15801561108f57600080fd5b505af11580156110a3573d6000803e3d6000fd5b505050505b600080336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146111b6576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa158015611121573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061114591906114a3565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa15801561118f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906111b391906114a3565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a9261120a92339216908a90600401611653565b6000604051808303816000875af1158015611229573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526112519190810190611688565b9250336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611321576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b15801561130457600080fd5b505af1158015611318573d6000803e3d6000fd5b505050506113a5565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b15801561138c57600080fd5b505af11580156113a0573d6000803e3d6000fd5b505050505b505095945050505050565b6000602082840312156113c257600080fd5b5035919050565b80151581146113d757600080fd5b50565b6000806000606084860312156113ef57600080fd5b83359250602084013591506040840135611408816113c9565b809150509250925092565b6000806040838503121561142657600080fd5b50508035926020909101359150565b6000806040838503121561144857600080fd5b82359150602083013561145a816113c9565b809150509250929050565b634e487b7160e01b600052602160045260246000fd5b60208101600e831061149d57634e487b7160e01b600052602160045260246000fd5b91905290565b6000602082840312156114b557600080fd5b5051919050565b634e487b7160e01b600052604160045260246000fd5b80516001600160a01b03811681146114e957600080fd5b919050565b600060c0828403121561150057600080fd5b60405160c0810181811067ffffffffffffffff82111715611523576115236114bc565b60405261152f836114d2565b815261153d602084016114d2565b602082015261154e604084016114d2565b604082015261155f606084016114d2565b6060820152611570608084016114d2565b608082015260a0830151611583816113c9565b60a08201529392505050565b6000602082840312156115a157600080fd5b81516105e5816113c9565b60005b838110156115c75781810151838201526020016115af565b83811115610ed35750506000910152565b600081518084526115f08160208601602086016115ac565b601f01601f19169290920160200192915050565b6020815260006105e560208301846115d8565b60006020828403121561162957600080fd5b6105e5826114d2565b60006020828403121561164457600080fd5b8151600e81106105e557600080fd5b6001600160a01b0384811682528316602082015260606040820181905260009061167f908301846115d8565b95945050505050565b60006020828403121561169a57600080fd5b815167ffffffffffffffff808211156116b257600080fd5b818401915084601f8301126116c657600080fd5b8151818111156116d8576116d86114bc565b604051601f8201601f19908116603f01168101908382118183101715611700576117006114bc565b8160405282815287602084870101111561171957600080fd5b61172a8360208301602088016115ac565b97965050505050505056fea264697066735822122009531d6b404aa097e0954cb87543704bc2a08868580e76849b2ddef691bba11f64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ConvexV1BoosterAdapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ConvexV1BoosterAdapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ConvexV1BoosterAdapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ConvexV1BoosterAdapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ConvexV1BoosterAdapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CONVEXV1BOOSTERADAPTER_ABI.clone(),
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
                CONVEXV1BOOSTERADAPTER_ABI.clone(),
                CONVEXV1BOOSTERADAPTER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `crv` (0x6a4874a1) function"]
        pub fn crv(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([106, 72, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x43a0d066) function"]
        pub fn deposit(
            &self,
            pid: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            stake: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 160, 208, 102], (pid, amount, stake))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositAll` (0x60759fce) function"]
        pub fn deposit_all(
            &self,
            pid: ethers::core::types::U256,
            stake: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 117, 159, 206], (pid, stake))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minter` (0x07546172) function"]
        pub fn minter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([7, 84, 97, 114], ())
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
        #[doc = "Calls the contract's `pidToPhantomToken` (0x251d48c0) function"]
        pub fn pid_to_phantom_token(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([37, 29, 72, 192], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolInfo` (0x1526fe27) function"]
        pub fn pool_info(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, PoolInfo> {
            self.0
                .method_hash([21, 38, 254, 39], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolLength` (0x081e3eda) function"]
        pub fn pool_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([8, 30, 62, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `staker` (0x5ebaf1db) function"]
        pub fn staker(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 186, 241, 219], ())
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
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateStakedPhantomTokensMap` (0x9b51ecd3) function"]
        pub fn update_staked_phantom_tokens_map(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 81, 236, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x441a3e70) function"]
        pub fn withdraw(
            &self,
            pid: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 26, 62, 112], (pid, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAll` (0x958e2d31) function"]
        pub fn withdraw_all(
            &self,
            pid: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([149, 142, 45, 49], pid)
                .expect("method not found (this should never happen)")
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ConvexV1BoosterAdapterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ConvexV1BoosterAdapter<M>
    {
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
    pub enum ConvexV1BoosterAdapterEvents {
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for ConvexV1BoosterAdapterEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ConvexV1BoosterAdapterEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ConvexV1BoosterAdapterEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ConvexV1BoosterAdapterEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConvexV1BoosterAdapterEvents::PausedFilter(element) => element.fmt(f),
                ConvexV1BoosterAdapterEvents::UnpausedFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `crv`function with signature `crv()` and selector `[106, 72, 116, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "crv", abi = "crv()")]
    pub struct CrvCall;
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,uint256,bool)` and selector `[67, 160, 208, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint256,bool)")]
    pub struct DepositCall {
        pub pid: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub stake: bool,
    }
    #[doc = "Container type for all input parameters for the `depositAll`function with signature `depositAll(uint256,bool)` and selector `[96, 117, 159, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "depositAll", abi = "depositAll(uint256,bool)")]
    pub struct DepositAllCall {
        pub pid: ethers::core::types::U256,
        pub stake: bool,
    }
    #[doc = "Container type for all input parameters for the `minter`function with signature `minter()` and selector `[7, 84, 97, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
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
    #[doc = "Container type for all input parameters for the `pidToPhantomToken`function with signature `pidToPhantomToken(uint256)` and selector `[37, 29, 72, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pidToPhantomToken", abi = "pidToPhantomToken(uint256)")]
    pub struct PidToPhantomTokenCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `poolInfo`function with signature `poolInfo(uint256)` and selector `[21, 38, 254, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolInfo", abi = "poolInfo(uint256)")]
    pub struct PoolInfoCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `poolLength`function with signature `poolLength()` and selector `[8, 30, 62, 218]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolLength", abi = "poolLength()")]
    pub struct PoolLengthCall;
    #[doc = "Container type for all input parameters for the `staker`function with signature `staker()` and selector `[94, 186, 241, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "staker", abi = "staker()")]
    pub struct StakerCall;
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
    #[doc = "Container type for all input parameters for the `updateStakedPhantomTokensMap`function with signature `updateStakedPhantomTokensMap()` and selector `[155, 81, 236, 211]`"]
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
        name = "updateStakedPhantomTokensMap",
        abi = "updateStakedPhantomTokensMap()"
    )]
    pub struct UpdateStakedPhantomTokensMapCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,uint256)` and selector `[68, 26, 62, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,uint256)")]
    pub struct WithdrawCall {
        pub pid: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdrawAll`function with signature `withdrawAll(uint256)` and selector `[149, 142, 45, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawAll", abi = "withdrawAll(uint256)")]
    pub struct WithdrawAllCall {
        pub pid: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConvexV1BoosterAdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Crv(CrvCall),
        Deposit(DepositCall),
        DepositAll(DepositAllCall),
        Minter(MinterCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PidToPhantomToken(PidToPhantomTokenCall),
        PoolInfo(PoolInfoCall),
        PoolLength(PoolLengthCall),
        Staker(StakerCall),
        TargetContract(TargetContractCall),
        Unpause(UnpauseCall),
        UpdateStakedPhantomTokensMap(UpdateStakedPhantomTokensMapCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
    }
    impl ethers::core::abi::AbiDecode for ConvexV1BoosterAdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) = <CrvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ConvexV1BoosterAdapterCalls::Crv(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::DepositAll(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Minter(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <PidToPhantomTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::PidToPhantomToken(decoded));
            }
            if let Ok(decoded) =
                <PoolInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::PoolInfo(decoded));
            }
            if let Ok(decoded) =
                <PoolLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::PoolLength(decoded));
            }
            if let Ok(decoded) = <StakerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Staker(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::TargetContract(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateStakedPhantomTokensMapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ConvexV1BoosterAdapterCalls::UpdateStakedPhantomTokensMap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1BoosterAdapterCalls::WithdrawAll(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ConvexV1BoosterAdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ConvexV1BoosterAdapterCalls::GearboxAdapterType(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::CreditFacade(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::CreditManager(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Crv(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Deposit(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::DepositAll(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Minter(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Pause(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Paused(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::PidToPhantomToken(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::PoolInfo(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::PoolLength(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Staker(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::TargetContract(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::Unpause(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::UpdateStakedPhantomTokensMap(element) => {
                    element.encode()
                }
                ConvexV1BoosterAdapterCalls::Withdraw(element) => element.encode(),
                ConvexV1BoosterAdapterCalls::WithdrawAll(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ConvexV1BoosterAdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConvexV1BoosterAdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::CreditFacade(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::CreditManager(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Crv(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Deposit(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::DepositAll(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Minter(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Pause(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Paused(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::PidToPhantomToken(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::PoolInfo(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::PoolLength(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Staker(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::TargetContract(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::Unpause(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::UpdateStakedPhantomTokensMap(element) => {
                    element.fmt(f)
                }
                ConvexV1BoosterAdapterCalls::Withdraw(element) => element.fmt(f),
                ConvexV1BoosterAdapterCalls::WithdrawAll(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            ConvexV1BoosterAdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            ConvexV1BoosterAdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            ConvexV1BoosterAdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            ConvexV1BoosterAdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CrvCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: CrvCall) -> Self {
            ConvexV1BoosterAdapterCalls::Crv(var)
        }
    }
    impl ::std::convert::From<DepositCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: DepositCall) -> Self {
            ConvexV1BoosterAdapterCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositAllCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: DepositAllCall) -> Self {
            ConvexV1BoosterAdapterCalls::DepositAll(var)
        }
    }
    impl ::std::convert::From<MinterCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: MinterCall) -> Self {
            ConvexV1BoosterAdapterCalls::Minter(var)
        }
    }
    impl ::std::convert::From<PauseCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: PauseCall) -> Self {
            ConvexV1BoosterAdapterCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: PausedCall) -> Self {
            ConvexV1BoosterAdapterCalls::Paused(var)
        }
    }
    impl ::std::convert::From<PidToPhantomTokenCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: PidToPhantomTokenCall) -> Self {
            ConvexV1BoosterAdapterCalls::PidToPhantomToken(var)
        }
    }
    impl ::std::convert::From<PoolInfoCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: PoolInfoCall) -> Self {
            ConvexV1BoosterAdapterCalls::PoolInfo(var)
        }
    }
    impl ::std::convert::From<PoolLengthCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: PoolLengthCall) -> Self {
            ConvexV1BoosterAdapterCalls::PoolLength(var)
        }
    }
    impl ::std::convert::From<StakerCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: StakerCall) -> Self {
            ConvexV1BoosterAdapterCalls::Staker(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            ConvexV1BoosterAdapterCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: UnpauseCall) -> Self {
            ConvexV1BoosterAdapterCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UpdateStakedPhantomTokensMapCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: UpdateStakedPhantomTokensMapCall) -> Self {
            ConvexV1BoosterAdapterCalls::UpdateStakedPhantomTokensMap(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: WithdrawCall) -> Self {
            ConvexV1BoosterAdapterCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAllCall> for ConvexV1BoosterAdapterCalls {
        fn from(var: WithdrawAllCall) -> Self {
            ConvexV1BoosterAdapterCalls::WithdrawAll(var)
        }
    }
}
