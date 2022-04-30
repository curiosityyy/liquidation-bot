pub use addressprovider_mod::*;
#[allow(clippy::too_many_arguments)]
mod addressprovider_mod {
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
    #[doc = "AddressProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ADDRESSPROVIDER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"service\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AddressSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"addresses\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getACL\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAccountFactory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getContractsRegister\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDataCompressor\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getGearToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriceOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTreasuryContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getWETHGateway\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getWethToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setACL\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAccountFactory\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setContractsRegister\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDataCompressor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGearToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPriceOracle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTreasuryContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setWETHGateway\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setWethToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ADDRESSPROVIDER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061001a3361005f565b60405130906f20a2222922a9a9afa82927ab24a222a960811b907fb37614c7d254ea8d16eb81fa11dddaeb266aa8ba4917980859c7740aff30c69190600090a36100af565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b610906806100be6000396000f3fe608060405234801561001057600080fd5b506004361061014d5760003560e01c806377532ed9116100c3578063bcaead981161007c578063bcaead9814610273578063c5120b3914610286578063c513c9bb14610299578063ce3c4ae4146102a1578063f2fde38b146102b4578063fca513a8146102c757600080fd5b806377532ed91461022457806386e09c081461022c5780638da5cb5b1461023f5780639068a86814610250578063addc1a7614610258578063affd92431461026b57600080fd5b80634c252f91116101155780634c252f91146101af578063530e784f146101b757806354fd4d50146101ca578063699f200f146101e0578063715018a61461020957806376aad6051461021157600080fd5b8063060678c21461015257806308737695146101775780631ed651101461017f57806321da58371461019457806326c74fc3146101a7575b600080fd5b61015a6102cf565b6040516001600160a01b0390911681526020015b60405180910390f35b61015a6102f1565b61019261018d3660046107fd565b610302565b005b6101926101a23660046107fd565b610356565b61015a610399565b61015a6103b8565b6101926101c53660046107fd565b6103d0565b6101d2600281565b60405190815260200161016e565b61015a6101ee36600461082d565b6001602052600090815260409020546001600160a01b031681565b610192610413565b61019261021f3660046107fd565b610449565b61015a610483565b61019261023a3660046107fd565b61049d565b6000546001600160a01b031661015a565b61015a6104de565b6101926102663660046107fd565b6104fb565b61015a610541565b6101926102813660046107fd565b610559565b6101926102943660046107fd565b61059a565b61015a6105e0565b6101926102af3660046107fd565b610600565b6101926102c23660046107fd565b610649565b61015a6106e1565b60006102ec6e2220aa20afa1a7a6a82922a9a9a7a960891b6106f7565b905090565b60006102ec621050d360ea1b6106f7565b6000546001600160a01b031633146103355760405162461bcd60e51b815260040161032c90610846565b60405180910390fd5b610353701514915054d5549657d0d3d395149050d5607a1b82610754565b50565b6000546001600160a01b031633146103805760405162461bcd60e51b815260040161032c90610846565b6103536b574554485f4741544557415960a01b82610754565b60006102ec701514915054d5549657d0d3d395149050d5607a1b6106f7565b60006102ec692ba2aa242faa27a5a2a760b11b6106f7565b6000546001600160a01b031633146103fa5760405162461bcd60e51b815260040161032c90610846565b6103536b50524943455f4f5241434c4560a01b82610754565b6000546001600160a01b0316331461043d5760405162461bcd60e51b815260040161032c90610846565b61044760006107ad565b565b6000546001600160a01b031633146104735760405162461bcd60e51b815260040161032c90610846565b610353621050d360ea1b82610754565b60006102ec6b574554485f4741544557415960a01b6106f7565b6000546001600160a01b031633146104c75760405162461bcd60e51b815260040161032c90610846565b610353692ba2aa242faa27a5a2a760b11b82610754565b60006102ec6e4143434f554e545f464143544f525960881b6106f7565b6000546001600160a01b031633146105255760405162461bcd60e51b815260040161032c90610846565b6103536e4143434f554e545f464143544f525960881b82610754565b60006102ec6923a2a0a92faa27a5a2a760b11b6106f7565b6000546001600160a01b031633146105835760405162461bcd60e51b815260040161032c90610846565b6103536923a2a0a92faa27a5a2a760b11b82610754565b6000546001600160a01b031633146105c45760405162461bcd60e51b815260040161032c90610846565b6103536e2220aa20afa1a7a6a82922a9a9a7a960891b82610754565b60006102ec7121a7a72a2920a1aa29afa922a3a4a9aa22a960711b6106f7565b6000546001600160a01b0316331461062a5760405162461bcd60e51b815260040161032c90610846565b6103537121a7a72a2920a1aa29afa922a3a4a9aa22a960711b82610754565b6000546001600160a01b031633146106735760405162461bcd60e51b815260040161032c90610846565b6001600160a01b0381166106d85760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161032c565b610353816107ad565b60006102ec6b50524943455f4f5241434c4560a01b5b6000818152600160209081526040808320548151808301909252600382526241503160e81b928201929092526001600160a01b03909116908161074d5760405162461bcd60e51b815260040161032c919061087b565b5092915050565b60008281526001602052604080822080546001600160a01b0319166001600160a01b0385169081179091559051909184917fb37614c7d254ea8d16eb81fa11dddaeb266aa8ba4917980859c7740aff30c6919190a35050565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b60006020828403121561080f57600080fd5b81356001600160a01b038116811461082657600080fd5b9392505050565b60006020828403121561083f57600080fd5b5035919050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b600060208083528351808285015260005b818110156108a85785810183015185820160400152820161088c565b818111156108ba576000604083870101525b50601f01601f191692909201604001939250505056fea26469706673582212200e16a611b9fa163fbf158dd4882ac98a2b9f696335fca7b29bf6296d545a09e564736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct AddressProvider<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AddressProvider<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AddressProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AddressProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AddressProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ADDRESSPROVIDER_ABI.clone(), client)
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
                ADDRESSPROVIDER_ABI.clone(),
                ADDRESSPROVIDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addresses` (0x699f200f) function"]
        pub fn addresses(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([105, 159, 32, 15], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getACL` (0x08737695) function"]
        pub fn get_acl(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 115, 118, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountFactory` (0x9068a868) function"]
        pub fn get_account_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([144, 104, 168, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getContractsRegister` (0xc513c9bb) function"]
        pub fn get_contracts_register(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([197, 19, 201, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDataCompressor` (0x060678c2) function"]
        pub fn get_data_compressor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([6, 6, 120, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getGearToken` (0xaffd9243) function"]
        pub fn get_gear_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([175, 253, 146, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracle` (0xfca513a8) function"]
        pub fn get_price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 165, 19, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTreasuryContract` (0x26c74fc3) function"]
        pub fn get_treasury_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([38, 199, 79, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWETHGateway` (0x77532ed9) function"]
        pub fn get_weth_gateway(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([119, 83, 46, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWethToken` (0x4c252f91) function"]
        pub fn get_weth_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([76, 37, 47, 145], ())
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setACL` (0x76aad605) function"]
        pub fn set_acl(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 170, 214, 5], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAccountFactory` (0xaddc1a76) function"]
        pub fn set_account_factory(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 220, 26, 118], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setContractsRegister` (0xce3c4ae4) function"]
        pub fn set_contracts_register(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 60, 74, 228], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDataCompressor` (0xc5120b39) function"]
        pub fn set_data_compressor(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 18, 11, 57], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGearToken` (0xbcaead98) function"]
        pub fn set_gear_token(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 174, 173, 152], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracle` (0x530e784f) function"]
        pub fn set_price_oracle(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 14, 120, 79], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTreasuryContract` (0x1ed65110) function"]
        pub fn set_treasury_contract(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 214, 81, 16], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWETHGateway` (0x21da5837) function"]
        pub fn set_weth_gateway(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 218, 88, 55], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWethToken` (0x86e09c08) function"]
        pub fn set_weth_token(
            &self,
            address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 224, 156, 8], address)
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
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddressSet` event"]
        pub fn address_set_filter(&self) -> ethers::contract::builders::Event<M, AddressSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AddressProviderEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for AddressProvider<M> {
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
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address)")]
    pub struct AddressSetFilter {
        #[ethevent(indexed)]
        pub service: [u8; 32],
        #[ethevent(indexed)]
        pub new_address: ethers::core::types::Address,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AddressProviderEvents {
        AddressSetFilter(AddressSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers::contract::EthLogDecode for AddressProviderEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(AddressProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AddressProviderEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AddressProviderEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AddressProviderEvents::AddressSetFilter(element) => element.fmt(f),
                AddressProviderEvents::OwnershipTransferredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addresses`function with signature `addresses(bytes32)` and selector `[105, 159, 32, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addresses", abi = "addresses(bytes32)")]
    pub struct AddressesCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `getACL`function with signature `getACL()` and selector `[8, 115, 118, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getACL", abi = "getACL()")]
    pub struct GetACLCall;
    #[doc = "Container type for all input parameters for the `getAccountFactory`function with signature `getAccountFactory()` and selector `[144, 104, 168, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getAccountFactory", abi = "getAccountFactory()")]
    pub struct GetAccountFactoryCall;
    #[doc = "Container type for all input parameters for the `getContractsRegister`function with signature `getContractsRegister()` and selector `[197, 19, 201, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getContractsRegister", abi = "getContractsRegister()")]
    pub struct GetContractsRegisterCall;
    #[doc = "Container type for all input parameters for the `getDataCompressor`function with signature `getDataCompressor()` and selector `[6, 6, 120, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDataCompressor", abi = "getDataCompressor()")]
    pub struct GetDataCompressorCall;
    #[doc = "Container type for all input parameters for the `getGearToken`function with signature `getGearToken()` and selector `[175, 253, 146, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getGearToken", abi = "getGearToken()")]
    pub struct GetGearTokenCall;
    #[doc = "Container type for all input parameters for the `getPriceOracle`function with signature `getPriceOracle()` and selector `[252, 165, 19, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriceOracle", abi = "getPriceOracle()")]
    pub struct GetPriceOracleCall;
    #[doc = "Container type for all input parameters for the `getTreasuryContract`function with signature `getTreasuryContract()` and selector `[38, 199, 79, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTreasuryContract", abi = "getTreasuryContract()")]
    pub struct GetTreasuryContractCall;
    #[doc = "Container type for all input parameters for the `getWETHGateway`function with signature `getWETHGateway()` and selector `[119, 83, 46, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getWETHGateway", abi = "getWETHGateway()")]
    pub struct GetWETHGatewayCall;
    #[doc = "Container type for all input parameters for the `getWethToken`function with signature `getWethToken()` and selector `[76, 37, 47, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getWethToken", abi = "getWethToken()")]
    pub struct GetWethTokenCall;
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
    #[doc = "Container type for all input parameters for the `setACL`function with signature `setACL(address)` and selector `[118, 170, 214, 5]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setACL", abi = "setACL(address)")]
    pub struct SetACLCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setAccountFactory`function with signature `setAccountFactory(address)` and selector `[173, 220, 26, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAccountFactory", abi = "setAccountFactory(address)")]
    pub struct SetAccountFactoryCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setContractsRegister`function with signature `setContractsRegister(address)` and selector `[206, 60, 74, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setContractsRegister", abi = "setContractsRegister(address)")]
    pub struct SetContractsRegisterCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDataCompressor`function with signature `setDataCompressor(address)` and selector `[197, 18, 11, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDataCompressor", abi = "setDataCompressor(address)")]
    pub struct SetDataCompressorCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setGearToken`function with signature `setGearToken(address)` and selector `[188, 174, 173, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setGearToken", abi = "setGearToken(address)")]
    pub struct SetGearTokenCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPriceOracle`function with signature `setPriceOracle(address)` and selector `[83, 14, 120, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setPriceOracle", abi = "setPriceOracle(address)")]
    pub struct SetPriceOracleCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setTreasuryContract`function with signature `setTreasuryContract(address)` and selector `[30, 214, 81, 16]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setTreasuryContract", abi = "setTreasuryContract(address)")]
    pub struct SetTreasuryContractCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setWETHGateway`function with signature `setWETHGateway(address)` and selector `[33, 218, 88, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setWETHGateway", abi = "setWETHGateway(address)")]
    pub struct SetWETHGatewayCall {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setWethToken`function with signature `setWethToken(address)` and selector `[134, 224, 156, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setWethToken", abi = "setWethToken(address)")]
    pub struct SetWethTokenCall {
        pub address: ethers::core::types::Address,
    }
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
    pub enum AddressProviderCalls {
        Addresses(AddressesCall),
        GetACL(GetACLCall),
        GetAccountFactory(GetAccountFactoryCall),
        GetContractsRegister(GetContractsRegisterCall),
        GetDataCompressor(GetDataCompressorCall),
        GetGearToken(GetGearTokenCall),
        GetPriceOracle(GetPriceOracleCall),
        GetTreasuryContract(GetTreasuryContractCall),
        GetWETHGateway(GetWETHGatewayCall),
        GetWethToken(GetWethTokenCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetACL(SetACLCall),
        SetAccountFactory(SetAccountFactoryCall),
        SetContractsRegister(SetContractsRegisterCall),
        SetDataCompressor(SetDataCompressorCall),
        SetGearToken(SetGearTokenCall),
        SetPriceOracle(SetPriceOracleCall),
        SetTreasuryContract(SetTreasuryContractCall),
        SetWETHGateway(SetWETHGatewayCall),
        SetWethToken(SetWethTokenCall),
        TransferOwnership(TransferOwnershipCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for AddressProviderCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::Addresses(decoded));
            }
            if let Ok(decoded) = <GetACLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetACL(decoded));
            }
            if let Ok(decoded) =
                <GetAccountFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetAccountFactory(decoded));
            }
            if let Ok(decoded) =
                <GetContractsRegisterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetContractsRegister(decoded));
            }
            if let Ok(decoded) =
                <GetDataCompressorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetDataCompressor(decoded));
            }
            if let Ok(decoded) =
                <GetGearTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetGearToken(decoded));
            }
            if let Ok(decoded) =
                <GetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <GetTreasuryContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetTreasuryContract(decoded));
            }
            if let Ok(decoded) =
                <GetWETHGatewayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetWETHGateway(decoded));
            }
            if let Ok(decoded) =
                <GetWethTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::GetWethToken(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetACLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetACL(decoded));
            }
            if let Ok(decoded) =
                <SetAccountFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetAccountFactory(decoded));
            }
            if let Ok(decoded) =
                <SetContractsRegisterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetContractsRegister(decoded));
            }
            if let Ok(decoded) =
                <SetDataCompressorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetDataCompressor(decoded));
            }
            if let Ok(decoded) =
                <SetGearTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetGearToken(decoded));
            }
            if let Ok(decoded) =
                <SetPriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetPriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetTreasuryContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetTreasuryContract(decoded));
            }
            if let Ok(decoded) =
                <SetWETHGatewayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetWETHGateway(decoded));
            }
            if let Ok(decoded) =
                <SetWethTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::SetWethToken(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AddressProviderCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AddressProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AddressProviderCalls::Addresses(element) => element.encode(),
                AddressProviderCalls::GetACL(element) => element.encode(),
                AddressProviderCalls::GetAccountFactory(element) => element.encode(),
                AddressProviderCalls::GetContractsRegister(element) => element.encode(),
                AddressProviderCalls::GetDataCompressor(element) => element.encode(),
                AddressProviderCalls::GetGearToken(element) => element.encode(),
                AddressProviderCalls::GetPriceOracle(element) => element.encode(),
                AddressProviderCalls::GetTreasuryContract(element) => element.encode(),
                AddressProviderCalls::GetWETHGateway(element) => element.encode(),
                AddressProviderCalls::GetWethToken(element) => element.encode(),
                AddressProviderCalls::Owner(element) => element.encode(),
                AddressProviderCalls::RenounceOwnership(element) => element.encode(),
                AddressProviderCalls::SetACL(element) => element.encode(),
                AddressProviderCalls::SetAccountFactory(element) => element.encode(),
                AddressProviderCalls::SetContractsRegister(element) => element.encode(),
                AddressProviderCalls::SetDataCompressor(element) => element.encode(),
                AddressProviderCalls::SetGearToken(element) => element.encode(),
                AddressProviderCalls::SetPriceOracle(element) => element.encode(),
                AddressProviderCalls::SetTreasuryContract(element) => element.encode(),
                AddressProviderCalls::SetWETHGateway(element) => element.encode(),
                AddressProviderCalls::SetWethToken(element) => element.encode(),
                AddressProviderCalls::TransferOwnership(element) => element.encode(),
                AddressProviderCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AddressProviderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AddressProviderCalls::Addresses(element) => element.fmt(f),
                AddressProviderCalls::GetACL(element) => element.fmt(f),
                AddressProviderCalls::GetAccountFactory(element) => element.fmt(f),
                AddressProviderCalls::GetContractsRegister(element) => element.fmt(f),
                AddressProviderCalls::GetDataCompressor(element) => element.fmt(f),
                AddressProviderCalls::GetGearToken(element) => element.fmt(f),
                AddressProviderCalls::GetPriceOracle(element) => element.fmt(f),
                AddressProviderCalls::GetTreasuryContract(element) => element.fmt(f),
                AddressProviderCalls::GetWETHGateway(element) => element.fmt(f),
                AddressProviderCalls::GetWethToken(element) => element.fmt(f),
                AddressProviderCalls::Owner(element) => element.fmt(f),
                AddressProviderCalls::RenounceOwnership(element) => element.fmt(f),
                AddressProviderCalls::SetACL(element) => element.fmt(f),
                AddressProviderCalls::SetAccountFactory(element) => element.fmt(f),
                AddressProviderCalls::SetContractsRegister(element) => element.fmt(f),
                AddressProviderCalls::SetDataCompressor(element) => element.fmt(f),
                AddressProviderCalls::SetGearToken(element) => element.fmt(f),
                AddressProviderCalls::SetPriceOracle(element) => element.fmt(f),
                AddressProviderCalls::SetTreasuryContract(element) => element.fmt(f),
                AddressProviderCalls::SetWETHGateway(element) => element.fmt(f),
                AddressProviderCalls::SetWethToken(element) => element.fmt(f),
                AddressProviderCalls::TransferOwnership(element) => element.fmt(f),
                AddressProviderCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesCall> for AddressProviderCalls {
        fn from(var: AddressesCall) -> Self {
            AddressProviderCalls::Addresses(var)
        }
    }
    impl ::std::convert::From<GetACLCall> for AddressProviderCalls {
        fn from(var: GetACLCall) -> Self {
            AddressProviderCalls::GetACL(var)
        }
    }
    impl ::std::convert::From<GetAccountFactoryCall> for AddressProviderCalls {
        fn from(var: GetAccountFactoryCall) -> Self {
            AddressProviderCalls::GetAccountFactory(var)
        }
    }
    impl ::std::convert::From<GetContractsRegisterCall> for AddressProviderCalls {
        fn from(var: GetContractsRegisterCall) -> Self {
            AddressProviderCalls::GetContractsRegister(var)
        }
    }
    impl ::std::convert::From<GetDataCompressorCall> for AddressProviderCalls {
        fn from(var: GetDataCompressorCall) -> Self {
            AddressProviderCalls::GetDataCompressor(var)
        }
    }
    impl ::std::convert::From<GetGearTokenCall> for AddressProviderCalls {
        fn from(var: GetGearTokenCall) -> Self {
            AddressProviderCalls::GetGearToken(var)
        }
    }
    impl ::std::convert::From<GetPriceOracleCall> for AddressProviderCalls {
        fn from(var: GetPriceOracleCall) -> Self {
            AddressProviderCalls::GetPriceOracle(var)
        }
    }
    impl ::std::convert::From<GetTreasuryContractCall> for AddressProviderCalls {
        fn from(var: GetTreasuryContractCall) -> Self {
            AddressProviderCalls::GetTreasuryContract(var)
        }
    }
    impl ::std::convert::From<GetWETHGatewayCall> for AddressProviderCalls {
        fn from(var: GetWETHGatewayCall) -> Self {
            AddressProviderCalls::GetWETHGateway(var)
        }
    }
    impl ::std::convert::From<GetWethTokenCall> for AddressProviderCalls {
        fn from(var: GetWethTokenCall) -> Self {
            AddressProviderCalls::GetWethToken(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for AddressProviderCalls {
        fn from(var: OwnerCall) -> Self {
            AddressProviderCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for AddressProviderCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            AddressProviderCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetACLCall> for AddressProviderCalls {
        fn from(var: SetACLCall) -> Self {
            AddressProviderCalls::SetACL(var)
        }
    }
    impl ::std::convert::From<SetAccountFactoryCall> for AddressProviderCalls {
        fn from(var: SetAccountFactoryCall) -> Self {
            AddressProviderCalls::SetAccountFactory(var)
        }
    }
    impl ::std::convert::From<SetContractsRegisterCall> for AddressProviderCalls {
        fn from(var: SetContractsRegisterCall) -> Self {
            AddressProviderCalls::SetContractsRegister(var)
        }
    }
    impl ::std::convert::From<SetDataCompressorCall> for AddressProviderCalls {
        fn from(var: SetDataCompressorCall) -> Self {
            AddressProviderCalls::SetDataCompressor(var)
        }
    }
    impl ::std::convert::From<SetGearTokenCall> for AddressProviderCalls {
        fn from(var: SetGearTokenCall) -> Self {
            AddressProviderCalls::SetGearToken(var)
        }
    }
    impl ::std::convert::From<SetPriceOracleCall> for AddressProviderCalls {
        fn from(var: SetPriceOracleCall) -> Self {
            AddressProviderCalls::SetPriceOracle(var)
        }
    }
    impl ::std::convert::From<SetTreasuryContractCall> for AddressProviderCalls {
        fn from(var: SetTreasuryContractCall) -> Self {
            AddressProviderCalls::SetTreasuryContract(var)
        }
    }
    impl ::std::convert::From<SetWETHGatewayCall> for AddressProviderCalls {
        fn from(var: SetWETHGatewayCall) -> Self {
            AddressProviderCalls::SetWETHGateway(var)
        }
    }
    impl ::std::convert::From<SetWethTokenCall> for AddressProviderCalls {
        fn from(var: SetWethTokenCall) -> Self {
            AddressProviderCalls::SetWethToken(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for AddressProviderCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            AddressProviderCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<VersionCall> for AddressProviderCalls {
        fn from(var: VersionCall) -> Self {
            AddressProviderCalls::Version(var)
        }
    }
}
