pub use yearnpricefeed_mod::*;
#[allow(clippy::too_many_arguments)]
mod yearnpricefeed_mod {
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
    #[doc = "YearnPriceFeed was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static YEARNPRICEFEED_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressProvider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_yVault\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_priceFeed\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_lowerBound\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_upperBound\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"IncorrectLimitsException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PricePerShareOutOfRangeExpcetion\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"lowerBound\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"upperBound\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewLimiterParams\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimalsDivider\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lowerBound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeed\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_lowerBound\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_upperBound\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLimiter\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"upperBound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"yVault\",\"outputs\":[{\"internalType\":\"contract IYVault\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static YEARNPRICEFEED_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101006040523480156200001257600080fd5b506040516200116138038062001161833981016040819052620000359162000267565b6000805460ff1916905560408051808201909152600281526105a360f41b602082015285906001600160a01b0382166200008d5760405162461bcd60e51b8152600401620000849190620002c4565b60405180910390fd5b50806001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000cd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000f391906200031c565b6001600160a01b039081166080528516159050806200011957506001600160a01b038316155b156200013857604051635919af9760e11b815260040160405180910390fd5b6001600160a01b0380851660c081905290841660a0526040805163313ce56760e01b8152905163313ce567916004808201926020929091908290030181865afa1580156200018a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001b0919062000341565b620001bd90600a6200047b565b60e052620001cc8282620001d7565b50505050506200048c565b811580620001e457508181105b1562000203576040516316b03c1d60e31b815260040160405180910390fd5b6001829055600281905560408051838152602081018390527f82e7ee47180a631312683eeb2a85ad264c9af490d54de5a75bbdb95b968c6de2910160405180910390a15050565b80516001600160a01b03811681146200026257600080fd5b919050565b600080600080600060a086880312156200028057600080fd5b6200028b866200024a565b94506200029b602087016200024a565b9350620002ab604087016200024a565b6060870151608090970151959894975095949392505050565b600060208083528351808285015260005b81811015620002f357858101830151858201604001528201620002d5565b8181111562000306576000604083870101525b50601f01601f1916929092016040019392505050565b6000602082840312156200032f57600080fd5b6200033a826200024a565b9392505050565b6000602082840312156200035457600080fd5b815160ff811681146200033a57600080fd5b634e487b7160e01b600052601160045260246000fd5b600181815b80851115620003bd578160001904821115620003a157620003a162000366565b80851615620003af57918102915b93841c939080029062000381565b509250929050565b600082620003d65750600162000475565b81620003e55750600062000475565b8160018114620003fe5760028114620004095762000429565b600191505062000475565b60ff8411156200041d576200041d62000366565b50506001821b62000475565b5060208310610133831016604e8410600b84101617156200044e575081810a62000475565b6200045a83836200037c565b806000190482111562000471576200047162000366565b0290505b92915050565b60006200033a60ff841683620003c5565b60805160a05160c05160e051610c63620004fe6000396000818161023201526107e501526000818161012801526107320152600081816101b0015281816103440152818161049b0152818161051f015261069a01526000818161027a015281816103de01526105b80152610c636000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c8063741bef1a1161008c578063a384d6ff11610066578063a384d6ff14610224578063a834559e1461022d578063b09ad8a014610254578063feaf968c1461025d57600080fd5b8063741bef1a146101ab5780638456cb59146101d25780639a6fc8f5146101da57600080fd5b80633f4ba83a116100c85780633f4ba83a1461016257806354fd4d501461016a5780635c975abb146101805780637284e4161461019657600080fd5b80630bdea781146100ef578063313ce5671461010457806333303f8e14610123575b600080fd5b6101026100fd3660046109a1565b610265565b005b61010c610340565b60405160ff90911681526020015b60405180910390f35b61014a7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161011a565b6101026103c9565b610172610497565b60405190815260200161011a565b60005460ff16604051901515815260200161011a565b61019e61051b565b60405161011a91906109f3565b61014a7f000000000000000000000000000000000000000000000000000000000000000081565b6101026105a3565b6101ed6101e8366004610a41565b61066f565b6040805169ffffffffffffffffffff968716815260208101959095528401929092526060830152909116608082015260a00161011a565b61017260015481565b6101727f000000000000000000000000000000000000000000000000000000000000000081565b61017260025481565b6101ed610690565b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa1580156102c9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102ed9190610a65565b6040518060400160405280600481526020016320a1a61960e11b815250906103315760405162461bcd60e51b815260040161032891906109f3565b60405180910390fd5b5061033c8282610822565b5050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156103a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103c49190610a87565b905090565b604051630d4eb5db60e41b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d4eb5db090602401602060405180830381865afa15801561042d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104519190610a65565b6040518060400160405280600481526020016341434c3160e01b8152509061048c5760405162461bcd60e51b815260040161032891906109f3565b50610495610893565b565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166354fd4d506040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104f7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103c49190610aaa565b60607f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637284e4166040518163ffffffff1660e01b8152600401600060405180830381865afa15801561057b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526103c49190810190610ad9565b604051630e907b1960e21b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633a41ec6490602401602060405180830381865afa158015610607573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061062b9190610a65565b6040518060400160405280600481526020016341434c3160e01b815250906106665760405162461bcd60e51b815260040161032891906109f3565b50610495610926565b600080600080600060405163024e46f760e41b815260040160405180910390fd5b60008060008060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa1580156106f6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061071a9190610b86565b809550819650829750839850849950505050505060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166399530b066040518163ffffffff1660e01b8152600401602060405180830381865afa15801561078e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107b29190610aaa565b90506001548110806107c5575060025481115b156107e3576040516316b03c1d60e31b815260040160405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000061080e8683610bde565b6108189190610c0b565b9450509091929394565b81158061082e57508181105b1561084c576040516316b03c1d60e31b815260040160405180910390fd5b6001829055600281905560408051838152602081018390527f82e7ee47180a631312683eeb2a85ad264c9af490d54de5a75bbdb95b968c6de2910160405180910390a15050565b60005460ff166108dc5760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b6044820152606401610328565b6000805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b60005460ff161561096c5760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b6044820152606401610328565b6000805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586109093390565b600080604083850312156109b457600080fd5b50508035926020909101359150565b60005b838110156109de5781810151838201526020016109c6565b838111156109ed576000848401525b50505050565b6020815260008251806020840152610a128160408501602087016109c3565b601f01601f19169190910160400192915050565b69ffffffffffffffffffff81168114610a3e57600080fd5b50565b600060208284031215610a5357600080fd5b8135610a5e81610a26565b9392505050565b600060208284031215610a7757600080fd5b81518015158114610a5e57600080fd5b600060208284031215610a9957600080fd5b815160ff81168114610a5e57600080fd5b600060208284031215610abc57600080fd5b5051919050565b634e487b7160e01b600052604160045260246000fd5b600060208284031215610aeb57600080fd5b815167ffffffffffffffff80821115610b0357600080fd5b818401915084601f830112610b1757600080fd5b815181811115610b2957610b29610ac3565b604051601f8201601f19908116603f01168101908382118183101715610b5157610b51610ac3565b81604052828152876020848701011115610b6a57600080fd5b610b7b8360208301602088016109c3565b979650505050505050565b600080600080600060a08688031215610b9e57600080fd5b8551610ba981610a26565b809550506020860151935060408601519250606086015191506080860151610bd081610a26565b809150509295509295909350565b6000816000190483118215151615610c0657634e487b7160e01b600052601160045260246000fd5b500290565b600082610c2857634e487b7160e01b600052601260045260246000fd5b50049056fea264697066735822122060b4866887a839282a3fbe316780ee8f10678c55f0332ce23347eba8416998dc64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct YearnPriceFeed<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for YearnPriceFeed<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for YearnPriceFeed<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(YearnPriceFeed))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> YearnPriceFeed<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), YEARNPRICEFEED_ABI.clone(), client)
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
                YEARNPRICEFEED_ABI.clone(),
                YEARNPRICEFEED_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimalsDivider` (0xa834559e) function"]
        pub fn decimals_divider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 52, 85, 158], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `description` (0x7284e416) function"]
        pub fn description(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundData` (0x9a6fc8f5) function"]
        pub fn get_round_data(
            &self,
            p0: u128,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundData` (0xfeaf968c) function"]
        pub fn latest_round_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lowerBound` (0xa384d6ff) function"]
        pub fn lower_bound(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([163, 132, 214, 255], ())
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
        #[doc = "Calls the contract's `priceFeed` (0x741bef1a) function"]
        pub fn price_feed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([116, 27, 239, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLimiter` (0x0bdea781) function"]
        pub fn set_limiter(
            &self,
            lower_bound: ethers::core::types::U256,
            upper_bound: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 222, 167, 129], (lower_bound, upper_bound))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upperBound` (0xb09ad8a0) function"]
        pub fn upper_bound(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([176, 154, 216, 160], ())
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
        #[doc = "Calls the contract's `yVault` (0x33303f8e) function"]
        pub fn y_vault(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([51, 48, 63, 142], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewLimiterParams` event"]
        pub fn new_limiter_params_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewLimiterParamsFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, YearnPriceFeedEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for YearnPriceFeed<M> {
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
    #[ethevent(name = "NewLimiterParams", abi = "NewLimiterParams(uint256,uint256)")]
    pub struct NewLimiterParamsFilter {
        pub lower_bound: ethers::core::types::U256,
        pub upper_bound: ethers::core::types::U256,
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
    pub enum YearnPriceFeedEvents {
        NewLimiterParamsFilter(NewLimiterParamsFilter),
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for YearnPriceFeedEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewLimiterParamsFilter::decode_log(log) {
                return Ok(YearnPriceFeedEvents::NewLimiterParamsFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(YearnPriceFeedEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(YearnPriceFeedEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for YearnPriceFeedEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                YearnPriceFeedEvents::NewLimiterParamsFilter(element) => element.fmt(f),
                YearnPriceFeedEvents::PausedFilter(element) => element.fmt(f),
                YearnPriceFeedEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `decimalsDivider`function with signature `decimalsDivider()` and selector `[168, 52, 85, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimalsDivider", abi = "decimalsDivider()")]
    pub struct DecimalsDividerCall;
    #[doc = "Container type for all input parameters for the `description`function with signature `description()` and selector `[114, 132, 228, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    #[doc = "Container type for all input parameters for the `getRoundData`function with signature `getRoundData(uint80)` and selector `[154, 111, 200, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall(pub u128);
    #[doc = "Container type for all input parameters for the `latestRoundData`function with signature `latestRoundData()` and selector `[254, 175, 150, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    #[doc = "Container type for all input parameters for the `lowerBound`function with signature `lowerBound()` and selector `[163, 132, 214, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lowerBound", abi = "lowerBound()")]
    pub struct LowerBoundCall;
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
    #[doc = "Container type for all input parameters for the `priceFeed`function with signature `priceFeed()` and selector `[116, 27, 239, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeed", abi = "priceFeed()")]
    pub struct PriceFeedCall;
    #[doc = "Container type for all input parameters for the `setLimiter`function with signature `setLimiter(uint256,uint256)` and selector `[11, 222, 167, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setLimiter", abi = "setLimiter(uint256,uint256)")]
    pub struct SetLimiterCall {
        pub lower_bound: ethers::core::types::U256,
        pub upper_bound: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `upperBound`function with signature `upperBound()` and selector `[176, 154, 216, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "upperBound", abi = "upperBound()")]
    pub struct UpperBoundCall;
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
    #[doc = "Container type for all input parameters for the `yVault`function with signature `yVault()` and selector `[51, 48, 63, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "yVault", abi = "yVault()")]
    pub struct YvaultCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum YearnPriceFeedCalls {
        Decimals(DecimalsCall),
        DecimalsDivider(DecimalsDividerCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        LowerBound(LowerBoundCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PriceFeed(PriceFeedCall),
        SetLimiter(SetLimiterCall),
        Unpause(UnpauseCall),
        UpperBound(UpperBoundCall),
        Version(VersionCall),
        Yvault(YvaultCall),
    }
    impl ethers::core::abi::AbiDecode for YearnPriceFeedCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecimalsDividerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::DecimalsDivider(decoded));
            }
            if let Ok(decoded) =
                <DescriptionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Description(decoded));
            }
            if let Ok(decoded) =
                <GetRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::GetRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <LowerBoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::LowerBound(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <PriceFeedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::PriceFeed(decoded));
            }
            if let Ok(decoded) =
                <SetLimiterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::SetLimiter(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpperBoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::UpperBound(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Version(decoded));
            }
            if let Ok(decoded) = <YvaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(YearnPriceFeedCalls::Yvault(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for YearnPriceFeedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                YearnPriceFeedCalls::Decimals(element) => element.encode(),
                YearnPriceFeedCalls::DecimalsDivider(element) => element.encode(),
                YearnPriceFeedCalls::Description(element) => element.encode(),
                YearnPriceFeedCalls::GetRoundData(element) => element.encode(),
                YearnPriceFeedCalls::LatestRoundData(element) => element.encode(),
                YearnPriceFeedCalls::LowerBound(element) => element.encode(),
                YearnPriceFeedCalls::Pause(element) => element.encode(),
                YearnPriceFeedCalls::Paused(element) => element.encode(),
                YearnPriceFeedCalls::PriceFeed(element) => element.encode(),
                YearnPriceFeedCalls::SetLimiter(element) => element.encode(),
                YearnPriceFeedCalls::Unpause(element) => element.encode(),
                YearnPriceFeedCalls::UpperBound(element) => element.encode(),
                YearnPriceFeedCalls::Version(element) => element.encode(),
                YearnPriceFeedCalls::Yvault(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for YearnPriceFeedCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                YearnPriceFeedCalls::Decimals(element) => element.fmt(f),
                YearnPriceFeedCalls::DecimalsDivider(element) => element.fmt(f),
                YearnPriceFeedCalls::Description(element) => element.fmt(f),
                YearnPriceFeedCalls::GetRoundData(element) => element.fmt(f),
                YearnPriceFeedCalls::LatestRoundData(element) => element.fmt(f),
                YearnPriceFeedCalls::LowerBound(element) => element.fmt(f),
                YearnPriceFeedCalls::Pause(element) => element.fmt(f),
                YearnPriceFeedCalls::Paused(element) => element.fmt(f),
                YearnPriceFeedCalls::PriceFeed(element) => element.fmt(f),
                YearnPriceFeedCalls::SetLimiter(element) => element.fmt(f),
                YearnPriceFeedCalls::Unpause(element) => element.fmt(f),
                YearnPriceFeedCalls::UpperBound(element) => element.fmt(f),
                YearnPriceFeedCalls::Version(element) => element.fmt(f),
                YearnPriceFeedCalls::Yvault(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DecimalsCall> for YearnPriceFeedCalls {
        fn from(var: DecimalsCall) -> Self {
            YearnPriceFeedCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecimalsDividerCall> for YearnPriceFeedCalls {
        fn from(var: DecimalsDividerCall) -> Self {
            YearnPriceFeedCalls::DecimalsDivider(var)
        }
    }
    impl ::std::convert::From<DescriptionCall> for YearnPriceFeedCalls {
        fn from(var: DescriptionCall) -> Self {
            YearnPriceFeedCalls::Description(var)
        }
    }
    impl ::std::convert::From<GetRoundDataCall> for YearnPriceFeedCalls {
        fn from(var: GetRoundDataCall) -> Self {
            YearnPriceFeedCalls::GetRoundData(var)
        }
    }
    impl ::std::convert::From<LatestRoundDataCall> for YearnPriceFeedCalls {
        fn from(var: LatestRoundDataCall) -> Self {
            YearnPriceFeedCalls::LatestRoundData(var)
        }
    }
    impl ::std::convert::From<LowerBoundCall> for YearnPriceFeedCalls {
        fn from(var: LowerBoundCall) -> Self {
            YearnPriceFeedCalls::LowerBound(var)
        }
    }
    impl ::std::convert::From<PauseCall> for YearnPriceFeedCalls {
        fn from(var: PauseCall) -> Self {
            YearnPriceFeedCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for YearnPriceFeedCalls {
        fn from(var: PausedCall) -> Self {
            YearnPriceFeedCalls::Paused(var)
        }
    }
    impl ::std::convert::From<PriceFeedCall> for YearnPriceFeedCalls {
        fn from(var: PriceFeedCall) -> Self {
            YearnPriceFeedCalls::PriceFeed(var)
        }
    }
    impl ::std::convert::From<SetLimiterCall> for YearnPriceFeedCalls {
        fn from(var: SetLimiterCall) -> Self {
            YearnPriceFeedCalls::SetLimiter(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for YearnPriceFeedCalls {
        fn from(var: UnpauseCall) -> Self {
            YearnPriceFeedCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UpperBoundCall> for YearnPriceFeedCalls {
        fn from(var: UpperBoundCall) -> Self {
            YearnPriceFeedCalls::UpperBound(var)
        }
    }
    impl ::std::convert::From<VersionCall> for YearnPriceFeedCalls {
        fn from(var: VersionCall) -> Self {
            YearnPriceFeedCalls::Version(var)
        }
    }
    impl ::std::convert::From<YvaultCall> for YearnPriceFeedCalls {
        fn from(var: YvaultCall) -> Self {
            YearnPriceFeedCalls::Yvault(var)
        }
    }
}
