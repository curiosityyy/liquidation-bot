pub use uniswapv2adapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv2adapter_mod {
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
    #[doc = "UniswapV2Adapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV2ADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_router\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"addLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"addLiquidityETH\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountIn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveOut\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOut\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountsIn\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountsOut\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveA\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reserveB\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"quote\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountB\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"removeLiquidity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"removeLiquidityETH\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"removeLiquidityETHSupportingFeeOnTransferTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"removeLiquidityETHWithPermit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"removeLiquidityETHWithPermitSupportingFeeOnTransferTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"removeLiquidityWithPermit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rateMinRAY\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapAllTokensForTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapETHForExactTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapExactETHForTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapExactETHForTokensSupportingFeeOnTransferTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"swapExactTokensForETH\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"swapExactTokensForETHSupportingFeeOnTransferTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapExactTokensForTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"swapExactTokensForTokensSupportingFeeOnTransferTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"swapTokensForExactETH\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMax\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapTokensForExactTokens\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNISWAPV2ADAPTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60c06040523480156200001157600080fd5b5060405162001cac38038062001cac833981016040819052620000349162000134565b81816001600160a01b03821615806200005457506001600160a01b038116155b156200007357604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa158015620000be573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e491906200016c565b6001600160a01b0390811660a052600080546001600160a01b031916929091169190911790555050600180555062000191565b80516001600160a01b03811681146200012f57600080fd5b919050565b600080604083850312156200014857600080fd5b620001538362000117565b9150620001636020840162000117565b90509250929050565b6000602082840312156200017f57600080fd5b6200018a8262000117565b9392505050565b60805160a051611ab6620001f6600039600081816102b401528181610ea6015261104e015260008181610491015281816106ef0152818161090201528181610b0001528181610e3901528181610fc0015281816110b001526111380152611ab66000f3fe6080604052600436106101cd5760003560e01c8063ad5c4648116100f7578063c12c21c011610095578063ded9382a11610064578063ded9382a1461050a578063e8e3370014610525578063f305d71914610560578063fb3bdb411461037857600080fd5b8063c12c21c01461047f578063c45a0155146104b3578063ce30bbdb146104c8578063d06ca61f146104ea57600080fd5b8063b6f9de95116100d1578063b6f9de9514610416578063baa2abde14610424578063bd90df701461043f578063bdbeaa311461045f57600080fd5b8063ad5c4648146103c6578063ad615dec146103db578063af2979eb146103fb57600080fd5b80634a25d94a1161016f578063791ac9471161013e578063791ac9471461032e5780637ff36ab51461037857806385f8c259146103865780638803dbee146103a657600080fd5b80634a25d94a1461023a5780635b0d59841461030e5780635c11d7951461032e57806378aa73a41461035057600080fd5b80631f00ca74116101ab5780631f00ca74146102675780632195995c146102875780632f7a1881146102a257806338ed1739146102ee57600080fd5b806302751cec146101d2578063054d50d41461020c57806318cbafe51461023a575b600080fd5b3480156101de57600080fd5b506101f26101ed3660046111b8565b61056e565b604080519283526020830191909152015b60405180910390f35b34801561021857600080fd5b5061022c610227366004611216565b61058a565b604051908152602001610203565b34801561024657600080fd5b5061025a61025536600461128e565b61060f565b6040516102039190611301565b34801561027357600080fd5b5061025a610282366004611345565b61062a565b34801561029357600080fd5b506101f26101ed3660046113b7565b3480156102ae57600080fd5b506102d67f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610203565b3480156102fa57600080fd5b5061025a61030936600461128e565b6106a4565b34801561031a57600080fd5b5061022c61032936600461145f565b61084c565b34801561033a57600080fd5b5061034e61034936600461128e565b610867565b005b34801561035c57600080fd5b50610365600281565b60405161ffff9091168152602001610203565b61025a6102553660046114f3565b34801561039257600080fd5b5061022c6103a1366004611216565b610880565b3480156103b257600080fd5b5061025a6103c136600461128e565b6108c0565b3480156103d257600080fd5b506102d6610a01565b3480156103e757600080fd5b5061022c6103f6366004611216565b610a7e565b34801561040757600080fd5b5061022c6103293660046111b8565b61034e6103493660046114f3565b34801561043057600080fd5b506101f26101ed36600461155a565b34801561044b57600080fd5b506000546102d6906001600160a01b031681565b34801561046b57600080fd5b5061025a61047a3660046115cc565b610abe565b34801561048b57600080fd5b506102d67f000000000000000000000000000000000000000000000000000000000000000081565b3480156104bf57600080fd5b506102d6610cc9565b3480156104d457600080fd5b506104dd600181565b604051610203919061161f565b3480156104f657600080fd5b5061025a610505366004611345565b610d1d565b34801561051657600080fd5b506101f26101ed36600461145f565b34801561053157600080fd5b50610545610540366004611647565b610d52565b60408051938452602084019290925290820152606001610203565b6105456105403660046111b8565b60008060405163024e46f760e41b815260040160405180910390fd5b60008054604051630153543560e21b81526004810186905260248101859052604481018490526001600160a01b039091169063054d50d4906064015b602060405180830381865afa1580156105e3573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061060791906116c3565b949350505050565b606060405163024e46f760e41b815260040160405180910390fd5b6000546040516307c0329d60e21b81526060916001600160a01b031690631f00ca749061065f90879087908790600401611725565b600060405180830381865afa15801561067c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610607919081019061178f565b6060600260015414156106d25760405162461bcd60e51b81526004016106c990611835565b60405180910390fd5b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa15801561073e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610762919061186c565b905060008686600081811061077957610779611890565b905060200201602081019061078e91906118a6565b90506000878761079f6001826118d9565b8181106107ae576107ae611890565b90506020020160208101906107c391906118a6565b90506108278383836338ed173960e01b8e8e8e8e8b8e6040516024016107ee969594939291906118f0565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526001610d70565b80602001905181019061083a919061178f565b600180559a9950505050505050505050565b600060405163024e46f760e41b815260040160405180910390fd5b60405163024e46f760e41b815260040160405180910390fd5b600080546040516385f8c25960e01b81526004810186905260248101859052604481018490526001600160a01b03909116906385f8c259906064016105c6565b6060600260015414156108e55760405162461bcd60e51b81526004016106c990611835565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610951573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610975919061186c565b905060008686600081811061098c5761098c611890565b90506020020160208101906109a191906118a6565b9050600087876109b26001826118d9565b8181106109c1576109c1611890565b90506020020160208101906109d691906118a6565b9050610827838383638803dbee60e01b8e8e8e8e8b8e6040516024016107ee969594939291906118f0565b60008060009054906101000a90046001600160a01b03166001600160a01b031663ad5c46486040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a55573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a79919061186c565b905090565b60008054604051632b58577b60e21b81526004810186905260248101859052604481018490526001600160a01b039091169063ad615dec906064016105c6565b606060026001541415610ae35760405162461bcd60e51b81526004016106c990611835565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610b4f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b73919061186c565b9050600085856000818110610b8a57610b8a611890565b9050602002016020810190610b9f91906118a6565b905060008686610bb06001826118d9565b818110610bbf57610bbf611890565b9050602002016020810190610bd491906118a6565b6040516370a0823160e01b81526001600160a01b0385811660048301529192506000918416906370a0823190602401602060405180830381865afa158015610c20573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c4491906116c3565b9050610ca58484846338ed173960e01b610c5f6001876118d9565b6b033b2e3c9fd0803ce80000008f610c7860018a6118d9565b610c82919061192e565b610c8c919061194d565b8e8e8c8f6040516024016107ee969594939291906118f0565b806020019051810190610cb8919061178f565b600180559998505050505050505050565b60008060009054906101000a90046001600160a01b03166001600160a01b031663c45a01556040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a55573d6000803e3d6000fd5b60005460405163d06ca61f60e01b81526060916001600160a01b03169063d06ca61f9061065f90879087908790600401611725565b600080600060405163024e46f760e41b815260040160405180910390fd5b60608115610e9857600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa158015610dd7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610dfb91906116c3565b1015610e98576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b158015610e7f57600080fd5b505af1158015610e93573d6000803e3d6000fd5b505050505b600080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610fa6576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa158015610f11573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f3591906116c3565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa158015610f7f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fa391906116c3565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a92610ffa92339216908a9060040161199f565b6000604051808303816000875af1158015611019573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261104191908101906119ec565b9250336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611111576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b1580156110f457600080fd5b505af1158015611108573d6000803e3d6000fd5b50505050611195565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b15801561117c57600080fd5b505af1158015611190573d6000803e3d6000fd5b505050505b505095945050505050565b6001600160a01b03811681146111b557600080fd5b50565b60008060008060008060c087890312156111d157600080fd5b86356111dc816111a0565b95506020870135945060408701359350606087013592506080870135611201816111a0565b8092505060a087013590509295509295509295565b60008060006060848603121561122b57600080fd5b505081359360208301359350604090920135919050565b60008083601f84011261125457600080fd5b50813567ffffffffffffffff81111561126c57600080fd5b6020830191508360208260051b850101111561128757600080fd5b9250929050565b60008060008060008060a087890312156112a757600080fd5b8635955060208701359450604087013567ffffffffffffffff8111156112cc57600080fd5b6112d889828a01611242565b90955093505060608701356112ec816111a0565b80925050608087013590509295509295509295565b6020808252825182820181905260009190848201906040850190845b818110156113395783518352928401929184019160010161131d565b50909695505050505050565b60008060006040848603121561135a57600080fd5b83359250602084013567ffffffffffffffff81111561137857600080fd5b61138486828701611242565b9497909650939450505050565b803580151581146113a157600080fd5b919050565b803560ff811681146113a157600080fd5b60008060008060008060008060008060006101608c8e0312156113d957600080fd5b8b356113e4816111a0565b9a5060208c01356113f4816111a0565b995060408c0135985060608c0135975060808c0135965060a08c0135611419816111a0565b955060c08c0135945061142e60e08d01611391565b935061143d6101008d016113a6565b92506101208c013591506101408c013590509295989b509295989b9093969950565b6000806000806000806000806000806101408b8d03121561147f57600080fd5b8a3561148a816111a0565b995060208b0135985060408b0135975060608b0135965060808b01356114af816111a0565b955060a08b013594506114c460c08c01611391565b93506114d260e08c016113a6565b92506101008b013591506101208b013590509295989b9194979a5092959850565b60008060008060006080868803121561150b57600080fd5b85359450602086013567ffffffffffffffff81111561152957600080fd5b61153588828901611242565b9095509350506040860135611549816111a0565b949793965091946060013592915050565b600080600080600080600060e0888a03121561157557600080fd5b8735611580816111a0565b96506020880135611590816111a0565b955060408801359450606088013593506080880135925060a08801356115b5816111a0565b8092505060c0880135905092959891949750929550565b600080600080606085870312156115e257600080fd5b84359350602085013567ffffffffffffffff81111561160057600080fd5b61160c87828801611242565b9598909750949560400135949350505050565b60208101600e831061164157634e487b7160e01b600052602160045260246000fd5b91905290565b600080600080600080600080610100898b03121561166457600080fd5b883561166f816111a0565b9750602089013561167f816111a0565b965060408901359550606089013594506080890135935060a0890135925060c08901356116ab816111a0565b8092505060e089013590509295985092959890939650565b6000602082840312156116d557600080fd5b5051919050565b8183526000602080850194508260005b8581101561171a5781356116ff816111a0565b6001600160a01b0316875295820195908201906001016116ec565b509495945050505050565b83815260406020820152600061173f6040830184866116dc565b95945050505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561178757611787611748565b604052919050565b600060208083850312156117a257600080fd5b825167ffffffffffffffff808211156117ba57600080fd5b818501915085601f8301126117ce57600080fd5b8151818111156117e0576117e0611748565b8060051b91506117f184830161175e565b818152918301840191848101908884111561180b57600080fd5b938501935b8385101561182957845182529385019390850190611810565b98975050505050505050565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b60006020828403121561187e57600080fd5b8151611889816111a0565b9392505050565b634e487b7160e01b600052603260045260246000fd5b6000602082840312156118b857600080fd5b8135611889816111a0565b634e487b7160e01b600052601160045260246000fd5b6000828210156118eb576118eb6118c3565b500390565b86815285602082015260a06040820152600061191060a0830186886116dc565b6001600160a01b039490941660608301525060800152949350505050565b6000816000190483118215151615611948576119486118c3565b500290565b60008261196a57634e487b7160e01b600052601260045260246000fd5b500490565b60005b8381101561198a578181015183820152602001611972565b83811115611999576000848401525b50505050565b600060018060a01b0380861683528085166020840152506060604083015282518060608401526119d681608085016020870161196f565b601f01601f191691909101608001949350505050565b6000602082840312156119fe57600080fd5b815167ffffffffffffffff80821115611a1657600080fd5b818401915084601f830112611a2a57600080fd5b815181811115611a3c57611a3c611748565b611a4f601f8201601f191660200161175e565b9150808252856020828501011115611a6657600080fd5b611a7781602084016020860161196f565b5094935050505056fea26469706673582212203624499c9c0fd9f34ba3b2c5a87eac93c0210d59263ce3d354dadf2c19321ceb64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct UniswapV2Adapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV2Adapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV2Adapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV2Adapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV2Adapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UNISWAPV2ADAPTER_ABI.clone(), client)
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
                UNISWAPV2ADAPTER_ABI.clone(),
                UNISWAPV2ADAPTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `addLiquidity` (0xe8e33700) function"]
        pub fn add_liquidity(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::U256,
            p5: ethers::core::types::U256,
            p6: ethers::core::types::Address,
            p7: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([232, 227, 55, 0], (p0, p1, p2, p3, p4, p5, p6, p7))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xf305d719) function"]
        pub fn add_liquidity_eth(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Address,
            p5: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([243, 5, 215, 25], (p0, p1, p2, p3, p4, p5))
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
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountIn` (0x85f8c259) function"]
        pub fn get_amount_in(
            &self,
            amount_out: ethers::core::types::U256,
            reserve_in: ethers::core::types::U256,
            reserve_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 248, 194, 89], (amount_out, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountOut` (0x054d50d4) function"]
        pub fn get_amount_out(
            &self,
            amount_in: ethers::core::types::U256,
            reserve_in: ethers::core::types::U256,
            reserve_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([5, 77, 80, 212], (amount_in, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsIn` (0x1f00ca74) function"]
        pub fn get_amounts_in(
            &self,
            amount_out: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([31, 0, 202, 116], (amount_out, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsOut` (0xd06ca61f) function"]
        pub fn get_amounts_out(
            &self,
            amount_in: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([208, 108, 166, 31], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quote` (0xad615dec) function"]
        pub fn quote(
            &self,
            amount_a: ethers::core::types::U256,
            reserve_a: ethers::core::types::U256,
            reserve_b: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([173, 97, 93, 236], (amount_a, reserve_a, reserve_b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidity` (0xbaa2abde) function"]
        pub fn remove_liquidity(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::U256,
            p5: ethers::core::types::Address,
            p6: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([186, 162, 171, 222], (p0, p1, p2, p3, p4, p5, p6))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0x02751cec) function"]
        pub fn remove_liquidity_eth(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Address,
            p5: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([2, 117, 28, 236], (p0, p1, p2, p3, p4, p5))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHSupportingFeeOnTransferTokens` (0xaf2979eb) function"]
        pub fn remove_liquidity_eth_supporting_fee_on_transfer_tokens(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Address,
            p5: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([175, 41, 121, 235], (p0, p1, p2, p3, p4, p5))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermit` (0xded9382a) function"]
        pub fn remove_liquidity_eth_with_permit(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Address,
            p5: ethers::core::types::U256,
            p6: bool,
            p7: u8,
            p8: [u8; 32],
            p9: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([222, 217, 56, 42], (p0, p1, p2, p3, p4, p5, p6, p7, p8, p9))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens` (0x5b0d5984) function"]
        pub fn remove_liquidity_eth_with_permit_supporting_fee_on_transfer_tokens(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::Address,
            p5: ethers::core::types::U256,
            p6: bool,
            p7: u8,
            p8: [u8; 32],
            p9: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([91, 13, 89, 132], (p0, p1, p2, p3, p4, p5, p6, p7, p8, p9))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityWithPermit` (0x2195995c) function"]
        pub fn remove_liquidity_with_permit(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
            p4: ethers::core::types::U256,
            p5: ethers::core::types::Address,
            p6: ethers::core::types::U256,
            p7: bool,
            p8: u8,
            p9: [u8; 32],
            p10: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [33, 149, 153, 92],
                    (p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapAllTokensForTokens` (0xbdbeaa31) function"]
        pub fn swap_all_tokens_for_tokens(
            &self,
            rate_min_ray: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([189, 190, 170, 49], (rate_min_ray, path, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapETHForExactTokens` (0xfb3bdb41) function"]
        pub fn swap_eth_for_exact_tokens(
            &self,
            p0: ethers::core::types::U256,
            p1: ::std::vec::Vec<ethers::core::types::Address>,
            p2: ethers::core::types::Address,
            p3: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([251, 59, 219, 65], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokens` (0x7ff36ab5) function"]
        pub fn swap_exact_eth_for_tokens(
            &self,
            p0: ethers::core::types::U256,
            p1: ::std::vec::Vec<ethers::core::types::Address>,
            p2: ethers::core::types::Address,
            p3: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([127, 243, 106, 181], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokensSupportingFeeOnTransferTokens` (0xb6f9de95) function"]
        pub fn swap_exact_eth_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            p0: ethers::core::types::U256,
            p1: ::std::vec::Vec<ethers::core::types::Address>,
            p2: ethers::core::types::Address,
            p3: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 249, 222, 149], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETH` (0x18cbafe5) function"]
        pub fn swap_exact_tokens_for_eth(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
            p2: ::std::vec::Vec<ethers::core::types::Address>,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([24, 203, 175, 229], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETHSupportingFeeOnTransferTokens` (0x791ac947) function"]
        pub fn swap_exact_tokens_for_eth_supporting_fee_on_transfer_tokens(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
            p2: ::std::vec::Vec<ethers::core::types::Address>,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 26, 201, 71], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokens` (0x38ed1739) function"]
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            p3: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [56, 237, 23, 57],
                    (amount_in, amount_out_min, path, p3, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokensSupportingFeeOnTransferTokens` (0x5c11d795) function"]
        pub fn swap_exact_tokens_for_tokens_supporting_fee_on_transfer_tokens(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
            p2: ::std::vec::Vec<ethers::core::types::Address>,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 17, 215, 149], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactETH` (0x4a25d94a) function"]
        pub fn swap_tokens_for_exact_eth(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
            p2: ::std::vec::Vec<ethers::core::types::Address>,
            p3: ethers::core::types::Address,
            p4: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([74, 37, 217, 74], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactTokens` (0x8803dbee) function"]
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ethers::core::types::U256,
            amount_in_max: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            p3: ethers::core::types::Address,
            deadline: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [136, 3, 219, 238],
                    (amount_out, amount_in_max, path, p3, deadline),
                )
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for UniswapV2Adapter<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `WETH`function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
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
    #[doc = "Container type for all input parameters for the `addLiquidity`function with signature `addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)` and selector `[232, 227, 55, 0]`"]
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
        name = "addLiquidity",
        abi = "addLiquidity(address,address,uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `addLiquidityETH`function with signature `addLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[243, 5, 215, 25]`"]
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
        abi = "addLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct AddLiquidityETHCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
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
    #[doc = "Container type for all input parameters for the `getAmountIn`function with signature `getAmountIn(uint256,uint256,uint256)` and selector `[133, 248, 194, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountIn", abi = "getAmountIn(uint256,uint256,uint256)")]
    pub struct GetAmountInCall {
        pub amount_out: ethers::core::types::U256,
        pub reserve_in: ethers::core::types::U256,
        pub reserve_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAmountOut`function with signature `getAmountOut(uint256,uint256,uint256)` and selector `[5, 77, 80, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint256,uint256,uint256)")]
    pub struct GetAmountOutCall {
        pub amount_in: ethers::core::types::U256,
        pub reserve_in: ethers::core::types::U256,
        pub reserve_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getAmountsIn`function with signature `getAmountsIn(uint256,address[])` and selector `[31, 0, 202, 116]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountsIn", abi = "getAmountsIn(uint256,address[])")]
    pub struct GetAmountsInCall {
        pub amount_out: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getAmountsOut`function with signature `getAmountsOut(uint256,address[])` and selector `[208, 108, 166, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAmountsOut", abi = "getAmountsOut(uint256,address[])")]
    pub struct GetAmountsOutCall {
        pub amount_in: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `quote`function with signature `quote(uint256,uint256,uint256)` and selector `[173, 97, 93, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "quote", abi = "quote(uint256,uint256,uint256)")]
    pub struct QuoteCall {
        pub amount_a: ethers::core::types::U256,
        pub reserve_a: ethers::core::types::U256,
        pub reserve_b: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidity`function with signature `removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)` and selector `[186, 162, 171, 222]`"]
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
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `removeLiquidityETH`function with signature `removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)` and selector `[2, 117, 28, 236]`"]
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
        abi = "removeLiquidityETH(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `removeLiquidityETHSupportingFeeOnTransferTokens`function with signature `removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)` and selector `[175, 41, 121, 235]`"]
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
        name = "removeLiquidityETHSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct RemoveLiquidityETHSupportingFeeOnTransferTokensCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `removeLiquidityETHWithPermit`function with signature `removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[222, 217, 56, 42]`"]
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
        name = "removeLiquidityETHWithPermit",
        abi = "removeLiquidityETHWithPermit(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub bool,
        pub u8,
        pub [u8; 32],
        pub [u8; 32],
    );
    #[doc = "Container type for all input parameters for the `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens`function with signature `removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[91, 13, 89, 132]`"]
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
        name = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
        abi = "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens(address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub bool,
        pub u8,
        pub [u8; 32],
        pub [u8; 32],
    );
    #[doc = "Container type for all input parameters for the `removeLiquidityWithPermit`function with signature `removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)` and selector `[33, 149, 153, 92]`"]
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
        name = "removeLiquidityWithPermit",
        abi = "removeLiquidityWithPermit(address,address,uint256,uint256,uint256,address,uint256,bool,uint8,bytes32,bytes32)"
    )]
    pub struct RemoveLiquidityWithPermitCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub bool,
        pub u8,
        pub [u8; 32],
        pub [u8; 32],
    );
    #[doc = "Container type for all input parameters for the `swapAllTokensForTokens`function with signature `swapAllTokensForTokens(uint256,address[],uint256)` and selector `[189, 190, 170, 49]`"]
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
        name = "swapAllTokensForTokens",
        abi = "swapAllTokensForTokens(uint256,address[],uint256)"
    )]
    pub struct SwapAllTokensForTokensCall {
        pub rate_min_ray: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapETHForExactTokens`function with signature `swapETHForExactTokens(uint256,address[],address,uint256)` and selector `[251, 59, 219, 65]`"]
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
        name = "swapETHForExactTokens",
        abi = "swapETHForExactTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapETHForExactTokensCall(
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapExactETHForTokens`function with signature `swapExactETHForTokens(uint256,address[],address,uint256)` and selector `[127, 243, 106, 181]`"]
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
        name = "swapExactETHForTokens",
        abi = "swapExactETHForTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensCall(
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapExactETHForTokensSupportingFeeOnTransferTokens`function with signature `swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)` and selector `[182, 249, 222, 149]`"]
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
        name = "swapExactETHForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactETHForTokensSupportingFeeOnTransferTokens(uint256,address[],address,uint256)"
    )]
    pub struct SwapExactETHForTokensSupportingFeeOnTransferTokensCall(
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapExactTokensForETH`function with signature `swapExactTokensForETH(uint256,uint256,address[],address,uint256)` and selector `[24, 203, 175, 229]`"]
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
        name = "swapExactTokensForETH",
        abi = "swapExactTokensForETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapExactTokensForETHSupportingFeeOnTransferTokens`function with signature `swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `[121, 26, 201, 71]`"]
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
        name = "swapExactTokensForETHSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForETHSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForETHSupportingFeeOnTransferTokensCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapExactTokensForTokens`function with signature `swapExactTokensForTokens(uint256,uint256,address[],address,uint256)` and selector `[56, 237, 23, 57]`"]
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub p3: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapExactTokensForTokensSupportingFeeOnTransferTokens`function with signature `swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)` and selector `[92, 17, 215, 149]`"]
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
        name = "swapExactTokensForTokensSupportingFeeOnTransferTokens",
        abi = "swapExactTokensForTokensSupportingFeeOnTransferTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapExactTokensForTokensSupportingFeeOnTransferTokensCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapTokensForExactETH`function with signature `swapTokensForExactETH(uint256,uint256,address[],address,uint256)` and selector `[74, 37, 217, 74]`"]
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
        name = "swapTokensForExactETH",
        abi = "swapTokensForExactETH(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactETHCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ::std::vec::Vec<ethers::core::types::Address>,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapTokensForExactTokens`function with signature `swapTokensForExactTokens(uint256,uint256,address[],address,uint256)` and selector `[136, 3, 219, 238]`"]
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
        name = "swapTokensForExactTokens",
        abi = "swapTokensForExactTokens(uint256,uint256,address[],address,uint256)"
    )]
    pub struct SwapTokensForExactTokensCall {
        pub amount_out: ethers::core::types::U256,
        pub amount_in_max: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub p3: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
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
    pub enum UniswapV2AdapterCalls {
        Weth(WethCall),
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Factory(FactoryCall),
        GetAmountIn(GetAmountInCall),
        GetAmountOut(GetAmountOutCall),
        GetAmountsIn(GetAmountsInCall),
        GetAmountsOut(GetAmountsOutCall),
        Quote(QuoteCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHSupportingFeeOnTransferTokens(
            RemoveLiquidityETHSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityETHWithPermit(RemoveLiquidityETHWithPermitCall),
        RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(
            RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall,
        ),
        RemoveLiquidityWithPermit(RemoveLiquidityWithPermitCall),
        SwapAllTokensForTokens(SwapAllTokensForTokensCall),
        SwapETHForExactTokens(SwapETHForExactTokensCall),
        SwapExactETHForTokens(SwapExactETHForTokensCall),
        SwapExactETHForTokensSupportingFeeOnTransferTokens(
            SwapExactETHForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForETH(SwapExactTokensForETHCall),
        SwapExactTokensForETHSupportingFeeOnTransferTokens(
            SwapExactTokensForETHSupportingFeeOnTransferTokensCall,
        ),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        SwapExactTokensForTokensSupportingFeeOnTransferTokens(
            SwapExactTokensForTokensSupportingFeeOnTransferTokensCall,
        ),
        SwapTokensForExactETH(SwapTokensForExactETHCall),
        SwapTokensForExactTokens(SwapTokensForExactTokensCall),
        TargetContract(TargetContractCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV2AdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV2AdapterCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetAmountInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::GetAmountIn(decoded));
            }
            if let Ok(decoded) =
                <GetAmountOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::GetAmountOut(decoded));
            }
            if let Ok(decoded) =
                <GetAmountsInCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::GetAmountsIn(decoded));
            }
            if let Ok(decoded) =
                <GetAmountsOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::GetAmountsOut(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::Quote(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::RemoveLiquidityETH(decoded));
            }
            if let Ok (decoded) = < RemoveLiquidityETHSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniswapV2AdapterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <RemoveLiquidityETHWithPermitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV2AdapterCalls::RemoveLiquidityETHWithPermit(decoded));
            }
            if let Ok (decoded) = < RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniswapV2AdapterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <RemoveLiquidityWithPermitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV2AdapterCalls::RemoveLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) =
                <SwapAllTokensForTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::SwapAllTokensForTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapETHForExactTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::SwapETHForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <SwapExactETHForTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::SwapExactETHForTokens(decoded));
            }
            if let Ok (decoded) = < SwapExactETHForTokensSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniswapV2AdapterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapExactTokensForETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::SwapExactTokensForETH(decoded));
            }
            if let Ok (decoded) = < SwapExactTokensForETHSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniswapV2AdapterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapExactTokensForTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV2AdapterCalls::SwapExactTokensForTokens(decoded));
            }
            if let Ok (decoded) = < SwapExactTokensForTokensSupportingFeeOnTransferTokensCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (UniswapV2AdapterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (decoded)) }
            if let Ok(decoded) =
                <SwapTokensForExactETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::SwapTokensForExactETH(decoded));
            }
            if let Ok(decoded) =
                <SwapTokensForExactTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV2AdapterCalls::SwapTokensForExactTokens(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV2AdapterCalls::TargetContract(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV2AdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self { UniswapV2AdapterCalls :: Weth (element) => element . encode () , UniswapV2AdapterCalls :: GearboxAdapterType (element) => element . encode () , UniswapV2AdapterCalls :: GearboxAdapterVersion (element) => element . encode () , UniswapV2AdapterCalls :: AddLiquidity (element) => element . encode () , UniswapV2AdapterCalls :: AddLiquidityETH (element) => element . encode () , UniswapV2AdapterCalls :: CreditFacade (element) => element . encode () , UniswapV2AdapterCalls :: CreditManager (element) => element . encode () , UniswapV2AdapterCalls :: Factory (element) => element . encode () , UniswapV2AdapterCalls :: GetAmountIn (element) => element . encode () , UniswapV2AdapterCalls :: GetAmountOut (element) => element . encode () , UniswapV2AdapterCalls :: GetAmountsIn (element) => element . encode () , UniswapV2AdapterCalls :: GetAmountsOut (element) => element . encode () , UniswapV2AdapterCalls :: Quote (element) => element . encode () , UniswapV2AdapterCalls :: RemoveLiquidity (element) => element . encode () , UniswapV2AdapterCalls :: RemoveLiquidityETH (element) => element . encode () , UniswapV2AdapterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (element) => element . encode () , UniswapV2AdapterCalls :: RemoveLiquidityETHWithPermit (element) => element . encode () , UniswapV2AdapterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (element) => element . encode () , UniswapV2AdapterCalls :: RemoveLiquidityWithPermit (element) => element . encode () , UniswapV2AdapterCalls :: SwapAllTokensForTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapETHForExactTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapExactETHForTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapExactTokensForETH (element) => element . encode () , UniswapV2AdapterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapExactTokensForTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (element) => element . encode () , UniswapV2AdapterCalls :: SwapTokensForExactETH (element) => element . encode () , UniswapV2AdapterCalls :: SwapTokensForExactTokens (element) => element . encode () , UniswapV2AdapterCalls :: TargetContract (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for UniswapV2AdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { UniswapV2AdapterCalls :: Weth (element) => element . fmt (f) , UniswapV2AdapterCalls :: GearboxAdapterType (element) => element . fmt (f) , UniswapV2AdapterCalls :: GearboxAdapterVersion (element) => element . fmt (f) , UniswapV2AdapterCalls :: AddLiquidity (element) => element . fmt (f) , UniswapV2AdapterCalls :: AddLiquidityETH (element) => element . fmt (f) , UniswapV2AdapterCalls :: CreditFacade (element) => element . fmt (f) , UniswapV2AdapterCalls :: CreditManager (element) => element . fmt (f) , UniswapV2AdapterCalls :: Factory (element) => element . fmt (f) , UniswapV2AdapterCalls :: GetAmountIn (element) => element . fmt (f) , UniswapV2AdapterCalls :: GetAmountOut (element) => element . fmt (f) , UniswapV2AdapterCalls :: GetAmountsIn (element) => element . fmt (f) , UniswapV2AdapterCalls :: GetAmountsOut (element) => element . fmt (f) , UniswapV2AdapterCalls :: Quote (element) => element . fmt (f) , UniswapV2AdapterCalls :: RemoveLiquidity (element) => element . fmt (f) , UniswapV2AdapterCalls :: RemoveLiquidityETH (element) => element . fmt (f) , UniswapV2AdapterCalls :: RemoveLiquidityETHSupportingFeeOnTransferTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: RemoveLiquidityETHWithPermit (element) => element . fmt (f) , UniswapV2AdapterCalls :: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: RemoveLiquidityWithPermit (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapAllTokensForTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapETHForExactTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapExactETHForTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapExactETHForTokensSupportingFeeOnTransferTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapExactTokensForETH (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapExactTokensForETHSupportingFeeOnTransferTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapExactTokensForTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapExactTokensForTokensSupportingFeeOnTransferTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapTokensForExactETH (element) => element . fmt (f) , UniswapV2AdapterCalls :: SwapTokensForExactTokens (element) => element . fmt (f) , UniswapV2AdapterCalls :: TargetContract (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<WethCall> for UniswapV2AdapterCalls {
        fn from(var: WethCall) -> Self {
            UniswapV2AdapterCalls::Weth(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for UniswapV2AdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            UniswapV2AdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for UniswapV2AdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            UniswapV2AdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for UniswapV2AdapterCalls {
        fn from(var: AddLiquidityCall) -> Self {
            UniswapV2AdapterCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<AddLiquidityETHCall> for UniswapV2AdapterCalls {
        fn from(var: AddLiquidityETHCall) -> Self {
            UniswapV2AdapterCalls::AddLiquidityETH(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for UniswapV2AdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            UniswapV2AdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for UniswapV2AdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            UniswapV2AdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for UniswapV2AdapterCalls {
        fn from(var: FactoryCall) -> Self {
            UniswapV2AdapterCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetAmountInCall> for UniswapV2AdapterCalls {
        fn from(var: GetAmountInCall) -> Self {
            UniswapV2AdapterCalls::GetAmountIn(var)
        }
    }
    impl ::std::convert::From<GetAmountOutCall> for UniswapV2AdapterCalls {
        fn from(var: GetAmountOutCall) -> Self {
            UniswapV2AdapterCalls::GetAmountOut(var)
        }
    }
    impl ::std::convert::From<GetAmountsInCall> for UniswapV2AdapterCalls {
        fn from(var: GetAmountsInCall) -> Self {
            UniswapV2AdapterCalls::GetAmountsIn(var)
        }
    }
    impl ::std::convert::From<GetAmountsOutCall> for UniswapV2AdapterCalls {
        fn from(var: GetAmountsOutCall) -> Self {
            UniswapV2AdapterCalls::GetAmountsOut(var)
        }
    }
    impl ::std::convert::From<QuoteCall> for UniswapV2AdapterCalls {
        fn from(var: QuoteCall) -> Self {
            UniswapV2AdapterCalls::Quote(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for UniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            UniswapV2AdapterCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHCall> for UniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityETHCall) -> Self {
            UniswapV2AdapterCalls::RemoveLiquidityETH(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHSupportingFeeOnTransferTokensCall>
        for UniswapV2AdapterCalls
    {
        fn from(var: RemoveLiquidityETHSupportingFeeOnTransferTokensCall) -> Self {
            UniswapV2AdapterCalls::RemoveLiquidityETHSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHWithPermitCall> for UniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityETHWithPermitCall) -> Self {
            UniswapV2AdapterCalls::RemoveLiquidityETHWithPermit(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall>
        for UniswapV2AdapterCalls
    {
        fn from(var: RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokensCall) -> Self {
            UniswapV2AdapterCalls::RemoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityWithPermitCall> for UniswapV2AdapterCalls {
        fn from(var: RemoveLiquidityWithPermitCall) -> Self {
            UniswapV2AdapterCalls::RemoveLiquidityWithPermit(var)
        }
    }
    impl ::std::convert::From<SwapAllTokensForTokensCall> for UniswapV2AdapterCalls {
        fn from(var: SwapAllTokensForTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapAllTokensForTokens(var)
        }
    }
    impl ::std::convert::From<SwapETHForExactTokensCall> for UniswapV2AdapterCalls {
        fn from(var: SwapETHForExactTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapETHForExactTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactETHForTokensCall> for UniswapV2AdapterCalls {
        fn from(var: SwapExactETHForTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapExactETHForTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactETHForTokensSupportingFeeOnTransferTokensCall>
        for UniswapV2AdapterCalls
    {
        fn from(var: SwapExactETHForTokensSupportingFeeOnTransferTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapExactETHForTokensSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForETHCall> for UniswapV2AdapterCalls {
        fn from(var: SwapExactTokensForETHCall) -> Self {
            UniswapV2AdapterCalls::SwapExactTokensForETH(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForETHSupportingFeeOnTransferTokensCall>
        for UniswapV2AdapterCalls
    {
        fn from(var: SwapExactTokensForETHSupportingFeeOnTransferTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapExactTokensForETHSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensCall> for UniswapV2AdapterCalls {
        fn from(var: SwapExactTokensForTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapExactTokensForTokens(var)
        }
    }
    impl ::std::convert::From<SwapExactTokensForTokensSupportingFeeOnTransferTokensCall>
        for UniswapV2AdapterCalls
    {
        fn from(var: SwapExactTokensForTokensSupportingFeeOnTransferTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapExactTokensForTokensSupportingFeeOnTransferTokens(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactETHCall> for UniswapV2AdapterCalls {
        fn from(var: SwapTokensForExactETHCall) -> Self {
            UniswapV2AdapterCalls::SwapTokensForExactETH(var)
        }
    }
    impl ::std::convert::From<SwapTokensForExactTokensCall> for UniswapV2AdapterCalls {
        fn from(var: SwapTokensForExactTokensCall) -> Self {
            UniswapV2AdapterCalls::SwapTokensForExactTokens(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for UniswapV2AdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            UniswapV2AdapterCalls::TargetContract(var)
        }
    }
}
