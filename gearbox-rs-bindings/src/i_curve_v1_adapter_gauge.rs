pub use icurvev1adaptergauge_mod::*;
#[allow(clippy::too_many_arguments)]
mod icurvev1adaptergauge_mod {
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
    #[doc = "ICurveV1AdapterGauge was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICURVEV1ADAPTERGAUGE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"TokenIsNotAddedToCreditManagerException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[8]\",\"name\":\"_reward_tokens\",\"type\":\"address[8]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claim_historic_rewards\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claim_rewards\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimable_reward\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"claimable_tokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"controller\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crv_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"curveLPtoken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraReward1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"extraReward2\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"future_epoch_time\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"gauge\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"kick\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lp_token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"reward_tokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"can_deposit\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"set_approve_deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"user_checkpoint\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"voting_escrow\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ICURVEV1ADAPTERGAUGE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct ICurveV1AdapterGauge<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ICurveV1AdapterGauge<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ICurveV1AdapterGauge<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICurveV1AdapterGauge))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ICurveV1AdapterGauge<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ICURVEV1ADAPTERGAUGE_ABI.clone(),
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
                ICURVEV1ADAPTERGAUGE_ABI.clone(),
                ICURVEV1ADAPTERGAUGE_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `claim_historic_rewards` (0xb9fa7a69) function"]
        pub fn claim_historic_rewards(
            &self,
            reward_tokens: [ethers::core::types::Address; 8usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 250, 122, 105], reward_tokens)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claim_rewards` (0xe6f1daf2) function"]
        pub fn claim_rewards(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 241, 218, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimable_reward` (0xd2797b59) function"]
        pub fn claimable_reward(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([210, 121, 123, 89], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimable_tokens` (0x33134583) function"]
        pub fn claimable_tokens(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([51, 19, 69, 131], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `controller` (0xf77c4791) function"]
        pub fn controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([247, 124, 71, 145], ())
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
        #[doc = "Calls the contract's `crv_token` (0x76d8b117) function"]
        pub fn crv_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([118, 216, 177, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `curveLPtoken` (0x927188d9) function"]
        pub fn curve_l_ptoken(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([146, 113, 136, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb6b55f25) function"]
        pub fn deposit(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 181, 95, 37], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extraReward1` (0xda5b383f) function"]
        pub fn extra_reward_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([218, 91, 56, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `extraReward2` (0x97c3413b) function"]
        pub fn extra_reward_2(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([151, 195, 65, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `future_epoch_time` (0xbe5d1be9) function"]
        pub fn future_epoch_time(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([190, 93, 27, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gauge` (0xa6f19c84) function"]
        pub fn gauge(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([166, 241, 156, 132], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `kick` (0x96c55175) function"]
        pub fn kick(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 197, 81, 117], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lp_token` (0x82c63066) function"]
        pub fn lp_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([130, 198, 48, 102], ())
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
        #[doc = "Calls the contract's `reward_tokens` (0x54c49fe9) function"]
        pub fn reward_tokens(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([84, 196, 159, 233], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `set_approve_deposit` (0x1d2747d4) function"]
        pub fn set_approve_deposit(
            &self,
            addr: ethers::core::types::Address,
            can_deposit: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 39, 71, 212], (addr, can_deposit))
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
        #[doc = "Calls the contract's `user_checkpoint` (0x4b820093) function"]
        pub fn user_checkpoint(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([75, 130, 0, 147], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `voting_escrow` (0xdfe05031) function"]
        pub fn voting_escrow(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([223, 224, 80, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], value)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ICurveV1AdapterGauge<M>
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
    #[doc = "Container type for all input parameters for the `claim_historic_rewards`function with signature `claim_historic_rewards(address[8])` and selector `[185, 250, 122, 105]`"]
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
        name = "claim_historic_rewards",
        abi = "claim_historic_rewards(address[8])"
    )]
    pub struct ClaimHistoricRewardsCall {
        pub reward_tokens: [ethers::core::types::Address; 8usize],
    }
    #[doc = "Container type for all input parameters for the `claim_rewards`function with signature `claim_rewards()` and selector `[230, 241, 218, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claim_rewards", abi = "claim_rewards()")]
    pub struct ClaimRewardsCall;
    #[doc = "Container type for all input parameters for the `claimable_reward`function with signature `claimable_reward(address)` and selector `[210, 121, 123, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimable_reward", abi = "claimable_reward(address)")]
    pub struct ClaimableRewardCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimable_tokens`function with signature `claimable_tokens(address)` and selector `[51, 19, 69, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "claimable_tokens", abi = "claimable_tokens(address)")]
    pub struct ClaimableTokensCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `controller`function with signature `controller()` and selector `[247, 124, 71, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "controller", abi = "controller()")]
    pub struct ControllerCall;
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
    #[doc = "Container type for all input parameters for the `crv_token`function with signature `crv_token()` and selector `[118, 216, 177, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "crv_token", abi = "crv_token()")]
    pub struct CrvTokenCall;
    #[doc = "Container type for all input parameters for the `curveLPtoken`function with signature `curveLPtoken()` and selector `[146, 113, 136, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "curveLPtoken", abi = "curveLPtoken()")]
    pub struct CurveLPtokenCall;
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256)` and selector `[182, 181, 95, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositCall {
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `extraReward1`function with signature `extraReward1()` and selector `[218, 91, 56, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "extraReward1", abi = "extraReward1()")]
    pub struct ExtraReward1Call;
    #[doc = "Container type for all input parameters for the `extraReward2`function with signature `extraReward2()` and selector `[151, 195, 65, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "extraReward2", abi = "extraReward2()")]
    pub struct ExtraReward2Call;
    #[doc = "Container type for all input parameters for the `future_epoch_time`function with signature `future_epoch_time()` and selector `[190, 93, 27, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "future_epoch_time", abi = "future_epoch_time()")]
    pub struct FutureEpochTimeCall;
    #[doc = "Container type for all input parameters for the `gauge`function with signature `gauge()` and selector `[166, 241, 156, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "gauge", abi = "gauge()")]
    pub struct GaugeCall;
    #[doc = "Container type for all input parameters for the `kick`function with signature `kick(address)` and selector `[150, 197, 81, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "kick", abi = "kick(address)")]
    pub struct KickCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `lp_token`function with signature `lp_token()` and selector `[130, 198, 48, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "lp_token", abi = "lp_token()")]
    pub struct LpTokenCall;
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
    #[doc = "Container type for all input parameters for the `reward_tokens`function with signature `reward_tokens(uint256)` and selector `[84, 196, 159, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "reward_tokens", abi = "reward_tokens(uint256)")]
    pub struct RewardTokensCall {
        pub i: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `set_approve_deposit`function with signature `set_approve_deposit(address,bool)` and selector `[29, 39, 71, 212]`"]
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
        name = "set_approve_deposit",
        abi = "set_approve_deposit(address,bool)"
    )]
    pub struct SetApproveDepositCall {
        pub addr: ethers::core::types::Address,
        pub can_deposit: bool,
    }
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
    #[doc = "Container type for all input parameters for the `user_checkpoint`function with signature `user_checkpoint(address)` and selector `[75, 130, 0, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "user_checkpoint", abi = "user_checkpoint(address)")]
    pub struct UserCheckpointCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `voting_escrow`function with signature `voting_escrow()` and selector `[223, 224, 80, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "voting_escrow", abi = "voting_escrow()")]
    pub struct VotingEscrowCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256)` and selector `[46, 26, 125, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ICurveV1AdapterGaugeCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        ClaimHistoricRewards(ClaimHistoricRewardsCall),
        ClaimRewards(ClaimRewardsCall),
        ClaimableReward(ClaimableRewardCall),
        ClaimableTokens(ClaimableTokensCall),
        Controller(ControllerCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        CrvToken(CrvTokenCall),
        CurveLPtoken(CurveLPtokenCall),
        Deposit(DepositCall),
        ExtraReward1(ExtraReward1Call),
        ExtraReward2(ExtraReward2Call),
        FutureEpochTime(FutureEpochTimeCall),
        Gauge(GaugeCall),
        Kick(KickCall),
        LpToken(LpTokenCall),
        Minter(MinterCall),
        RewardTokens(RewardTokensCall),
        SetApproveDeposit(SetApproveDepositCall),
        TargetContract(TargetContractCall),
        UserCheckpoint(UserCheckpointCall),
        VotingEscrow(VotingEscrowCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for ICurveV1AdapterGaugeCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <ClaimHistoricRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::ClaimHistoricRewards(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <ClaimableRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::ClaimableReward(decoded));
            }
            if let Ok(decoded) =
                <ClaimableTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::ClaimableTokens(decoded));
            }
            if let Ok(decoded) =
                <ControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::Controller(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::CreditManager(decoded));
            }
            if let Ok(decoded) =
                <CrvTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::CrvToken(decoded));
            }
            if let Ok(decoded) =
                <CurveLPtokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::CurveLPtoken(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExtraReward1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::ExtraReward1(decoded));
            }
            if let Ok(decoded) =
                <ExtraReward2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::ExtraReward2(decoded));
            }
            if let Ok(decoded) =
                <FutureEpochTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::FutureEpochTime(decoded));
            }
            if let Ok(decoded) = <GaugeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::Gauge(decoded));
            }
            if let Ok(decoded) = <KickCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ICurveV1AdapterGaugeCalls::Kick(decoded));
            }
            if let Ok(decoded) =
                <LpTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::LpToken(decoded));
            }
            if let Ok(decoded) = <MinterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::Minter(decoded));
            }
            if let Ok(decoded) =
                <RewardTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::RewardTokens(decoded));
            }
            if let Ok(decoded) =
                <SetApproveDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::SetApproveDeposit(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::TargetContract(decoded));
            }
            if let Ok(decoded) =
                <UserCheckpointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::UserCheckpoint(decoded));
            }
            if let Ok(decoded) =
                <VotingEscrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::VotingEscrow(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ICurveV1AdapterGaugeCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ICurveV1AdapterGaugeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ICurveV1AdapterGaugeCalls::GearboxAdapterType(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::GearboxAdapterVersion(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::ClaimHistoricRewards(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::ClaimRewards(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::ClaimableReward(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::ClaimableTokens(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::Controller(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::CreditFacade(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::CreditManager(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::CrvToken(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::CurveLPtoken(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::Deposit(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::ExtraReward1(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::ExtraReward2(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::FutureEpochTime(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::Gauge(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::Kick(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::LpToken(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::Minter(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::RewardTokens(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::SetApproveDeposit(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::TargetContract(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::UserCheckpoint(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::VotingEscrow(element) => element.encode(),
                ICurveV1AdapterGaugeCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ICurveV1AdapterGaugeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ICurveV1AdapterGaugeCalls::GearboxAdapterType(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::GearboxAdapterVersion(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::ClaimHistoricRewards(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::ClaimRewards(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::ClaimableReward(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::ClaimableTokens(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::Controller(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::CreditFacade(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::CreditManager(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::CrvToken(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::CurveLPtoken(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::Deposit(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::ExtraReward1(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::ExtraReward2(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::FutureEpochTime(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::Gauge(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::Kick(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::LpToken(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::Minter(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::RewardTokens(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::SetApproveDeposit(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::TargetContract(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::UserCheckpoint(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::VotingEscrow(element) => element.fmt(f),
                ICurveV1AdapterGaugeCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            ICurveV1AdapterGaugeCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            ICurveV1AdapterGaugeCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<ClaimHistoricRewardsCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: ClaimHistoricRewardsCall) -> Self {
            ICurveV1AdapterGaugeCalls::ClaimHistoricRewards(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: ClaimRewardsCall) -> Self {
            ICurveV1AdapterGaugeCalls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<ClaimableRewardCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: ClaimableRewardCall) -> Self {
            ICurveV1AdapterGaugeCalls::ClaimableReward(var)
        }
    }
    impl ::std::convert::From<ClaimableTokensCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: ClaimableTokensCall) -> Self {
            ICurveV1AdapterGaugeCalls::ClaimableTokens(var)
        }
    }
    impl ::std::convert::From<ControllerCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: ControllerCall) -> Self {
            ICurveV1AdapterGaugeCalls::Controller(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: CreditFacadeCall) -> Self {
            ICurveV1AdapterGaugeCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: CreditManagerCall) -> Self {
            ICurveV1AdapterGaugeCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CrvTokenCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: CrvTokenCall) -> Self {
            ICurveV1AdapterGaugeCalls::CrvToken(var)
        }
    }
    impl ::std::convert::From<CurveLPtokenCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: CurveLPtokenCall) -> Self {
            ICurveV1AdapterGaugeCalls::CurveLPtoken(var)
        }
    }
    impl ::std::convert::From<DepositCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: DepositCall) -> Self {
            ICurveV1AdapterGaugeCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<ExtraReward1Call> for ICurveV1AdapterGaugeCalls {
        fn from(var: ExtraReward1Call) -> Self {
            ICurveV1AdapterGaugeCalls::ExtraReward1(var)
        }
    }
    impl ::std::convert::From<ExtraReward2Call> for ICurveV1AdapterGaugeCalls {
        fn from(var: ExtraReward2Call) -> Self {
            ICurveV1AdapterGaugeCalls::ExtraReward2(var)
        }
    }
    impl ::std::convert::From<FutureEpochTimeCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: FutureEpochTimeCall) -> Self {
            ICurveV1AdapterGaugeCalls::FutureEpochTime(var)
        }
    }
    impl ::std::convert::From<GaugeCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: GaugeCall) -> Self {
            ICurveV1AdapterGaugeCalls::Gauge(var)
        }
    }
    impl ::std::convert::From<KickCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: KickCall) -> Self {
            ICurveV1AdapterGaugeCalls::Kick(var)
        }
    }
    impl ::std::convert::From<LpTokenCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: LpTokenCall) -> Self {
            ICurveV1AdapterGaugeCalls::LpToken(var)
        }
    }
    impl ::std::convert::From<MinterCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: MinterCall) -> Self {
            ICurveV1AdapterGaugeCalls::Minter(var)
        }
    }
    impl ::std::convert::From<RewardTokensCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: RewardTokensCall) -> Self {
            ICurveV1AdapterGaugeCalls::RewardTokens(var)
        }
    }
    impl ::std::convert::From<SetApproveDepositCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: SetApproveDepositCall) -> Self {
            ICurveV1AdapterGaugeCalls::SetApproveDeposit(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: TargetContractCall) -> Self {
            ICurveV1AdapterGaugeCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<UserCheckpointCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: UserCheckpointCall) -> Self {
            ICurveV1AdapterGaugeCalls::UserCheckpoint(var)
        }
    }
    impl ::std::convert::From<VotingEscrowCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: VotingEscrowCall) -> Self {
            ICurveV1AdapterGaugeCalls::VotingEscrow(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ICurveV1AdapterGaugeCalls {
        fn from(var: WithdrawCall) -> Self {
            ICurveV1AdapterGaugeCalls::Withdraw(var)
        }
    }
}
