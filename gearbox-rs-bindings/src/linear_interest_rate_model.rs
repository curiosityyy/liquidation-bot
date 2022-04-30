pub use linearinterestratemodel_mod::*;
#[allow(clippy::too_many_arguments)]
mod linearinterestratemodel_mod {
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
    #[doc = "LinearInterestRateModel was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LINEARINTERESTRATEMODEL_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"U_optimal\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"R_base\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"R_slope1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"R_slope2\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_R_base_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_R_slope1_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_R_slope2_RAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_U_Optimal_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_U_Optimal_inverted_WAD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"expectedLiquidity\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"availableLiquidity\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calcBorrowRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getModelParameters\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"U_optimal\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"R_base\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"R_slope1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"R_slope2\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LINEARINTERESTRATEMODEL_BYTECODE: ethers::contract::Lazy<
        ethers::core::types::Bytes,
    > = ethers::contract::Lazy::new(|| {
        "0x6101206040523480156200001257600080fd5b5060405162000a1138038062000a1183398101604081905262000035916200021c565b604080518082019091526002815261049560f41b60208201526127108511156200007d5760405162461bcd60e51b815260040162000074919062000253565b60405180910390fd5b50604080518082019091526002815261049560f41b6020820152612710841115620000bd5760405162461bcd60e51b815260040162000074919062000253565b50604080518082019091526002815261049560f41b6020820152612710831115620000fd5760405162461bcd60e51b815260040162000074919062000253565b5060006200012285670de0b6b3a7640000620001c260201b6200043c1790919060201c565b608081905290506200013d81670de0b6b3a7640000620002c1565b60a052620001646b033b2e3c9fd0803ce800000085620001c2602090811b6200043c17901c565b60c0526200018b6b033b2e3c9fd0803ce800000084620001c2602090811b6200043c17901c565b60e052620001b26b033b2e3c9fd0803ce800000083620001c2602090811b6200043c17901c565b61010052506200033b9350505050565b6000821580620001d0575081155b15620001df5750600062000216565b612710620001ef600282620002db565b620001fb8486620002fe565b62000207919062000320565b620002139190620002db565b90505b92915050565b600080600080608085870312156200023357600080fd5b505082516020840151604085015160609095015191969095509092509050565b600060208083528351808285015260005b81811015620002825785810183015185820160400152820162000264565b8181111562000295576000604083870101525b50601f01601f1916929092016040019392505050565b634e487b7160e01b600052601160045260246000fd5b600082821015620002d657620002d6620002ab565b500390565b600082620002f957634e487b7160e01b600052601260045260246000fd5b500490565b60008160001904831182151516156200031b576200031b620002ab565b500290565b60008219821115620003365762000336620002ab565b500190565b60805160a05160c05160e05161010051610631620003e0600039600081816101840152818161030b015261041701526000818160e70152818161025a0152818161033e01526103f401526000818161010e015281816101bc0152818161028e0152818161035f01526103d101526000818161015d01526102bc01526000818160b80152818161020d01528181610235015281816102e001526103a101526106316000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c80639cd3fdb51161005b5780639cd3fdb514610109578063c8284e6d14610130578063f81d438114610158578063fc4b2b781461017f57600080fd5b806342568d441461008d57806350ced104146100b357806354fd4d50146100da5780639aec06ea146100e2575b600080fd5b6100a061009b3660046104fe565b6101a6565b6040519081526020015b60405180910390f35b6100a07f000000000000000000000000000000000000000000000000000000000000000081565b6100a0600181565b6100a07f000000000000000000000000000000000000000000000000000000000000000081565b6100a07f000000000000000000000000000000000000000000000000000000000000000081565b610138610397565b6040805194855260208501939093529183015260608201526080016100aa565b6100a07f000000000000000000000000000000000000000000000000000000000000000081565b6100a07f000000000000000000000000000000000000000000000000000000000000000081565b60008215806101b457508183105b156101e057507f0000000000000000000000000000000000000000000000000000000000000000610391565b6000836101ed8482610536565b6101ff90670de0b6b3a764000061054d565b610209919061056c565b90507f00000000000000000000000000000000000000000000000000000000000000008110156102ba577f000000000000000000000000000000000000000000000000000000000000000061027e827f000000000000000000000000000000000000000000000000000000000000000061054d565b610288919061056c565b6102b2907f000000000000000000000000000000000000000000000000000000000000000061058e565b915050610391565b7f00000000000000000000000000000000000000000000000000000000000000006103057f000000000000000000000000000000000000000000000000000000000000000083610536565b61032f907f000000000000000000000000000000000000000000000000000000000000000061054d565b610339919061056c565b6103837f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000061058e565b61038d919061058e565b9150505b92915050565b60008080806103ce7f0000000000000000000000000000000000000000000000000000000000000000670de0b6b3a7640000610489565b947f000000000000000000000000000000000000000000000000000000000000000094507f000000000000000000000000000000000000000000000000000000000000000093507f000000000000000000000000000000000000000000000000000000000000000092509050565b6000821580610449575081155b1561045657506000610391565b61271061046460028261056c565b61046e848661054d565b610478919061058e565b610482919061056c565b9392505050565b6040805180820190915260028152614d3360f01b6020820152600090826104cc5760405162461bcd60e51b81526004016104c391906105a6565b60405180910390fd5b5060006104da60028461056c565b905082816104ea6127108761054d565b6104f4919061058e565b61038d919061056c565b6000806040838503121561051157600080fd5b50508035926020909101359150565b634e487b7160e01b600052601160045260246000fd5b60008282101561054857610548610520565b500390565b600081600019048311821515161561056757610567610520565b500290565b60008261058957634e487b7160e01b600052601260045260246000fd5b500490565b600082198211156105a1576105a1610520565b500190565b600060208083528351808285015260005b818110156105d3578581018301518582016040015282016105b7565b818111156105e5576000604083870101525b50601f01601f191692909201604001939250505056fea2646970667358221220f2af4f39514065786ab8cccdeae0b64142810ea39993ad1dba919f69d249e2aa64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
    });
    #[derive(Clone)]
    pub struct LinearInterestRateModel<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for LinearInterestRateModel<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LinearInterestRateModel<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LinearInterestRateModel))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> LinearInterestRateModel<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                LINEARINTERESTRATEMODEL_ABI.clone(),
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
                LINEARINTERESTRATEMODEL_ABI.clone(),
                LINEARINTERESTRATEMODEL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `_R_base_RAY` (0x9cd3fdb5) function"]
        pub fn r_base_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([156, 211, 253, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_R_slope1_RAY` (0x9aec06ea) function"]
        pub fn r_slope_1_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([154, 236, 6, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_R_slope2_RAY` (0xfc4b2b78) function"]
        pub fn r_slope_2_ray(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 75, 43, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_U_Optimal_WAD` (0x50ced104) function"]
        pub fn u_optimal_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([80, 206, 209, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_U_Optimal_inverted_WAD` (0xf81d4381) function"]
        pub fn u_optimal_inverted_wad(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([248, 29, 67, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcBorrowRate` (0x42568d44) function"]
        pub fn calc_borrow_rate(
            &self,
            expected_liquidity: ethers::core::types::U256,
            available_liquidity: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([66, 86, 141, 68], (expected_liquidity, available_liquidity))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getModelParameters` (0xc8284e6d) function"]
        pub fn get_model_parameters(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([200, 40, 78, 109], ())
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for LinearInterestRateModel<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `_R_base_RAY`function with signature `_R_base_RAY()` and selector `[156, 211, 253, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_R_base_RAY", abi = "_R_base_RAY()")]
    pub struct RBaseRAYCall;
    #[doc = "Container type for all input parameters for the `_R_slope1_RAY`function with signature `_R_slope1_RAY()` and selector `[154, 236, 6, 234]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_R_slope1_RAY", abi = "_R_slope1_RAY()")]
    pub struct RSlope1RAYCall;
    #[doc = "Container type for all input parameters for the `_R_slope2_RAY`function with signature `_R_slope2_RAY()` and selector `[252, 75, 43, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_R_slope2_RAY", abi = "_R_slope2_RAY()")]
    pub struct RSlope2RAYCall;
    #[doc = "Container type for all input parameters for the `_U_Optimal_WAD`function with signature `_U_Optimal_WAD()` and selector `[80, 206, 209, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_U_Optimal_WAD", abi = "_U_Optimal_WAD()")]
    pub struct UOptimalWADCall;
    #[doc = "Container type for all input parameters for the `_U_Optimal_inverted_WAD`function with signature `_U_Optimal_inverted_WAD()` and selector `[248, 29, 67, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_U_Optimal_inverted_WAD", abi = "_U_Optimal_inverted_WAD()")]
    pub struct UOptimalInvertedWADCall;
    #[doc = "Container type for all input parameters for the `calcBorrowRate`function with signature `calcBorrowRate(uint256,uint256)` and selector `[66, 86, 141, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "calcBorrowRate", abi = "calcBorrowRate(uint256,uint256)")]
    pub struct CalcBorrowRateCall {
        pub expected_liquidity: ethers::core::types::U256,
        pub available_liquidity: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getModelParameters`function with signature `getModelParameters()` and selector `[200, 40, 78, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getModelParameters", abi = "getModelParameters()")]
    pub struct GetModelParametersCall;
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
    pub enum LinearInterestRateModelCalls {
        RBaseRAY(RBaseRAYCall),
        RSlope1RAY(RSlope1RAYCall),
        RSlope2RAY(RSlope2RAYCall),
        UOptimalWAD(UOptimalWADCall),
        UOptimalInvertedWAD(UOptimalInvertedWADCall),
        CalcBorrowRate(CalcBorrowRateCall),
        GetModelParameters(GetModelParametersCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for LinearInterestRateModelCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <RBaseRAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::RBaseRAY(decoded));
            }
            if let Ok(decoded) =
                <RSlope1RAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::RSlope1RAY(decoded));
            }
            if let Ok(decoded) =
                <RSlope2RAYCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::RSlope2RAY(decoded));
            }
            if let Ok(decoded) =
                <UOptimalWADCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::UOptimalWAD(decoded));
            }
            if let Ok(decoded) =
                <UOptimalInvertedWADCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::UOptimalInvertedWAD(decoded));
            }
            if let Ok(decoded) =
                <CalcBorrowRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::CalcBorrowRate(decoded));
            }
            if let Ok(decoded) =
                <GetModelParametersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::GetModelParameters(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LinearInterestRateModelCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LinearInterestRateModelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LinearInterestRateModelCalls::RBaseRAY(element) => element.encode(),
                LinearInterestRateModelCalls::RSlope1RAY(element) => element.encode(),
                LinearInterestRateModelCalls::RSlope2RAY(element) => element.encode(),
                LinearInterestRateModelCalls::UOptimalWAD(element) => element.encode(),
                LinearInterestRateModelCalls::UOptimalInvertedWAD(element) => element.encode(),
                LinearInterestRateModelCalls::CalcBorrowRate(element) => element.encode(),
                LinearInterestRateModelCalls::GetModelParameters(element) => element.encode(),
                LinearInterestRateModelCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LinearInterestRateModelCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LinearInterestRateModelCalls::RBaseRAY(element) => element.fmt(f),
                LinearInterestRateModelCalls::RSlope1RAY(element) => element.fmt(f),
                LinearInterestRateModelCalls::RSlope2RAY(element) => element.fmt(f),
                LinearInterestRateModelCalls::UOptimalWAD(element) => element.fmt(f),
                LinearInterestRateModelCalls::UOptimalInvertedWAD(element) => element.fmt(f),
                LinearInterestRateModelCalls::CalcBorrowRate(element) => element.fmt(f),
                LinearInterestRateModelCalls::GetModelParameters(element) => element.fmt(f),
                LinearInterestRateModelCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<RBaseRAYCall> for LinearInterestRateModelCalls {
        fn from(var: RBaseRAYCall) -> Self {
            LinearInterestRateModelCalls::RBaseRAY(var)
        }
    }
    impl ::std::convert::From<RSlope1RAYCall> for LinearInterestRateModelCalls {
        fn from(var: RSlope1RAYCall) -> Self {
            LinearInterestRateModelCalls::RSlope1RAY(var)
        }
    }
    impl ::std::convert::From<RSlope2RAYCall> for LinearInterestRateModelCalls {
        fn from(var: RSlope2RAYCall) -> Self {
            LinearInterestRateModelCalls::RSlope2RAY(var)
        }
    }
    impl ::std::convert::From<UOptimalWADCall> for LinearInterestRateModelCalls {
        fn from(var: UOptimalWADCall) -> Self {
            LinearInterestRateModelCalls::UOptimalWAD(var)
        }
    }
    impl ::std::convert::From<UOptimalInvertedWADCall> for LinearInterestRateModelCalls {
        fn from(var: UOptimalInvertedWADCall) -> Self {
            LinearInterestRateModelCalls::UOptimalInvertedWAD(var)
        }
    }
    impl ::std::convert::From<CalcBorrowRateCall> for LinearInterestRateModelCalls {
        fn from(var: CalcBorrowRateCall) -> Self {
            LinearInterestRateModelCalls::CalcBorrowRate(var)
        }
    }
    impl ::std::convert::From<GetModelParametersCall> for LinearInterestRateModelCalls {
        fn from(var: GetModelParametersCall) -> Self {
            LinearInterestRateModelCalls::GetModelParameters(var)
        }
    }
    impl ::std::convert::From<VersionCall> for LinearInterestRateModelCalls {
        fn from(var: VersionCall) -> Self {
            LinearInterestRateModelCalls::Version(var)
        }
    }
}
