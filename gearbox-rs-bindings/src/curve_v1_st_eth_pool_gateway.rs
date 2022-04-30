pub use curvev1stethpoolgateway_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvev1stethpoolgateway_mod {
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
    #[doc = "CurveV1StETHPoolGateway was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVEV1STETHPOOLGATEWAY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_weth\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_steth\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_pool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"amounts\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_mint_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add_liquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"coins\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"exchange_underlying\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy_underlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_virtual_price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"min_amounts\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"amounts\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"max_burn_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_imbalance\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_token_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_one_coin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token0\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVEV1STETHPOOLGATEWAY_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x6101006040523480156200001257600080fd5b5060405162001b7938038062001b798339810160408190526200003591620001ac565b6001600160a01b03831615806200005357506001600160a01b038216155b806200006657506001600160a01b038116155b156200008557604051635919af9760e11b815260040160405180910390fd5b6001600160a01b0380841660805282811660a052811660c081905260408051634163183360e11b815290516382c63066916004808201926020929091908290030181865afa158015620000dc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001029190620001f6565b6001600160a01b0390811660e05260a05160c05160405163095ea7b360e01b81529083166004820152600019602482015291169063095ea7b3906044016020604051808303816000875af11580156200015f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200018591906200021b565b505050506200023f565b80516001600160a01b0381168114620001a757600080fd5b919050565b600080600060608486031215620001c257600080fd5b620001cd846200018f565b9250620001dd602085016200018f565b9150620001ed604085016200018f565b90509250925092565b6000602082840312156200020957600080fd5b62000214826200018f565b9392505050565b6000602082840312156200022e57600080fd5b815180151581146200021457600080fd5b60805160a05160c05160e0516117f6620003836000396000818161025f015281816103390152818161051f01528181610596015281816105ce01528181610c4501528181610fb601526111bb0152600081816101ab0152818161048c0152818161061d0152818161097d01528181610abb01528181610c8401528181610e3001528181610eca0152610ff50152600081816102e801528181610449015281816107e90152818161085f01528181610a0701528181610a6401528181610dd601528181610f8001528181611174015261125701526000818161015f01528181610387015281816103d20152818161068f015281816107230152818161079a015281816108ae015281816108ec01528181610b2101528181610bb501528181610cef01528181610d8301528181610f5901528181611080015261111401526117f66000f3fe6080604052600436106100ec5760003560e01c80635e0d443f1161008a578063c661065711610059578063c6610657146102b6578063d21220a7146102d6578063e31032731461030a578063fc0c546a1461032a57600080fd5b80635e0d443f1461022d57806382c630661461024d578063a6417ed614610281578063bb7b8b80146102a157600080fd5b806316f0115b116100c657806316f0115b146101995780631a4d01d2146101cd5780633df02124146101ed5780635b36389c1461020d57600080fd5b806307211ef7146100f85780630b4c7e4d1461012b5780630dfe16811461014d57600080fd5b366100f357005b600080fd5b34801561010457600080fd5b50610118610113366004611510565b61035d565b6040519081526020015b60405180910390f35b34801561013757600080fd5b5061014b6101463660046115ca565b610378565b005b34801561015957600080fd5b506101817f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610122565b3480156101a557600080fd5b506101817f000000000000000000000000000000000000000000000000000000000000000081565b3480156101d957600080fd5b5061014b6101e83660046115f5565b6105c1565b3480156101f957600080fd5b5061014b61020836600461161a565b610886565b34801561021957600080fd5b5061014b61022836600461165c565b610c38565b34801561023957600080fd5b50610118610248366004611510565b610e05565b34801561025957600080fd5b506101817f000000000000000000000000000000000000000000000000000000000000000081565b34801561028d57600080fd5b5061014b61029c36600461161a565b610ead565b3480156102ad57600080fd5b50610118610ec6565b3480156102c257600080fd5b506101816102d1366004611689565b610f4f565b3480156102e257600080fd5b506101817f000000000000000000000000000000000000000000000000000000000000000081565b34801561031657600080fd5b5061014b6103253660046115ca565b610fa9565b34801561033657600080fd5b507f0000000000000000000000000000000000000000000000000000000000000000610181565b600060405163024e46f760e41b815260040160405180910390fd5b8151156104375781516103b9907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169033903090611286565b8151604051632e1a7d4d60e01b815260048101919091527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690632e1a7d4d90602401600060405180830381600087803b15801561041e57600080fd5b505af1158015610432573d6000803e3d6000fd5b505050505b60208201511561047b57815161047b907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169033903090611286565b8151604051630b4c7e4d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691630b4c7e4d916104cb90869086906004016116db565b6000604051808303818588803b1580156104e457600080fd5b505af11580156104f8573d6000803e3d6000fd5b50506040516370a0823160e01b81523060048201526105bd93503392506001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691506370a08231906024015b602060405180830381865afa158015610568573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061058c91906116f6565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691906112f1565b5050565b6105f66001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333086611286565b604051630d2680e960e11b815260048101849052600f83900b6024820152604481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690631a4d01d290606401600060405180830381600087803b15801561066957600080fd5b505af115801561067d573d6000803e3d6000fd5b5050505081600f0b600014156107c6577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0476040518263ffffffff1660e01b81526004016000604051808303818588803b1580156106e857600080fd5b505af11580156106fc573d6000803e3d6000fd5b50506040516370a0823160e01b81523060048201526107c193503392506001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691506370a08231906024015b602060405180830381865afa15801561076c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061079091906116f6565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691906112f1565b505050565b6040516370a0823160e01b81523060048201526107c19033906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015610831573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061085591906116f6565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691906112f1565b83600f0b600014801561089c575082600f0b6001145b15610a3c576108d66001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333085611286565b604051632e1a7d4d60e01b8152600481018390527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690632e1a7d4d90602401600060405180830381600087803b15801561093857600080fd5b505af115801561094c573d6000803e3d6000fd5b5050604051630f7c084960e21b8152600f87810b600483015286900b602482015260448101859052606481018490527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169250633df02124915084906084016000604051808303818588803b1580156109cc57600080fd5b505af11580156109e0573d6000803e3d6000fd5b50506040516370a0823160e01b8152306004820152610a3793503392506001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691506370a0823190602401610814565b610c32565b83600f0b6001148015610a52575082600f0b6000145b15610be557610a8c6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333085611286565b604051630f7c084960e21b8152600f85810b600483015284900b602482015260448101839052606481018290527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633df0212490608401600060405180830381600087803b158015610b0757600080fd5b505af1158015610b1b573d6000803e3d6000fd5b505050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0476040518263ffffffff1660e01b81526004016000604051808303818588803b158015610b7a57600080fd5b505af1158015610b8e573d6000803e3d6000fd5b50506040516370a0823160e01b8152306004820152610a3793503392506001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691506370a082319060240161074f565b60405162461bcd60e51b815260206004820152601860248201527f496e636f727265637420692c6a20706172616d6574657273000000000000000060448201526064015b60405180910390fd5b50505050565b610c6d6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333085611286565b6040516316cd8e2760e21b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635b36389c90610cbb908590859060040161170f565b600060405180830381600087803b158015610cd557600080fd5b505af1158015610ce9573d6000803e3d6000fd5b505050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0476040518263ffffffff1660e01b81526004016000604051808303818588803b158015610d4857600080fd5b505af1158015610d5c573d6000803e3d6000fd5b50506040516370a0823160e01b8152306004820152610db393503392506001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691506370a082319060240161074f565b6040516370a0823160e01b81523060048201526105bd9033906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a0823190602401610814565b604051635e0d443f60e01b8152600f84810b600483015283900b6024820152604481018290526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635e0d443f90606401602060405180830381865afa158015610e7f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ea391906116f6565b90505b9392505050565b60405163024e46f760e41b815260040160405180910390fd5b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663bb7b8b806040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f26573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f4a91906116f6565b905090565b600081610f7d57507f0000000000000000000000000000000000000000000000000000000000000000919050565b507f0000000000000000000000000000000000000000000000000000000000000000919050565b919050565b610fde6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333084611286565b60405163e310327360e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e31032739061102c90859085906004016116db565b600060405180830381600087803b15801561104657600080fd5b505af115801561105a573d6000803e3d6000fd5b50505050600182600060028110611073576110736116a2565b60200201511115611144577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663d0e30db0476040518263ffffffff1660e01b81526004016000604051808303818588803b1580156110d957600080fd5b505af11580156110ed573d6000803e3d6000fd5b50506040516370a0823160e01b815230600482015261114493503392506001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001691506370a082319060240161074f565b6020820151600110156111a3576040516370a0823160e01b81523060048201526111a39033906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a0823190602401610814565b6040516370a0823160e01b81523060048201526001907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906370a0823190602401602060405180830381865afa15801561120a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061122e91906116f6565b11156105bd576040516370a0823160e01b81523060048201526105bd9033906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a082319060240161054b565b6040516001600160a01b0380851660248301528316604482015260648101829052610c329085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152611321565b6040516001600160a01b0383166024820152604481018290526107c190849063a9059cbb60e01b906064016112ba565b6000611376826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166113f39092919063ffffffff16565b8051909150156107c157808060200190518101906113949190611723565b6107c15760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610c29565b6060610ea3848460008585843b61144c5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610c29565b600080866001600160a01b031685876040516114689190611771565b60006040518083038185875af1925050503d80600081146114a5576040519150601f19603f3d011682016040523d82523d6000602084013e6114aa565b606091505b50915091506114ba8282866114c5565b979650505050505050565b606083156114d4575081610ea6565b8251156114e45782518084602001fd5b8160405162461bcd60e51b8152600401610c29919061178d565b8035600f81900b8114610fa457600080fd5b60008060006060848603121561152557600080fd5b61152e846114fe565b925061153c602085016114fe565b9150604084013590509250925092565b600082601f83011261155d57600080fd5b6040516040810181811067ffffffffffffffff8211171561158e57634e487b7160e01b600052604160045260246000fd5b80604052508060408401858111156115a557600080fd5b845b818110156115bf5780358352602092830192016115a7565b509195945050505050565b600080606083850312156115dd57600080fd5b6115e7848461154c565b946040939093013593505050565b60008060006060848603121561160a57600080fd5b8335925061153c602085016114fe565b6000806000806080858703121561163057600080fd5b611639856114fe565b9350611647602086016114fe565b93969395505050506040820135916060013590565b6000806060838503121561166f57600080fd5b82359150611680846020850161154c565b90509250929050565b60006020828403121561169b57600080fd5b5035919050565b634e487b7160e01b600052603260045260246000fd5b8060005b6002811015610c325781518452602093840193909101906001016116bc565b606081016116e982856116b8565b8260408301529392505050565b60006020828403121561170857600080fd5b5051919050565b82815260608101610ea660208301846116b8565b60006020828403121561173557600080fd5b81518015158114610ea657600080fd5b60005b83811015611760578181015183820152602001611748565b83811115610c325750506000910152565b60008251611783818460208701611745565b9190910192915050565b60208152600082518060208401526117ac816040850160208701611745565b601f01601f1916919091016040019291505056fea2646970667358221220e8d3263129318b0010afa014290affa6d6d82e310ea00db1782b32b100b5992364736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct CurveV1StETHPoolGateway<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveV1StETHPoolGateway<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveV1StETHPoolGateway<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveV1StETHPoolGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveV1StETHPoolGateway<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CURVEV1STETHPOOLGATEWAY_ABI.clone(),
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
                CURVEV1STETHPOOLGATEWAY_ABI.clone(),
                CURVEV1STETHPOOLGATEWAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `add_liquidity` (0x0b4c7e4d) function"]
        pub fn add_liquidity(
            &self,
            amounts: [ethers::core::types::U256; 2usize],
            min_mint_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 76, 126, 77], (amounts, min_mint_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `coins` (0xc6610657) function"]
        pub fn coins(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([198, 97, 6, 87], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchange` (0x3df02124) function"]
        pub fn exchange(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
            min_dy: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 240, 33, 36], (i, j, dx, min_dy))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exchange_underlying` (0xa6417ed6) function"]
        pub fn exchange_underlying(
            &self,
            p0: i128,
            p1: i128,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 65, 126, 214], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_dy` (0x5e0d443f) function"]
        pub fn get_dy(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 13, 68, 63], (i, j, dx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_dy_underlying` (0x07211ef7) function"]
        pub fn get_dy_underlying(
            &self,
            i: i128,
            j: i128,
            dx: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 33, 30, 247], (i, j, dx))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `get_virtual_price` (0xbb7b8b80) function"]
        pub fn get_virtual_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([187, 123, 139, 128], ())
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
        #[doc = "Calls the contract's `pool` (0x16f0115b) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove_liquidity` (0x5b36389c) function"]
        pub fn remove_liquidity(
            &self,
            amount: ethers::core::types::U256,
            min_amounts: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 54, 56, 156], (amount, min_amounts))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove_liquidity_imbalance` (0xe3103273) function"]
        pub fn remove_liquidity_imbalance(
            &self,
            amounts: [ethers::core::types::U256; 2usize],
            max_burn_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 16, 50, 115], (amounts, max_burn_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove_liquidity_one_coin` (0x1a4d01d2) function"]
        pub fn remove_liquidity_one_coin(
            &self,
            token_amount: ethers::core::types::U256,
            i: i128,
            min_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 77, 1, 210], (token_amount, i, min_amount))
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
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CurveV1StETHPoolGateway<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `add_liquidity`function with signature `add_liquidity(uint256[2],uint256)` and selector `[11, 76, 126, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "add_liquidity", abi = "add_liquidity(uint256[2],uint256)")]
    pub struct AddLiquidityCall {
        pub amounts: [ethers::core::types::U256; 2usize],
        pub min_mint_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `coins`function with signature `coins(uint256)` and selector `[198, 97, 6, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "coins", abi = "coins(uint256)")]
    pub struct CoinsCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exchange`function with signature `exchange(int128,int128,uint256,uint256)` and selector `[61, 240, 33, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exchange", abi = "exchange(int128,int128,uint256,uint256)")]
    pub struct ExchangeCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
        pub min_dy: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exchange_underlying`function with signature `exchange_underlying(int128,int128,uint256,uint256)` and selector `[166, 65, 126, 214]`"]
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
        name = "exchange_underlying",
        abi = "exchange_underlying(int128,int128,uint256,uint256)"
    )]
    pub struct ExchangeUnderlyingCall(
        pub i128,
        pub i128,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `get_dy`function with signature `get_dy(int128,int128,uint256)` and selector `[94, 13, 68, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_dy", abi = "get_dy(int128,int128,uint256)")]
    pub struct GetDyCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `get_dy_underlying`function with signature `get_dy_underlying(int128,int128,uint256)` and selector `[7, 33, 30, 247]`"]
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
        name = "get_dy_underlying",
        abi = "get_dy_underlying(int128,int128,uint256)"
    )]
    pub struct GetDyUnderlyingCall {
        pub i: i128,
        pub j: i128,
        pub dx: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `get_virtual_price`function with signature `get_virtual_price()` and selector `[187, 123, 139, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "get_virtual_price", abi = "get_virtual_price()")]
    pub struct GetVirtualPriceCall;
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
    #[doc = "Container type for all input parameters for the `pool`function with signature `pool()` and selector `[22, 240, 17, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    #[doc = "Container type for all input parameters for the `remove_liquidity`function with signature `remove_liquidity(uint256,uint256[2])` and selector `[91, 54, 56, 156]`"]
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
        name = "remove_liquidity",
        abi = "remove_liquidity(uint256,uint256[2])"
    )]
    pub struct RemoveLiquidityCall {
        pub amount: ethers::core::types::U256,
        pub min_amounts: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `remove_liquidity_imbalance`function with signature `remove_liquidity_imbalance(uint256[2],uint256)` and selector `[227, 16, 50, 115]`"]
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
        name = "remove_liquidity_imbalance",
        abi = "remove_liquidity_imbalance(uint256[2],uint256)"
    )]
    pub struct RemoveLiquidityImbalanceCall {
        pub amounts: [ethers::core::types::U256; 2usize],
        pub max_burn_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `remove_liquidity_one_coin`function with signature `remove_liquidity_one_coin(uint256,int128,uint256)` and selector `[26, 77, 1, 210]`"]
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
        name = "remove_liquidity_one_coin",
        abi = "remove_liquidity_one_coin(uint256,int128,uint256)"
    )]
    pub struct RemoveLiquidityOneCoinCall {
        pub token_amount: ethers::core::types::U256,
        pub i: i128,
        pub min_amount: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `token0`function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1`function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CurveV1StETHPoolGatewayCalls {
        AddLiquidity(AddLiquidityCall),
        Coins(CoinsCall),
        Exchange(ExchangeCall),
        ExchangeUnderlying(ExchangeUnderlyingCall),
        GetDy(GetDyCall),
        GetDyUnderlying(GetDyUnderlyingCall),
        GetVirtualPrice(GetVirtualPriceCall),
        LpToken(LpTokenCall),
        Pool(PoolCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityImbalance(RemoveLiquidityImbalanceCall),
        RemoveLiquidityOneCoin(RemoveLiquidityOneCoinCall),
        Token(TokenCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ethers::core::abi::AbiDecode for CurveV1StETHPoolGatewayCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <CoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::Coins(decoded));
            }
            if let Ok(decoded) =
                <ExchangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::Exchange(decoded));
            }
            if let Ok(decoded) =
                <ExchangeUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::ExchangeUnderlying(decoded));
            }
            if let Ok(decoded) = <GetDyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::GetDy(decoded));
            }
            if let Ok(decoded) =
                <GetDyUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::GetDyUnderlying(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::GetVirtualPrice(decoded));
            }
            if let Ok(decoded) =
                <LpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::LpToken(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CurveV1StETHPoolGatewayCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityImbalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CurveV1StETHPoolGatewayCalls::RemoveLiquidityImbalance(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RemoveLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::RemoveLiquidityOneCoin(
                    decoded,
                ));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::Token(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1StETHPoolGatewayCalls::Token1(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveV1StETHPoolGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveV1StETHPoolGatewayCalls::AddLiquidity(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::Coins(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::Exchange(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::ExchangeUnderlying(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::GetDy(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::GetDyUnderlying(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::GetVirtualPrice(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::LpToken(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::Pool(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::RemoveLiquidity(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::RemoveLiquidityImbalance(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::RemoveLiquidityOneCoin(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::Token(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::Token0(element) => element.encode(),
                CurveV1StETHPoolGatewayCalls::Token1(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveV1StETHPoolGatewayCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveV1StETHPoolGatewayCalls::AddLiquidity(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::Coins(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::Exchange(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::ExchangeUnderlying(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::GetDy(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::GetDyUnderlying(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::GetVirtualPrice(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::LpToken(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::Pool(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::RemoveLiquidity(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::RemoveLiquidityImbalance(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::RemoveLiquidityOneCoin(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::Token(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::Token0(element) => element.fmt(f),
                CurveV1StETHPoolGatewayCalls::Token1(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: AddLiquidityCall) -> Self {
            CurveV1StETHPoolGatewayCalls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<CoinsCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: CoinsCall) -> Self {
            CurveV1StETHPoolGatewayCalls::Coins(var)
        }
    }
    impl ::std::convert::From<ExchangeCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: ExchangeCall) -> Self {
            CurveV1StETHPoolGatewayCalls::Exchange(var)
        }
    }
    impl ::std::convert::From<ExchangeUnderlyingCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: ExchangeUnderlyingCall) -> Self {
            CurveV1StETHPoolGatewayCalls::ExchangeUnderlying(var)
        }
    }
    impl ::std::convert::From<GetDyCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: GetDyCall) -> Self {
            CurveV1StETHPoolGatewayCalls::GetDy(var)
        }
    }
    impl ::std::convert::From<GetDyUnderlyingCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: GetDyUnderlyingCall) -> Self {
            CurveV1StETHPoolGatewayCalls::GetDyUnderlying(var)
        }
    }
    impl ::std::convert::From<GetVirtualPriceCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: GetVirtualPriceCall) -> Self {
            CurveV1StETHPoolGatewayCalls::GetVirtualPrice(var)
        }
    }
    impl ::std::convert::From<LpTokenCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: LpTokenCall) -> Self {
            CurveV1StETHPoolGatewayCalls::LpToken(var)
        }
    }
    impl ::std::convert::From<PoolCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: PoolCall) -> Self {
            CurveV1StETHPoolGatewayCalls::Pool(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: RemoveLiquidityCall) -> Self {
            CurveV1StETHPoolGatewayCalls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityImbalanceCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: RemoveLiquidityImbalanceCall) -> Self {
            CurveV1StETHPoolGatewayCalls::RemoveLiquidityImbalance(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityOneCoinCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: RemoveLiquidityOneCoinCall) -> Self {
            CurveV1StETHPoolGatewayCalls::RemoveLiquidityOneCoin(var)
        }
    }
    impl ::std::convert::From<TokenCall> for CurveV1StETHPoolGatewayCalls {
        fn from(var: TokenCall) -> Self {
            CurveV1StETHPoolGatewayCalls::Token(var)
        }
    }
    impl ::std::convert::From<Token0Call> for CurveV1StETHPoolGatewayCalls {
        fn from(var: Token0Call) -> Self {
            CurveV1StETHPoolGatewayCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for CurveV1StETHPoolGatewayCalls {
        fn from(var: Token1Call) -> Self {
            CurveV1StETHPoolGatewayCalls::Token1(var)
        }
    }
}
