pub use multicall2_mod::*;
#[allow(clippy::too_many_arguments)]
mod multicall2_mod {
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
    #[doc = "Multicall2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MULTICALL2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct Multicall2.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"aggregate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"returnData\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Multicall2.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"blockAndAggregate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Multicall2.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockCoinbase\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"coinbase\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockDifficulty\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"difficulty\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockGasLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"gaslimit\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBlockTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getEthBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastBlockHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"requireSuccess\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct Multicall2.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tryAggregate\",\"outputs\":[{\"internalType\":\"struct Multicall2.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"requireSuccess\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct Multicall2.Call[]\",\"name\":\"calls\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"callData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tryBlockAndAggregate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Multicall2.Result[]\",\"name\":\"returnData\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MULTICALL2_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506109d0806100206000396000f3fe608060405234801561001057600080fd5b50600436106100b45760003560e01c806372425d9d1161007157806372425d9d1461013a57806386d516e814610140578063a8b0574e14610146578063bce38bd714610154578063c3077fa914610174578063ee82ac5e1461018757600080fd5b80630f28c97d146100b9578063252dba42146100ce57806327e86d6e146100ef578063399542e9146100f757806342cbb15c146101195780634d2301cc1461011f575b600080fd5b425b6040519081526020015b60405180910390f35b6100e16100dc3660046106ea565b610199565b6040516100c5929190610783565b6100bb610321565b61010a6101053660046107ed565b610334565b6040516100c5939291906108aa565b436100bb565b6100bb61012d3660046108d2565b6001600160a01b03163190565b446100bb565b456100bb565b6040514181526020016100c5565b6101676101623660046107ed565b61034c565b6040516100c591906108f4565b61010a6101823660046106ea565b610506565b6100bb610195366004610907565b4090565b8051439060609067ffffffffffffffff8111156101b8576101b8610523565b6040519080825280602002602001820160405280156101eb57816020015b60608152602001906001900390816101d65790505b50905060005b835181101561031b5760008085838151811061020f5761020f610920565b6020026020010151600001516001600160a01b031686848151811061023657610236610920565b60200260200101516020015160405161024f9190610936565b6000604051808303816000865af19150503d806000811461028c576040519150601f19603f3d011682016040523d82523d6000602084013e610291565b606091505b5091509150816102e85760405162461bcd60e51b815260206004820181905260248201527f4d756c746963616c6c206167677265676174653a2063616c6c206661696c656460448201526064015b60405180910390fd5b808484815181106102fb576102fb610920565b60200260200101819052505050808061031390610968565b9150506101f1565b50915091565b600061032e600143610983565b40905090565b4380406060610343858561034c565b90509250925092565b6060815167ffffffffffffffff81111561036857610368610523565b6040519080825280602002602001820160405280156103ae57816020015b6040805180820190915260008152606060208201528152602001906001900390816103865790505b50905060005b82518110156104ff576000808483815181106103d2576103d2610920565b6020026020010151600001516001600160a01b03168584815181106103f9576103f9610920565b6020026020010151602001516040516104129190610936565b6000604051808303816000865af19150503d806000811461044f576040519150601f19603f3d011682016040523d82523d6000602084013e610454565b606091505b509150915085156104b657816104b65760405162461bcd60e51b815260206004820152602160248201527f4d756c746963616c6c32206167677265676174653a2063616c6c206661696c656044820152601960fa1b60648201526084016102df565b60405180604001604052808315158152602001828152508484815181106104df576104df610920565b6020026020010181905250505080806104f790610968565b9150506103b4565b5092915050565b6000806060610516600185610334565b9196909550909350915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561055c5761055c610523565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561058b5761058b610523565b604052919050565b80356001600160a01b03811681146105aa57600080fd5b919050565b6000601f83818401126105c157600080fd5b8235602067ffffffffffffffff808311156105de576105de610523565b8260051b6105ed838201610562565b938452868101830193838101908986111561060757600080fd5b84890192505b858310156106dd578235848111156106255760008081fd5b89016040601f19828d03810182131561063e5760008081fd5b610646610539565b610651898501610593565b815282840135888111156106655760008081fd5b8085019450508d603f85011261067b5760008081fd5b888401358881111561068f5761068f610523565b61069e8a848e84011601610562565b92508083528e848287010111156106b55760008081fd5b808486018b85013760009083018a01528089019190915284525050918401919084019061060d565b9998505050505050505050565b6000602082840312156106fc57600080fd5b813567ffffffffffffffff81111561071357600080fd5b61071f848285016105af565b949350505050565b60005b8381101561074257818101518382015260200161072a565b83811115610751576000848401525b50505050565b6000815180845261076f816020860160208601610727565b601f01601f19169290920160200192915050565b600060408201848352602060408185015281855180845260608601915060608160051b870101935082870160005b828110156107df57605f198887030184526107cd868351610757565b955092840192908401906001016107b1565b509398975050505050505050565b6000806040838503121561080057600080fd5b8235801515811461081057600080fd5b9150602083013567ffffffffffffffff81111561082c57600080fd5b610838858286016105af565b9150509250929050565b6000815180845260208085019450848260051b860182860160005b8581101561089d5783830389528151805115158452850151604086850181905261088981860183610757565b9a87019a945050509084019060010161085d565b5090979650505050505050565b8381528260208201526060604082015260006108c96060830184610842565b95945050505050565b6000602082840312156108e457600080fd5b6108ed82610593565b9392505050565b6020815260006108ed6020830184610842565b60006020828403121561091957600080fd5b5035919050565b634e487b7160e01b600052603260045260246000fd5b60008251610948818460208701610727565b9190910192915050565b634e487b7160e01b600052601160045260246000fd5b600060001982141561097c5761097c610952565b5060010190565b60008282101561099557610995610952565b50039056fea2646970667358221220a42437fa7bd5f4a89a9af3909a070a885e7a156670ad71036f38166060fc0d2164736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Multicall2<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Multicall2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Multicall2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Multicall2))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Multicall2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MULTICALL2_ABI.clone(), client).into()
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
                MULTICALL2_ABI.clone(),
                MULTICALL2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `aggregate` (0x252dba42) function"]
        pub fn aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ::std::vec::Vec<ethers::core::types::Bytes>,
            ),
        > {
            self.0
                .method_hash([37, 45, 186, 66], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockAndAggregate` (0xc3077fa9) function"]
        pub fn block_and_aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([195, 7, 127, 169], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockHash` (0xee82ac5e) function"]
        pub fn get_block_hash(
            &self,
            block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([238, 130, 172, 94], block_number)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockNumber` (0x42cbb15c) function"]
        pub fn get_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockCoinbase` (0xa8b0574e) function"]
        pub fn get_current_block_coinbase(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([168, 176, 87, 78], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockDifficulty` (0x72425d9d) function"]
        pub fn get_current_block_difficulty(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([114, 66, 93, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockGasLimit` (0x86d516e8) function"]
        pub fn get_current_block_gas_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([134, 213, 22, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function"]
        pub fn get_current_block_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEthBalance` (0x4d2301cc) function"]
        pub fn get_eth_balance(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastBlockHash` (0x27e86d6e) function"]
        pub fn get_last_block_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([39, 232, 109, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tryAggregate` (0xbce38bd7) function"]
        pub fn try_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([188, 227, 139, 215], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tryBlockAndAggregate` (0x399542e9) function"]
        pub fn try_block_and_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([57, 149, 66, 233], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Multicall2<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `aggregate`function with signature `aggregate((address,bytes)[])` and selector `[37, 45, 186, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "aggregate", abi = "aggregate((address,bytes)[])")]
    pub struct AggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    #[doc = "Container type for all input parameters for the `blockAndAggregate`function with signature `blockAndAggregate((address,bytes)[])` and selector `[195, 7, 127, 169]`"]
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
        name = "blockAndAggregate",
        abi = "blockAndAggregate((address,bytes)[])"
    )]
    pub struct BlockAndAggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    #[doc = "Container type for all input parameters for the `getBlockHash`function with signature `getBlockHash(uint256)` and selector `[238, 130, 172, 94]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBlockHash", abi = "getBlockHash(uint256)")]
    pub struct GetBlockHashCall {
        pub block_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getBlockNumber`function with signature `getBlockNumber()` and selector `[66, 203, 177, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockCoinbase`function with signature `getCurrentBlockCoinbase()` and selector `[168, 176, 87, 78]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentBlockCoinbase", abi = "getCurrentBlockCoinbase()")]
    pub struct GetCurrentBlockCoinbaseCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockDifficulty`function with signature `getCurrentBlockDifficulty()` and selector `[114, 66, 93, 157]`"]
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
        name = "getCurrentBlockDifficulty",
        abi = "getCurrentBlockDifficulty()"
    )]
    pub struct GetCurrentBlockDifficultyCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockGasLimit`function with signature `getCurrentBlockGasLimit()` and selector `[134, 213, 22, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentBlockGasLimit", abi = "getCurrentBlockGasLimit()")]
    pub struct GetCurrentBlockGasLimitCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockTimestamp`function with signature `getCurrentBlockTimestamp()` and selector `[15, 40, 201, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentBlockTimestamp", abi = "getCurrentBlockTimestamp()")]
    pub struct GetCurrentBlockTimestampCall;
    #[doc = "Container type for all input parameters for the `getEthBalance`function with signature `getEthBalance(address)` and selector `[77, 35, 1, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getEthBalance", abi = "getEthBalance(address)")]
    pub struct GetEthBalanceCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getLastBlockHash`function with signature `getLastBlockHash()` and selector `[39, 232, 109, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLastBlockHash", abi = "getLastBlockHash()")]
    pub struct GetLastBlockHashCall;
    #[doc = "Container type for all input parameters for the `tryAggregate`function with signature `tryAggregate(bool,(address,bytes)[])` and selector `[188, 227, 139, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tryAggregate", abi = "tryAggregate(bool,(address,bytes)[])")]
    pub struct TryAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    #[doc = "Container type for all input parameters for the `tryBlockAndAggregate`function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `[57, 149, 66, 233]`"]
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
        name = "tryBlockAndAggregate",
        abi = "tryBlockAndAggregate(bool,(address,bytes)[])"
    )]
    pub struct TryBlockAndAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum Multicall2Calls {
        Aggregate(AggregateCall),
        BlockAndAggregate(BlockAndAggregateCall),
        GetBlockHash(GetBlockHashCall),
        GetBlockNumber(GetBlockNumberCall),
        GetCurrentBlockCoinbase(GetCurrentBlockCoinbaseCall),
        GetCurrentBlockDifficulty(GetCurrentBlockDifficultyCall),
        GetCurrentBlockGasLimit(GetCurrentBlockGasLimitCall),
        GetCurrentBlockTimestamp(GetCurrentBlockTimestampCall),
        GetEthBalance(GetEthBalanceCall),
        GetLastBlockHash(GetLastBlockHashCall),
        TryAggregate(TryAggregateCall),
        TryBlockAndAggregate(TryBlockAndAggregateCall),
    }
    impl ethers::core::abi::AbiDecode for Multicall2Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::Aggregate(decoded));
            }
            if let Ok(decoded) =
                <BlockAndAggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::BlockAndAggregate(decoded));
            }
            if let Ok(decoded) =
                <GetBlockHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::GetBlockHash(decoded));
            }
            if let Ok(decoded) =
                <GetBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::GetBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockCoinbaseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::GetCurrentBlockCoinbase(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockDifficultyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(Multicall2Calls::GetCurrentBlockDifficulty(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockGasLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::GetCurrentBlockGasLimit(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockTimestampCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(Multicall2Calls::GetCurrentBlockTimestamp(decoded));
            }
            if let Ok(decoded) =
                <GetEthBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::GetEthBalance(decoded));
            }
            if let Ok(decoded) =
                <GetLastBlockHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::GetLastBlockHash(decoded));
            }
            if let Ok(decoded) =
                <TryAggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::TryAggregate(decoded));
            }
            if let Ok(decoded) =
                <TryBlockAndAggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(Multicall2Calls::TryBlockAndAggregate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for Multicall2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Multicall2Calls::Aggregate(element) => element.encode(),
                Multicall2Calls::BlockAndAggregate(element) => element.encode(),
                Multicall2Calls::GetBlockHash(element) => element.encode(),
                Multicall2Calls::GetBlockNumber(element) => element.encode(),
                Multicall2Calls::GetCurrentBlockCoinbase(element) => element.encode(),
                Multicall2Calls::GetCurrentBlockDifficulty(element) => element.encode(),
                Multicall2Calls::GetCurrentBlockGasLimit(element) => element.encode(),
                Multicall2Calls::GetCurrentBlockTimestamp(element) => element.encode(),
                Multicall2Calls::GetEthBalance(element) => element.encode(),
                Multicall2Calls::GetLastBlockHash(element) => element.encode(),
                Multicall2Calls::TryAggregate(element) => element.encode(),
                Multicall2Calls::TryBlockAndAggregate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for Multicall2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Multicall2Calls::Aggregate(element) => element.fmt(f),
                Multicall2Calls::BlockAndAggregate(element) => element.fmt(f),
                Multicall2Calls::GetBlockHash(element) => element.fmt(f),
                Multicall2Calls::GetBlockNumber(element) => element.fmt(f),
                Multicall2Calls::GetCurrentBlockCoinbase(element) => element.fmt(f),
                Multicall2Calls::GetCurrentBlockDifficulty(element) => element.fmt(f),
                Multicall2Calls::GetCurrentBlockGasLimit(element) => element.fmt(f),
                Multicall2Calls::GetCurrentBlockTimestamp(element) => element.fmt(f),
                Multicall2Calls::GetEthBalance(element) => element.fmt(f),
                Multicall2Calls::GetLastBlockHash(element) => element.fmt(f),
                Multicall2Calls::TryAggregate(element) => element.fmt(f),
                Multicall2Calls::TryBlockAndAggregate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AggregateCall> for Multicall2Calls {
        fn from(var: AggregateCall) -> Self {
            Multicall2Calls::Aggregate(var)
        }
    }
    impl ::std::convert::From<BlockAndAggregateCall> for Multicall2Calls {
        fn from(var: BlockAndAggregateCall) -> Self {
            Multicall2Calls::BlockAndAggregate(var)
        }
    }
    impl ::std::convert::From<GetBlockHashCall> for Multicall2Calls {
        fn from(var: GetBlockHashCall) -> Self {
            Multicall2Calls::GetBlockHash(var)
        }
    }
    impl ::std::convert::From<GetBlockNumberCall> for Multicall2Calls {
        fn from(var: GetBlockNumberCall) -> Self {
            Multicall2Calls::GetBlockNumber(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockCoinbaseCall> for Multicall2Calls {
        fn from(var: GetCurrentBlockCoinbaseCall) -> Self {
            Multicall2Calls::GetCurrentBlockCoinbase(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockDifficultyCall> for Multicall2Calls {
        fn from(var: GetCurrentBlockDifficultyCall) -> Self {
            Multicall2Calls::GetCurrentBlockDifficulty(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockGasLimitCall> for Multicall2Calls {
        fn from(var: GetCurrentBlockGasLimitCall) -> Self {
            Multicall2Calls::GetCurrentBlockGasLimit(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockTimestampCall> for Multicall2Calls {
        fn from(var: GetCurrentBlockTimestampCall) -> Self {
            Multicall2Calls::GetCurrentBlockTimestamp(var)
        }
    }
    impl ::std::convert::From<GetEthBalanceCall> for Multicall2Calls {
        fn from(var: GetEthBalanceCall) -> Self {
            Multicall2Calls::GetEthBalance(var)
        }
    }
    impl ::std::convert::From<GetLastBlockHashCall> for Multicall2Calls {
        fn from(var: GetLastBlockHashCall) -> Self {
            Multicall2Calls::GetLastBlockHash(var)
        }
    }
    impl ::std::convert::From<TryAggregateCall> for Multicall2Calls {
        fn from(var: TryAggregateCall) -> Self {
            Multicall2Calls::TryAggregate(var)
        }
    }
    impl ::std::convert::From<TryBlockAndAggregateCall> for Multicall2Calls {
        fn from(var: TryBlockAndAggregateCall) -> Self {
            Multicall2Calls::TryBlockAndAggregate(var)
        }
    }
    #[doc = "`Call(address,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Call {
        pub target: ethers::core::types::Address,
        pub call_data: ethers::core::types::Bytes,
    }
    #[doc = "`Result(bool,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Result {
        pub success: bool,
        pub return_data: ethers::core::types::Bytes,
    }
}
