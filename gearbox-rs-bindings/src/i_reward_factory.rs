pub use irewardfactory_mod::*;
#[allow(clippy::too_many_arguments)]
mod irewardfactory_mod {
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
    #[doc = "IRewardFactory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IREWARDFACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"CreateCrvRewards\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"CreateTokenRewards\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"activeRewardCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addActiveReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeActiveReward\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAccess\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IREWARDFACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IRewardFactory<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IRewardFactory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IRewardFactory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IRewardFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IRewardFactory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IREWARDFACTORY_ABI.clone(), client)
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
                IREWARDFACTORY_ABI.clone(),
                IREWARDFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `CreateCrvRewards` (0x58cbfd45) function"]
        pub fn create_crv_rewards(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([88, 203, 253, 69], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CreateTokenRewards` (0xf8d6122e) function"]
        pub fn create_token_rewards(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 214, 18, 46], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `activeRewardCount` (0x0d5843f7) function"]
        pub fn active_reward_count(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([13, 88, 67, 247], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addActiveReward` (0xb7f927b1) function"]
        pub fn add_active_reward(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 249, 39, 177], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeActiveReward` (0xef9126ad) function"]
        pub fn remove_active_reward(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([239, 145, 38, 173], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAccess` (0xb84614a5) function"]
        pub fn set_access(
            &self,
            p0: ethers::core::types::Address,
            p1: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 70, 20, 165], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IRewardFactory<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `CreateCrvRewards`function with signature `CreateCrvRewards(uint256,address)` and selector `[88, 203, 253, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "CreateCrvRewards", abi = "CreateCrvRewards(uint256,address)")]
    pub struct CreateCrvRewardsCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `CreateTokenRewards`function with signature `CreateTokenRewards(address,address,address)` and selector `[248, 214, 18, 46]`"]
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
        name = "CreateTokenRewards",
        abi = "CreateTokenRewards(address,address,address)"
    )]
    pub struct CreateTokenRewardsCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `activeRewardCount`function with signature `activeRewardCount(address)` and selector `[13, 88, 67, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "activeRewardCount", abi = "activeRewardCount(address)")]
    pub struct ActiveRewardCountCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `addActiveReward`function with signature `addActiveReward(address,uint256)` and selector `[183, 249, 39, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addActiveReward", abi = "addActiveReward(address,uint256)")]
    pub struct AddActiveRewardCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `removeActiveReward`function with signature `removeActiveReward(address,uint256)` and selector `[239, 145, 38, 173]`"]
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
        name = "removeActiveReward",
        abi = "removeActiveReward(address,uint256)"
    )]
    pub struct RemoveActiveRewardCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `setAccess`function with signature `setAccess(address,bool)` and selector `[184, 70, 20, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setAccess", abi = "setAccess(address,bool)")]
    pub struct SetAccessCall(pub ethers::core::types::Address, pub bool);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IRewardFactoryCalls {
        CreateCrvRewards(CreateCrvRewardsCall),
        CreateTokenRewards(CreateTokenRewardsCall),
        ActiveRewardCount(ActiveRewardCountCall),
        AddActiveReward(AddActiveRewardCall),
        RemoveActiveReward(RemoveActiveRewardCall),
        SetAccess(SetAccessCall),
    }
    impl ethers::core::abi::AbiDecode for IRewardFactoryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateCrvRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardFactoryCalls::CreateCrvRewards(decoded));
            }
            if let Ok(decoded) =
                <CreateTokenRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardFactoryCalls::CreateTokenRewards(decoded));
            }
            if let Ok(decoded) =
                <ActiveRewardCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardFactoryCalls::ActiveRewardCount(decoded));
            }
            if let Ok(decoded) =
                <AddActiveRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardFactoryCalls::AddActiveReward(decoded));
            }
            if let Ok(decoded) =
                <RemoveActiveRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardFactoryCalls::RemoveActiveReward(decoded));
            }
            if let Ok(decoded) =
                <SetAccessCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IRewardFactoryCalls::SetAccess(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IRewardFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IRewardFactoryCalls::CreateCrvRewards(element) => element.encode(),
                IRewardFactoryCalls::CreateTokenRewards(element) => element.encode(),
                IRewardFactoryCalls::ActiveRewardCount(element) => element.encode(),
                IRewardFactoryCalls::AddActiveReward(element) => element.encode(),
                IRewardFactoryCalls::RemoveActiveReward(element) => element.encode(),
                IRewardFactoryCalls::SetAccess(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IRewardFactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IRewardFactoryCalls::CreateCrvRewards(element) => element.fmt(f),
                IRewardFactoryCalls::CreateTokenRewards(element) => element.fmt(f),
                IRewardFactoryCalls::ActiveRewardCount(element) => element.fmt(f),
                IRewardFactoryCalls::AddActiveReward(element) => element.fmt(f),
                IRewardFactoryCalls::RemoveActiveReward(element) => element.fmt(f),
                IRewardFactoryCalls::SetAccess(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateCrvRewardsCall> for IRewardFactoryCalls {
        fn from(var: CreateCrvRewardsCall) -> Self {
            IRewardFactoryCalls::CreateCrvRewards(var)
        }
    }
    impl ::std::convert::From<CreateTokenRewardsCall> for IRewardFactoryCalls {
        fn from(var: CreateTokenRewardsCall) -> Self {
            IRewardFactoryCalls::CreateTokenRewards(var)
        }
    }
    impl ::std::convert::From<ActiveRewardCountCall> for IRewardFactoryCalls {
        fn from(var: ActiveRewardCountCall) -> Self {
            IRewardFactoryCalls::ActiveRewardCount(var)
        }
    }
    impl ::std::convert::From<AddActiveRewardCall> for IRewardFactoryCalls {
        fn from(var: AddActiveRewardCall) -> Self {
            IRewardFactoryCalls::AddActiveReward(var)
        }
    }
    impl ::std::convert::From<RemoveActiveRewardCall> for IRewardFactoryCalls {
        fn from(var: RemoveActiveRewardCall) -> Self {
            IRewardFactoryCalls::RemoveActiveReward(var)
        }
    }
    impl ::std::convert::From<SetAccessCall> for IRewardFactoryCalls {
        fn from(var: SetAccessCall) -> Self {
            IRewardFactoryCalls::SetAccess(var)
        }
    }
}
