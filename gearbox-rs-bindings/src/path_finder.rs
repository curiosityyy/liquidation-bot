pub use pathfinder_mod::*;
#[allow(clippy::too_many_arguments)]
mod pathfinder_mod {
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
    #[doc = "PathFinder was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PATHFINDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_addressProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"addressProvider\",\"outputs\":[{\"internalType\":\"contract AddressProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"swapInterface\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"router\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"swapType\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"bestUniPath\",\"outputs\":[{\"internalType\":\"struct PathFinder.TradePath\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expectedAmount\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"contractsRegister\",\"outputs\":[{\"internalType\":\"contract ContractsRegister\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"swapType\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"convertPathToPathV3\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"result\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ethToUsdPriceFeed\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"router\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"borrower\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"connectorTokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getClosurePaths\",\"outputs\":[{\"internalType\":\"struct PathFinder.TradePath[]\",\"name\":\"result\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expectedAmount\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPrices\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"prices\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"priceOracle\",\"outputs\":[{\"internalType\":\"contract PriceOracle\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wethToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PATHFINDER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101006040523480156200001257600080fd5b50604051620017f9380380620017f98339810160408190526200003591620001b9565b6001600160a01b03811660808190526040805163c513c9bb60e01b8152905163c513c9bb916004808201926020929091908290030181865afa15801562000080573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620000a69190620001b9565b6001600160a01b031660a0816001600160a01b0316815250506080516001600160a01b031663fca513a86040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000100573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001269190620001b9565b6001600160a01b031660c0816001600160a01b0316815250506080516001600160a01b0316634c252f916040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000180573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001a69190620001b9565b6001600160a01b031660e05250620001eb565b600060208284031215620001cc57600080fd5b81516001600160a01b0381168114620001e457600080fd5b9392505050565b60805160a05160c05160e0516115c0620002396000396000818161014e015261082101526000818160e0015261079c01526000818161018b01526105700152600061010701526115c06000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c806354fd4d501161006657806354fd4d50146101705780637a0c7b21146101865780638b7ce872146101ad5780638fb5a482146101cd578063e2430f93146101ed57600080fd5b80630c1bc044146100a35780632630c12f146100db5780632954018c1461010257806333a576a1146101295780634b57b0be14610149575b600080fd5b6100be735f4ec3df9cbd43714fe2740f5e3616155c5b841981565b6040516001600160a01b0390911681526020015b60405180910390f35b6100be7f000000000000000000000000000000000000000000000000000000000000000081565b6100be7f000000000000000000000000000000000000000000000000000000000000000081565b61013c610137366004610ea5565b61020d565b6040516100d29190610fa3565b6100be7f000000000000000000000000000000000000000000000000000000000000000081565b610178600181565b6040519081526020016100d2565b6100be7f000000000000000000000000000000000000000000000000000000000000000081565b6101c06101bb366004610fbd565b61054c565b6040516100d2919061102c565b6101e06101db36600461108e565b610627565b6040516100d29190611103565b6102006101fb366004611147565b6108d6565b6040516100d291906111d9565b61023160405180606001604052806060815260200160008152602001600081525090565b82610276576040805160036060820181815260e08301909352909182918160200160208202803683370190505081526020016000815260200160008152509050610541565b60408051600280825260608201835260009260208301908036833701905050905085816000815181106102ab576102ab6111ec565b60200260200101906001600160a01b031690816001600160a01b03168152505084816001815181106102df576102df6111ec565b60200260200101906001600160a01b031690816001600160a01b0316815250506000806103218b8b8b868a60018f1461031a57600019610a54565b6000610a54565b91509150606060008215610333578491505b60005b87518110156104b1576040805160038082526080820190925290602082016060803683370190505095508a86600081518110610374576103746111ec565b60200260200101906001600160a01b031690816001600160a01b03168152505089866002815181106103a8576103a86111ec565b60200260200101906001600160a01b031690816001600160a01b0316815250508a6001600160a01b03168882815181106103e4576103e46111ec565b60200260200101516001600160a01b03161415801561042e5750896001600160a01b031688828151811061041a5761041a6111ec565b60200260200101516001600160a01b031614155b1561049f57878181518110610445576104456111ec565b602002602001015186600181518110610460576104606111ec565b60200260200101906001600160a01b031690816001600160a01b03168152505061048e8e8e8e898d8a610a54565b94509150831561049f578194508592505b806104a981611218565b915050610336565b5060006000198514156104c357600094505b84158015906104d157508815155b156105205760018c1461050057886104f186670de0b6b3a7640000611233565b6104fb9190611252565b61051d565b846105138a670de0b6b3a7640000611233565b61051d9190611252565b90505b60408051606081018252938452602084019190915282019390935293505050505b979650505050505050565b604051636fbc6f6b60e01b81526001600160a01b03808516600483015260609185917f00000000000000000000000000000000000000000000000000000000000000001690636fbc6f6b90602401602060405180830381865afa1580156105b7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105db9190611274565b60405180604001604052806002815260200161043560f41b8152509061061d5760405162461bcd60e51b815260040161061491906111d9565b60405180910390fd5b5050949350505050565b60606000735f4ec3df9cbd43714fe2740f5e3616155c5b84196001600160a01b031663feaf968c6040518163ffffffff1660e01b815260040160a060405180830381865afa15801561067d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106a191906112b0565b50919350859250505067ffffffffffffffff8111156106c2576106c2610dc8565b6040519080825280602002602001820160405280156106eb578160200160208202803683370190505b50915060005b838110156108ce57600085858381811061070d5761070d6111ec565b90506020020160208101906107229190611300565b6001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa15801561075f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610783919061131b565b60ff169050670de0b6b3a7640000836001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663b66102df6107cc85600a611422565b8a8a888181106107de576107de6111ec565b90506020020160208101906107f39190611300565b6040516001600160e01b031960e085901b16815260048101929092526001600160a01b0390811660248301527f0000000000000000000000000000000000000000000000000000000000000000166044820152606401602060405180830381865afa158015610866573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061088a919061142e565b6108949190611233565b61089e9190611252565b8483815181106108b0576108b06111ec565b602090810291909101015250806108c681611218565b9150506106f1565b505092915050565b6060610bb860018314156109f15760005b600185516108f59190611447565b8110156109805761096c858281518110610911576109116111ec565b60200260200101518360405160200161095592919060609290921b6bffffffffffffffffffffffff1916825260e81b6001600160e81b031916601482015260170190565b60408051601f198184030181529190528490610a9e565b92508061097881611218565b9150506108e7565b506109ea84600186516109939190611447565b815181106109a3576109a36111ec565b60200260200101516040516020016109d3919060609190911b6bffffffffffffffffffffffff1916815260140190565b60408051601f198184030181529190528390610a9e565b9150610a4d565b600060018551610a019190611447565b90505b8015610a3357610a1f858281518110610911576109116111ec565b925080610a2b8161145e565b915050610a04565b50610a4a846000815181106109a3576109a36111ec565b91505b5092915050565b600080600188600d811115610a6b57610a6b611475565b14610a8257610a7d8787878787610b1c565b610a8f565b610a8f8787878787610c59565b91509150965096945050505050565b6060806040519050835180825260208201818101602087015b81831015610acf578051835260209283019201610ab7565b50855184518101855292509050808201602086015b81831015610afc578051835260209283019201610ae4565b508651929092011591909101601f01601f19166040525090505b92915050565b60008060006001871415610bb457876001600160a01b031663cdca1753610b43888a6108d6565b876040518363ffffffff1660e01b8152600401610b6192919061148b565b6020604051808303816000875af1925050508015610b9c575060408051601f3d908101601f19168201909252610b999181019061142e565b60015b610bad578360009250925050610c4f565b9050610c12565b6002871415610bd657876001600160a01b0316632f80bb1d610b43888a6108d6565b60405162461bcd60e51b8152602060048201526011602482015270556e6b6e6f776e2073776170207479706560781b6044820152606401610614565b600187148015610c2157508381115b80610c365750600287148015610c3657508381105b15610c4657915060019050610c4f565b83600092509250505b9550959350505050565b60008060006001871415610d135760405163d06ca61f60e01b81526001600160a01b0389169063d06ca61f90610c959088908a906004016114ad565b600060405180830381865afa925050508015610cd357506040513d6000823e601f3d908101601f19168201604052610cd09190810190611504565b60015b610ce4578360009250925050610c4f565b8060018851610cf39190611447565b81518110610d0357610d036111ec565b6020026020010151915050610c12565b6002871415610bd6576040516307c0329d60e21b81526001600160a01b03891690631f00ca7490610d4a9088908a906004016114ad565b600060405180830381865afa925050508015610d8857506040513d6000823e601f3d908101601f19168201604052610d859190810190611504565b60015b610d99578360009250925050610c4f565b80600081518110610d0357610d036111ec565b80356001600160a01b0381168114610dc357600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715610e0757610e07610dc8565b604052919050565b600067ffffffffffffffff821115610e2957610e29610dc8565b5060051b60200190565b600082601f830112610e4457600080fd5b81356020610e59610e5483610e0f565b610dde565b82815260059290921b84018101918181019086841115610e7857600080fd5b8286015b84811015610e9a57610e8d81610dac565b8352918301918301610e7c565b509695505050505050565b600080600080600080600060e0888a031215610ec057600080fd5b8735600e8110610ecf57600080fd5b9650610edd60208901610dac565b955060408801359450610ef260608901610dac565b9350610f0060808901610dac565b925060a0880135915060c088013567ffffffffffffffff811115610f2357600080fd5b610f2f8a828b01610e33565b91505092959891949750929550565b805160608084528151908401819052600091602091908201906080860190845b81811015610f835783516001600160a01b031683529284019291840191600101610f5e565b505082850151838701526040850151604087015280935050505092915050565b602081526000610fb66020830184610f3e565b9392505050565b60008060008060808587031215610fd357600080fd5b610fdc85610dac565b9350610fea60208601610dac565b9250610ff860408601610dac565b9150606085013567ffffffffffffffff81111561101457600080fd5b61102087828801610e33565b91505092959194509250565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b8281101561108157603f1988860301845261106f858351610f3e565b94509285019290850190600101611053565b5092979650505050505050565b600080602083850312156110a157600080fd5b823567ffffffffffffffff808211156110b957600080fd5b818501915085601f8301126110cd57600080fd5b8135818111156110dc57600080fd5b8660208260051b85010111156110f157600080fd5b60209290920196919550909350505050565b6020808252825182820181905260009190848201906040850190845b8181101561113b5783518352928401929184019160010161111f565b50909695505050505050565b6000806040838503121561115a57600080fd5b823567ffffffffffffffff81111561117157600080fd5b61117d85828601610e33565b95602094909401359450505050565b6000815180845260005b818110156111b257602081850181015186830182015201611196565b818111156111c4576000602083870101525b50601f01601f19169290920160200192915050565b602081526000610fb6602083018461118c565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561122c5761122c611202565b5060010190565b600081600019048311821515161561124d5761124d611202565b500290565b60008261126f57634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561128657600080fd5b81518015158114610fb657600080fd5b805169ffffffffffffffffffff81168114610dc357600080fd5b600080600080600060a086880312156112c857600080fd5b6112d186611296565b94506020860151935060408601519250606086015191506112f460808701611296565b90509295509295909350565b60006020828403121561131257600080fd5b610fb682610dac565b60006020828403121561132d57600080fd5b815160ff81168114610fb657600080fd5b600181815b8085111561137957816000190482111561135f5761135f611202565b8085161561136c57918102915b93841c9390800290611343565b509250929050565b60008261139057506001610b16565b8161139d57506000610b16565b81600181146113b357600281146113bd576113d9565b6001915050610b16565b60ff8411156113ce576113ce611202565b50506001821b610b16565b5060208310610133831016604e8410600b84101617156113fc575081810a610b16565b611406838361133e565b806000190482111561141a5761141a611202565b029392505050565b6000610fb68383611381565b60006020828403121561144057600080fd5b5051919050565b60008282101561145957611459611202565b500390565b60008161146d5761146d611202565b506000190190565b634e487b7160e01b600052602160045260246000fd5b60408152600061149e604083018561118c565b90508260208301529392505050565b6000604082018483526020604081850152818551808452606086019150828701935060005b818110156114f75784516001600160a01b0316835293830193918301916001016114d2565b5090979650505050505050565b6000602080838503121561151757600080fd5b825167ffffffffffffffff81111561152e57600080fd5b8301601f8101851361153f57600080fd5b805161154d610e5482610e0f565b81815260059190911b8201830190838101908783111561156c57600080fd5b928401925b828410156105415783518252928401929084019061157156fea2646970667358221220a0337a6beac7d14080d0cb6abdf9e693ed220c37dd2f29fb6dc091d99cc8f18864736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct PathFinder<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PathFinder<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PathFinder<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PathFinder))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PathFinder<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PATHFINDER_ABI.clone(), client).into()
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
                PATHFINDER_ABI.clone(),
                PATHFINDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addressProvider` (0x2954018c) function"]
        pub fn address_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 84, 1, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bestUniPath` (0x33a576a1) function"]
        pub fn best_uni_path(
            &self,
            swap_interface: u8,
            router: ethers::core::types::Address,
            swap_type: ethers::core::types::U256,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, TradePath> {
            self.0
                .method_hash(
                    [51, 165, 118, 161],
                    (swap_interface, router, swap_type, from, to, amount, tokens),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `contractsRegister` (0x7a0c7b21) function"]
        pub fn contracts_register(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([122, 12, 123, 33], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertPathToPathV3` (0xe2430f93) function"]
        pub fn convert_path_to_path_v3(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            swap_type: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([226, 67, 15, 147], (path, swap_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ethToUsdPriceFeed` (0x0c1bc044) function"]
        pub fn eth_to_usd_price_feed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([12, 27, 192, 68], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClosurePaths` (0x8b7ce872) function"]
        pub fn get_closure_paths(
            &self,
            router: ethers::core::types::Address,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
            connector_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TradePath>> {
            self.0
                .method_hash(
                    [139, 124, 232, 114],
                    (router, credit_manager, borrower, connector_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPrices` (0x8fb5a482) function"]
        pub fn get_prices(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([143, 181, 164, 130], tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceOracle` (0x2630c12f) function"]
        pub fn price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 48, 193, 47], ())
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
        #[doc = "Calls the contract's `wethToken` (0x4b57b0be) function"]
        pub fn weth_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([75, 87, 176, 190], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PathFinder<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addressProvider`function with signature `addressProvider()` and selector `[41, 84, 1, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addressProvider", abi = "addressProvider()")]
    pub struct AddressProviderCall;
    #[doc = "Container type for all input parameters for the `bestUniPath`function with signature `bestUniPath(uint8,address,uint256,address,address,uint256,address[])` and selector `[51, 165, 118, 161]`"]
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
        name = "bestUniPath",
        abi = "bestUniPath(uint8,address,uint256,address,address,uint256,address[])"
    )]
    pub struct BestUniPathCall {
        pub swap_interface: u8,
        pub router: ethers::core::types::Address,
        pub swap_type: ethers::core::types::U256,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `contractsRegister`function with signature `contractsRegister()` and selector `[122, 12, 123, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "contractsRegister", abi = "contractsRegister()")]
    pub struct ContractsRegisterCall;
    #[doc = "Container type for all input parameters for the `convertPathToPathV3`function with signature `convertPathToPathV3(address[],uint256)` and selector `[226, 67, 15, 147]`"]
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
        name = "convertPathToPathV3",
        abi = "convertPathToPathV3(address[],uint256)"
    )]
    pub struct ConvertPathToPathV3Call {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub swap_type: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `ethToUsdPriceFeed`function with signature `ethToUsdPriceFeed()` and selector `[12, 27, 192, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ethToUsdPriceFeed", abi = "ethToUsdPriceFeed()")]
    pub struct EthToUsdPriceFeedCall;
    #[doc = "Container type for all input parameters for the `getClosurePaths`function with signature `getClosurePaths(address,address,address,address[])` and selector `[139, 124, 232, 114]`"]
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
        name = "getClosurePaths",
        abi = "getClosurePaths(address,address,address,address[])"
    )]
    pub struct GetClosurePathsCall {
        pub router: ethers::core::types::Address,
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub connector_tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getPrices`function with signature `getPrices(address[])` and selector `[143, 181, 164, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPrices", abi = "getPrices(address[])")]
    pub struct GetPricesCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `priceOracle`function with signature `priceOracle()` and selector `[38, 48, 193, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "priceOracle", abi = "priceOracle()")]
    pub struct PriceOracleCall;
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PathFinderCalls {
        AddressProvider(AddressProviderCall),
        BestUniPath(BestUniPathCall),
        ContractsRegister(ContractsRegisterCall),
        ConvertPathToPathV3(ConvertPathToPathV3Call),
        EthToUsdPriceFeed(EthToUsdPriceFeedCall),
        GetClosurePaths(GetClosurePathsCall),
        GetPrices(GetPricesCall),
        PriceOracle(PriceOracleCall),
        Version(VersionCall),
        WethToken(WethTokenCall),
    }
    impl ethers::core::abi::AbiDecode for PathFinderCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::AddressProvider(decoded));
            }
            if let Ok(decoded) =
                <BestUniPathCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::BestUniPath(decoded));
            }
            if let Ok(decoded) =
                <ContractsRegisterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::ContractsRegister(decoded));
            }
            if let Ok(decoded) =
                <ConvertPathToPathV3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::ConvertPathToPathV3(decoded));
            }
            if let Ok(decoded) =
                <EthToUsdPriceFeedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::EthToUsdPriceFeed(decoded));
            }
            if let Ok(decoded) =
                <GetClosurePathsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::GetClosurePaths(decoded));
            }
            if let Ok(decoded) =
                <GetPricesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::GetPrices(decoded));
            }
            if let Ok(decoded) =
                <PriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::PriceOracle(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <WethTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PathFinderCalls::WethToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PathFinderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PathFinderCalls::AddressProvider(element) => element.encode(),
                PathFinderCalls::BestUniPath(element) => element.encode(),
                PathFinderCalls::ContractsRegister(element) => element.encode(),
                PathFinderCalls::ConvertPathToPathV3(element) => element.encode(),
                PathFinderCalls::EthToUsdPriceFeed(element) => element.encode(),
                PathFinderCalls::GetClosurePaths(element) => element.encode(),
                PathFinderCalls::GetPrices(element) => element.encode(),
                PathFinderCalls::PriceOracle(element) => element.encode(),
                PathFinderCalls::Version(element) => element.encode(),
                PathFinderCalls::WethToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PathFinderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PathFinderCalls::AddressProvider(element) => element.fmt(f),
                PathFinderCalls::BestUniPath(element) => element.fmt(f),
                PathFinderCalls::ContractsRegister(element) => element.fmt(f),
                PathFinderCalls::ConvertPathToPathV3(element) => element.fmt(f),
                PathFinderCalls::EthToUsdPriceFeed(element) => element.fmt(f),
                PathFinderCalls::GetClosurePaths(element) => element.fmt(f),
                PathFinderCalls::GetPrices(element) => element.fmt(f),
                PathFinderCalls::PriceOracle(element) => element.fmt(f),
                PathFinderCalls::Version(element) => element.fmt(f),
                PathFinderCalls::WethToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressProviderCall> for PathFinderCalls {
        fn from(var: AddressProviderCall) -> Self {
            PathFinderCalls::AddressProvider(var)
        }
    }
    impl ::std::convert::From<BestUniPathCall> for PathFinderCalls {
        fn from(var: BestUniPathCall) -> Self {
            PathFinderCalls::BestUniPath(var)
        }
    }
    impl ::std::convert::From<ContractsRegisterCall> for PathFinderCalls {
        fn from(var: ContractsRegisterCall) -> Self {
            PathFinderCalls::ContractsRegister(var)
        }
    }
    impl ::std::convert::From<ConvertPathToPathV3Call> for PathFinderCalls {
        fn from(var: ConvertPathToPathV3Call) -> Self {
            PathFinderCalls::ConvertPathToPathV3(var)
        }
    }
    impl ::std::convert::From<EthToUsdPriceFeedCall> for PathFinderCalls {
        fn from(var: EthToUsdPriceFeedCall) -> Self {
            PathFinderCalls::EthToUsdPriceFeed(var)
        }
    }
    impl ::std::convert::From<GetClosurePathsCall> for PathFinderCalls {
        fn from(var: GetClosurePathsCall) -> Self {
            PathFinderCalls::GetClosurePaths(var)
        }
    }
    impl ::std::convert::From<GetPricesCall> for PathFinderCalls {
        fn from(var: GetPricesCall) -> Self {
            PathFinderCalls::GetPrices(var)
        }
    }
    impl ::std::convert::From<PriceOracleCall> for PathFinderCalls {
        fn from(var: PriceOracleCall) -> Self {
            PathFinderCalls::PriceOracle(var)
        }
    }
    impl ::std::convert::From<VersionCall> for PathFinderCalls {
        fn from(var: VersionCall) -> Self {
            PathFinderCalls::Version(var)
        }
    }
    impl ::std::convert::From<WethTokenCall> for PathFinderCalls {
        fn from(var: WethTokenCall) -> Self {
            PathFinderCalls::WethToken(var)
        }
    }
    #[doc = "`TradePath(address[],uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TradePath {
        pub path: Vec<ethers::core::types::Address>,
        pub rate: ethers::core::types::U256,
        pub expected_amount: ethers::core::types::U256,
    }
}
