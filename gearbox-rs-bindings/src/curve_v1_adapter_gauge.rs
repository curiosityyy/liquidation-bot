pub use curvev1adaptergauge_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvev1adaptergauge_mod {
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
    #[doc = "CurveV1AdapterGauge was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVEV1ADAPTERGAUGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_gauge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"TokenIsNotAddedToCreditManagerException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[8]\",\"name\":\"_reward_tokens\",\"type\":\"address[8]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claim_historic_rewards\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claim_rewards\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimable_reward\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimable_tokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crv_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"curveLPtoken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraReward1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraReward2\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"future_epoch_time\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"kick\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reward_tokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"can_deposit\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"set_approve_deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"user_checkpoint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"voting_escrow\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVEV1ADAPTERGAUGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101406040523480156200001257600080fd5b50604051620019ed380380620019ed8339810160408190526200003591620005b3565b81816001600160a01b03821615806200005557506001600160a01b038116155b156200007457604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa158015620000bf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e59190620005eb565b6001600160a01b0390811660a052600080546001600160a01b0319169282169290921790915560018055821660e081905260408051634163183360e11b815290519192506382c630669160048083019260209291908290030181865afa15801562000154573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200017a9190620005eb565b6001600160a01b0390811660c0526040516354c49fe960e01b8152600060048201819052918291908416906354c49fe990602401602060405180830381865afa925050508015620001ea575060408051601f3d908101601f19168201909252620001e791810190620005eb565b60015b620001f557620002d4565b506040516354c49fe960e01b8152600060048201526001600160a01b038416906354c49fe990602401602060405180830381865afa1580156200023c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002629190620005eb565b6040516354c49fe960e01b8152600160048201529092506001600160a01b038416906354c49fe990602401602060405180830381865afa158015620002ab573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002d19190620005eb565b90505b6001600160a01b03828116610100528181166101205260805160e051604051630f67c5bd60e41b8152908316600482015291169063f67c5bd090602401602060405180830381865afa1580156200032f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000355919062000610565b620003875760e051604051632262b1b560e21b81526001600160a01b0390911660048201526024015b60405180910390fd5b60805160c051604051630f67c5bd60e41b81526001600160a01b03918216600482015291169063f67c5bd090602401602060405180830381865afa158015620003d4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003fa919062000610565b620004285760c051604051632262b1b560e21b81526001600160a01b0390911660048201526024016200037e565b6001600160a01b03821615801590620004af5750608051604051630f67c5bd60e41b81526001600160a01b0384811660048301529091169063f67c5bd090602401602060405180830381865afa15801562000487573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620004ad919062000610565b155b15620004da57604051632262b1b560e21b81526001600160a01b03831660048201526024016200037e565b6001600160a01b03811615801590620005615750608051604051630f67c5bd60e41b81526001600160a01b0383811660048301529091169063f67c5bd090602401602060405180830381865afa15801562000539573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200055f919062000610565b155b156200058c57604051632262b1b560e21b81526001600160a01b03821660048201526024016200037e565b505050506200062a565b80516001600160a01b0381168114620005ae57600080fd5b919050565b60008060408385031215620005c757600080fd5b620005d28362000596565b9150620005e26020840162000596565b90509250929050565b600060208284031215620005fe57600080fd5b620006098262000596565b9392505050565b6000602082840312156200062357600080fd5b5051919050565b60805160a05160c05160e05161010051610120516112ee620006ff600039600081816102a801528181610b0e0152610b5c015260008181610381015281816108070152610a890152600081816102cf0152818161045601526106a70152600081816102730152818161047701526106860152600081816101ca01528181610d190152610ec101526000818161033201528181610850015281816108e1015281816109e601528181610ab101528181610b8401528181610cac01528181610e3301528181610f230152610fab01526112ee6000f3fe608060405234801561001057600080fd5b50600436106101735760003560e01c806397c3413b116100de578063c12c21c011610097578063da5b383f11610071578063da5b383f1461037c578063dfe05031146103a3578063e6f1daf2146103ab578063f77c4791146103b357600080fd5b8063c12c21c01461032d578063ce30bbdb14610354578063d2797b591461036957600080fd5b806397c3413b146102a3578063a6f19c84146102ca578063b6b55f25146102f1578063b9fa7a6914610304578063bd90df7014610312578063be5d1be91461032557600080fd5b806354c49fe91161013057806354c49fe91461023057806376d8b1171461024357806378aa73a41461024b57806382c6306614610266578063927188d91461026e57806396c551751461029557600080fd5b806307546172146101785780631d2747d41461019d5780632e1a7d4d146101b25780632f7a1881146101c557806333134583146101ec5780634b8200931461020d575b600080fd5b6101806103bb565b6040516001600160a01b0390911681526020015b60405180910390f35b6101b06101ab366004611028565b610438565b005b6101b06101c0366004611066565b610451565b6101807f000000000000000000000000000000000000000000000000000000000000000081565b6101ff6101fa36600461107f565b6104da565b604051908152602001610194565b61022061021b36600461107f565b610550565b6040519015158152602001610194565b61018061023e366004611066565b61056b565b6101806105d9565b610253600181565b60405161ffff9091168152602001610194565b61018061062d565b6101807f000000000000000000000000000000000000000000000000000000000000000081565b6101b06101ab36600461107f565b6101807f000000000000000000000000000000000000000000000000000000000000000081565b6101807f000000000000000000000000000000000000000000000000000000000000000081565b6101b06102ff366004611066565b610681565b6101b06101ab3660046110a3565b600054610180906001600160a01b031681565b6101ff610706565b6101807f000000000000000000000000000000000000000000000000000000000000000081565b61035c600781565b60405161019491906110ce565b6101ff61037736600461107f565b61077e565b6101807f000000000000000000000000000000000000000000000000000000000000000081565b6101806107b1565b6101b0610805565b61018061096f565b60008060009054906101000a90046001600160a01b03166001600160a01b031663075461726040518163ffffffff1660e01b8152600401602060405180830381865afa15801561040f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061043391906110f6565b905090565b60405163024e46f760e41b815260040160405180910390fd5b6104d67f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006000368080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250600192506109c3915050565b5050565b60008054604051633313458360e01b81526001600160a01b038481166004830152909116906333134583906024015b602060405180830381865afa158015610526573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061054a9190611113565b92915050565b600060405163024e46f760e41b815260040160405180910390fd5b600080546040516354c49fe960e01b8152600481018490526001600160a01b03909116906354c49fe990602401602060405180830381865afa1580156105b5573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061054a91906110f6565b60008060009054906101000a90046001600160a01b03166001600160a01b03166376d8b1176040518163ffffffff1660e01b8152600401602060405180830381865afa15801561040f573d6000803e3d6000fd5b60008060009054906101000a90046001600160a01b03166001600160a01b03166382c630666040518163ffffffff1660e01b8152600401602060405180830381865afa15801561040f573d6000803e3d6000fd5b6104d67f00000000000000000000000000000000000000000000000000000000000000007f00000000000000000000000000000000000000000000000000000000000000006000368080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250600192506109c3915050565b60008060009054906101000a90046001600160a01b03166001600160a01b031663be5d1be96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561075a573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104339190611113565b6000805460405163d2797b5960e01b81526001600160a01b0384811660048301529091169063d2797b5990602401610509565b60008060009054906101000a90046001600160a01b03166001600160a01b031663dfe050316040518163ffffffff1660e01b8152600401602060405180830381865afa15801561040f573d6000803e3d6000fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661083857600080fd5b604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa15801561089f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108c391906110f6565b6000805460405163367203a560e11b81529293506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811693636ce4074a9361091b9333931691369060040161112c565b6000604051808303816000875af115801561093a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261096291908101906111be565b5061096c81610a6a565b50565b60008060009054906101000a90046001600160a01b03166001600160a01b031663f77c47916040518163ffffffff1660e01b8152600401602060405180830381865afa15801561040f573d6000803e3d6000fd5b604051633a562dc160e21b81523360048201526060906000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e958b70490602401602060405180830381865afa158015610a2d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a5191906110f6565b9050610a608187878787610be3565b9695505050505050565b60405163028f1f8b60e51b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b158015610af557600080fd5b505af1158015610b09573d6000803e3d6000fd5b5050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031615905061096c5760405163028f1f8b60e51b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b158015610bc857600080fd5b505af1158015610bdc573d6000803e3d6000fd5b5050505050565b60608115610d0b57600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa158015610c4a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c6e9190611113565b1015610d0b576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b158015610cf257600080fd5b505af1158015610d06573d6000803e3d6000fd5b505050505b600080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610e19576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa158015610d84573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610da89190611113565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa158015610df2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e169190611113565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a92610e6d92339216908a9060040161126b565b6000604051808303816000875af1158015610e8c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610eb491908101906111be565b9250336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610f84576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b158015610f6757600080fd5b505af1158015610f7b573d6000803e3d6000fd5b50505050611008565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b158015610fef57600080fd5b505af1158015611003573d6000803e3d6000fd5b505050505b505095945050505050565b6001600160a01b038116811461096c57600080fd5b6000806040838503121561103b57600080fd5b823561104681611013565b91506020830135801515811461105b57600080fd5b809150509250929050565b60006020828403121561107857600080fd5b5035919050565b60006020828403121561109157600080fd5b813561109c81611013565b9392505050565b60006101008083850312156110b757600080fd5b8381840111156110c657600080fd5b509092915050565b60208101600e83106110f057634e487b7160e01b600052602160045260246000fd5b91905290565b60006020828403121561110857600080fd5b815161109c81611013565b60006020828403121561112557600080fd5b5051919050565b6001600160a01b0385811682528416602082015260606040820181905281018290526000828460808401376000608084840101526080601f19601f850116830101905095945050505050565b634e487b7160e01b600052604160045260246000fd5b60005b838110156111a9578181015183820152602001611191565b838111156111b8576000848401525b50505050565b6000602082840312156111d057600080fd5b815167ffffffffffffffff808211156111e857600080fd5b818401915084601f8301126111fc57600080fd5b81518181111561120e5761120e611178565b604051601f8201601f19908116603f0116810190838211818310171561123657611236611178565b8160405282815287602084870101111561124f57600080fd5b61126083602083016020880161118e565b979650505050505050565b600060018060a01b0380861683528085166020840152506060604083015282518060608401526112a281608085016020870161118e565b601f01601f19169190910160800194935050505056fea2646970667358221220de64c7e7e013fe884e1d2d43314a3a9a7ba098c8e52c2e1b96827789efbfbb6b64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CurveV1AdapterGauge<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveV1AdapterGauge<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveV1AdapterGauge<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveV1AdapterGauge))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveV1AdapterGauge<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CURVEV1ADAPTERGAUGE_ABI.clone(), client)
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
                CURVEV1ADAPTERGAUGE_ABI.clone(),
                CURVEV1ADAPTERGAUGE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `claim_historic_rewards` (0xb9fa7a69) function"]
        pub fn claim_historic_rewards(
            &self,
            reward_tokens: [ethers::core::types::Address; 8usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 250, 122, 105], reward_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claim_rewards` (0xe6f1daf2) function"]
        pub fn claim_rewards(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 241, 218, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimable_reward` (0xd2797b59) function"]
        pub fn claimable_reward(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([210, 121, 123, 89], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimable_tokens` (0x33134583) function"]
        pub fn claimable_tokens(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([51, 19, 69, 131], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `controller` (0xf77c4791) function"]
        pub fn controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([247, 124, 71, 145], ())
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
        #[doc = "Calls the contract's `crv_token` (0x76d8b117) function"]
        pub fn crv_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([118, 216, 177, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `curveLPtoken` (0x927188d9) function"]
        pub fn curve_l_ptoken(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([146, 113, 136, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb6b55f25) function"]
        pub fn deposit(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 181, 95, 37], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extraReward1` (0xda5b383f) function"]
        pub fn extra_reward_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([218, 91, 56, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extraReward2` (0x97c3413b) function"]
        pub fn extra_reward_2(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([151, 195, 65, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `future_epoch_time` (0xbe5d1be9) function"]
        pub fn future_epoch_time(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([190, 93, 27, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gauge` (0xa6f19c84) function"]
        pub fn gauge(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([166, 241, 156, 132], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `kick` (0x96c55175) function"]
        pub fn kick(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 197, 81, 117], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lp_token` (0x82c63066) function"]
        pub fn lp_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([130, 198, 48, 102], ())
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
        #[doc = "Calls the contract's `reward_tokens` (0x54c49fe9) function"]
        pub fn reward_tokens(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([84, 196, 159, 233], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `set_approve_deposit` (0x1d2747d4) function"]
        pub fn set_approve_deposit(
            &self,
            addr: ethers::core::types::Address,
            can_deposit: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 39, 71, 212], (addr, can_deposit))
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
        #[doc = "Calls the contract's `user_checkpoint` (0x4b820093) function"]
        pub fn user_checkpoint(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([75, 130, 0, 147], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `voting_escrow` (0xdfe05031) function"]
        pub fn voting_escrow(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([223, 224, 80, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], value)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CurveV1AdapterGauge<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[doc = "Container type for all input parameters for the `claim_historic_rewards`function with signature `claim_historic_rewards(address[8])` and selector `[185, 250, 122, 105]`"]
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
        name = "claim_historic_rewards",
        abi = "claim_historic_rewards(address[8])"
    )]
    pub struct ClaimHistoricRewardsCall {
        pub reward_tokens: [ethers::core::types::Address; 8usize],
    }
    #[doc = "Container type for all input parameters for the `claim_rewards`function with signature `claim_rewards()` and selector `[230, 241, 218, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claim_rewards", abi = "claim_rewards()")]
    pub struct ClaimRewardsCall;
    #[doc = "Container type for all input parameters for the `claimable_reward`function with signature `claimable_reward(address)` and selector `[210, 121, 123, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimable_reward", abi = "claimable_reward(address)")]
    pub struct ClaimableRewardCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimable_tokens`function with signature `claimable_tokens(address)` and selector `[51, 19, 69, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimable_tokens", abi = "claimable_tokens(address)")]
    pub struct ClaimableTokensCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `controller`function with signature `controller()` and selector `[247, 124, 71, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "controller", abi = "controller()")]
    pub struct ControllerCall;
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
    #[doc = "Container type for all input parameters for the `crv_token`function with signature `crv_token()` and selector `[118, 216, 177, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "crv_token", abi = "crv_token()")]
    pub struct CrvTokenCall;
    #[doc = "Container type for all input parameters for the `curveLPtoken`function with signature `curveLPtoken()` and selector `[146, 113, 136, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "curveLPtoken", abi = "curveLPtoken()")]
    pub struct CurveLPtokenCall;
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
    pub struct DepositCall {
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `extraReward1`function with signature `extraReward1()` and selector `[218, 91, 56, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "extraReward1", abi = "extraReward1()")]
    pub struct ExtraReward1Call;
    #[doc = "Container type for all input parameters for the `extraReward2`function with signature `extraReward2()` and selector `[151, 195, 65, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "extraReward2", abi = "extraReward2()")]
    pub struct ExtraReward2Call;
    #[doc = "Container type for all input parameters for the `future_epoch_time`function with signature `future_epoch_time()` and selector `[190, 93, 27, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "future_epoch_time", abi = "future_epoch_time()")]
    pub struct FutureEpochTimeCall;
    #[doc = "Container type for all input parameters for the `gauge`function with signature `gauge()` and selector `[166, 241, 156, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gauge", abi = "gauge()")]
    pub struct GaugeCall;
    #[doc = "Container type for all input parameters for the `kick`function with signature `kick(address)` and selector `[150, 197, 81, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "kick", abi = "kick(address)")]
    pub struct KickCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lp_token`function with signature `lp_token()` and selector `[130, 198, 48, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lp_token", abi = "lp_token()")]
    pub struct LpTokenCall;
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
    #[doc = "Container type for all input parameters for the `reward_tokens`function with signature `reward_tokens(uint256)` and selector `[84, 196, 159, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "reward_tokens", abi = "reward_tokens(uint256)")]
    pub struct RewardTokensCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `set_approve_deposit`function with signature `set_approve_deposit(address,bool)` and selector `[29, 39, 71, 212]`"]
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
        name = "set_approve_deposit",
        abi = "set_approve_deposit(address,bool)"
    )]
    pub struct SetApproveDepositCall {
        pub addr: ethers::core::types::Address,
        pub can_deposit: bool,
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
    #[doc = "Container type for all input parameters for the `user_checkpoint`function with signature `user_checkpoint(address)` and selector `[75, 130, 0, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "user_checkpoint", abi = "user_checkpoint(address)")]
    pub struct UserCheckpointCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `voting_escrow`function with signature `voting_escrow()` and selector `[223, 224, 80, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "voting_escrow", abi = "voting_escrow()")]
    pub struct VotingEscrowCall;
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
    pub struct WithdrawCall {
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CurveV1AdapterGaugeCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        ClaimHistoricRewards(ClaimHistoricRewardsCall),
        ClaimRewards(ClaimRewardsCall),
        ClaimableReward(ClaimableRewardCall),
        ClaimableTokens(ClaimableTokensCall),
        Controller(ControllerCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        CrvToken(CrvTokenCall),
        CurveLPtoken(CurveLPtokenCall),
        Deposit(DepositCall),
        ExtraReward1(ExtraReward1Call),
        ExtraReward2(ExtraReward2Call),
        FutureEpochTime(FutureEpochTimeCall),
        Gauge(GaugeCall),
        Kick(KickCall),
        LpToken(LpTokenCall),
        Minter(MinterCall),
        RewardTokens(RewardTokensCall),
        SetApproveDeposit(SetApproveDepositCall),
        TargetContract(TargetContractCall),
        UserCheckpoint(UserCheckpointCall),
        VotingEscrow(VotingEscrowCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for CurveV1AdapterGaugeCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <ClaimHistoricRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::ClaimHistoricRewards(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <ClaimableRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::ClaimableReward(decoded));
            }
            if let Ok(decoded) =
                <ClaimableTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::ClaimableTokens(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <CrvTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::CrvToken(decoded));
            }
            if let Ok(decoded) =
                <CurveLPtokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::CurveLPtoken(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExtraReward1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::ExtraReward1(decoded));
            }
            if let Ok(decoded) =
                <ExtraReward2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::ExtraReward2(decoded));
            }
            if let Ok(decoded) =
                <FutureEpochTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::FutureEpochTime(decoded));
            }
            if let Ok(decoded) = <GaugeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::Gauge(decoded));
            }
            if let Ok(decoded) = <KickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CurveV1AdapterGaugeCalls::Kick(decoded));
            }
            if let Ok(decoded) =
                <LpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::LpToken(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::Minter(decoded));
            }
            if let Ok(decoded) =
                <RewardTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::RewardTokens(decoded));
            }
            if let Ok(decoded) =
                <SetApproveDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::SetApproveDeposit(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::TargetContract(decoded));
            }
            if let Ok(decoded) =
                <UserCheckpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::UserCheckpoint(decoded));
            }
            if let Ok(decoded) =
                <VotingEscrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::VotingEscrow(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterGaugeCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveV1AdapterGaugeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveV1AdapterGaugeCalls::GearboxAdapterType(element) => element.encode(),
                CurveV1AdapterGaugeCalls::GearboxAdapterVersion(element) => element.encode(),
                CurveV1AdapterGaugeCalls::ClaimHistoricRewards(element) => element.encode(),
                CurveV1AdapterGaugeCalls::ClaimRewards(element) => element.encode(),
                CurveV1AdapterGaugeCalls::ClaimableReward(element) => element.encode(),
                CurveV1AdapterGaugeCalls::ClaimableTokens(element) => element.encode(),
                CurveV1AdapterGaugeCalls::Controller(element) => element.encode(),
                CurveV1AdapterGaugeCalls::CreditFacade(element) => element.encode(),
                CurveV1AdapterGaugeCalls::CreditManager(element) => element.encode(),
                CurveV1AdapterGaugeCalls::CrvToken(element) => element.encode(),
                CurveV1AdapterGaugeCalls::CurveLPtoken(element) => element.encode(),
                CurveV1AdapterGaugeCalls::Deposit(element) => element.encode(),
                CurveV1AdapterGaugeCalls::ExtraReward1(element) => element.encode(),
                CurveV1AdapterGaugeCalls::ExtraReward2(element) => element.encode(),
                CurveV1AdapterGaugeCalls::FutureEpochTime(element) => element.encode(),
                CurveV1AdapterGaugeCalls::Gauge(element) => element.encode(),
                CurveV1AdapterGaugeCalls::Kick(element) => element.encode(),
                CurveV1AdapterGaugeCalls::LpToken(element) => element.encode(),
                CurveV1AdapterGaugeCalls::Minter(element) => element.encode(),
                CurveV1AdapterGaugeCalls::RewardTokens(element) => element.encode(),
                CurveV1AdapterGaugeCalls::SetApproveDeposit(element) => element.encode(),
                CurveV1AdapterGaugeCalls::TargetContract(element) => element.encode(),
                CurveV1AdapterGaugeCalls::UserCheckpoint(element) => element.encode(),
                CurveV1AdapterGaugeCalls::VotingEscrow(element) => element.encode(),
                CurveV1AdapterGaugeCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveV1AdapterGaugeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveV1AdapterGaugeCalls::GearboxAdapterType(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::GearboxAdapterVersion(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::ClaimHistoricRewards(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::ClaimRewards(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::ClaimableReward(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::ClaimableTokens(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::Controller(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::CreditFacade(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::CreditManager(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::CrvToken(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::CurveLPtoken(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::Deposit(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::ExtraReward1(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::ExtraReward2(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::FutureEpochTime(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::Gauge(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::Kick(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::LpToken(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::Minter(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::RewardTokens(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::SetApproveDeposit(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::TargetContract(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::UserCheckpoint(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::VotingEscrow(element) => element.fmt(f),
                CurveV1AdapterGaugeCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for CurveV1AdapterGaugeCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            CurveV1AdapterGaugeCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for CurveV1AdapterGaugeCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            CurveV1AdapterGaugeCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<ClaimHistoricRewardsCall> for CurveV1AdapterGaugeCalls {
        fn from(var: ClaimHistoricRewardsCall) -> Self {
            CurveV1AdapterGaugeCalls::ClaimHistoricRewards(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for CurveV1AdapterGaugeCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            CurveV1AdapterGaugeCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<ClaimableRewardCall> for CurveV1AdapterGaugeCalls {
        fn from(var: ClaimableRewardCall) -> Self {
            CurveV1AdapterGaugeCalls::ClaimableReward(var)
        }
    }
    impl ::std::convert::From<ClaimableTokensCall> for CurveV1AdapterGaugeCalls {
        fn from(var: ClaimableTokensCall) -> Self {
            CurveV1AdapterGaugeCalls::ClaimableTokens(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for CurveV1AdapterGaugeCalls {
        fn from(var: ControllerCall) -> Self {
            CurveV1AdapterGaugeCalls::Controller(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for CurveV1AdapterGaugeCalls {
        fn from(var: CreditFacadeCall) -> Self {
            CurveV1AdapterGaugeCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for CurveV1AdapterGaugeCalls {
        fn from(var: CreditManagerCall) -> Self {
            CurveV1AdapterGaugeCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CrvTokenCall> for CurveV1AdapterGaugeCalls {
        fn from(var: CrvTokenCall) -> Self {
            CurveV1AdapterGaugeCalls::CrvToken(var)
        }
    }
    impl ::std::convert::From<CurveLPtokenCall> for CurveV1AdapterGaugeCalls {
        fn from(var: CurveLPtokenCall) -> Self {
            CurveV1AdapterGaugeCalls::CurveLPtoken(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CurveV1AdapterGaugeCalls {
        fn from(var: DepositCall) -> Self {
            CurveV1AdapterGaugeCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<ExtraReward1Call> for CurveV1AdapterGaugeCalls {
        fn from(var: ExtraReward1Call) -> Self {
            CurveV1AdapterGaugeCalls::ExtraReward1(var)
        }
    }
    impl ::std::convert::From<ExtraReward2Call> for CurveV1AdapterGaugeCalls {
        fn from(var: ExtraReward2Call) -> Self {
            CurveV1AdapterGaugeCalls::ExtraReward2(var)
        }
    }
    impl ::std::convert::From<FutureEpochTimeCall> for CurveV1AdapterGaugeCalls {
        fn from(var: FutureEpochTimeCall) -> Self {
            CurveV1AdapterGaugeCalls::FutureEpochTime(var)
        }
    }
    impl ::std::convert::From<GaugeCall> for CurveV1AdapterGaugeCalls {
        fn from(var: GaugeCall) -> Self {
            CurveV1AdapterGaugeCalls::Gauge(var)
        }
    }
    impl ::std::convert::From<KickCall> for CurveV1AdapterGaugeCalls {
        fn from(var: KickCall) -> Self {
            CurveV1AdapterGaugeCalls::Kick(var)
        }
    }
    impl ::std::convert::From<LpTokenCall> for CurveV1AdapterGaugeCalls {
        fn from(var: LpTokenCall) -> Self {
            CurveV1AdapterGaugeCalls::LpToken(var)
        }
    }
    impl ::std::convert::From<MinterCall> for CurveV1AdapterGaugeCalls {
        fn from(var: MinterCall) -> Self {
            CurveV1AdapterGaugeCalls::Minter(var)
        }
    }
    impl ::std::convert::From<RewardTokensCall> for CurveV1AdapterGaugeCalls {
        fn from(var: RewardTokensCall) -> Self {
            CurveV1AdapterGaugeCalls::RewardTokens(var)
        }
    }
    impl ::std::convert::From<SetApproveDepositCall> for CurveV1AdapterGaugeCalls {
        fn from(var: SetApproveDepositCall) -> Self {
            CurveV1AdapterGaugeCalls::SetApproveDeposit(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for CurveV1AdapterGaugeCalls {
        fn from(var: TargetContractCall) -> Self {
            CurveV1AdapterGaugeCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<UserCheckpointCall> for CurveV1AdapterGaugeCalls {
        fn from(var: UserCheckpointCall) -> Self {
            CurveV1AdapterGaugeCalls::UserCheckpoint(var)
        }
    }
    impl ::std::convert::From<VotingEscrowCall> for CurveV1AdapterGaugeCalls {
        fn from(var: VotingEscrowCall) -> Self {
            CurveV1AdapterGaugeCalls::VotingEscrow(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CurveV1AdapterGaugeCalls {
        fn from(var: WithdrawCall) -> Self {
            CurveV1AdapterGaugeCalls::Withdraw(var)
        }
    }
}
