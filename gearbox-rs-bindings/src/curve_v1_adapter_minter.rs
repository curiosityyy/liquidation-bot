pub use curvev1adapterminter_mod::*;
#[allow(clippy::too_many_arguments)]
mod curvev1adapterminter_mod {
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
    #[doc = "CurveV1AdapterMinter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVEV1ADAPTERMINTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_creditManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_minter\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressException\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"_gearboxAdapterType\",\"outputs\":[{\"internalType\":\"enum AdapterType\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_gearboxAdapterVersion\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditFacade\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"creditManager\",\"outputs\":[{\"internalType\":\"contract ICreditManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"crv\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"gauge_addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContract\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CURVEV1ADAPTERMINTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60e060405234801561001057600080fd5b5060405161080838038061080883398101604081905261002f916101b9565b81816001600160a01b038216158061004e57506001600160a01b038116155b1561006c57604051635919af9760e11b815260040160405180910390fd5b6001600160a01b038216608081905260408051632f7a188160e01b81529051632f7a1881916004808201926020929091908290030181865afa1580156100b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100da91906101ec565b6001600160a01b0390811660a052600080546001600160a01b031916928216929092179091556001805582161515905061012757604051635919af9760e11b815260040160405180910390fd5b806001600160a01b031663fc0c546a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610165573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061018991906101ec565b6001600160a01b031660c0525061020e9050565b80516001600160a01b03811681146101b457600080fd5b919050565b600080604083850312156101cc57600080fd5b6101d58361019d565b91506101e36020840161019d565b90509250929050565b6000602082840312156101fe57600080fd5b6102078261019d565b9392505050565b60805160a05160c0516105b16102576000396000818160d601526102c3015260006092015260008181610140015281816101910152818161022201526102eb01526105b16000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c8063bd90df701161005b578063bd90df7014610128578063c12c21c01461013b578063ce30bbdb14610162578063fc0c546a1461017157600080fd5b80632f7a18811461008d5780636a4874a1146100d15780636a627842146100f857806378aa73a41461010d575b600080fd5b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b61010b6101063660046103e0565b610179565b005b610115600181565b60405161ffff90911681526020016100c8565b6000546100b4906001600160a01b031681565b6100b47f000000000000000000000000000000000000000000000000000000000000000081565b60086040516100c89190610404565b6100b461034b565b604051633a562dc160e21b81523360048201526000907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063e958b70490602401602060405180830381865afa1580156101e0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610204919061042c565b6000805460405163367203a560e11b81529293506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811693636ce4074a9361025c93339316913690600401610449565b6000604051808303816000875af115801561027b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526102a391908101906104ab565b5060405163028f1f8b60e51b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000811660248301527f000000000000000000000000000000000000000000000000000000000000000016906351e3f16090604401600060405180830381600087803b15801561032f57600080fd5b505af1158015610343573d6000803e3d6000fd5b505050505050565b60008060009054906101000a90046001600160a01b03166001600160a01b031663fc0c546a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561039f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103c3919061042c565b905090565b6001600160a01b03811681146103dd57600080fd5b50565b6000602082840312156103f257600080fd5b81356103fd816103c8565b9392505050565b60208101600e831061042657634e487b7160e01b600052602160045260246000fd5b91905290565b60006020828403121561043e57600080fd5b81516103fd816103c8565b6001600160a01b0385811682528416602082015260606040820181905281018290526000828460808401376000608084840101526080601f19601f850116830101905095945050505050565b634e487b7160e01b600052604160045260246000fd5b600060208083850312156104be57600080fd5b825167ffffffffffffffff808211156104d657600080fd5b818501915085601f8301126104ea57600080fd5b8151818111156104fc576104fc610495565b604051601f8201601f19908116603f0116810190838211818310171561052457610524610495565b81604052828152888684870101111561053c57600080fd5b600093505b8284101561055e5784840186015181850187015292850192610541565b8284111561056f5760008684830101525b9897505050505050505056fea2646970667358221220d0d4e23311c2d203c9d4a8cacf6117eebbabe5e694c4f6827b046b51b5dee01a64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct CurveV1AdapterMinter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveV1AdapterMinter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveV1AdapterMinter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveV1AdapterMinter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveV1AdapterMinter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                CURVEV1ADAPTERMINTER_ABI.clone(),
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
                CURVEV1ADAPTERMINTER_ABI.clone(),
                CURVEV1ADAPTERMINTER_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `mint` (0x6a627842) function"]
        pub fn mint(
            &self,
            gauge_addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 98, 120, 66], gauge_addr)
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
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for CurveV1AdapterMinter<M>
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
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address)` and selector `[106, 98, 120, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address)")]
    pub struct MintCall {
        pub gauge_addr: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CurveV1AdapterMinterCalls {
        GearboxAdapterType(GearboxAdapterTypeCall),
        GearboxAdapterVersion(GearboxAdapterVersionCall),
        CreditFacade(CreditFacadeCall),
        CreditManager(CreditManagerCall),
        Crv(CrvCall),
        Mint(MintCall),
        TargetContract(TargetContractCall),
        Token(TokenCall),
    }
    impl ethers::core::abi::AbiDecode for CurveV1AdapterMinterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GearboxAdapterTypeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterMinterCalls::GearboxAdapterType(decoded));
            }
            if let Ok(decoded) =
                <GearboxAdapterVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterMinterCalls::GearboxAdapterVersion(decoded));
            }
            if let Ok(decoded) =
                <CreditFacadeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterMinterCalls::CreditFacade(decoded));
            }
            if let Ok(decoded) =
                <CreditManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterMinterCalls::CreditManager(decoded));
            }
            if let Ok(decoded) = <CrvCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CurveV1AdapterMinterCalls::Crv(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CurveV1AdapterMinterCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <TargetContractCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterMinterCalls::TargetContract(decoded));
            }
            if let Ok(decoded) = <TokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveV1AdapterMinterCalls::Token(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveV1AdapterMinterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveV1AdapterMinterCalls::GearboxAdapterType(element) => element.encode(),
                CurveV1AdapterMinterCalls::GearboxAdapterVersion(element) => element.encode(),
                CurveV1AdapterMinterCalls::CreditFacade(element) => element.encode(),
                CurveV1AdapterMinterCalls::CreditManager(element) => element.encode(),
                CurveV1AdapterMinterCalls::Crv(element) => element.encode(),
                CurveV1AdapterMinterCalls::Mint(element) => element.encode(),
                CurveV1AdapterMinterCalls::TargetContract(element) => element.encode(),
                CurveV1AdapterMinterCalls::Token(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveV1AdapterMinterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveV1AdapterMinterCalls::GearboxAdapterType(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::GearboxAdapterVersion(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::CreditFacade(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::CreditManager(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::Crv(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::Mint(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::TargetContract(element) => element.fmt(f),
                CurveV1AdapterMinterCalls::Token(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GearboxAdapterTypeCall> for CurveV1AdapterMinterCalls {
        fn from(var: GearboxAdapterTypeCall) -> Self {
            CurveV1AdapterMinterCalls::GearboxAdapterType(var)
        }
    }
    impl ::std::convert::From<GearboxAdapterVersionCall> for CurveV1AdapterMinterCalls {
        fn from(var: GearboxAdapterVersionCall) -> Self {
            CurveV1AdapterMinterCalls::GearboxAdapterVersion(var)
        }
    }
    impl ::std::convert::From<CreditFacadeCall> for CurveV1AdapterMinterCalls {
        fn from(var: CreditFacadeCall) -> Self {
            CurveV1AdapterMinterCalls::CreditFacade(var)
        }
    }
    impl ::std::convert::From<CreditManagerCall> for CurveV1AdapterMinterCalls {
        fn from(var: CreditManagerCall) -> Self {
            CurveV1AdapterMinterCalls::CreditManager(var)
        }
    }
    impl ::std::convert::From<CrvCall> for CurveV1AdapterMinterCalls {
        fn from(var: CrvCall) -> Self {
            CurveV1AdapterMinterCalls::Crv(var)
        }
    }
    impl ::std::convert::From<MintCall> for CurveV1AdapterMinterCalls {
        fn from(var: MintCall) -> Self {
            CurveV1AdapterMinterCalls::Mint(var)
        }
    }
    impl ::std::convert::From<TargetContractCall> for CurveV1AdapterMinterCalls {
        fn from(var: TargetContractCall) -> Self {
            CurveV1AdapterMinterCalls::TargetContract(var)
        }
    }
    impl ::std::convert::From<TokenCall> for CurveV1AdapterMinterCalls {
        fn from(var: TokenCall) -> Self {
            CurveV1AdapterMinterCalls::Token(var)
        }
    }
}
