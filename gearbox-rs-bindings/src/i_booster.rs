pub use ibooster_mod::*;
#[allow(clippy::too_many_arguments)]
mod ibooster_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IBooster was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IBOOSTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crv\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_stake\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_stake\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolInfo\",\"outputs\":[{\"internalType\":\"struct IBooster.PoolInfo\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"lptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gauge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"crvRewards\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stash\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"shutdown\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"staker\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static IBOOSTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IBooster<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IBooster<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IBooster<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IBooster))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IBooster<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IBOOSTER_ABI.clone(), client).into()
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
                IBOOSTER_ABI.clone(),
                IBOOSTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `crv` (0x6a4874a1) function"]
        pub fn crv(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([106, 72, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x43a0d066) function"]
        pub fn deposit(
            &self,
            pid: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            stake: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 160, 208, 102], (pid, amount, stake))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositAll` (0x60759fce) function"]
        pub fn deposit_all(
            &self,
            pid: ethers::core::types::U256,
            stake: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 117, 159, 206], (pid, stake))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minter` (0x07546172) function"]
        pub fn minter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([7, 84, 97, 114], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolInfo` (0x1526fe27) function"]
        pub fn pool_info(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, PoolInfo> {
            self.0
                .method_hash([21, 38, 254, 39], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolLength` (0x081e3eda) function"]
        pub fn pool_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([8, 30, 62, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `staker` (0x5ebaf1db) function"]
        pub fn staker(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([94, 186, 241, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x441a3e70) function"]
        pub fn withdraw(
            &self,
            pid: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 26, 62, 112], (pid, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAll` (0x958e2d31) function"]
        pub fn withdraw_all(
            &self,
            pid: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([149, 142, 45, 49], pid)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IBooster<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `crv`function with signature `crv()` and selector `[106, 72, 116, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "crv", abi = "crv()")]
    pub struct CrvCall;
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,uint256,bool)` and selector `[67, 160, 208, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,uint256,bool)")]
    pub struct DepositCall {
        pub pid: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub stake: bool,
    }
    #[doc = "Container type for all input parameters for the `depositAll`function with signature `depositAll(uint256,bool)` and selector `[96, 117, 159, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "depositAll", abi = "depositAll(uint256,bool)")]
    pub struct DepositAllCall {
        pub pid: ethers::core::types::U256,
        pub stake: bool,
    }
    #[doc = "Container type for all input parameters for the `minter`function with signature `minter()` and selector `[7, 84, 97, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
    #[doc = "Container type for all input parameters for the `poolInfo`function with signature `poolInfo(uint256)` and selector `[21, 38, 254, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolInfo", abi = "poolInfo(uint256)")]
    pub struct PoolInfoCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `poolLength`function with signature `poolLength()` and selector `[8, 30, 62, 218]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "poolLength", abi = "poolLength()")]
    pub struct PoolLengthCall;
    #[doc = "Container type for all input parameters for the `staker`function with signature `staker()` and selector `[94, 186, 241, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "staker", abi = "staker()")]
    pub struct StakerCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,uint256)` and selector `[68, 26, 62, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,uint256)")]
    pub struct WithdrawCall {
        pub pid: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdrawAll`function with signature `withdrawAll(uint256)` and selector `[149, 142, 45, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawAll", abi = "withdrawAll(uint256)")]
    pub struct WithdrawAllCall {
        pub pid: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IBoosterCalls {
        Crv(CrvCall),
        Deposit(DepositCall),
        DepositAll(DepositAllCall),
        Minter(MinterCall),
        PoolInfo(PoolInfoCall),
        PoolLength(PoolLengthCall),
        Staker(StakerCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
    }
    impl ethers::core::abi::AbiDecode for IBoosterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CrvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IBoosterCalls::Crv(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::DepositAll(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::Minter(decoded));
            }
            if let Ok(decoded) =
                <PoolInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::PoolInfo(decoded));
            }
            if let Ok(decoded) =
                <PoolLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::PoolLength(decoded));
            }
            if let Ok(decoded) = <StakerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::Staker(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IBoosterCalls::WithdrawAll(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IBoosterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IBoosterCalls::Crv(element) => element.encode(),
                IBoosterCalls::Deposit(element) => element.encode(),
                IBoosterCalls::DepositAll(element) => element.encode(),
                IBoosterCalls::Minter(element) => element.encode(),
                IBoosterCalls::PoolInfo(element) => element.encode(),
                IBoosterCalls::PoolLength(element) => element.encode(),
                IBoosterCalls::Staker(element) => element.encode(),
                IBoosterCalls::Withdraw(element) => element.encode(),
                IBoosterCalls::WithdrawAll(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IBoosterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IBoosterCalls::Crv(element) => element.fmt(f),
                IBoosterCalls::Deposit(element) => element.fmt(f),
                IBoosterCalls::DepositAll(element) => element.fmt(f),
                IBoosterCalls::Minter(element) => element.fmt(f),
                IBoosterCalls::PoolInfo(element) => element.fmt(f),
                IBoosterCalls::PoolLength(element) => element.fmt(f),
                IBoosterCalls::Staker(element) => element.fmt(f),
                IBoosterCalls::Withdraw(element) => element.fmt(f),
                IBoosterCalls::WithdrawAll(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CrvCall> for IBoosterCalls {
        fn from(var: CrvCall) -> Self {
            IBoosterCalls::Crv(var)
        }
    }
    impl ::std::convert::From<DepositCall> for IBoosterCalls {
        fn from(var: DepositCall) -> Self {
            IBoosterCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositAllCall> for IBoosterCalls {
        fn from(var: DepositAllCall) -> Self {
            IBoosterCalls::DepositAll(var)
        }
    }
    impl ::std::convert::From<MinterCall> for IBoosterCalls {
        fn from(var: MinterCall) -> Self {
            IBoosterCalls::Minter(var)
        }
    }
    impl ::std::convert::From<PoolInfoCall> for IBoosterCalls {
        fn from(var: PoolInfoCall) -> Self {
            IBoosterCalls::PoolInfo(var)
        }
    }
    impl ::std::convert::From<PoolLengthCall> for IBoosterCalls {
        fn from(var: PoolLengthCall) -> Self {
            IBoosterCalls::PoolLength(var)
        }
    }
    impl ::std::convert::From<StakerCall> for IBoosterCalls {
        fn from(var: StakerCall) -> Self {
            IBoosterCalls::Staker(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IBoosterCalls {
        fn from(var: WithdrawCall) -> Self {
            IBoosterCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAllCall> for IBoosterCalls {
        fn from(var: WithdrawAllCall) -> Self {
            IBoosterCalls::WithdrawAll(var)
        }
    }
}
