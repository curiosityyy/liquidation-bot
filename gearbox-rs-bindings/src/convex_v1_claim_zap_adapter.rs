pub use convexv1claimzapadapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod convexv1claimzapadapter_mod {
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
    #[doc = "ConvexV1ClaimZapAdapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONVEXV1CLAIMZAPADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_claimZap\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"rewardContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"extraRewardContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokenRewardContracts\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"tokenRewardTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewards\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONVEXV1CLAIMZAPADAPTER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x60c060405234801561001057600080fd5b50604051610ce6380380610ce68339818101604052604081101561003357600080fd5b50805160209091015181816001600160a01b038216158061005b57506001600160a01b038116155b1561007957604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa1580156100c3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100e79190610119565b6001600160a01b0390811660a052600080546001600160a01b0319169290911691909117905550506001805550610149565b60006020828403121561012b57600080fd5b81516001600160a01b038116811461014257600080fd5b9392505050565b60805160a051610b566101906000396000606c01526000818161025a0152818161032b015281816105c601528181610741015281816108d10152610a0a0152610b566000f3fe608060405234801561001057600080fd5b50600436106100625760003560e01c80632f7a1881146100675780635a7b87f2146100aa57806378aa73a414610223578063bd90df7014610242578063c12c21c014610255578063ce30bbdb1461027c575b600080fd5b61008e7f000000000000000000000000000000000000000000000000000000000000000081565b604080516001600160a01b039092168252519081900360200190f35b61022160048036036101208110156100c157600080fd5b810190602081018135600160201b8111156100db57600080fd5b8201836020820111156100ed57600080fd5b803590602001918460208302840111600160201b8311171561010e57600080fd5b919390929091602081019035600160201b81111561012b57600080fd5b82018360208201111561013d57600080fd5b803590602001918460208302840111600160201b8311171561015e57600080fd5b919390929091602081019035600160201b81111561017b57600080fd5b82018360208201111561018d57600080fd5b803590602001918460208302840111600160201b831117156101ae57600080fd5b919390929091602081019035600160201b8111156101cb57600080fd5b8201836020820111156101dd57600080fd5b803590602001918460208302840111600160201b831117156101fe57600080fd5b9193509150803590602081013590604081013590606081013590608001356102ab565b005b61022b600181565b6040805161ffff9092168252519081900360200190f35b60005461008e906001600160a01b031681565b61008e7f000000000000000000000000000000000000000000000000000000000000000081565b610284600c81565b6040518082600d81111561029a5761029a610aca565b815260200191505060405180910390f35b60006102ba6084823681610ae0565b6000808080806102ce366101248184610ae0565b604051602001808a8a808284378083019250505088815260200187815260200186815260200185815260200184815260200183838082843780830192505050995050505050505050505060405160208183030381529060405290507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636ce4074a3360008054906101000a90046001600160a01b0316846040518463ffffffff1660e01b815260040180846001600160a01b03168152602001836001600160a01b0316815260200180602001828103825283818151815260200191508051906020019080838360005b838110156103d85781810151838201526020016103c0565b50505050905090810190601f1680156104055780820380516001836020036101000a031916815260200191505b509450505050506000604051808303816000875af115801561042b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052602081101561045457600080fd5b8101908080516040519392919084600160201b82111561047357600080fd5b90830190602082018581111561048857600080fd5b8251600160201b8111828201881017156104a157600080fd5b82525081516020918201929091019080838360005b838110156104ce5781810151838201526020016104b6565b50505050905090810190601f1680156104fb5780820380516001836020036101000a031916815260200191505b50604052505050506105b28e8e80806020026020016040519081016040528093929190818152602001838360200280828437600081840152601f19601f820116905080830192505050505050508d8d8080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525050604080516020808f0282810182019093528e82529093508e92508d9182918501908490808284376000920191909152506105c292505050565b5050505050505050505050505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663e958b704336040518263ffffffff1660e01b815260040180826001600160a01b03168152602001915050602060405180830381865afa158015610636573d6000803e3d6000fd5b505050506040513d602081101561064c57600080fd5b505190506000805b85518110156107e05785818151811061066f5761066f610b0a565b60200260200101516001600160a01b031663f7c618c16040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106b4573d6000803e3d6000fd5b505050506040513d60208110156106ca57600080fd5b5051604080516370a0823160e01b81526001600160a01b0386811660048301529151929450600192918516916370a08231916024808201926020929091908290030181865afa158015610721573d6000803e3d6000fd5b505050506040513d602081101561073757600080fd5b505111156107d8577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166351e3f16084846040518363ffffffff1660e01b815260040180836001600160a01b03168152602001826001600160a01b0316815260200192505050600060405180830381600087803b1580156107bf57600080fd5b505af11580156107d3573d6000803e3d6000fd5b505050505b600101610654565b5060005b8451811015610970578481815181106107ff576107ff610b0a565b60200260200101516001600160a01b031663f7c618c16040518163ffffffff1660e01b8152600401602060405180830381865afa158015610844573d6000803e3d6000fd5b505050506040513d602081101561085a57600080fd5b5051604080516370a0823160e01b81526001600160a01b0386811660048301529151929450600192918516916370a08231916024808201926020929091908290030181865afa1580156108b1573d6000803e3d6000fd5b505050506040513d60208110156108c757600080fd5b50511115610968577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166351e3f16084846040518363ffffffff1660e01b815260040180836001600160a01b03168152602001826001600160a01b0316815260200192505050600060405180830381600087803b15801561094f57600080fd5b505af1158015610963573d6000803e3d6000fd5b505050505b6001016107e4565b5060005b8351811015610ac257600184828151811061099157610991610b0a565b60200260200101516001600160a01b03166370a08231856040518263ffffffff1660e01b815260040180826001600160a01b03168152602001915050602060405180830381865afa1580156109ea573d6000803e3d6000fd5b505050506040513d6020811015610a0057600080fd5b50511115610aba577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166351e3f16084868481518110610a4a57610a4a610b0a565b60200260200101516040518363ffffffff1660e01b815260040180836001600160a01b03168152602001826001600160a01b0316815260200192505050600060405180830381600087803b158015610aa157600080fd5b505af1158015610ab5573d6000803e3d6000fd5b505050505b600101610974565b505050505050565b634e487b7160e01b600052602160045260246000fd5b60008085851115610af057600080fd5b83861115610afd57600080fd5b5050820193919092039150565b634e487b7160e01b600052603260045260246000fdfea2646970667358221220ba4f6259d1e2fd8a8747d9442130e5bfcb0eb1cf43e78d216e2a92bfa4b84ea264736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct ConvexV1ClaimZapAdapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ConvexV1ClaimZapAdapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ConvexV1ClaimZapAdapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ConvexV1ClaimZapAdapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ConvexV1ClaimZapAdapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CONVEXV1CLAIMZAPADAPTER_ABI.clone(),
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
                CONVEXV1CLAIMZAPADAPTER_ABI.clone(),
                CONVEXV1CLAIMZAPADAPTER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `claimRewards` (0x5a7b87f2) function"]
        pub fn claim_rewards(
            &self,
            reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
            extra_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
            token_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
            token_reward_tokens: ::std::vec::Vec<ethers::core::types::Address>,
            p4: ethers::core::types::U256,
            p5: ethers::core::types::U256,
            p6: ethers::core::types::U256,
            p7: ethers::core::types::U256,
            p8: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [90, 123, 135, 242],
                    (
                        reward_contracts,
                        extra_reward_contracts,
                        token_reward_contracts,
                        token_reward_tokens,
                        p4,
                        p5,
                        p6,
                        p7,
                        p8,
                    ),
                )
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
        #[doc = "Calls the contract's `targetContract` (0xbd90df70) function"]
        pub fn target_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([189, 144, 223, 112], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ConvexV1ClaimZapAdapter<M>
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
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(address[],address[],address[],address[],uint256,uint256,uint256,uint256,uint256)` and selector `[90, 123, 135, 242]`"]
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
        name = "claimRewards",
        abi = "claimRewards(address[],address[],address[],address[],uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ClaimRewardsCall {
        pub reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
        pub extra_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
        pub token_reward_contracts: ::std::vec::Vec<ethers::core::types::Address>,
        pub token_reward_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub p4: ethers::core::types::U256,
        pub p5: ethers::core::types::U256,
        pub p6: ethers::core::types::U256,
        pub p7: ethers::core::types::U256,
        pub p8: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConvexV1ClaimZapAdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        ClaimRewards(ClaimRewardsCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        TargetContract(TargetContractCall),
    }
    impl ethers::core::abi::AbiDecode for ConvexV1ClaimZapAdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1ClaimZapAdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1ClaimZapAdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1ClaimZapAdapterCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1ClaimZapAdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1ClaimZapAdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexV1ClaimZapAdapterCalls::TargetContract(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ConvexV1ClaimZapAdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ConvexV1ClaimZapAdapterCalls::GearboxAdapterType(element) => element.encode(),
                ConvexV1ClaimZapAdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                ConvexV1ClaimZapAdapterCalls::ClaimRewards(element) => element.encode(),
                ConvexV1ClaimZapAdapterCalls::CreditFacade(element) => element.encode(),
                ConvexV1ClaimZapAdapterCalls::CreditManager(element) => element.encode(),
                ConvexV1ClaimZapAdapterCalls::TargetContract(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ConvexV1ClaimZapAdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConvexV1ClaimZapAdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                ConvexV1ClaimZapAdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                ConvexV1ClaimZapAdapterCalls::ClaimRewards(element) => element.fmt(f),
                ConvexV1ClaimZapAdapterCalls::CreditFacade(element) => element.fmt(f),
                ConvexV1ClaimZapAdapterCalls::CreditManager(element) => element.fmt(f),
                ConvexV1ClaimZapAdapterCalls::TargetContract(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for ConvexV1ClaimZapAdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            ConvexV1ClaimZapAdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for ConvexV1ClaimZapAdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            ConvexV1ClaimZapAdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for ConvexV1ClaimZapAdapterCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            ConvexV1ClaimZapAdapterCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for ConvexV1ClaimZapAdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            ConvexV1ClaimZapAdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for ConvexV1ClaimZapAdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            ConvexV1ClaimZapAdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for ConvexV1ClaimZapAdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            ConvexV1ClaimZapAdapterCalls::TargetContract(var)
        }
    }
}
