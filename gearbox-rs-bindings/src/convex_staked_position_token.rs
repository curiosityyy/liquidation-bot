pub use convexstakedpositiontoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod convexstakedpositiontoken_mod {
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
    #[doc = "ConvexStakedPositionToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONVEXSTAKEDPOSITIONTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_pool\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_lptoken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pool\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"underlying\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONVEXSTAKEDPOSITIONTOKEN_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60e06040523480156200001157600080fd5b5060405162000aae38038062000aae8339810160408190526200003491620002c1565b80816001600160a01b03166306fdde036040518163ffffffff1660e01b8152600401600060405180830381865afa15801562000074573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526200009e919081019062000342565b604051602001620000b09190620003fa565b604051602081830303815290604052826001600160a01b03166395d89b416040518163ffffffff1660e01b8152600401600060405180830381865afa158015620000fe573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405262000128919081019062000342565b6040516020016200013a919062000441565b604051602081830303815290604052836001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000188573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001ae91906200046f565b8151620001c3906000906020850190620001fe565b508251620001d9906001906020860190620001fe565b5060ff1660a05250506001600160a01b039081166080529190911660c05250620004d8565b8280546200020c906200049b565b90600052602060002090601f0160209004810192826200023057600085556200027b565b82601f106200024b57805160ff19168380011785556200027b565b828001600101855582156200027b579182015b828111156200027b5782518255916020019190600101906200025e565b50620002899291506200028d565b5090565b5b808211156200028957600081556001016200028e565b80516001600160a01b0381168114620002bc57600080fd5b919050565b60008060408385031215620002d557600080fd5b620002e083620002a4565b9150620002f060208401620002a4565b90509250929050565b634e487b7160e01b600052604160045260246000fd5b60005b838110156200032c57818101518382015260200162000312565b838111156200033c576000848401525b50505050565b6000602082840312156200035557600080fd5b81516001600160401b03808211156200036d57600080fd5b818401915084601f8301126200038257600080fd5b815181811115620003975762000397620002f9565b604051601f8201601f19908116603f01168101908382118183101715620003c257620003c2620002f9565b81604052828152876020848701011115620003dc57600080fd5b620003ef8360208301602088016200030f565b979650505050505050565b7f436f6e766578205374616b656420506f736974696f6e20000000000000000000815260008251620004348160178501602087016200030f565b9190910160170192915050565b636376782d60e01b815260008251620004628160048501602087016200030f565b9190910160040192915050565b6000602082840312156200048257600080fd5b815160ff811681146200049457600080fd5b9392505050565b600181811c90821680620004b057607f821691505b60208210811415620004d257634e487b7160e01b600052602260045260246000fd5b50919050565b60805160a05160c051610599620005156000396000818160f401526103630152600061015701526000818161019001526102bc01526105996000f3fe608060405234801561001057600080fd5b50600436106100a95760003560e01c8063313ce56711610071578063313ce567146101525780636f307dc31461018b57806370a08231146101b257806395d89b41146101c5578063a9059cbb146100cc578063dd62ed3e146101cd57600080fd5b806306fdde03146100ae578063095ea7b3146100cc57806316f0115b146100ef57806318160ddd1461012e57806323b872dd14610144575b600080fd5b6100b66101db565b6040516100c391906103e3565b60405180910390f35b6100df6100da366004610454565b610269565b60405190151581526020016100c3565b6101167f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100c3565b6101366102b8565b6040519081526020016100c3565b6100df6100da36600461047e565b6101797f000000000000000000000000000000000000000000000000000000000000000081565b60405160ff90911681526020016100c3565b6101167f000000000000000000000000000000000000000000000000000000000000000081565b6101366101c03660046104ba565b610341565b6100b66103d6565b6101366100da3660046104dc565b600180546101e89061050f565b80601f01602080910402602001604051908101604052809291908181526020018280546102149061050f565b80156102615780601f1061023657610100808354040283529160200191610261565b820191906000526020600020905b81548152906001019060200180831161024457829003601f168201915b505050505081565b60405162461bcd60e51b815260206004820152601860248201527f5068616e746f6d20746f6b656e3a20666f7262696464656e0000000000000000604482015260009060640160405180910390fd5b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610318573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061033c919061054a565b905090565b6040516370a0823160e01b81526001600160a01b0382811660048301526000917f0000000000000000000000000000000000000000000000000000000000000000909116906370a0823190602401602060405180830381865afa1580156103ac573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103d0919061054a565b92915050565b600080546101e89061050f565b600060208083528351808285015260005b81811015610410578581018301518582016040015282016103f4565b81811115610422576000604083870101525b50601f01601f1916929092016040019392505050565b80356001600160a01b038116811461044f57600080fd5b919050565b6000806040838503121561046757600080fd5b61047083610438565b946020939093013593505050565b60008060006060848603121561049357600080fd5b61049c84610438565b92506104aa60208501610438565b9150604084013590509250925092565b6000602082840312156104cc57600080fd5b6104d582610438565b9392505050565b600080604083850312156104ef57600080fd5b6104f883610438565b915061050660208401610438565b90509250929050565b600181811c9082168061052357607f821691505b6020821081141561054457634e487b7160e01b600052602260045260246000fd5b50919050565b60006020828403121561055c57600080fd5b505191905056fea2646970667358221220d449b7cde378814d0ed96a6f196435ffa8a500660caf7e7628f22acd511be9cf64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct ConvexStakedPositionToken<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ConvexStakedPositionToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ConvexStakedPositionToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ConvexStakedPositionToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ConvexStakedPositionToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CONVEXSTAKEDPOSITIONTOKEN_ABI.clone(),
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
                CONVEXSTAKEDPOSITIONTOKEN_ABI.clone(),
                CONVEXSTAKEDPOSITIONTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlying` (0x6f307dc3) function"]
        pub fn underlying(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 48, 125, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, ConvexStakedPositionTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ConvexStakedPositionToken<M>
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConvexStakedPositionTokenEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ConvexStakedPositionTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ConvexStakedPositionTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ConvexStakedPositionTokenEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ConvexStakedPositionTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConvexStakedPositionTokenEvents::ApprovalFilter(element) => element.fmt(f),
                ConvexStakedPositionTokenEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
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
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `underlying`function with signature `underlying()` and selector `[111, 48, 125, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "underlying", abi = "underlying()")]
    pub struct UnderlyingCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConvexStakedPositionTokenCalls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Name(NameCall),
        Pool(PoolCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Underlying(UnderlyingCall),
    }
    impl ethers::core::abi::AbiDecode for ConvexStakedPositionTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ConvexStakedPositionTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ConvexStakedPositionTokenCalls::Pool(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexStakedPositionTokenCalls::Underlying(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ConvexStakedPositionTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ConvexStakedPositionTokenCalls::Allowance(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Approve(element) => element.encode(),
                ConvexStakedPositionTokenCalls::BalanceOf(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Decimals(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Name(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Pool(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Symbol(element) => element.encode(),
                ConvexStakedPositionTokenCalls::TotalSupply(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Transfer(element) => element.encode(),
                ConvexStakedPositionTokenCalls::TransferFrom(element) => element.encode(),
                ConvexStakedPositionTokenCalls::Underlying(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ConvexStakedPositionTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConvexStakedPositionTokenCalls::Allowance(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Approve(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::BalanceOf(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Decimals(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Name(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Pool(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Symbol(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::TotalSupply(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Transfer(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::TransferFrom(element) => element.fmt(f),
                ConvexStakedPositionTokenCalls::Underlying(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllowanceCall> for ConvexStakedPositionTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            ConvexStakedPositionTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for ConvexStakedPositionTokenCalls {
        fn from(var: ApproveCall) -> Self {
            ConvexStakedPositionTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ConvexStakedPositionTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            ConvexStakedPositionTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for ConvexStakedPositionTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            ConvexStakedPositionTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<NameCall> for ConvexStakedPositionTokenCalls {
        fn from(var: NameCall) -> Self {
            ConvexStakedPositionTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<PoolCall> for ConvexStakedPositionTokenCalls {
        fn from(var: PoolCall) -> Self {
            ConvexStakedPositionTokenCalls::Pool(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ConvexStakedPositionTokenCalls {
        fn from(var: SymbolCall) -> Self {
            ConvexStakedPositionTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ConvexStakedPositionTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            ConvexStakedPositionTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for ConvexStakedPositionTokenCalls {
        fn from(var: TransferCall) -> Self {
            ConvexStakedPositionTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ConvexStakedPositionTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            ConvexStakedPositionTokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnderlyingCall> for ConvexStakedPositionTokenCalls {
        fn from(var: UnderlyingCall) -> Self {
            ConvexStakedPositionTokenCalls::Underlying(var)
        }
    }
}
