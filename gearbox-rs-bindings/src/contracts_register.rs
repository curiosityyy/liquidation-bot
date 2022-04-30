pub use contractsregister_mod::*;
#[allow(clippy::too_many_arguments)]
mod contractsregister_mod {
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
    #[doc = "ContractsRegister was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONTRACTSREGISTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addressProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"creditManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewCreditManagerAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"pool\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewPoolAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newCreditManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addCreditManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPoolAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPool\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManagers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditManagers\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCreditManagersCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPools\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPoolsCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCreditManager\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPool\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pools\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONTRACTSREGISTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a060405234801561001057600080fd5b50604051610c85380380610c8583398101604081905261002f916100fb565b6000805460ff1916905560408051808201909152600281526105a360f41b602082015281906001600160a01b0382166100845760405162461bcd60e51b815260040161007b919061012b565b60405180910390fd5b50806001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100c3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100e791906100fb565b6001600160a01b0316608052506101809050565b60006020828403121561010d57600080fd5b81516001600160a01b038116811461012457600080fd5b9392505050565b600060208083528351808285015260005b818110156101585785810183015185820160400152820161013c565b8181111561016a576000604083870101525b50601f01601f1916929092016040019392505050565b608051610ad56101b06000396000818161024d01528181610386015281816104c201526106ac0152610ad56000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c80638456cb591161008c578063b4ac686011610066578063b4ac6860146101d8578063c29277cd146101e0578063d914cd4b146101e8578063e26b2f63146101fb57600080fd5b80638456cb59146101b557806394144856146101bd578063ac4afa38146101c557600080fd5b80635b16ebb7116100c85780635b16ebb71461013f5780635c975abb14610172578063673a2a1f1461017d5780636fbc6f6b1461019257600080fd5b80631e16e4fc146100ef5780633f4ba83a1461011f57806354fd4d5014610129575b600080fd5b6101026100fd366004610992565b61020e565b6040516001600160a01b0390911681526020015b60405180910390f35b610127610238565b005b610131600181565b604051908152602001610116565b61016261014d3660046109ab565b60026020526000908152604090205460ff1681565b6040519015158152602001610116565b60005460ff16610162565b61018561030f565b60405161011691906109db565b6101626101a03660046109ab565b60046020526000908152604090205460ff1681565b610127610371565b61018561043d565b6101026101d3366004610992565b61049d565b600154610131565b600354610131565b6101276101f63660046109ab565b6104ad565b6101276102093660046109ab565b610697565b6003818154811061021e57600080fd5b6000918252602090912001546001600160a01b0316905081565b604051630d4eb5db60e41b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d4eb5db090602401602060405180830381865afa15801561029c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102c09190610a28565b6040518060400160405280600481526020016341434c3160e01b815250906103045760405162461bcd60e51b81526004016102fb9190610a4a565b60405180910390fd5b5061030d610884565b565b6060600180548060200260200160405190810160405280929190818152602001828054801561036757602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610349575b5050505050905090565b604051630e907b1960e21b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633a41ec6490602401602060405180830381865afa1580156103d5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103f99190610a28565b6040518060400160405280600481526020016341434c3160e01b815250906104345760405162461bcd60e51b81526004016102fb9190610a4a565b5061030d610917565b60606003805480602002602001604051908101604052809291908181526020018280548015610367576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610349575050505050905090565b6001818154811061021e57600080fd5b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa158015610511573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105359190610a28565b6040518060400160405280600481526020016320a1a61960e11b815250906105705760405162461bcd60e51b81526004016102fb9190610a4a565b5060408051808201909152600281526105a360f41b60208201526001600160a01b0382166105b15760405162461bcd60e51b81526004016102fb9190610a4a565b506001600160a01b038116600090815260026020908152604091829020548251808401909352600383526243523160e81b9183019190915260ff161561060a5760405162461bcd60e51b81526004016102fb9190610a4a565b506001805480820182557fb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf60180546001600160a01b0319166001600160a01b038416908117909155600081815260026020526040808220805460ff1916909417909355915190917ff816b5143086c89d103a0683286be86c2b741e83ebfa75135aae606e2f5c6e5391a250565b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa1580156106fb573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061071f9190610a28565b6040518060400160405280600481526020016320a1a61960e11b8152509061075a5760405162461bcd60e51b81526004016102fb9190610a4a565b5060408051808201909152600281526105a360f41b60208201526001600160a01b03821661079b5760405162461bcd60e51b81526004016102fb9190610a4a565b506001600160a01b038116600090815260046020908152604091829020548251808401909352600383526221a91960e91b9183019190915260ff16156107f45760405162461bcd60e51b81526004016102fb9190610a4a565b506003805460018082019092557fc2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85b0180546001600160a01b0319166001600160a01b038416908117909155600081815260046020526040808220805460ff1916909417909355915190917f58ad3cfc4b6552a53c8c4128ae9b080e14b4378a159280643a62c6f709cee24f91a250565b60005460ff166108cd5760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b60448201526064016102fb565b6000805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b60005460ff161561095d5760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b60448201526064016102fb565b6000805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586108fa3390565b6000602082840312156109a457600080fd5b5035919050565b6000602082840312156109bd57600080fd5b81356001600160a01b03811681146109d457600080fd5b9392505050565b6020808252825182820181905260009190848201906040850190845b81811015610a1c5783516001600160a01b0316835292840192918401916001016109f7565b50909695505050505050565b600060208284031215610a3a57600080fd5b815180151581146109d457600080fd5b600060208083528351808285015260005b81811015610a7757858101830151858201604001528201610a5b565b81811115610a89576000604083870101525b50601f01601f191692909201604001939250505056fea264697066735822122047df6bb42bbff6ee6a0695b557599fed3eac17a40a2d22e8bbcfc398eb784cad64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ContractsRegister<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ContractsRegister<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ContractsRegister<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ContractsRegister))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ContractsRegister<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CONTRACTSREGISTER_ABI.clone(), client)
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
                CONTRACTSREGISTER_ABI.clone(),
                CONTRACTSREGISTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addCreditManager` (0xe26b2f63) function"]
        pub fn add_credit_manager(
            &self,
            new_credit_manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 107, 47, 99], new_credit_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPool` (0xd914cd4b) function"]
        pub fn add_pool(
            &self,
            new_pool_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 20, 205, 75], new_pool_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagers` (0x1e16e4fc) function"]
        pub fn credit_managers(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([30, 22, 228, 252], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagers` (0x94144856) function"]
        pub fn get_credit_managers(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([148, 20, 72, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagersCount` (0xc29277cd) function"]
        pub fn get_credit_managers_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([194, 146, 119, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPools` (0x673a2a1f) function"]
        pub fn get_pools(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([103, 58, 42, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolsCount` (0xb4ac6860) function"]
        pub fn get_pools_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([180, 172, 104, 96], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCreditManager` (0x6fbc6f6b) function"]
        pub fn is_credit_manager(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([111, 188, 111, 107], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPool` (0x5b16ebb7) function"]
        pub fn is_pool(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([91, 22, 235, 183], p0)
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
        #[doc = "Calls the contract's `pools` (0xac4afa38) function"]
        pub fn pools(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([172, 74, 250, 56], p0)
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
        #[doc = "Gets the contract's `NewCreditManagerAdded` event"]
        pub fn new_credit_manager_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCreditManagerAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPoolAdded` event"]
        pub fn new_pool_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPoolAddedFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ContractsRegisterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ContractsRegister<M>
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
    #[ethevent(name = "NewCreditManagerAdded", abi = "NewCreditManagerAdded(address)")]
    pub struct NewCreditManagerAddedFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers::core::types::Address,
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
    #[ethevent(name = "NewPoolAdded", abi = "NewPoolAdded(address)")]
    pub struct NewPoolAddedFilter {
        #[ethevent(indexed)]
        pub pool: ethers::core::types::Address,
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
    pub enum ContractsRegisterEvents {
        NewCreditManagerAddedFilter(NewCreditManagerAddedFilter),
        NewPoolAddedFilter(NewPoolAddedFilter),
        PausedFilter(PausedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for ContractsRegisterEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewCreditManagerAddedFilter::decode_log(log) {
                return Ok(ContractsRegisterEvents::NewCreditManagerAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPoolAddedFilter::decode_log(log) {
                return Ok(ContractsRegisterEvents::NewPoolAddedFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ContractsRegisterEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ContractsRegisterEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ContractsRegisterEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ContractsRegisterEvents::NewCreditManagerAddedFilter(element) => element.fmt(f),
                ContractsRegisterEvents::NewPoolAddedFilter(element) => element.fmt(f),
                ContractsRegisterEvents::PausedFilter(element) => element.fmt(f),
                ContractsRegisterEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addCreditManager`function with signature `addCreditManager(address)` and selector `[226, 107, 47, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addCreditManager", abi = "addCreditManager(address)")]
    pub struct AddCreditManagerCall {
        pub new_credit_manager: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addPool`function with signature `addPool(address)` and selector `[217, 20, 205, 75]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addPool", abi = "addPool(address)")]
    pub struct AddPoolCall {
        pub new_pool_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `creditManagers`function with signature `creditManagers(uint256)` and selector `[30, 22, 228, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "creditManagers", abi = "creditManagers(uint256)")]
    pub struct CreditManagersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `getCreditManagers`function with signature `getCreditManagers()` and selector `[148, 20, 72, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCreditManagers", abi = "getCreditManagers()")]
    pub struct GetCreditManagersCall;
    #[doc = "Container type for all input parameters for the `getCreditManagersCount`function with signature `getCreditManagersCount()` and selector `[194, 146, 119, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCreditManagersCount", abi = "getCreditManagersCount()")]
    pub struct GetCreditManagersCountCall;
    #[doc = "Container type for all input parameters for the `getPools`function with signature `getPools()` and selector `[103, 58, 42, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPools", abi = "getPools()")]
    pub struct GetPoolsCall;
    #[doc = "Container type for all input parameters for the `getPoolsCount`function with signature `getPoolsCount()` and selector `[180, 172, 104, 96]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPoolsCount", abi = "getPoolsCount()")]
    pub struct GetPoolsCountCall;
    #[doc = "Container type for all input parameters for the `isCreditManager`function with signature `isCreditManager(address)` and selector `[111, 188, 111, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isCreditManager", abi = "isCreditManager(address)")]
    pub struct IsCreditManagerCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `isPool`function with signature `isPool(address)` and selector `[91, 22, 235, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPool", abi = "isPool(address)")]
    pub struct IsPoolCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `pools`function with signature `pools(uint256)` and selector `[172, 74, 250, 56]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall(pub ethers::core::types::U256);
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
    pub enum ContractsRegisterCalls {
        AddCreditManager(AddCreditManagerCall),
        AddPool(AddPoolCall),
        CreditManagers(CreditManagersCall),
        GetCreditManagers(GetCreditManagersCall),
        GetCreditManagersCount(GetCreditManagersCountCall),
        GetPools(GetPoolsCall),
        GetPoolsCount(GetPoolsCountCall),
        IsCreditManager(IsCreditManagerCall),
        IsPool(IsPoolCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Pools(PoolsCall),
        Unpause(UnpauseCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for ContractsRegisterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddCreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::AddCreditManager(decoded));
            }
            if let Ok(decoded) =
                <AddPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::AddPool(decoded));
            }
            if let Ok(decoded) =
                <CreditManagersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::CreditManagers(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::GetCreditManagers(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::GetCreditManagersCount(decoded));
            }
            if let Ok(decoded) =
                <GetPoolsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::GetPools(decoded));
            }
            if let Ok(decoded) =
                <GetPoolsCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::GetPoolsCount(decoded));
            }
            if let Ok(decoded) =
                <IsCreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::IsCreditManager(decoded));
            }
            if let Ok(decoded) = <IsPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::IsPool(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::Paused(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::Pools(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractsRegisterCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ContractsRegisterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ContractsRegisterCalls::AddCreditManager(element) => element.encode(),
                ContractsRegisterCalls::AddPool(element) => element.encode(),
                ContractsRegisterCalls::CreditManagers(element) => element.encode(),
                ContractsRegisterCalls::GetCreditManagers(element) => element.encode(),
                ContractsRegisterCalls::GetCreditManagersCount(element) => element.encode(),
                ContractsRegisterCalls::GetPools(element) => element.encode(),
                ContractsRegisterCalls::GetPoolsCount(element) => element.encode(),
                ContractsRegisterCalls::IsCreditManager(element) => element.encode(),
                ContractsRegisterCalls::IsPool(element) => element.encode(),
                ContractsRegisterCalls::Pause(element) => element.encode(),
                ContractsRegisterCalls::Paused(element) => element.encode(),
                ContractsRegisterCalls::Pools(element) => element.encode(),
                ContractsRegisterCalls::Unpause(element) => element.encode(),
                ContractsRegisterCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ContractsRegisterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ContractsRegisterCalls::AddCreditManager(element) => element.fmt(f),
                ContractsRegisterCalls::AddPool(element) => element.fmt(f),
                ContractsRegisterCalls::CreditManagers(element) => element.fmt(f),
                ContractsRegisterCalls::GetCreditManagers(element) => element.fmt(f),
                ContractsRegisterCalls::GetCreditManagersCount(element) => element.fmt(f),
                ContractsRegisterCalls::GetPools(element) => element.fmt(f),
                ContractsRegisterCalls::GetPoolsCount(element) => element.fmt(f),
                ContractsRegisterCalls::IsCreditManager(element) => element.fmt(f),
                ContractsRegisterCalls::IsPool(element) => element.fmt(f),
                ContractsRegisterCalls::Pause(element) => element.fmt(f),
                ContractsRegisterCalls::Paused(element) => element.fmt(f),
                ContractsRegisterCalls::Pools(element) => element.fmt(f),
                ContractsRegisterCalls::Unpause(element) => element.fmt(f),
                ContractsRegisterCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddCreditManagerCall> for ContractsRegisterCalls {
        fn from(var: AddCreditManagerCall) -> Self {
            ContractsRegisterCalls::AddCreditManager(var)
        }
    }
    impl ::std::convert::From<AddPoolCall> for ContractsRegisterCalls {
        fn from(var: AddPoolCall) -> Self {
            ContractsRegisterCalls::AddPool(var)
        }
    }
    impl ::std::convert::From<CreditManagersCall> for ContractsRegisterCalls {
        fn from(var: CreditManagersCall) -> Self {
            ContractsRegisterCalls::CreditManagers(var)
        }
    }
    impl ::std::convert::From<GetCreditManagersCall> for ContractsRegisterCalls {
        fn from(var: GetCreditManagersCall) -> Self {
            ContractsRegisterCalls::GetCreditManagers(var)
        }
    }
    impl ::std::convert::From<GetCreditManagersCountCall> for ContractsRegisterCalls {
        fn from(var: GetCreditManagersCountCall) -> Self {
            ContractsRegisterCalls::GetCreditManagersCount(var)
        }
    }
    impl ::std::convert::From<GetPoolsCall> for ContractsRegisterCalls {
        fn from(var: GetPoolsCall) -> Self {
            ContractsRegisterCalls::GetPools(var)
        }
    }
    impl ::std::convert::From<GetPoolsCountCall> for ContractsRegisterCalls {
        fn from(var: GetPoolsCountCall) -> Self {
            ContractsRegisterCalls::GetPoolsCount(var)
        }
    }
    impl ::std::convert::From<IsCreditManagerCall> for ContractsRegisterCalls {
        fn from(var: IsCreditManagerCall) -> Self {
            ContractsRegisterCalls::IsCreditManager(var)
        }
    }
    impl ::std::convert::From<IsPoolCall> for ContractsRegisterCalls {
        fn from(var: IsPoolCall) -> Self {
            ContractsRegisterCalls::IsPool(var)
        }
    }
    impl ::std::convert::From<PauseCall> for ContractsRegisterCalls {
        fn from(var: PauseCall) -> Self {
            ContractsRegisterCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for ContractsRegisterCalls {
        fn from(var: PausedCall) -> Self {
            ContractsRegisterCalls::Paused(var)
        }
    }
    impl ::std::convert::From<PoolsCall> for ContractsRegisterCalls {
        fn from(var: PoolsCall) -> Self {
            ContractsRegisterCalls::Pools(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for ContractsRegisterCalls {
        fn from(var: UnpauseCall) -> Self {
            ContractsRegisterCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<VersionCall> for ContractsRegisterCalls {
        fn from(var: VersionCall) -> Self {
            ContractsRegisterCalls::Version(var)
        }
    }
}
