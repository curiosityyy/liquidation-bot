pub use acl_mod::*;
#[allow(clippy::too_many_arguments)]
mod acl_mod {
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
    #[doc = "ACL was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ACL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PausableAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PausableAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UnpausableAdminAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"UnpausableAdminRemoved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPausableAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addUnpausableAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isConfigurator\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPausableAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isUnpausableAdmin\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pausableAdminSet\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removePausableAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeUnpausableAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"unpausableAdminSet\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ACL_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061001a3361001f565b61006f565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6105dd8061007e6000396000f3fe608060405234801561001057600080fd5b50600436106100cf5760003560e01c8063732818191161008c578063adce758d11610066578063adce758d146101de578063ba306df1146101f1578063d4eb5db014610204578063f2fde38b1461023057600080fd5b8063732818191461018d578063819ad68e146101b05780638da5cb5b146101c357600080fd5b806335914829146100d45780633a41ec641461010c5780634910832f1461013857806354fd4d501461014d5780635f259aba14610163578063715018a614610185575b600080fd5b6100f76100e2366004610542565b60016020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b6100f761011a366004610542565b6001600160a01b031660009081526001602052604090205460ff1690565b61014b610146366004610542565b610243565b005b610155600181565b604051908152602001610103565b6100f7610171366004610542565b6000546001600160a01b0391821691161490565b61014b6102c5565b6100f761019b366004610542565b60026020526000908152604090205460ff1681565b61014b6101be366004610542565b6102fb565b6000546040516001600160a01b039091168152602001610103565b61014b6101ec366004610542565b610371565b61014b6101ff366004610542565b6103e4565b6100f7610212366004610542565b6001600160a01b031660009081526002602052604090205460ff1690565b61014b61023e366004610542565b610457565b6000546001600160a01b031633146102765760405162461bcd60e51b815260040161026d90610572565b60405180910390fd5b6001600160a01b0381166000818152600160208190526040808320805460ff1916909217909155517fae26b1cfe9454ba87274a4e8330b6654684362d0f3d7bbd17f7449a1d38387c69190a250565b6000546001600160a01b031633146102ef5760405162461bcd60e51b815260040161026d90610572565b6102f960006104f2565b565b6000546001600160a01b031633146103255760405162461bcd60e51b815260040161026d90610572565b6001600160a01b038116600081815260026020526040808220805460ff19166001179055517fd400da6c0c0a894dacc0981730b88af0545d00272ee8fff1437bf560ff245fc49190a250565b6000546001600160a01b0316331461039b5760405162461bcd60e51b815260040161026d90610572565b6001600160a01b038116600081815260026020526040808220805460ff19169055517f1998397e7203f7baca9d6f41b9e4da6e768daac5caad4234fb9bf5869d2715459190a250565b6000546001600160a01b0316331461040e5760405162461bcd60e51b815260040161026d90610572565b6001600160a01b038116600081815260016020526040808220805460ff19169055517f28b01395b7e25d20552a0c8dc8ecd3b1d4abc986f14dad7885fd45b6fd73c8d99190a250565b6000546001600160a01b031633146104815760405162461bcd60e51b815260040161026d90610572565b6001600160a01b0381166104e65760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161026d565b6104ef816104f2565b50565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b60006020828403121561055457600080fd5b81356001600160a01b038116811461056b57600080fd5b9392505050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260408201526060019056fea26469706673582212202de07632982b33dcad559c17e21a77fdaf7bcc0720d33f1c2bde322846c94c6c64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct ACL<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ACL<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ACL<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ACL))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ACL<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ACL_ABI.clone(), client).into()
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
                ACL_ABI.clone(),
                ACL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addPausableAdmin` (0x4910832f) function"]
        pub fn add_pausable_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 16, 131, 47], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addUnpausableAdmin` (0x819ad68e) function"]
        pub fn add_unpausable_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 154, 214, 142], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isConfigurator` (0x5f259aba) function"]
        pub fn is_configurator(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([95, 37, 154, 186], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPausableAdmin` (0x3a41ec64) function"]
        pub fn is_pausable_admin(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([58, 65, 236, 100], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isUnpausableAdmin` (0xd4eb5db0) function"]
        pub fn is_unpausable_admin(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([212, 235, 93, 176], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pausableAdminSet` (0x35914829) function"]
        pub fn pausable_admin_set(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([53, 145, 72, 41], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removePausableAdmin` (0xba306df1) function"]
        pub fn remove_pausable_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 48, 109, 241], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeUnpausableAdmin` (0xadce758d) function"]
        pub fn remove_unpausable_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 206, 117, 141], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpausableAdminSet` (0x73281819) function"]
        pub fn unpausable_admin_set(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([115, 40, 24, 25], p0)
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
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PausableAdminAdded` event"]
        pub fn pausable_admin_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausableAdminAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PausableAdminRemoved` event"]
        pub fn pausable_admin_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausableAdminRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UnpausableAdminAdded` event"]
        pub fn unpausable_admin_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UnpausableAdminAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UnpausableAdminRemoved` event"]
        pub fn unpausable_admin_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UnpausableAdminRemovedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ACLEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ACL<M> {
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
    #[ethevent(name = "PausableAdminAdded", abi = "PausableAdminAdded(address)")]
    pub struct PausableAdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(name = "PausableAdminRemoved", abi = "PausableAdminRemoved(address)")]
    pub struct PausableAdminRemovedFilter {
        #[ethevent(indexed)]
        pub admin: ethers::core::types::Address,
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
    #[ethevent(name = "UnpausableAdminAdded", abi = "UnpausableAdminAdded(address)")]
    pub struct UnpausableAdminAddedFilter {
        #[ethevent(indexed)]
        pub new_admin: ethers::core::types::Address,
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
    #[ethevent(
        name = "UnpausableAdminRemoved",
        abi = "UnpausableAdminRemoved(address)"
    )]
    pub struct UnpausableAdminRemovedFilter {
        #[ethevent(indexed)]
        pub admin: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ACLEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausableAdminAddedFilter(PausableAdminAddedFilter),
        PausableAdminRemovedFilter(PausableAdminRemovedFilter),
        UnpausableAdminAddedFilter(UnpausableAdminAddedFilter),
        UnpausableAdminRemovedFilter(UnpausableAdminRemovedFilter),
    }
    impl ethers::contract::EthLogDecode for ACLEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ACLEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausableAdminAddedFilter::decode_log(log) {
                return Ok(ACLEvents::PausableAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = PausableAdminRemovedFilter::decode_log(log) {
                return Ok(ACLEvents::PausableAdminRemovedFilter(decoded));
            }
            if let Ok(decoded) = UnpausableAdminAddedFilter::decode_log(log) {
                return Ok(ACLEvents::UnpausableAdminAddedFilter(decoded));
            }
            if let Ok(decoded) = UnpausableAdminRemovedFilter::decode_log(log) {
                return Ok(ACLEvents::UnpausableAdminRemovedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ACLEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ACLEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                ACLEvents::PausableAdminAddedFilter(element) => element.fmt(f),
                ACLEvents::PausableAdminRemovedFilter(element) => element.fmt(f),
                ACLEvents::UnpausableAdminAddedFilter(element) => element.fmt(f),
                ACLEvents::UnpausableAdminRemovedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addPausableAdmin`function with signature `addPausableAdmin(address)` and selector `[73, 16, 131, 47]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addPausableAdmin", abi = "addPausableAdmin(address)")]
    pub struct AddPausableAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addUnpausableAdmin`function with signature `addUnpausableAdmin(address)` and selector `[129, 154, 214, 142]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addUnpausableAdmin", abi = "addUnpausableAdmin(address)")]
    pub struct AddUnpausableAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isConfigurator`function with signature `isConfigurator(address)` and selector `[95, 37, 154, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isConfigurator", abi = "isConfigurator(address)")]
    pub struct IsConfiguratorCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isPausableAdmin`function with signature `isPausableAdmin(address)` and selector `[58, 65, 236, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isPausableAdmin", abi = "isPausableAdmin(address)")]
    pub struct IsPausableAdminCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isUnpausableAdmin`function with signature `isUnpausableAdmin(address)` and selector `[212, 235, 93, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isUnpausableAdmin", abi = "isUnpausableAdmin(address)")]
    pub struct IsUnpausableAdminCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `pausableAdminSet`function with signature `pausableAdminSet(address)` and selector `[53, 145, 72, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pausableAdminSet", abi = "pausableAdminSet(address)")]
    pub struct PausableAdminSetCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `removePausableAdmin`function with signature `removePausableAdmin(address)` and selector `[186, 48, 109, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removePausableAdmin", abi = "removePausableAdmin(address)")]
    pub struct RemovePausableAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `removeUnpausableAdmin`function with signature `removeUnpausableAdmin(address)` and selector `[173, 206, 117, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeUnpausableAdmin", abi = "removeUnpausableAdmin(address)")]
    pub struct RemoveUnpausableAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unpausableAdminSet`function with signature `unpausableAdminSet(address)` and selector `[115, 40, 24, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unpausableAdminSet", abi = "unpausableAdminSet(address)")]
    pub struct UnpausableAdminSetCall(pub ethers::core::types::Address);
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
    pub enum ACLCalls {
        AddPausableAdmin(AddPausableAdminCall),
        AddUnpausableAdmin(AddUnpausableAdminCall),
        IsConfigurator(IsConfiguratorCall),
        IsPausableAdmin(IsPausableAdminCall),
        IsUnpausableAdmin(IsUnpausableAdminCall),
        Owner(OwnerCall),
        PausableAdminSet(PausableAdminSetCall),
        RemovePausableAdmin(RemovePausableAdminCall),
        RemoveUnpausableAdmin(RemoveUnpausableAdminCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UnpausableAdminSet(UnpausableAdminSetCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for ACLCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddPausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::AddPausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <AddUnpausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::AddUnpausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsConfiguratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::IsConfigurator(decoded));
            }
            if let Ok(decoded) =
                <IsPausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::IsPausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <IsUnpausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::IsUnpausableAdmin(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PausableAdminSetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::PausableAdminSet(decoded));
            }
            if let Ok(decoded) =
                <RemovePausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::RemovePausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <RemoveUnpausableAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::RemoveUnpausableAdmin(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnpausableAdminSetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::UnpausableAdminSet(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ACLCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ACLCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ACLCalls::AddPausableAdmin(element) => element.encode(),
                ACLCalls::AddUnpausableAdmin(element) => element.encode(),
                ACLCalls::IsConfigurator(element) => element.encode(),
                ACLCalls::IsPausableAdmin(element) => element.encode(),
                ACLCalls::IsUnpausableAdmin(element) => element.encode(),
                ACLCalls::Owner(element) => element.encode(),
                ACLCalls::PausableAdminSet(element) => element.encode(),
                ACLCalls::RemovePausableAdmin(element) => element.encode(),
                ACLCalls::RemoveUnpausableAdmin(element) => element.encode(),
                ACLCalls::RenounceOwnership(element) => element.encode(),
                ACLCalls::TransferOwnership(element) => element.encode(),
                ACLCalls::UnpausableAdminSet(element) => element.encode(),
                ACLCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ACLCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ACLCalls::AddPausableAdmin(element) => element.fmt(f),
                ACLCalls::AddUnpausableAdmin(element) => element.fmt(f),
                ACLCalls::IsConfigurator(element) => element.fmt(f),
                ACLCalls::IsPausableAdmin(element) => element.fmt(f),
                ACLCalls::IsUnpausableAdmin(element) => element.fmt(f),
                ACLCalls::Owner(element) => element.fmt(f),
                ACLCalls::PausableAdminSet(element) => element.fmt(f),
                ACLCalls::RemovePausableAdmin(element) => element.fmt(f),
                ACLCalls::RemoveUnpausableAdmin(element) => element.fmt(f),
                ACLCalls::RenounceOwnership(element) => element.fmt(f),
                ACLCalls::TransferOwnership(element) => element.fmt(f),
                ACLCalls::UnpausableAdminSet(element) => element.fmt(f),
                ACLCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddPausableAdminCall> for ACLCalls {
        fn from(var: AddPausableAdminCall) -> Self {
            ACLCalls::AddPausableAdmin(var)
        }
    }
    impl ::std::convert::From<AddUnpausableAdminCall> for ACLCalls {
        fn from(var: AddUnpausableAdminCall) -> Self {
            ACLCalls::AddUnpausableAdmin(var)
        }
    }
    impl ::std::convert::From<IsConfiguratorCall> for ACLCalls {
        fn from(var: IsConfiguratorCall) -> Self {
            ACLCalls::IsConfigurator(var)
        }
    }
    impl ::std::convert::From<IsPausableAdminCall> for ACLCalls {
        fn from(var: IsPausableAdminCall) -> Self {
            ACLCalls::IsPausableAdmin(var)
        }
    }
    impl ::std::convert::From<IsUnpausableAdminCall> for ACLCalls {
        fn from(var: IsUnpausableAdminCall) -> Self {
            ACLCalls::IsUnpausableAdmin(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ACLCalls {
        fn from(var: OwnerCall) -> Self {
            ACLCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PausableAdminSetCall> for ACLCalls {
        fn from(var: PausableAdminSetCall) -> Self {
            ACLCalls::PausableAdminSet(var)
        }
    }
    impl ::std::convert::From<RemovePausableAdminCall> for ACLCalls {
        fn from(var: RemovePausableAdminCall) -> Self {
            ACLCalls::RemovePausableAdmin(var)
        }
    }
    impl ::std::convert::From<RemoveUnpausableAdminCall> for ACLCalls {
        fn from(var: RemoveUnpausableAdminCall) -> Self {
            ACLCalls::RemoveUnpausableAdmin(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for ACLCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            ACLCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ACLCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ACLCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnpausableAdminSetCall> for ACLCalls {
        fn from(var: UnpausableAdminSetCall) -> Self {
            ACLCalls::UnpausableAdminSet(var)
        }
    }
    impl ::std::convert::From<VersionCall> for ACLCalls {
        fn from(var: VersionCall) -> Self {
            ACLCalls::Version(var)
        }
    }
}
