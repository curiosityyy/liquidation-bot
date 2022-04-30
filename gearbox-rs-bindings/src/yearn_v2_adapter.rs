pub use yearnv2adapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod yearnv2adapter_mod {
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
    #[doc = "YearnV2Adapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static YEARNV2ADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_yVault\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TokenNotAllowedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pricePerShare\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxShares\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxShares\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxShares\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxLoss\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"shares\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static YEARNV2ADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e06040523480156200001157600080fd5b50604051620019543803806200195483398101604081905262000034916200022d565b81816001600160a01b03821615806200005457506001600160a01b038116155b156200007357604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa158015620000be573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e4919062000265565b6001600160a01b0390811660a052600080546001600160a01b0319169290911691821790556001805560408051637e062a3560e11b8152905191925063fc0c546a9160048083019260209291908290030181865afa1580156200014b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000171919062000265565b6001600160a01b0390811660c081905260a05160405163f9eaee0d60e01b815260048101929092529091169063f9eaee0d90602401602060405180830381865afa158015620001c4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001ea91906200028a565b620002085760405163700ca0af60e01b815260040160405180910390fd5b5050620002ae565b80516001600160a01b03811681146200022857600080fd5b919050565b600080604083850312156200024157600080fd5b6200024c8362000210565b91506200025c6020840162000210565b90509250929050565b6000602082840312156200027857600080fd5b620002838262000210565b9392505050565b6000602082840312156200029d57600080fd5b815180151581146200028357600080fd5b60805160a05160c0516116066200034e6000396000818161033501528181610b3a01528181610d5a0152610dd70152600081816101e501528181610f3a01526110e20152600081816102cb015281816103a201528181610578015281816106bc01528181610808015281816109db01528181610a9c01528181610c7e01528181610ecd015281816110540152818161114401526111cc01526116066000f3fe608060405234801561001057600080fd5b506004361061014c5760003560e01c806378aa73a4116100c3578063c12c21c01161007c578063c12c21c0146102c6578063ce30bbdb146102ed578063d0e30db014610302578063dd62ed3e1461030a578063e63697c81461031d578063fc0c546a1461033057600080fd5b806378aa73a41461026757806395d89b411461028257806399530b061461028a578063a9059cbb14610292578063b6b55f25146102a0578063bd90df70146102b357600080fd5b80632e1a7d4d116101155780632e1a7d4d146101cd5780632f7a1881146101e0578063313ce5671461021f5780633ccfd60b146102395780636e553f651461024157806370a082311461025457600080fd5b8062f714ce1461015157806306fdde0314610177578063095ea7b31461018c57806318160ddd146101b257806323b872dd146101ba575b600080fd5b61016461015f36600461124c565b610357565b6040519081526020015b60405180910390f35b61017f61042d565b60405161016e91906112d8565b6101a261019a3660046112eb565b600192915050565b604051901515815260200161016e565b6101646104a3565b6101a26101c8366004611317565b61051b565b6101646101db366004611358565b610536565b6102077f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161016e565b610227610602565b60405160ff909116815260200161016e565b61016461067a565b61016461024f36600461124c565b6107c6565b610164610262366004611371565b610887565b61026f600281565b60405161ffff909116815260200161016e565b61017f6108fc565b610164610945565b6101a26101c83660046112eb565b6101646102ae366004611358565b610999565b600054610207906001600160a01b031681565b6102077f000000000000000000000000000000000000000000000000000000000000000081565b6102f5600981565b60405161016e919061138e565b610164610a5a565b6101646103183660046113b6565b610bbe565b61016461032b3660046113e4565b610c3c565b6102077f000000000000000000000000000000000000000000000000000000000000000081565b6000600260015414156103855760405162461bcd60e51b815260040161037c9061140b565b60405180910390fd5b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa1580156103f1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104159190611442565b90506104218185610d0a565b60018055949350505050565b60008054604080516306fdde0360e01b815290516060936001600160a01b03909316926306fdde0392600480820193918290030181865afa158015610476573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261049e91908101906114e9565b905090565b60008060009054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061049e919061153a565b600060405163024e46f760e41b815260040160405180910390fd5b60006002600154141561055b5760405162461bcd60e51b815260040161037c9061140b565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa1580156105c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105eb9190611442565b90506105f78184610d0a565b600180559392505050565b60008060009054906101000a90046001600160a01b03166001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015610656573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061049e9190611553565b60006002600154141561069f5760405162461bcd60e51b815260040161037c9061140b565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa15801561070b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061072f9190611442565b600080546040516370a0823160e01b81526001600160a01b03808516600483015293945091926001929116906370a0823190602401602060405180830381865afa158015610781573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107a5919061153a565b6107af9190611576565b90506107bb8282610d0a565b925050506001805590565b6000600260015414156107eb5760405162461bcd60e51b815260040161037c9061140b565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610857573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061087b9190611442565b90506104218185610d93565b600080546040516370a0823160e01b81526001600160a01b038481166004830152909116906370a0823190602401602060405180830381865afa1580156108d2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108f6919061153a565b92915050565b60008054604080516395d89b4160e01b815290516060936001600160a01b03909316926395d89b4192600480820193918290030181865afa158015610476573d6000803e3d6000fd5b60008060009054906101000a90046001600160a01b03166001600160a01b03166399530b066040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104f7573d6000803e3d6000fd5b6000600260015414156109be5760405162461bcd60e51b815260040161037c9061140b565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610a2a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a4e9190611442565b90506105f78184610d93565b600060026001541415610a7f5760405162461bcd60e51b815260040161037c9061140b565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610aeb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b0f9190611442565b6040516370a0823160e01b81526001600160a01b038083166004830152919250610bb49183916001917f000000000000000000000000000000000000000000000000000000000000000016906370a0823190602401602060405180830381865afa158015610b81573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ba5919061153a565b610baf9190611576565b610d93565b9150506001805590565b60008054604051636eb1769f60e11b81526001600160a01b03858116600483015284811660248301529091169063dd62ed3e90604401602060405180830381865afa158015610c11573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c35919061153a565b9392505050565b600060026001541415610c615760405162461bcd60e51b815260040161037c9061140b565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610ccd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610cf19190611442565b9050610cfd8186610d0a565b6001805595945050505050565b600080546040805160248082018690528251808303909101815260449091019091526020810180516001600160e01b0316632e1a7d4d60e01b179052610d809185916001600160a01b03909116907f00000000000000000000000000000000000000000000000000000000000000009085610e04565b806020019051810190610c35919061153a565b600080546040805160248082018690528251808303909101815260449091019091526020810180516001600160e01b031663b6b55f2560e01b179052610d809185917f0000000000000000000000000000000000000000000000000000000000000000916001600160a01b03169060015b60608115610f2c57600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa158015610e6b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e8f919061153a565b1015610f2c576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b158015610f1357600080fd5b505af1158015610f27573d6000803e3d6000fd5b505050505b600080336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461103a576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa158015610fa5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fc9919061153a565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa158015611013573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611037919061153a565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a9261108e92339216908a9060040161159b565b6000604051808303816000875af11580156110ad573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110d591908101906114e9565b9250336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146111a5576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b15801561118857600080fd5b505af115801561119c573d6000803e3d6000fd5b50505050611229565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b15801561121057600080fd5b505af1158015611224573d6000803e3d6000fd5b505050505b505095945050505050565b6001600160a01b038116811461124957600080fd5b50565b6000806040838503121561125f57600080fd5b82359150602083013561127181611234565b809150509250929050565b60005b8381101561129757818101518382015260200161127f565b838111156112a6576000848401525b50505050565b600081518084526112c481602086016020860161127c565b601f01601f19169290920160200192915050565b602081526000610c3560208301846112ac565b600080604083850312156112fe57600080fd5b823561130981611234565b946020939093013593505050565b60008060006060848603121561132c57600080fd5b833561133781611234565b9250602084013561134781611234565b929592945050506040919091013590565b60006020828403121561136a57600080fd5b5035919050565b60006020828403121561138357600080fd5b8135610c3581611234565b60208101600e83106113b057634e487b7160e01b600052602160045260246000fd5b91905290565b600080604083850312156113c957600080fd5b82356113d481611234565b9150602083013561127181611234565b6000806000606084860312156113f957600080fd5b83359250602084013561134781611234565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b60006020828403121561145457600080fd5b8151610c3581611234565b634e487b7160e01b600052604160045260246000fd5b600067ffffffffffffffff808411156114905761149061145f565b604051601f8501601f19908116603f011681019082821181831017156114b8576114b861145f565b816040528093508581528686860111156114d157600080fd5b6114df86602083018761127c565b5050509392505050565b6000602082840312156114fb57600080fd5b815167ffffffffffffffff81111561151257600080fd5b8201601f8101841361152357600080fd5b61153284825160208401611475565b949350505050565b60006020828403121561154c57600080fd5b5051919050565b60006020828403121561156557600080fd5b815160ff81168114610c3557600080fd5b60008282101561159657634e487b7160e01b600052601160045260246000fd5b500390565b6001600160a01b038481168252831660208201526060604082018190526000906115c7908301846112ac565b9594505050505056fea26469706673582212203a71adfbc503eb51d9edc64fa912f9530efb3b6a89e6e2f863634e8589f05f0c64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct YearnV2Adapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for YearnV2Adapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for YearnV2Adapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(YearnV2Adapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> YearnV2Adapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), YEARNV2ADAPTER_ABI.clone(), client)
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
                YEARNV2ADAPTER_ABI.clone(),
                YEARNV2ADAPTER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
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
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x6e553f65) function"]
        pub fn deposit_2(
            &self,
            amount: ethers::core::types::U256,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (amount, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb6b55f25) function"]
        pub fn deposit_with_amount(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 181, 95, 37], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xd0e30db0) function"]
        pub fn deposit_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pricePerShare` (0x99530b06) function"]
        pub fn price_per_share(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([153, 83, 11, 6], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x00f714ce) function"]
        pub fn withdraw_1(
            &self,
            max_shares: ethers::core::types::U256,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 247, 20, 206], (max_shares, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw_2(
            &self,
            max_shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([46, 26, 125, 77], max_shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x3ccfd60b) function"]
        pub fn withdraw_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([60, 207, 214, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xe63697c8) function"]
        pub fn withdraw_4(
            &self,
            max_shares: ethers::core::types::U256,
            p1: ethers::core::types::Address,
            max_loss: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([230, 54, 151, 200], (max_shares, p1, max_loss))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, YearnV2AdapterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for YearnV2Adapter<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum YearnV2AdapterEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for YearnV2AdapterEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(YearnV2AdapterEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(YearnV2AdapterEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for YearnV2AdapterEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                YearnV2AdapterEvents::ApprovalFilter(element) => element.fmt(f),
                YearnV2AdapterEvents::TransferFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,address)` and selector `[110, 85, 63, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct Deposit2Call {
        pub amount: ethers::core::types::U256,
        pub p1: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256)` and selector `[182, 181, 95, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositWithAmountCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit()` and selector `[208, 227, 13, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct Deposit0Call;
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `pricePerShare`function with signature `pricePerShare()` and selector `[153, 83, 11, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pricePerShare", abi = "pricePerShare()")]
    pub struct PricePerShareCall;
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
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
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,address)` and selector `[0, 247, 20, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address)")]
    pub struct Withdraw1Call {
        pub max_shares: ethers::core::types::U256,
        pub p1: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256)` and selector `[46, 26, 125, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct Withdraw2Call {
        pub max_shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw()` and selector `[60, 207, 214, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw()")]
    pub struct Withdraw0Call;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,address,uint256)` and selector `[230, 54, 151, 200]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,uint256)")]
    pub struct Withdraw4Call {
        pub max_shares: ethers::core::types::U256,
        pub p1: ethers::core::types::Address,
        pub max_loss: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum YearnV2AdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Decimals(DecimalsCall),
        Deposit2(Deposit2Call),
        DepositWithAmount(DepositWithAmountCall),
        Deposit0(Deposit0Call),
        Name(NameCall),
        PricePerShare(PricePerShareCall),
        Symbol(SymbolCall),
        TargetContract(TargetContractCall),
        Token(TokenCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Withdraw1(Withdraw1Call),
        Withdraw2(Withdraw2Call),
        Withdraw0(Withdraw0Call),
        Withdraw4(Withdraw4Call),
    }
    impl ethers::core::abi::AbiDecode for YearnV2AdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <Deposit2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Deposit2(decoded));
            }
            if let Ok(decoded) =
                <DepositWithAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::DepositWithAmount(decoded));
            }
            if let Ok(decoded) =
                <Deposit0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Deposit0(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(YearnV2AdapterCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PricePerShareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::PricePerShare(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::TargetContract(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <Withdraw1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Withdraw1(decoded));
            }
            if let Ok(decoded) =
                <Withdraw2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Withdraw2(decoded));
            }
            if let Ok(decoded) =
                <Withdraw0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Withdraw0(decoded));
            }
            if let Ok(decoded) =
                <Withdraw4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnV2AdapterCalls::Withdraw4(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for YearnV2AdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                YearnV2AdapterCalls::GearboxAdapterType(element) => element.encode(),
                YearnV2AdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                YearnV2AdapterCalls::Allowance(element) => element.encode(),
                YearnV2AdapterCalls::Approve(element) => element.encode(),
                YearnV2AdapterCalls::BalanceOf(element) => element.encode(),
                YearnV2AdapterCalls::CreditFacade(element) => element.encode(),
                YearnV2AdapterCalls::CreditManager(element) => element.encode(),
                YearnV2AdapterCalls::Decimals(element) => element.encode(),
                YearnV2AdapterCalls::Deposit2(element) => element.encode(),
                YearnV2AdapterCalls::DepositWithAmount(element) => element.encode(),
                YearnV2AdapterCalls::Deposit0(element) => element.encode(),
                YearnV2AdapterCalls::Name(element) => element.encode(),
                YearnV2AdapterCalls::PricePerShare(element) => element.encode(),
                YearnV2AdapterCalls::Symbol(element) => element.encode(),
                YearnV2AdapterCalls::TargetContract(element) => element.encode(),
                YearnV2AdapterCalls::Token(element) => element.encode(),
                YearnV2AdapterCalls::TotalSupply(element) => element.encode(),
                YearnV2AdapterCalls::Transfer(element) => element.encode(),
                YearnV2AdapterCalls::TransferFrom(element) => element.encode(),
                YearnV2AdapterCalls::Withdraw1(element) => element.encode(),
                YearnV2AdapterCalls::Withdraw2(element) => element.encode(),
                YearnV2AdapterCalls::Withdraw0(element) => element.encode(),
                YearnV2AdapterCalls::Withdraw4(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for YearnV2AdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                YearnV2AdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                YearnV2AdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                YearnV2AdapterCalls::Allowance(element) => element.fmt(f),
                YearnV2AdapterCalls::Approve(element) => element.fmt(f),
                YearnV2AdapterCalls::BalanceOf(element) => element.fmt(f),
                YearnV2AdapterCalls::CreditFacade(element) => element.fmt(f),
                YearnV2AdapterCalls::CreditManager(element) => element.fmt(f),
                YearnV2AdapterCalls::Decimals(element) => element.fmt(f),
                YearnV2AdapterCalls::Deposit2(element) => element.fmt(f),
                YearnV2AdapterCalls::DepositWithAmount(element) => element.fmt(f),
                YearnV2AdapterCalls::Deposit0(element) => element.fmt(f),
                YearnV2AdapterCalls::Name(element) => element.fmt(f),
                YearnV2AdapterCalls::PricePerShare(element) => element.fmt(f),
                YearnV2AdapterCalls::Symbol(element) => element.fmt(f),
                YearnV2AdapterCalls::TargetContract(element) => element.fmt(f),
                YearnV2AdapterCalls::Token(element) => element.fmt(f),
                YearnV2AdapterCalls::TotalSupply(element) => element.fmt(f),
                YearnV2AdapterCalls::Transfer(element) => element.fmt(f),
                YearnV2AdapterCalls::TransferFrom(element) => element.fmt(f),
                YearnV2AdapterCalls::Withdraw1(element) => element.fmt(f),
                YearnV2AdapterCalls::Withdraw2(element) => element.fmt(f),
                YearnV2AdapterCalls::Withdraw0(element) => element.fmt(f),
                YearnV2AdapterCalls::Withdraw4(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for YearnV2AdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            YearnV2AdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for YearnV2AdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            YearnV2AdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for YearnV2AdapterCalls {
        fn from(var: AllowanceCall) -> Self {
            YearnV2AdapterCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for YearnV2AdapterCalls {
        fn from(var: ApproveCall) -> Self {
            YearnV2AdapterCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for YearnV2AdapterCalls {
        fn from(var: BalanceOfCall) -> Self {
            YearnV2AdapterCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for YearnV2AdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            YearnV2AdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for YearnV2AdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            YearnV2AdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for YearnV2AdapterCalls {
        fn from(var: DecimalsCall) -> Self {
            YearnV2AdapterCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<Deposit2Call> for YearnV2AdapterCalls {
        fn from(var: Deposit2Call) -> Self {
            YearnV2AdapterCalls::Deposit2(var)
        }
    }
    impl ::std::convert::From<DepositWithAmountCall> for YearnV2AdapterCalls {
        fn from(var: DepositWithAmountCall) -> Self {
            YearnV2AdapterCalls::DepositWithAmount(var)
        }
    }
    impl ::std::convert::From<Deposit0Call> for YearnV2AdapterCalls {
        fn from(var: Deposit0Call) -> Self {
            YearnV2AdapterCalls::Deposit0(var)
        }
    }
    impl ::std::convert::From<NameCall> for YearnV2AdapterCalls {
        fn from(var: NameCall) -> Self {
            YearnV2AdapterCalls::Name(var)
        }
    }
    impl ::std::convert::From<PricePerShareCall> for YearnV2AdapterCalls {
        fn from(var: PricePerShareCall) -> Self {
            YearnV2AdapterCalls::PricePerShare(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for YearnV2AdapterCalls {
        fn from(var: SymbolCall) -> Self {
            YearnV2AdapterCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for YearnV2AdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            YearnV2AdapterCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<TokenCall> for YearnV2AdapterCalls {
        fn from(var: TokenCall) -> Self {
            YearnV2AdapterCalls::Token(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for YearnV2AdapterCalls {
        fn from(var: TotalSupplyCall) -> Self {
            YearnV2AdapterCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for YearnV2AdapterCalls {
        fn from(var: TransferCall) -> Self {
            YearnV2AdapterCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for YearnV2AdapterCalls {
        fn from(var: TransferFromCall) -> Self {
            YearnV2AdapterCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<Withdraw1Call> for YearnV2AdapterCalls {
        fn from(var: Withdraw1Call) -> Self {
            YearnV2AdapterCalls::Withdraw1(var)
        }
    }
    impl ::std::convert::From<Withdraw2Call> for YearnV2AdapterCalls {
        fn from(var: Withdraw2Call) -> Self {
            YearnV2AdapterCalls::Withdraw2(var)
        }
    }
    impl ::std::convert::From<Withdraw0Call> for YearnV2AdapterCalls {
        fn from(var: Withdraw0Call) -> Self {
            YearnV2AdapterCalls::Withdraw0(var)
        }
    }
    impl ::std::convert::From<Withdraw4Call> for YearnV2AdapterCalls {
        fn from(var: Withdraw4Call) -> Self {
            YearnV2AdapterCalls::Withdraw4(var)
        }
    }
}
