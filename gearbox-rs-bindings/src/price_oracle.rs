pub use priceoracle_mod::*;
#[allow(clippy::too_many_arguments)]
mod priceoracle_mod {
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
    #[doc = "PriceOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PRICEORACLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressProvider\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct PriceFeedConfig[]\",\"name\":\"defaults\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"priceFeed\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PriceFeedDecimalsNotEqual8Exception\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PriceOracleNotExistsException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TokenDecimalsGreater18ForbiddenException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroPriceException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"priceFeed\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewPriceFeed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"priceFeed\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPriceFeed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenFrom\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenTo\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convert\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertFromUSD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToUSD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountFrom\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenFrom\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountTo\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenTo\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fastCheck\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"collateralFrom\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"collateralTo\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceFeeds\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PRICEORACLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a06040523480156200001157600080fd5b5060405162001328380380620013288339810160408190526200003491620003e2565b6000805460ff1916905560408051808201909152600281526105a360f41b602082015282906001600160a01b0382166200008c5760405162461bcd60e51b8152600401620000839190620004e4565b60405180910390fd5b50806001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000cc573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000f291906200053c565b6001600160a01b031660805250805160005b8181101562000168576200015f83828151811062000126576200012662000561565b60200260200101516000015184838151811062000147576200014762000561565b6020026020010151602001516200017260201b60201c565b60010162000104565b50505050620006bf565b6001600160a01b03821615806200019057506001600160a01b038116155b15620001af57604051635919af9760e11b815260040160405180910390fd5b6000826001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015620001f0573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000216919062000577565b60ff16905060128111156200023e57604051630e7dca9b60e31b815260040160405180910390fd5b816001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200027d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620002a3919062000577565b60ff16600814620002c75760405163335b6d1760e01b815260040160405180910390fd5b6001600160a01b03838116600090815260016020526040902080546001600160a01b0319169184169190911790556200030281600a620006b1565b6001600160a01b038085166000818152600260205260408082209490945592519185169290917fe263805b03657ab13064915d0723c5ce14981547e7cba5283f66b9e5d81f6e6e9190a3505050565b80516001600160a01b03811681146200036957600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715620003a957620003a96200036e565b60405290565b604051601f8201601f191681016001600160401b0381118282101715620003da57620003da6200036e565b604052919050565b6000806040808486031215620003f757600080fd5b620004028462000351565b602085810151919450906001600160401b03808211156200042257600080fd5b818701915087601f8301126200043757600080fd5b8151818111156200044c576200044c6200036e565b6200045c848260051b01620003af565b818152848101925060069190911b8301840190898211156200047d57600080fd5b928401925b81841015620004d45785848b0312156200049c5760008081fd5b620004a662000384565b620004b18562000351565b8152620004c086860162000351565b818701528352928501929184019162000482565b8096505050505050509250929050565b600060208083528351808285015260005b818110156200051357858101830151858201604001528201620004f5565b8181111562000526576000604083870101525b50601f01601f1916929092016040019392505050565b6000602082840312156200054f57600080fd5b6200055a8262000351565b9392505050565b634e487b7160e01b600052603260045260246000fd5b6000602082840312156200058a57600080fd5b815160ff811681146200055a57600080fd5b634e487b7160e01b600052601160045260246000fd5b600181815b80851115620005f3578160001904821115620005d757620005d76200059c565b80851615620005e557918102915b93841c9390800290620005b7565b509250929050565b6000826200060c57506001620006ab565b816200061b57506000620006ab565b81600181146200063457600281146200063f576200065f565b6001915050620006ab565b60ff8411156200065357620006536200059c565b50506001821b620006ab565b5060208310610133831016604e8410600b841016171562000684575081810a620006ab565b620006908383620005b2565b8060001904821115620006a757620006a76200059c565b0290505b92915050565b60006200055a8383620005fb565b608051610c3f620006e9600039600081816101db0152818161031701526104000152610c3f6000f3fe608060405234801561001057600080fd5b50600436106100a95760003560e01c80638456cb59116100715780638456cb59146101245780639dcb511a1461012c578063b66102df1461016d578063d449a83214610180578063e8a97a3e146101a0578063f9a65030146101b357600080fd5b80633f4ba83a146100ae57806354fd4d50146100b85780635c975abb146100d35780635cecbd0e146100e95780637afb010414610111575b600080fd5b6100b66101c6565b005b6100c0600281565b6040519081526020015b60405180910390f35b60005460ff1660405190151581526020016100ca565b6100fc6100f73660046108c4565b61029d565b604080519283526020830191909152016100ca565b6100c061011f36600461090a565b6102c1565b6100b6610302565b61015561013a366004610936565b6001602052600090815260409020546001600160a01b031681565b6040516001600160a01b0390911681526020016100ca565b6100c061017b366004610951565b6103ce565b6100c061018e366004610936565b60026020526000908152604090205481565b6100b66101ae36600461098d565b6103eb565b6100c06101c136600461090a565b6104bd565b604051630d4eb5db60e41b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d4eb5db090602401602060405180830381865afa15801561022a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061024e91906109b7565b6040518060400160405280600481526020016341434c3160e01b815250906102925760405162461bcd60e51b815260040161028991906109e0565b60405180910390fd5b5061029b6104e9565b565b6000806102aa86866104bd565b91506102b684846104bd565b905094509492505050565b60006102cc8261057c565b6001600160a01b0383166000908152600260205260409020546102ef9085610a4b565b6102f99190610a6a565b90505b92915050565b604051630e907b1960e21b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633a41ec6490602401602060405180830381865afa158015610366573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061038a91906109b7565b6040518060400160405280600481526020016341434c3160e01b815250906103c55760405162461bcd60e51b815260040161028991906109e0565b5061029b61065a565b60006103e36103dd85856104bd565b836102c1565b949350505050565b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa15801561044f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061047391906109b7565b6040518060400160405280600481526020016320a1a61960e11b815250906104ae5760405162461bcd60e51b815260040161028991906109e0565b506104b982826106d5565b5050565b6001600160a01b0381166000908152600260205260408120546104df8361057c565b6102ef9085610a4b565b60005460ff166105325760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b6044820152606401610289565b6000805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6001600160a01b038181166000908152600160205260408120549091166105b6576040516325dc56c160e11b815260040160405180910390fd5b6001600160a01b03808316600090815260016020526040808220548151633fabe5a360e21b815291519293169163feaf968c9160048082019260a0929091908290030181865afa15801561060e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106329190610aa6565b50505091505080600014156102fc576040516356e05d2b60e01b815260040160405180910390fd5b60005460ff16156106a05760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b6044820152606401610289565b6000805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25861055f3390565b6001600160a01b03821615806106f257506001600160a01b038116155b1561071057604051635919af9760e11b815260040160405180910390fd5b6000826001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015610750573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107749190610af6565b60ff169050601281111561079b57604051630e7dca9b60e31b815260040160405180910390fd5b816001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156107d9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107fd9190610af6565b60ff166008146108205760405163335b6d1760e01b815260040160405180910390fd5b6001600160a01b03838116600090815260016020526040902080546001600160a01b03191691841691909117905561085981600a610bfd565b6001600160a01b038085166000818152600260205260408082209490945592519185169290917fe263805b03657ab13064915d0723c5ce14981547e7cba5283f66b9e5d81f6e6e9190a3505050565b80356001600160a01b03811681146108bf57600080fd5b919050565b600080600080608085870312156108da57600080fd5b843593506108ea602086016108a8565b9250604085013591506108ff606086016108a8565b905092959194509250565b6000806040838503121561091d57600080fd5b8235915061092d602084016108a8565b90509250929050565b60006020828403121561094857600080fd5b6102f9826108a8565b60008060006060848603121561096657600080fd5b83359250610976602085016108a8565b9150610984604085016108a8565b90509250925092565b600080604083850312156109a057600080fd5b6109a9836108a8565b915061092d602084016108a8565b6000602082840312156109c957600080fd5b815180151581146109d957600080fd5b9392505050565b600060208083528351808285015260005b81811015610a0d578581018301518582016040015282016109f1565b81811115610a1f576000604083870101525b50601f01601f1916929092016040019392505050565b634e487b7160e01b600052601160045260246000fd5b6000816000190483118215151615610a6557610a65610a35565b500290565b600082610a8757634e487b7160e01b600052601260045260246000fd5b500490565b805169ffffffffffffffffffff811681146108bf57600080fd5b600080600080600060a08688031215610abe57600080fd5b610ac786610a8c565b9450602086015193506040860151925060608601519150610aea60808701610a8c565b90509295509295909350565b600060208284031215610b0857600080fd5b815160ff811681146109d957600080fd5b600181815b80851115610b54578160001904821115610b3a57610b3a610a35565b80851615610b4757918102915b93841c9390800290610b1e565b509250929050565b600082610b6b575060016102fc565b81610b78575060006102fc565b8160018114610b8e5760028114610b9857610bb4565b60019150506102fc565b60ff841115610ba957610ba9610a35565b50506001821b6102fc565b5060208310610133831016604e8410600b8410161715610bd7575081810a6102fc565b610be18383610b19565b8060001904821115610bf557610bf5610a35565b029392505050565b60006102f98383610b5c56fea2646970667358221220aa24eae2c6ffa804a894f57cb90e0335cf4c799401c3f654442d9717abe0ff9f64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct PriceOracle<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PriceOracle<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PriceOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PriceOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PRICEORACLE_ABI.clone(), client).into()
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
                PRICEORACLE_ABI.clone(),
                PRICEORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addPriceFeed` (0xe8a97a3e) function"]
        pub fn add_price_feed(
            &self,
            token: ethers::core::types::Address,
            price_feed: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 169, 122, 62], (token, price_feed))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convert` (0xb66102df) function"]
        pub fn convert(
            &self,
            amount: ethers::core::types::U256,
            token_from: ethers::core::types::Address,
            token_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 97, 2, 223], (amount, token_from, token_to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertFromUSD` (0x7afb0104) function"]
        pub fn convert_from_usd(
            &self,
            amount: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 251, 1, 4], (amount, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertToUSD` (0xf9a65030) function"]
        pub fn convert_to_usd(
            &self,
            amount: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([249, 166, 80, 48], (amount, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0xd449a832) function"]
        pub fn decimals(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([212, 73, 168, 50], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fastCheck` (0x5cecbd0e) function"]
        pub fn fast_check(
            &self,
            amount_from: ethers::core::types::U256,
            token_from: ethers::core::types::Address,
            amount_to: ethers::core::types::U256,
            token_to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [92, 236, 189, 14],
                    (amount_from, token_from, amount_to, token_to),
                )
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
        #[doc = "Calls the contract's `priceFeeds` (0x9dcb511a) function"]
        pub fn price_feeds(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([157, 203, 81, 26], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
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
        #[doc = "Gets the contract's `NewPriceFeed` event"]
        pub fn new_price_feed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPriceFeedFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, PriceOracleEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PriceOracle<M> {
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
    #[ethevent(name = "NewPriceFeed", abi = "NewPriceFeed(address,address)")]
    pub struct NewPriceFeedFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub price_feed: ethers::core::types::Address,
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
    pub enum PriceOracleEvents {
        NewPriceFeedFilter(NewPriceFeedFilter),
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for PriceOracleEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewPriceFeedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::NewPriceFeedFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(PriceOracleEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PriceOracleEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleEvents::NewPriceFeedFilter(element) => element.fmt(f),
                PriceOracleEvents::PausedFilter(element) => element.fmt(f),
                PriceOracleEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addPriceFeed`function with signature `addPriceFeed(address,address)` and selector `[232, 169, 122, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addPriceFeed", abi = "addPriceFeed(address,address)")]
    pub struct AddPriceFeedCall {
        pub token: ethers::core::types::Address,
        pub price_feed: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convert`function with signature `convert(uint256,address,address)` and selector `[182, 97, 2, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "convert", abi = "convert(uint256,address,address)")]
    pub struct ConvertCall {
        pub amount: ethers::core::types::U256,
        pub token_from: ethers::core::types::Address,
        pub token_to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convertFromUSD`function with signature `convertFromUSD(uint256,address)` and selector `[122, 251, 1, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "convertFromUSD", abi = "convertFromUSD(uint256,address)")]
    pub struct ConvertFromUSDCall {
        pub amount: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `convertToUSD`function with signature `convertToUSD(uint256,address)` and selector `[249, 166, 80, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "convertToUSD", abi = "convertToUSD(uint256,address)")]
    pub struct ConvertToUSDCall {
        pub amount: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals(address)` and selector `[212, 73, 168, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "decimals", abi = "decimals(address)")]
    pub struct DecimalsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `fastCheck`function with signature `fastCheck(uint256,address,uint256,address)` and selector `[92, 236, 189, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fastCheck", abi = "fastCheck(uint256,address,uint256,address)")]
    pub struct FastCheckCall {
        pub amount_from: ethers::core::types::U256,
        pub token_from: ethers::core::types::Address,
        pub amount_to: ethers::core::types::U256,
        pub token_to: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `priceFeeds`function with signature `priceFeeds(address)` and selector `[157, 203, 81, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceFeeds", abi = "priceFeeds(address)")]
    pub struct PriceFeedsCall(pub ethers::core::types::Address);
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PriceOracleCalls {
        AddPriceFeed(AddPriceFeedCall),
        Convert(ConvertCall),
        ConvertFromUSD(ConvertFromUSDCall),
        ConvertToUSD(ConvertToUSDCall),
        Decimals(DecimalsCall),
        FastCheck(FastCheckCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PriceFeeds(PriceFeedsCall),
        Unpause(UnpauseCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for PriceOracleCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddPriceFeedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::AddPriceFeed(decoded));
            }
            if let Ok(decoded) =
                <ConvertCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::Convert(decoded));
            }
            if let Ok(decoded) =
                <ConvertFromUSDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::ConvertFromUSD(decoded));
            }
            if let Ok(decoded) =
                <ConvertToUSDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::ConvertToUSD(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <FastCheckCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::FastCheck(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <PriceFeedsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::PriceFeeds(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PriceOracleCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PriceOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PriceOracleCalls::AddPriceFeed(element) => element.encode(),
                PriceOracleCalls::Convert(element) => element.encode(),
                PriceOracleCalls::ConvertFromUSD(element) => element.encode(),
                PriceOracleCalls::ConvertToUSD(element) => element.encode(),
                PriceOracleCalls::Decimals(element) => element.encode(),
                PriceOracleCalls::FastCheck(element) => element.encode(),
                PriceOracleCalls::Pause(element) => element.encode(),
                PriceOracleCalls::Paused(element) => element.encode(),
                PriceOracleCalls::PriceFeeds(element) => element.encode(),
                PriceOracleCalls::Unpause(element) => element.encode(),
                PriceOracleCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PriceOracleCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PriceOracleCalls::AddPriceFeed(element) => element.fmt(f),
                PriceOracleCalls::Convert(element) => element.fmt(f),
                PriceOracleCalls::ConvertFromUSD(element) => element.fmt(f),
                PriceOracleCalls::ConvertToUSD(element) => element.fmt(f),
                PriceOracleCalls::Decimals(element) => element.fmt(f),
                PriceOracleCalls::FastCheck(element) => element.fmt(f),
                PriceOracleCalls::Pause(element) => element.fmt(f),
                PriceOracleCalls::Paused(element) => element.fmt(f),
                PriceOracleCalls::PriceFeeds(element) => element.fmt(f),
                PriceOracleCalls::Unpause(element) => element.fmt(f),
                PriceOracleCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddPriceFeedCall> for PriceOracleCalls {
        fn from(var: AddPriceFeedCall) -> Self {
            PriceOracleCalls::AddPriceFeed(var)
        }
    }
    impl ::std::convert::From<ConvertCall> for PriceOracleCalls {
        fn from(var: ConvertCall) -> Self {
            PriceOracleCalls::Convert(var)
        }
    }
    impl ::std::convert::From<ConvertFromUSDCall> for PriceOracleCalls {
        fn from(var: ConvertFromUSDCall) -> Self {
            PriceOracleCalls::ConvertFromUSD(var)
        }
    }
    impl ::std::convert::From<ConvertToUSDCall> for PriceOracleCalls {
        fn from(var: ConvertToUSDCall) -> Self {
            PriceOracleCalls::ConvertToUSD(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for PriceOracleCalls {
        fn from(var: DecimalsCall) -> Self {
            PriceOracleCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<FastCheckCall> for PriceOracleCalls {
        fn from(var: FastCheckCall) -> Self {
            PriceOracleCalls::FastCheck(var)
        }
    }
    impl ::std::convert::From<PauseCall> for PriceOracleCalls {
        fn from(var: PauseCall) -> Self {
            PriceOracleCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for PriceOracleCalls {
        fn from(var: PausedCall) -> Self {
            PriceOracleCalls::Paused(var)
        }
    }
    impl ::std::convert::From<PriceFeedsCall> for PriceOracleCalls {
        fn from(var: PriceFeedsCall) -> Self {
            PriceOracleCalls::PriceFeeds(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for PriceOracleCalls {
        fn from(var: UnpauseCall) -> Self {
            PriceOracleCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<VersionCall> for PriceOracleCalls {
        fn from(var: VersionCall) -> Self {
            PriceOracleCalls::Version(var)
        }
    }
}
