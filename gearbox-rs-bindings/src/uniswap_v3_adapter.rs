pub use uniswapv3adapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv3adapter_mod {
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
    #[doc = "UniswapV3Adapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV3ADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_router\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectPathLengthException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IUniswapV3Adapter.ExactAllInputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rateMinRAY\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exactAllInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct IUniswapV3Adapter.ExactAllInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rateMinRAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exactAllInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactInputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactOutputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactOutputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNISWAPV3ADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c06040523480156200001157600080fd5b506040516200195e3803806200195e833981016040819052620000349162000134565b81816001600160a01b03821615806200005457506001600160a01b038116155b156200007357604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa158015620000be573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e491906200016c565b6001600160a01b0390811660a052600080546001600160a01b031916929091169190911790555050600180555062000191565b80516001600160a01b03811681146200012f57600080fd5b919050565b600080604083850312156200014857600080fd5b620001538362000117565b9150620001636020840162000117565b90509250929050565b6000602082840312156200017f57600080fd5b6200018a8262000117565b9392505050565b60805160a0516117546200020a6000396000818160b301528181610c7e0152610e2601526000818161018001528181610275015281816103e10152818161052901528181610774015281816108770152818161096401528181610c1101528181610d9801528181610e880152610f1001526117546000f3fe60806040526004361061009c5760003560e01c8063c12c21c011610064578063c12c21c01461016e578063c7fbf4de146101a2578063ce30bbdb146101c2578063db3e2198146101e4578063f28c0498146101f7578063f4f18d901461020a57600080fd5b80632f7a1881146100a1578063414bf389146100f257806378aa73a414610113578063bd90df701461013b578063c04b8d591461015b575b600080fd5b3480156100ad57600080fd5b506100d57f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b610105610100366004611090565b61022a565b6040519081526020016100e9565b34801561011f57600080fd5b50610128600281565b60405161ffff90911681526020016100e9565b34801561014757600080fd5b506000546100d5906001600160a01b031681565b6101056101693660046110c6565b61039f565b34801561017a57600080fd5b506100d57f000000000000000000000000000000000000000000000000000000000000000081565b3480156101ae57600080fd5b506101056101bd366004611103565b610507565b3480156101ce57600080fd5b506101d7600281565b6040516100e99190611115565b6101056101f2366004611090565b610732565b6101056102053660046110c6565b610835565b34801561021657600080fd5b5061010561022536600461113d565b610942565b6000600260015414156102585760405162461bcd60e51b815260040161024f90611178565b60405180910390fd5b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa1580156102c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102e891906111c7565b905060006102fb3685900385018561132f565b6001600160a01b038316606082015290506103808261031d602087018761134c565b61032d604088016020890161134c565b60405163414bf38960e01b90610347908790602401611369565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526001610b48565b80602001905181019061039391906113d3565b60018055949350505050565b6000600260015414156103c45760405162461bcd60e51b815260040161024f90611178565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610430573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061045491906111c7565b90506000806104a061046686806113ec565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250610f7892505050565b915091506000856104b090611517565b6001600160a01b03851660208201526040519091506104e69085908590859063c04b8d5960e01b906103479087906024016115d0565b8060200190518101906104f991906113d3565b600180559695505050505050565b604051633a562dc160e21b815233600482015260009081906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e958b70490602401602060405180830381865afa158015610570573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061059491906111c7565b905060006105a5602085018561134c565b6040516370a0823160e01b81526001600160a01b03848116600483015291909116906370a0823190602401602060405180830381865afa1580156105ed573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061061191906113d3565b60408051610100810190915290915060009080610631602088018861134c565b6001600160a01b03168152602001866020016020810190610652919061134c565b6001600160a01b0316815260200161067060608801604089016115e3565b62ffffff1681526001600160a01b038516602082015260608088013560408301520161069d600185611614565b81526020016b033b2e3c9fd0803ce800000060808801356106bf600187611614565b6106c9919061162b565b6106d3919061164a565b81526020016106e860c0880160a0890161134c565b6001600160a01b03169052905061071683610706602088018861134c565b61032d6040890160208a0161134c565b80602001905181019061072991906113d3565b95945050505050565b6000600260015414156107575760405162461bcd60e51b815260040161024f90611178565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa1580156107c3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107e791906111c7565b905060006107fa3685900385018561132f565b6001600160a01b0383166060820152805160208201516040519293506103809285929190631b67c43360e31b90610347908790602401611369565b60006002600154141561085a5760405162461bcd60e51b815260040161024f90611178565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa1580156108c6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108ea91906111c7565b90506000806108fc61046686806113ec565b9150915060008561090c90611517565b6001600160a01b03851660208201526040519091506104e690859084908690631e51809360e31b906103479087906024016115d0565b604051633a562dc160e21b815233600482015260009081906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e958b70490602401602060405180830381865afa1580156109ab573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109cf91906111c7565b90506000806109e161046686806113ec565b6040516370a0823160e01b81526001600160a01b0386811660048301529294509092506000918416906370a0823190602401602060405180830381865afa158015610a30573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a5491906113d3565b6040805160a0810190915290915060009080610a7089806113ec565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050908252506001600160a01b0387166020808301919091528901356040820152606001610ad0600185611614565b81526020016b033b2e3c9fd0803ce800000060408a0135610af2600187611614565b610afc919061162b565b610b06919061164a565b8152509050610b2a85858563c04b8d5960e01b8560405160240161034791906115d0565b806020019051810190610b3d91906113d3565b979650505050505050565b60608115610c7057600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa158015610baf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bd391906113d3565b1015610c70576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b158015610c5757600080fd5b505af1158015610c6b573d6000803e3d6000fd5b505050505b600080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610d7e576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa158015610ce9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d0d91906113d3565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa158015610d57573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d7b91906113d3565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a92610dd292339216908a9060040161166c565b6000604051808303816000875af1158015610df1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e199190810190611698565b9250336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610ee9576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b158015610ecc57600080fd5b505af1158015610ee0573d6000803e3d6000fd5b50505050610f6d565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b158015610f5457600080fd5b505af1158015610f68573d6000803e3d6000fd5b505050505b505095945050505050565b6000806003610f896014600261162b565b610f939190611706565b83511015610fb457604051638287466d60e01b815260040160405180910390fd5b610fbf83600061100c565b9150611005610fd060036014611706565b610fdc60036014611706565b60148651610fea9190611614565b610ff4919061164a565b610ffe919061162b565b849061100c565b9050915091565b6000611019826014611706565b835110156110615760405162461bcd60e51b8152602060048201526015602482015274746f416464726573735f6f75744f66426f756e647360581b604482015260640161024f565b5081810160200151600160601b90045b92915050565b6000610100828403121561108a57600080fd5b50919050565b600061010082840312156110a357600080fd5b6110ad8383611077565b9392505050565b600060a0828403121561108a57600080fd5b6000602082840312156110d857600080fd5b813567ffffffffffffffff8111156110ef57600080fd5b6110fb848285016110b4565b949350505050565b600060c0828403121561108a57600080fd5b60208101600e831061113757634e487b7160e01b600052602160045260246000fd5b91905290565b60006020828403121561114f57600080fd5b813567ffffffffffffffff81111561116657600080fd5b8201606081850312156110ad57600080fd5b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b6001600160a01b03811681146111c457600080fd5b50565b6000602082840312156111d957600080fd5b81516110ad816111af565b634e487b7160e01b600052604160045260246000fd5b60405160a0810167ffffffffffffffff8111828210171561121d5761121d6111e4565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561124c5761124c6111e4565b604052919050565b803561125f816111af565b919050565b803562ffffff8116811461125f57600080fd5b600061010080838503121561128b57600080fd5b6040519081019067ffffffffffffffff821181831017156112ae576112ae6111e4565b81604052809250833591506112c2826111af565b8181526112d160208501611254565b60208201526112e260408501611264565b60408201526112f360608501611254565b60608201526080840135608082015260a084013560a082015260c084013560c082015261132260e08501611254565b60e0820152505092915050565b6000610100828403121561134257600080fd5b6110ad8383611277565b60006020828403121561135e57600080fd5b81356110ad816111af565b6101008101611071828480516001600160a01b03908116835260208083015182169084015260408083015162ffffff16908401526060808301518216908401526080808301519084015260a0828101519084015260c0808301519084015260e09182015116910152565b6000602082840312156113e557600080fd5b5051919050565b6000808335601e1984360301811261140357600080fd5b83018035915067ffffffffffffffff82111561141e57600080fd5b60200191503681900382131561143357600080fd5b9250929050565b600067ffffffffffffffff821115611454576114546111e4565b50601f01601f191660200190565b600060a0828403121561147457600080fd5b61147c6111fa565b9050813567ffffffffffffffff81111561149557600080fd5b8201601f810184136114a657600080fd5b803560206114bb6114b68361143a565b611223565b82815286828486010111156114cf57600080fd5b8282850183830137600081840183015284526114ec858201611254565b8185015250505060408201356040820152606082013560608201526080820135608082015292915050565b60006110713683611462565b60005b8381101561153e578181015183820152602001611526565b8381111561154d576000848401525b50505050565b6000815180845261156b816020860160208601611523565b601f01601f19169290920160200192915050565b6000815160a0845261159460a0850182611553565b6020848101516001600160a01b031690860152604080850151908601526060808501519086015260809384015193909401929092525090919050565b6020815260006110ad602083018461157f565b6000602082840312156115f557600080fd5b6110ad82611264565b634e487b7160e01b600052601160045260246000fd5b600082821015611626576116266115fe565b500390565b6000816000190483118215151615611645576116456115fe565b500290565b60008261166757634e487b7160e01b600052601260045260246000fd5b500490565b6001600160a01b0384811682528316602082015260606040820181905260009061072990830184611553565b6000602082840312156116aa57600080fd5b815167ffffffffffffffff8111156116c157600080fd5b8201601f810184136116d257600080fd5b80516116e06114b68261143a565b8181528560208385010111156116f557600080fd5b610729826020830160208601611523565b60008219821115611719576117196115fe565b50019056fea26469706673582212208216ebd3b349f770e65433ef54de49d654ecc8b4972d9f7c0696677fa831068f64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct UniswapV3Adapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV3Adapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV3Adapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3Adapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV3Adapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UNISWAPV3ADAPTER_ABI.clone(), client)
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
                UNISWAPV3ADAPTER_ABI.clone(),
                UNISWAPV3ADAPTER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `exactAllInput` (0xf4f18d90) function"]
        pub fn exact_all_input(
            &self,
            params: ExactAllInputParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([244, 241, 141, 144], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactAllInputSingle` (0xc7fbf4de) function"]
        pub fn exact_all_input_single(
            &self,
            params: ExactAllInputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([199, 251, 244, 222], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInput` (0xc04b8d59) function"]
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([192, 75, 141, 89], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInputSingle` (0x414bf389) function"]
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 75, 243, 137], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutput` (0xf28c0498) function"]
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 140, 4, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutputSingle` (0xdb3e2198) function"]
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 62, 33, 152], (params,))
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for UniswapV3Adapter<M> {
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
    #[doc = "Container type for all input parameters for the `exactAllInput`function with signature `exactAllInput((bytes,uint256,uint256))` and selector `[244, 241, 141, 144]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exactAllInput", abi = "exactAllInput((bytes,uint256,uint256))")]
    pub struct ExactAllInputCall {
        pub params: ExactAllInputParams,
    }
    #[doc = "Container type for all input parameters for the `exactAllInputSingle`function with signature `exactAllInputSingle((address,address,uint24,uint256,uint256,uint160))` and selector `[199, 251, 244, 222]`"]
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
        name = "exactAllInputSingle",
        abi = "exactAllInputSingle((address,address,uint24,uint256,uint256,uint160))"
    )]
    pub struct ExactAllInputSingleCall {
        pub params: ExactAllInputSingleParams,
    }
    #[doc = "Container type for all input parameters for the `exactInput`function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `[192, 75, 141, 89]`"]
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
        name = "exactInput",
        abi = "exactInput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    #[doc = "Container type for all input parameters for the `exactInputSingle`function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[65, 75, 243, 137]`"]
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
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    #[doc = "Container type for all input parameters for the `exactOutput`function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `[242, 140, 4, 152]`"]
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
        name = "exactOutput",
        abi = "exactOutput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }
    #[doc = "Container type for all input parameters for the `exactOutputSingle`function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[219, 62, 33, 152]`"]
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
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3AdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        ExactAllInput(ExactAllInputCall),
        ExactAllInputSingle(ExactAllInputSingleCall),
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        TargetContract(TargetContractCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV3AdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <ExactAllInputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::ExactAllInput(decoded));
            }
            if let Ok(decoded) =
                <ExactAllInputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::ExactAllInputSingle(decoded));
            }
            if let Ok(decoded) =
                <ExactInputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::ExactInput(decoded));
            }
            if let Ok(decoded) =
                <ExactInputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::ExactInputSingle(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::ExactOutput(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::ExactOutputSingle(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdapterCalls::TargetContract(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV3AdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3AdapterCalls::GearboxAdapterType(element) => element.encode(),
                UniswapV3AdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                UniswapV3AdapterCalls::CreditFacade(element) => element.encode(),
                UniswapV3AdapterCalls::CreditManager(element) => element.encode(),
                UniswapV3AdapterCalls::ExactAllInput(element) => element.encode(),
                UniswapV3AdapterCalls::ExactAllInputSingle(element) => element.encode(),
                UniswapV3AdapterCalls::ExactInput(element) => element.encode(),
                UniswapV3AdapterCalls::ExactInputSingle(element) => element.encode(),
                UniswapV3AdapterCalls::ExactOutput(element) => element.encode(),
                UniswapV3AdapterCalls::ExactOutputSingle(element) => element.encode(),
                UniswapV3AdapterCalls::TargetContract(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV3AdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3AdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                UniswapV3AdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                UniswapV3AdapterCalls::CreditFacade(element) => element.fmt(f),
                UniswapV3AdapterCalls::CreditManager(element) => element.fmt(f),
                UniswapV3AdapterCalls::ExactAllInput(element) => element.fmt(f),
                UniswapV3AdapterCalls::ExactAllInputSingle(element) => element.fmt(f),
                UniswapV3AdapterCalls::ExactInput(element) => element.fmt(f),
                UniswapV3AdapterCalls::ExactInputSingle(element) => element.fmt(f),
                UniswapV3AdapterCalls::ExactOutput(element) => element.fmt(f),
                UniswapV3AdapterCalls::ExactOutputSingle(element) => element.fmt(f),
                UniswapV3AdapterCalls::TargetContract(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for UniswapV3AdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            UniswapV3AdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for UniswapV3AdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            UniswapV3AdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for UniswapV3AdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            UniswapV3AdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for UniswapV3AdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            UniswapV3AdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<ExactAllInputCall> for UniswapV3AdapterCalls {
        fn from(var: ExactAllInputCall) -> Self {
            UniswapV3AdapterCalls::ExactAllInput(var)
        }
    }
    impl ::std::convert::From<ExactAllInputSingleCall> for UniswapV3AdapterCalls {
        fn from(var: ExactAllInputSingleCall) -> Self {
            UniswapV3AdapterCalls::ExactAllInputSingle(var)
        }
    }
    impl ::std::convert::From<ExactInputCall> for UniswapV3AdapterCalls {
        fn from(var: ExactInputCall) -> Self {
            UniswapV3AdapterCalls::ExactInput(var)
        }
    }
    impl ::std::convert::From<ExactInputSingleCall> for UniswapV3AdapterCalls {
        fn from(var: ExactInputSingleCall) -> Self {
            UniswapV3AdapterCalls::ExactInputSingle(var)
        }
    }
    impl ::std::convert::From<ExactOutputCall> for UniswapV3AdapterCalls {
        fn from(var: ExactOutputCall) -> Self {
            UniswapV3AdapterCalls::ExactOutput(var)
        }
    }
    impl ::std::convert::From<ExactOutputSingleCall> for UniswapV3AdapterCalls {
        fn from(var: ExactOutputSingleCall) -> Self {
            UniswapV3AdapterCalls::ExactOutputSingle(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for UniswapV3AdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            UniswapV3AdapterCalls::TargetContract(var)
        }
    }
}
