pub use iconvexv1boosteradapter_mod::*;
#[allow(clippy::too_many_arguments)]
mod iconvexv1boosteradapter_mod {
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
    #[doc = "IConvexV1BoosterAdapter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICONVEXV1BOOSTERADAPTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"booster\",\"outputs\":[{\"internalType\":\"contract IBooster\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crv\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_stake\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_stake\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolInfo\",\"outputs\":[{\"internalType\":\"struct IBooster.PoolInfo\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"lptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gauge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"crvRewards\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"stash\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"shutdown\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"poolLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"staker\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_pid\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICONVEXV1BOOSTERADAPTER_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct IConvexV1BoosterAdapter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for IConvexV1BoosterAdapter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IConvexV1BoosterAdapter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IConvexV1BoosterAdapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> IConvexV1BoosterAdapter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ICONVEXV1BOOSTERADAPTER_ABI.clone(),
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
                ICONVEXV1BOOSTERADAPTER_ABI.clone(),
                ICONVEXV1BOOSTERADAPTER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `booster` (0xc6def076) function"]
        pub fn booster(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([198, 222, 240, 118], ())
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
        #[doc = "Calls the contract's `targetContract` (0xbd90df70) function"]
        pub fn target_contract(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([189, 144, 223, 112], ())
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for IConvexV1BoosterAdapter<M>
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
    #[doc = "Container type for all input parameters for the `booster`function with signature `booster()` and selector `[198, 222, 240, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "booster", abi = "booster()")]
    pub struct BoosterCall;
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
    pub enum IConvexV1BoosterAdapterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        Booster(BoosterCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Crv(CrvCall),
        Deposit(DepositCall),
        DepositAll(DepositAllCall),
        Minter(MinterCall),
        PoolInfo(PoolInfoCall),
        PoolLength(PoolLengthCall),
        Staker(StakerCall),
        TargetContract(TargetContractCall),
        Withdraw(WithdrawCall),
        WithdrawAll(WithdrawAllCall),
    }
    impl ethers::core::abi::AbiDecode for IConvexV1BoosterAdapterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <BoosterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::Booster(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) = <CrvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IConvexV1BoosterAdapterCalls::Crv(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::DepositAll(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::Minter(decoded));
            }
            if let Ok(decoded) =
                <PoolInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::PoolInfo(decoded));
            }
            if let Ok(decoded) =
                <PoolLengthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::PoolLength(decoded));
            }
            if let Ok(decoded) = <StakerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::Staker(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::TargetContract(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IConvexV1BoosterAdapterCalls::WithdrawAll(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IConvexV1BoosterAdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IConvexV1BoosterAdapterCalls::GearboxAdapterType(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::GearboxAdapterVersion(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::Booster(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::CreditFacade(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::CreditManager(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::Crv(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::Deposit(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::DepositAll(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::Minter(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::PoolInfo(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::PoolLength(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::Staker(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::TargetContract(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::Withdraw(element) => element.encode(),
                IConvexV1BoosterAdapterCalls::WithdrawAll(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IConvexV1BoosterAdapterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IConvexV1BoosterAdapterCalls::GearboxAdapterType(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::Booster(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::CreditFacade(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::CreditManager(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::Crv(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::Deposit(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::DepositAll(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::Minter(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::PoolInfo(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::PoolLength(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::Staker(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::TargetContract(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::Withdraw(element) => element.fmt(f),
                IConvexV1BoosterAdapterCalls::WithdrawAll(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            IConvexV1BoosterAdapterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            IConvexV1BoosterAdapterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<BoosterCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: BoosterCall) -> Self {
            IConvexV1BoosterAdapterCalls::Booster(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            IConvexV1BoosterAdapterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: CreditManagerCall) -> Self {
            IConvexV1BoosterAdapterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CrvCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: CrvCall) -> Self {
            IConvexV1BoosterAdapterCalls::Crv(var)
        }
    }
    impl ::std::convert::From<DepositCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: DepositCall) -> Self {
            IConvexV1BoosterAdapterCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositAllCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: DepositAllCall) -> Self {
            IConvexV1BoosterAdapterCalls::DepositAll(var)
        }
    }
    impl ::std::convert::From<MinterCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: MinterCall) -> Self {
            IConvexV1BoosterAdapterCalls::Minter(var)
        }
    }
    impl ::std::convert::From<PoolInfoCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: PoolInfoCall) -> Self {
            IConvexV1BoosterAdapterCalls::PoolInfo(var)
        }
    }
    impl ::std::convert::From<PoolLengthCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: PoolLengthCall) -> Self {
            IConvexV1BoosterAdapterCalls::PoolLength(var)
        }
    }
    impl ::std::convert::From<StakerCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: StakerCall) -> Self {
            IConvexV1BoosterAdapterCalls::Staker(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: TargetContractCall) -> Self {
            IConvexV1BoosterAdapterCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: WithdrawCall) -> Self {
            IConvexV1BoosterAdapterCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAllCall> for IConvexV1BoosterAdapterCalls {
        fn from(var: WithdrawAllCall) -> Self {
            IConvexV1BoosterAdapterCalls::WithdrawAll(var)
        }
    }
}
