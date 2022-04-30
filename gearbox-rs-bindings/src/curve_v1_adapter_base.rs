pub use curvev1adapterbase_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvev1adapterbase_mod {
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
    #[doc = "CurveV1AdapterBase was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVEV1ADAPTERBASE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_curvePool\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add_all_liquidity_one_coin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"coins\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min_dy\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rateMinRAY\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exchange_all\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"exchange_underlying\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"j\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dx\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_dy_underlying\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"get_virtual_price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minRateRAY\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_all_liquidity_one_coin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int128\",\"name\":\"i\",\"type\":\"int128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove_liquidity_one_coin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVEV1ADAPTERBASE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101006040523480156200001257600080fd5b506040516200181438038062001814833981016040819052620000359162000258565b81816001600160a01b03821615806200005557506001600160a01b038116155b156200007457604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa158015620000bf573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000e5919062000290565b6001600160a01b0390811660a052600080546001600160a01b03191692821692909217909155600180558216151590506200013357604051635919af9760e11b815260040160405180910390fd5b6000816001600160a01b031663fc0c546a6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801562000192575060408051601f3d908101601f191682019092526200018f9181019062000290565b60015b6200021e57604051633795104960e01b81526001600160a01b03831660048201527390e00ace148ca3b23ac1bc8c240c2a7dd9c2d7f590633795104990602401602060405180830381865afa158015620001f0573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000216919062000290565b905062000221565b90505b6001600160a01b031660c081905260e05250620002b59050565b80516001600160a01b03811681146200025357600080fd5b919050565b600080604083850312156200026c57600080fd5b62000277836200023b565b915062000287602084016200023b565b90509250929050565b600060208284031215620002a357600080fd5b620002ae826200023b565b9392505050565b60805160a05160c05160e0516114d162000343600039600081816101f6015281816103fb01528181610bef0152610c6e015260006102a201526000818161015001528181610e4e0152610ff601526000818161024b0152818161070b01528181610ab801528181610b5401528181610de101528181610f680152818161105801526110e001526114d16000f3fe608060405234801561001057600080fd5b506004361061010b5760003560e01c806382c63066116100a2578063c12c21c011610071578063c12c21c014610246578063c66106571461026d578063ce30bbdb14610280578063ec026ca71461028f578063fc0c546a1461029d57600080fd5b806382c63066146101f1578063a6417ed614610218578063bb7b8b801461022b578063bd90df701461023357600080fd5b80633df02124116100de5780633df021241461019d5780635e0d443f146101b057806378aa73a4146101c357806379bea664146101de57600080fd5b806307211ef7146101105780631a4d01d2146101365780632f7a18811461014b57806333d2ebf21461018a575b600080fd5b61012361011e36600461115f565b6102c4565b6040519081526020015b60405180910390f35b61014961014436600461119b565b61034d565b005b6101727f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161012d565b6101496101983660046111c0565b610466565b6101496101ab3660046111ea565b61051a565b6101236101be36600461115f565b610687565b6101cb600281565b60405161ffff909116815260200161012d565b6101496101ec36600461115f565b6106cb565b6101727f000000000000000000000000000000000000000000000000000000000000000081565b6101496102263660046111ea565b61098b565b6101236109a4565b600054610172906001600160a01b031681565b6101727f000000000000000000000000000000000000000000000000000000000000000081565b61017261027b36600461122c565b610a21565b600360405161012d9190611245565b6101496102263660046111c0565b6101727f000000000000000000000000000000000000000000000000000000000000000081565b600080546040516307211ef760e01b8152600f86810b600483015285900b6024820152604481018490526001600160a01b03909116906307211ef7906064015b602060405180830381865afa158015610321573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610345919061126d565b949350505050565b600260015414156103795760405162461bcd60e51b815260040161037090611286565b60405180910390fd5b60026001556000805460405163c661065760e01b81526001600160801b03851660048201526001600160a01b039091169063c661065790602401602060405180830381865afa1580156103d0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103f491906112bd565b905061045b7f0000000000000000000000000000000000000000000000000000000000000000826000368080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525060019250610a95915050565b505060018055505050565b600260015414156104895760405162461bcd60e51b815260040161037090611286565b60026001556000805460405163c661065760e01b81526001600160801b03851660048201526001600160a01b039091169063c661065790602401602060405180830381865afa1580156104e0573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061050491906112bd565b9050610511838284610b3c565b50506001805550565b6002600154141561053d5760405162461bcd60e51b815260040161037090611286565b60026001556000805460405163c661065760e01b81526001600160801b03871660048201526001600160a01b039091169063c661065790602401602060405180830381865afa158015610594573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105b891906112bd565b6000805460405163c661065760e01b81526001600160801b038816600482015292935090916001600160a01b039091169063c661065790602401602060405180830381865afa15801561060f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061063391906112bd565b905061067a82826000368080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525060019250610a95915050565b5050600180555050505050565b60008054604051635e0d443f60e01b8152600f86810b600483015285900b6024820152604481018490526001600160a01b0390911690635e0d443f90606401610304565b600260015414156106ee5760405162461bcd60e51b815260040161037090611286565b6002600155604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa15801561075a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061077e91906112bd565b6000805460405163c661065760e01b81526001600160801b038816600482015292935090916001600160a01b039091169063c661065790602401602060405180830381865afa1580156107d5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107f991906112bd565b6000805460405163c661065760e01b81526001600160801b038816600482015292935090916001600160a01b039091169063c661065790602401602060405180830381865afa158015610850573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061087491906112bd565b6040516370a0823160e01b81526001600160a01b038581166004830152919250600091600191908516906370a0823190602401602060405180830381865afa1580156108c4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108e8919061126d565b6108f29190611303565b905060006b033b2e3c9fd0803ce800000061090d878461131a565b6109179190611339565b60408051600f8b810b60248301528a900b60448201526064810185905260848082018490528251808303909101815260a49091019091526020810180516001600160e01b0316630f7c084960e21b17905290915061097c908690869086906001610d18565b50506001805550505050505050565b60405163024e46f760e41b815260040160405180910390fd5b60008060009054906101000a90046001600160a01b03166001600160a01b031663bb7b8b806040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109f8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1c919061126d565b905090565b6000805460405163c661065760e01b8152600481018490526001600160a01b039091169063c661065790602401602060405180830381865afa158015610a6b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a8f91906112bd565b92915050565b604051633a562dc160e21b81523360048201526060906000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063e958b70490602401602060405180830381865afa158015610aff573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b2391906112bd565b9050610b328187878787610d18565b9695505050505050565b604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa158015610ba3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bc791906112bd565b6040516370a0823160e01b81526001600160a01b0382811660048301529192506000916001917f0000000000000000000000000000000000000000000000000000000000000000909116906370a0823190602401602060405180830381865afa158015610c38573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c5c919061126d565b610c669190611303565b9050610d10827f000000000000000000000000000000000000000000000000000000000000000086630d2680e960e11b858a6b033b2e3c9fd0803ce8000000610caf8b8461131a565b610cb99190611339565b6040516024810193909352600f9190910b6044830152606482015260840160408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526000610d18565b505050505050565b60608115610e4057600054604051636eb1769f60e11b81526001600160a01b03888116600483015291821660248201526b1fffffffffffffffffffffff9187169063dd62ed3e90604401602060405180830381865afa158015610d7f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610da3919061126d565b1015610e40576000546040516346fb371d60e01b81523360048201526001600160a01b039182166024820152868216604482015260001960648201527f0000000000000000000000000000000000000000000000000000000000000000909116906346fb371d90608401600060405180830381600087803b158015610e2757600080fd5b505af1158015610e3b573d6000803e3d6000fd5b505050505b600080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610f4e576040516370a0823160e01b81526001600160a01b0389811660048301528816906370a0823190602401602060405180830381865afa158015610eb9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610edd919061126d565b6040516370a0823160e01b81526001600160a01b038a81166004830152919350908716906370a0823190602401602060405180830381865afa158015610f27573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f4b919061126d565b90505b60005460405163367203a560e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692636ce4074a92610fa292339216908a9060040161138b565b6000604051808303816000875af1158015610fc1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610fe991908101906113ee565b9250336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146110b9576040516332a54f6d60e11b81526001600160a01b0389811660048301528881166024830152878116604483015260648201849052608482018390527f0000000000000000000000000000000000000000000000000000000000000000169063654a9eda9060a401600060405180830381600087803b15801561109c57600080fd5b505af11580156110b0573d6000803e3d6000fd5b5050505061113d565b60405163028f1f8b60e51b81526001600160a01b03898116600483015287811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b15801561112457600080fd5b505af1158015611138573d6000803e3d6000fd5b505050505b505095945050505050565b8035600f81900b811461115a57600080fd5b919050565b60008060006060848603121561117457600080fd5b61117d84611148565b925061118b60208501611148565b9150604084013590509250925092565b6000806000606084860312156111b057600080fd5b8335925061118b60208501611148565b600080604083850312156111d357600080fd5b6111dc83611148565b946020939093013593505050565b6000806000806080858703121561120057600080fd5b61120985611148565b935061121760208601611148565b93969395505050506040820135916060013590565b60006020828403121561123e57600080fd5b5035919050565b60208101600e831061126757634e487b7160e01b600052602160045260246000fd5b91905290565b60006020828403121561127f57600080fd5b5051919050565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b6000602082840312156112cf57600080fd5b81516001600160a01b03811681146112e657600080fd5b9392505050565b634e487b7160e01b600052601160045260246000fd5b600082821015611315576113156112ed565b500390565b6000816000190483118215151615611334576113346112ed565b500290565b60008261135657634e487b7160e01b600052601260045260246000fd5b500490565b60005b8381101561137657818101518382015260200161135e565b83811115611385576000848401525b50505050565b600060018060a01b0380861683528085166020840152506060604083015282518060608401526113c281608085016020870161135b565b601f01601f191691909101608001949350505050565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561140057600080fd5b815167ffffffffffffffff8082111561141857600080fd5b818401915084601f83011261142c57600080fd5b81518181111561143e5761143e6113d8565b604051601f8201601f19908116603f01168101908382118183101715611466576114666113d8565b8160405282815287602084870101111561147f57600080fd5b61149083602083016020880161135b565b97965050505050505056fea2646970667358221220898f9e66e306aa93501e9116c0dba0dade2e40a21926bdaf370f61736dec476464736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CurveV1AdapterBase<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveV1AdapterBase<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveV1AdapterBase<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveV1AdapterBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveV1AdapterBase<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CURVEV1ADAPTERBASE_ABI.clone(), client)
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
                CURVEV1ADAPTERBASE_ABI.clone(),
                CURVEV1ADAPTERBASE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `add_all_liquidity_one_coin` (0xec026ca7) function"]
        pub fn add_all_liquidity_one_coin(
            &self,
            p0: i128,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 2, 108, 167], (p0, p1))
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
        #[doc = "Calls the contract's `exchange_all` (0x79bea664) function"]
        pub fn exchange_all(
            &self,
            i: i128,
            j: i128,
            rate_min_ray: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 190, 166, 100], (i, j, rate_min_ray))
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
        #[doc = "Calls the contract's `remove_all_liquidity_one_coin` (0x33d2ebf2) function"]
        pub fn remove_all_liquidity_one_coin(
            &self,
            i: i128,
            min_rate_ray: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 210, 235, 242], (i, min_rate_ray))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `remove_liquidity_one_coin` (0x1a4d01d2) function"]
        pub fn remove_liquidity_one_coin(
            &self,
            p0: ethers::core::types::U256,
            i: i128,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 77, 1, 210], (p0, i, p2))
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CurveV1AdapterBase<M>
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
    #[doc = "Container type for all input parameters for the `add_all_liquidity_one_coin`function with signature `add_all_liquidity_one_coin(int128,uint256)` and selector `[236, 2, 108, 167]`"]
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
        name = "add_all_liquidity_one_coin",
        abi = "add_all_liquidity_one_coin(int128,uint256)"
    )]
    pub struct AddAllLiquidityOneCoinCall(pub i128, pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `exchange_all`function with signature `exchange_all(int128,int128,uint256)` and selector `[121, 190, 166, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "exchange_all", abi = "exchange_all(int128,int128,uint256)")]
    pub struct ExchangeAllCall {
        pub i: i128,
        pub j: i128,
        pub rate_min_ray: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `remove_all_liquidity_one_coin`function with signature `remove_all_liquidity_one_coin(int128,uint256)` and selector `[51, 210, 235, 242]`"]
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
        name = "remove_all_liquidity_one_coin",
        abi = "remove_all_liquidity_one_coin(int128,uint256)"
    )]
    pub struct RemoveAllLiquidityOneCoinCall {
        pub i: i128,
        pub min_rate_ray: ethers::core::types::U256,
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
        pub p0: ethers::core::types::U256,
        pub i: i128,
        pub p2: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CurveV1AdapterBaseCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        AddAllLiquidityOneCoin(AddAllLiquidityOneCoinCall),
        Coins(CoinsCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Exchange(ExchangeCall),
        ExchangeAll(ExchangeAllCall),
        ExchangeUnderlying(ExchangeUnderlyingCall),
        GetDy(GetDyCall),
        GetDyUnderlying(GetDyUnderlyingCall),
        GetVirtualPrice(GetVirtualPriceCall),
        LpToken(LpTokenCall),
        RemoveAllLiquidityOneCoin(RemoveAllLiquidityOneCoinCall),
        RemoveLiquidityOneCoin(RemoveLiquidityOneCoinCall),
        TargetContract(TargetContractCall),
        Token(TokenCall),
    }
    impl ethers::core::abi::AbiDecode for CurveV1AdapterBaseCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <AddAllLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::AddAllLiquidityOneCoin(decoded));
            }
            if let Ok(decoded) = <CoinsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::Coins(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <ExchangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::Exchange(decoded));
            }
            if let Ok(decoded) =
                <ExchangeAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::ExchangeAll(decoded));
            }
            if let Ok(decoded) =
                <ExchangeUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::ExchangeUnderlying(decoded));
            }
            if let Ok(decoded) = <GetDyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::GetDy(decoded));
            }
            if let Ok(decoded) =
                <GetDyUnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::GetDyUnderlying(decoded));
            }
            if let Ok(decoded) =
                <GetVirtualPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::GetVirtualPrice(decoded));
            }
            if let Ok(decoded) =
                <LpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::LpToken(decoded));
            }
            if let Ok(decoded) =
                <RemoveAllLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CurveV1AdapterBaseCalls::RemoveAllLiquidityOneCoin(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityOneCoinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::RemoveLiquidityOneCoin(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::TargetContract(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterBaseCalls::Token(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveV1AdapterBaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveV1AdapterBaseCalls::GearboxAdapterType(element) => element.encode(),
                CurveV1AdapterBaseCalls::GearboxAdapterVersion(element) => element.encode(),
                CurveV1AdapterBaseCalls::AddAllLiquidityOneCoin(element) => element.encode(),
                CurveV1AdapterBaseCalls::Coins(element) => element.encode(),
                CurveV1AdapterBaseCalls::CreditFacade(element) => element.encode(),
                CurveV1AdapterBaseCalls::CreditManager(element) => element.encode(),
                CurveV1AdapterBaseCalls::Exchange(element) => element.encode(),
                CurveV1AdapterBaseCalls::ExchangeAll(element) => element.encode(),
                CurveV1AdapterBaseCalls::ExchangeUnderlying(element) => element.encode(),
                CurveV1AdapterBaseCalls::GetDy(element) => element.encode(),
                CurveV1AdapterBaseCalls::GetDyUnderlying(element) => element.encode(),
                CurveV1AdapterBaseCalls::GetVirtualPrice(element) => element.encode(),
                CurveV1AdapterBaseCalls::LpToken(element) => element.encode(),
                CurveV1AdapterBaseCalls::RemoveAllLiquidityOneCoin(element) => element.encode(),
                CurveV1AdapterBaseCalls::RemoveLiquidityOneCoin(element) => element.encode(),
                CurveV1AdapterBaseCalls::TargetContract(element) => element.encode(),
                CurveV1AdapterBaseCalls::Token(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveV1AdapterBaseCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveV1AdapterBaseCalls::GearboxAdapterType(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::GearboxAdapterVersion(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::AddAllLiquidityOneCoin(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::Coins(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::CreditFacade(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::CreditManager(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::Exchange(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::ExchangeAll(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::ExchangeUnderlying(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::GetDy(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::GetDyUnderlying(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::GetVirtualPrice(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::LpToken(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::RemoveAllLiquidityOneCoin(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::RemoveLiquidityOneCoin(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::TargetContract(element) => element.fmt(f),
                CurveV1AdapterBaseCalls::Token(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for CurveV1AdapterBaseCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            CurveV1AdapterBaseCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for CurveV1AdapterBaseCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            CurveV1AdapterBaseCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<AddAllLiquidityOneCoinCall> for CurveV1AdapterBaseCalls {
        fn from(var: AddAllLiquidityOneCoinCall) -> Self {
            CurveV1AdapterBaseCalls::AddAllLiquidityOneCoin(var)
        }
    }
    impl ::std::convert::From<CoinsCall> for CurveV1AdapterBaseCalls {
        fn from(var: CoinsCall) -> Self {
            CurveV1AdapterBaseCalls::Coins(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for CurveV1AdapterBaseCalls {
        fn from(var: CreditFacadeCall) -> Self {
            CurveV1AdapterBaseCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for CurveV1AdapterBaseCalls {
        fn from(var: CreditManagerCall) -> Self {
            CurveV1AdapterBaseCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<ExchangeCall> for CurveV1AdapterBaseCalls {
        fn from(var: ExchangeCall) -> Self {
            CurveV1AdapterBaseCalls::Exchange(var)
        }
    }
    impl ::std::convert::From<ExchangeAllCall> for CurveV1AdapterBaseCalls {
        fn from(var: ExchangeAllCall) -> Self {
            CurveV1AdapterBaseCalls::ExchangeAll(var)
        }
    }
    impl ::std::convert::From<ExchangeUnderlyingCall> for CurveV1AdapterBaseCalls {
        fn from(var: ExchangeUnderlyingCall) -> Self {
            CurveV1AdapterBaseCalls::ExchangeUnderlying(var)
        }
    }
    impl ::std::convert::From<GetDyCall> for CurveV1AdapterBaseCalls {
        fn from(var: GetDyCall) -> Self {
            CurveV1AdapterBaseCalls::GetDy(var)
        }
    }
    impl ::std::convert::From<GetDyUnderlyingCall> for CurveV1AdapterBaseCalls {
        fn from(var: GetDyUnderlyingCall) -> Self {
            CurveV1AdapterBaseCalls::GetDyUnderlying(var)
        }
    }
    impl ::std::convert::From<GetVirtualPriceCall> for CurveV1AdapterBaseCalls {
        fn from(var: GetVirtualPriceCall) -> Self {
            CurveV1AdapterBaseCalls::GetVirtualPrice(var)
        }
    }
    impl ::std::convert::From<LpTokenCall> for CurveV1AdapterBaseCalls {
        fn from(var: LpTokenCall) -> Self {
            CurveV1AdapterBaseCalls::LpToken(var)
        }
    }
    impl ::std::convert::From<RemoveAllLiquidityOneCoinCall> for CurveV1AdapterBaseCalls {
        fn from(var: RemoveAllLiquidityOneCoinCall) -> Self {
            CurveV1AdapterBaseCalls::RemoveAllLiquidityOneCoin(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityOneCoinCall> for CurveV1AdapterBaseCalls {
        fn from(var: RemoveLiquidityOneCoinCall) -> Self {
            CurveV1AdapterBaseCalls::RemoveLiquidityOneCoin(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for CurveV1AdapterBaseCalls {
        fn from(var: TargetContractCall) -> Self {
            CurveV1AdapterBaseCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<TokenCall> for CurveV1AdapterBaseCalls {
        fn from(var: TokenCall) -> Self {
            CurveV1AdapterBaseCalls::Token(var)
        }
    }
}
