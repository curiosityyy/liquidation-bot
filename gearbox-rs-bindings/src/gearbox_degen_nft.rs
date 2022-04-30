pub use gearboxdegennft_mod::*;
#[allow(clippy::too_many_arguments)]
mod gearboxdegennft_mod {
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
    #[doc = "GearboxDegenNFT was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GEARBOXDEGENNFT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_addressProvider\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"GearboxDegenManagersOnlyException\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotImplementedException\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_manager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_manager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static GEARBOXDEGENNFT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a06040523480156200001157600080fd5b50604051620016c7380380620016c783398101604081905262000034916200031b565b604080518082018252601181527011d9585c989bde08111959d95b88139195607a1b60208083019182528351808501909452600a84526923a2a0a916a222a3a2a760b11b90840152815184939162000090916000919062000275565b508051620000a690600190602084019062000275565b50506006805460ff191690555060408051808201909152600281526105a360f41b60208201526001600160a01b038216620000ff5760405162461bcd60e51b8152600401620000f691906200034d565b60405180910390fd5b50806001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200013f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200016591906200031b565b6001600160a01b03166080816001600160a01b031681525050506000816001600160a01b031663087376956040518163ffffffff1660e01b8152600401602060405180830381865afa158015620001c0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001e691906200031b565b6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801562000224573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200024a91906200031b565b6001600160a01b03166000908152600760205260409020805460ff1916600117905550620003e29050565b8280546200028390620003a5565b90600052602060002090601f016020900481019282620002a75760008555620002f2565b82601f10620002c257805160ff1916838001178555620002f2565b82800160010185558215620002f2579182015b82811115620002f2578251825591602001919060010190620002d5565b506200030092915062000304565b5090565b5b8082111562000300576000815560010162000305565b6000602082840312156200032e57600080fd5b81516001600160a01b03811681146200034657600080fd5b9392505050565b600060208083528351808285015260005b818110156200037c578581018301518582016040015282016200035e565b818111156200038f576000604083870101525b50601f01601f1916929092016040019392505050565b600181811c90821680620003ba57607f821691505b60208210811415620003dc57634e487b7160e01b600052602260045260246000fd5b50919050565b6080516112b462000413600039600081816104760152818161055e015281816107a4015261087f01526112b46000f3fe608060405234801561001057600080fd5b506004361061012c5760003560e01c80635c975abb116100ad578063a22cb46511610071578063a22cb4651461024c578063ac18de431461025a578063b88d4fde1461026d578063c87b56dd1461027b578063e985e9c51461028e57600080fd5b80635c975abb146101fd5780636352211e1461020857806370a082311461021b5780638456cb591461023c57806395d89b411461024457600080fd5b80632d06177a116100f45780632d06177a146101bc5780633f4ba83a146101cf57806340c10f19146101d757806342842e0e146101ae57806342966c68146101ea57600080fd5b806301ffc9a71461013157806306fdde0314610159578063081812fc1461016e578063095ea7b31461019957806323b872dd146101ae575b600080fd5b61014461013f366004610e96565b6102ca565b60405190151581526020015b60405180910390f35b61016161031c565b6040516101509190610ef0565b61018161017c366004610f23565b6103ae565b6040516001600160a01b039091168152602001610150565b6101ac6101a7366004610f58565b610448565b005b6101ac6101a7366004610f82565b6101ac6101ca366004610fbe565b610461565b6101ac610549565b6101ac6101e5366004610f58565b610617565b6101ac6101f8366004610f23565b610655565b60065460ff16610144565b610181610216366004610f23565b610691565b61022e610229366004610fbe565b610708565b604051908152602001610150565b6101ac61078f565b61016161085b565b6101ac6101a7366004610fe7565b6101ac610268366004610fbe565b61086a565b6101ac6101a7366004611034565b610161610289366004610f23565b61094f565b61014461029c366004611110565b6001600160a01b03918216600090815260056020908152604080832093909416825291909152205460ff1690565b60006001600160e01b031982166380ac58cd60e01b14806102fb57506001600160e01b03198216635b5e139f60e01b145b8061031657506301ffc9a760e01b6001600160e01b03198316145b92915050565b60606000805461032b90611143565b80601f016020809104026020016040519081016040528092919081815260200182805461035790611143565b80156103a45780601f10610379576101008083540402835291602001916103a4565b820191906000526020600020905b81548152906001019060200180831161038757829003601f168201915b5050505050905090565b6000818152600260205260408120546001600160a01b031661042c5760405162461bcd60e51b815260206004820152602c60248201527f4552433732313a20617070726f76656420717565727920666f72206e6f6e657860448201526b34b9ba32b73a103a37b5b2b760a11b60648201526084015b60405180910390fd5b506000908152600460205260409020546001600160a01b031690565b60405163024e46f760e41b815260040160405180910390fd5b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa1580156104c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104e9919061117e565b6040518060400160405280600481526020016320a1a61960e11b815250906105245760405162461bcd60e51b81526004016104239190610ef0565b506001600160a01b03166000908152600760205260409020805460ff19166001179055565b604051630d4eb5db60e41b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d4eb5db090602401602060405180830381865afa1580156105ad573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105d1919061117e565b6040518060400160405280600481526020016341434c3160e01b8152509061060c5760405162461bcd60e51b81526004016104239190610ef0565b50610615610a37565b565b3360009081526007602052604090205460ff16610647576040516337f5ad8760e21b815260040160405180910390fd5b6106518282610aca565b5050565b3360009081526007602052604090205460ff16610685576040516337f5ad8760e21b815260040160405180910390fd5b61068e81610c0c565b50565b6000818152600260205260408120546001600160a01b0316806103165760405162461bcd60e51b815260206004820152602960248201527f4552433732313a206f776e657220717565727920666f72206e6f6e657869737460448201526832b73a103a37b5b2b760b91b6064820152608401610423565b60006001600160a01b0382166107735760405162461bcd60e51b815260206004820152602a60248201527f4552433732313a2062616c616e636520717565727920666f7220746865207a65604482015269726f206164647265737360b01b6064820152608401610423565b506001600160a01b031660009081526003602052604090205490565b604051630e907b1960e21b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690633a41ec6490602401602060405180830381865afa1580156107f3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610817919061117e565b6040518060400160405280600481526020016341434c3160e01b815250906108525760405162461bcd60e51b81526004016104239190610ef0565b50610615610ca7565b60606001805461032b90611143565b604051632f92cd5d60e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635f259aba90602401602060405180830381865afa1580156108ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108f2919061117e565b6040518060400160405280600481526020016320a1a61960e11b8152509061092d5760405162461bcd60e51b81526004016104239190610ef0565b506001600160a01b03166000908152600760205260409020805460ff19169055565b6000818152600260205260409020546060906001600160a01b03166109ce5760405162461bcd60e51b815260206004820152602f60248201527f4552433732314d657461646174613a2055524920717565727920666f72206e6f60448201526e3732bc34b9ba32b73a103a37b5b2b760891b6064820152608401610423565b60006109e560408051602081019091526000815290565b90506000815111610a055760405180602001604052806000815250610a30565b80610a0f84610d22565b604051602001610a2092919061119b565b6040516020818303038152906040525b9392505050565b60065460ff16610a805760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b6044820152606401610423565b6006805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6001600160a01b038216610b205760405162461bcd60e51b815260206004820181905260248201527f4552433732313a206d696e7420746f20746865207a65726f20616464726573736044820152606401610423565b6000818152600260205260409020546001600160a01b031615610b855760405162461bcd60e51b815260206004820152601c60248201527f4552433732313a20746f6b656e20616c7265616479206d696e746564000000006044820152606401610423565b6001600160a01b0382166000908152600360205260408120805460019290610bae9084906111e0565b909155505060008181526002602052604080822080546001600160a01b0319166001600160a01b03861690811790915590518392907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef908290a45050565b6000610c1782610691565b9050610c24600083610e28565b6001600160a01b0381166000908152600360205260408120805460019290610c4d9084906111f8565b909155505060008281526002602052604080822080546001600160a01b0319169055518391906001600160a01b038416907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef908390a45050565b60065460ff1615610ced5760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b6044820152606401610423565b6006805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258610aad3390565b606081610d465750506040805180820190915260018152600360fc1b602082015290565b8160005b8115610d705780610d5a8161120f565b9150610d699050600a83611240565b9150610d4a565b60008167ffffffffffffffff811115610d8b57610d8b61101e565b6040519080825280601f01601f191660200182016040528015610db5576020820181803683370190505b5090505b8415610e2057610dca6001836111f8565b9150610dd7600a86611254565b610de29060306111e0565b60f81b818381518110610df757610df7611268565b60200101906001600160f81b031916908160001a905350610e19600a86611240565b9450610db9565b949350505050565b600081815260046020526040902080546001600160a01b0319166001600160a01b0384169081179091558190610e5d82610691565b6001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45050565b600060208284031215610ea857600080fd5b81356001600160e01b031981168114610a3057600080fd5b60005b83811015610edb578181015183820152602001610ec3565b83811115610eea576000848401525b50505050565b6020815260008251806020840152610f0f816040850160208701610ec0565b601f01601f19169190910160400192915050565b600060208284031215610f3557600080fd5b5035919050565b80356001600160a01b0381168114610f5357600080fd5b919050565b60008060408385031215610f6b57600080fd5b610f7483610f3c565b946020939093013593505050565b600080600060608486031215610f9757600080fd5b610fa084610f3c565b9250610fae60208501610f3c565b9150604084013590509250925092565b600060208284031215610fd057600080fd5b610a3082610f3c565b801515811461068e57600080fd5b60008060408385031215610ffa57600080fd5b61100383610f3c565b9150602083013561101381610fd9565b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b6000806000806080858703121561104a57600080fd5b61105385610f3c565b935061106160208601610f3c565b925060408501359150606085013567ffffffffffffffff8082111561108557600080fd5b818701915087601f83011261109957600080fd5b8135818111156110ab576110ab61101e565b604051601f8201601f19908116603f011681019083821181831017156110d3576110d361101e565b816040528281528a60208487010111156110ec57600080fd5b82602086016020830137600060208483010152809550505050505092959194509250565b6000806040838503121561112357600080fd5b61112c83610f3c565b915061113a60208401610f3c565b90509250929050565b600181811c9082168061115757607f821691505b6020821081141561117857634e487b7160e01b600052602260045260246000fd5b50919050565b60006020828403121561119057600080fd5b8151610a3081610fd9565b600083516111ad818460208801610ec0565b8351908301906111c1818360208801610ec0565b01949350505050565b634e487b7160e01b600052601160045260246000fd5b600082198211156111f3576111f36111ca565b500190565b60008282101561120a5761120a6111ca565b500390565b6000600019821415611223576112236111ca565b5060010190565b634e487b7160e01b600052601260045260246000fd5b60008261124f5761124f61122a565b500490565b6000826112635761126361122a565b500690565b634e487b7160e01b600052603260045260246000fdfea2646970667358221220a0e83659dd164957dada9dd1382d7737786d12091b92104d4fee77ad52f2397664736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct GearboxDegenNFT<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for GearboxDegenNFT<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for GearboxDegenNFT<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(GearboxDegenNFT))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> GearboxDegenNFT<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GEARBOXDEGENNFT_ABI.clone(), client)
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
                GEARBOXDEGENNFT_ABI.clone(),
                GEARBOXDEGENNFT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `addManager` (0x2d06177a) function"]
        pub fn add_manager(
            &self,
            manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 6, 23, 122], manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x42966c68) function"]
        pub fn burn(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeManager` (0xac18de43) function"]
        pub fn remove_manager(
            &self,
            manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 24, 222, 67], manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0x42842e0e) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xb88d4fde) function"]
        pub fn safe_transfer_from_with_data(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenURI` (0xc87b56dd) function"]
        pub fn token_uri(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, GearboxDegenNFTEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for GearboxDegenNFT<M> {
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
        pub approved: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers::core::types::Address,
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
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GearboxDegenNFTEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        PausedFilter(PausedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for GearboxDegenNFTEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(GearboxDegenNFTEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(GearboxDegenNFTEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GearboxDegenNFTEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(GearboxDegenNFTEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(GearboxDegenNFTEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for GearboxDegenNFTEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GearboxDegenNFTEvents::ApprovalFilter(element) => element.fmt(f),
                GearboxDegenNFTEvents::ApprovalForAllFilter(element) => element.fmt(f),
                GearboxDegenNFTEvents::PausedFilter(element) => element.fmt(f),
                GearboxDegenNFTEvents::TransferFilter(element) => element.fmt(f),
                GearboxDegenNFTEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addManager`function with signature `addManager(address)` and selector `[45, 6, 23, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addManager", abi = "addManager(address)")]
    pub struct AddManagerCall {
        pub manager: ethers::core::types::Address,
    }
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
    pub struct ApproveCall {
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
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
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(uint256)` and selector `[66, 150, 108, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getApproved`function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll`function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `ownerOf`function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `pause`function with signature `pause()` and selector `[132, 86, 203, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused`function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `removeManager`function with signature `removeManager(address)` and selector `[172, 24, 222, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeManager", abi = "removeManager(address)")]
    pub struct RemoveManagerCall {
        pub manager: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom`function with signature `safeTransferFrom(address,address,uint256)` and selector `[66, 132, 46, 14]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom`function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `[184, 141, 79, 222]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithDataCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll`function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface`function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
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
    #[doc = "Container type for all input parameters for the `tokenURI`function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ethers::core::types::U256,
    }
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
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unpause`function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GearboxDegenNFTCalls {
        AddManager(AddManagerCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Mint(MintCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        Pause(PauseCall),
        Paused(PausedCall),
        RemoveManager(RemoveManagerCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithData(SafeTransferFromWithDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
    }
    impl ethers::core::abi::AbiDecode for GearboxDegenNFTCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::AddManager(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(GearboxDegenNFTCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(GearboxDegenNFTCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(GearboxDegenNFTCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::OwnerOf(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RemoveManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::RemoveManager(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GearboxDegenNFTCalls::SafeTransferFromWithData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearboxDegenNFTCalls::Unpause(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GearboxDegenNFTCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GearboxDegenNFTCalls::AddManager(element) => element.encode(),
                GearboxDegenNFTCalls::Approve(element) => element.encode(),
                GearboxDegenNFTCalls::BalanceOf(element) => element.encode(),
                GearboxDegenNFTCalls::Burn(element) => element.encode(),
                GearboxDegenNFTCalls::GetApproved(element) => element.encode(),
                GearboxDegenNFTCalls::IsApprovedForAll(element) => element.encode(),
                GearboxDegenNFTCalls::Mint(element) => element.encode(),
                GearboxDegenNFTCalls::Name(element) => element.encode(),
                GearboxDegenNFTCalls::OwnerOf(element) => element.encode(),
                GearboxDegenNFTCalls::Pause(element) => element.encode(),
                GearboxDegenNFTCalls::Paused(element) => element.encode(),
                GearboxDegenNFTCalls::RemoveManager(element) => element.encode(),
                GearboxDegenNFTCalls::SafeTransferFrom(element) => element.encode(),
                GearboxDegenNFTCalls::SafeTransferFromWithData(element) => element.encode(),
                GearboxDegenNFTCalls::SetApprovalForAll(element) => element.encode(),
                GearboxDegenNFTCalls::SupportsInterface(element) => element.encode(),
                GearboxDegenNFTCalls::Symbol(element) => element.encode(),
                GearboxDegenNFTCalls::TokenURI(element) => element.encode(),
                GearboxDegenNFTCalls::TransferFrom(element) => element.encode(),
                GearboxDegenNFTCalls::Unpause(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GearboxDegenNFTCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GearboxDegenNFTCalls::AddManager(element) => element.fmt(f),
                GearboxDegenNFTCalls::Approve(element) => element.fmt(f),
                GearboxDegenNFTCalls::BalanceOf(element) => element.fmt(f),
                GearboxDegenNFTCalls::Burn(element) => element.fmt(f),
                GearboxDegenNFTCalls::GetApproved(element) => element.fmt(f),
                GearboxDegenNFTCalls::IsApprovedForAll(element) => element.fmt(f),
                GearboxDegenNFTCalls::Mint(element) => element.fmt(f),
                GearboxDegenNFTCalls::Name(element) => element.fmt(f),
                GearboxDegenNFTCalls::OwnerOf(element) => element.fmt(f),
                GearboxDegenNFTCalls::Pause(element) => element.fmt(f),
                GearboxDegenNFTCalls::Paused(element) => element.fmt(f),
                GearboxDegenNFTCalls::RemoveManager(element) => element.fmt(f),
                GearboxDegenNFTCalls::SafeTransferFrom(element) => element.fmt(f),
                GearboxDegenNFTCalls::SafeTransferFromWithData(element) => element.fmt(f),
                GearboxDegenNFTCalls::SetApprovalForAll(element) => element.fmt(f),
                GearboxDegenNFTCalls::SupportsInterface(element) => element.fmt(f),
                GearboxDegenNFTCalls::Symbol(element) => element.fmt(f),
                GearboxDegenNFTCalls::TokenURI(element) => element.fmt(f),
                GearboxDegenNFTCalls::TransferFrom(element) => element.fmt(f),
                GearboxDegenNFTCalls::Unpause(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddManagerCall> for GearboxDegenNFTCalls {
        fn from(var: AddManagerCall) -> Self {
            GearboxDegenNFTCalls::AddManager(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for GearboxDegenNFTCalls {
        fn from(var: ApproveCall) -> Self {
            GearboxDegenNFTCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for GearboxDegenNFTCalls {
        fn from(var: BalanceOfCall) -> Self {
            GearboxDegenNFTCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for GearboxDegenNFTCalls {
        fn from(var: BurnCall) -> Self {
            GearboxDegenNFTCalls::Burn(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for GearboxDegenNFTCalls {
        fn from(var: GetApprovedCall) -> Self {
            GearboxDegenNFTCalls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for GearboxDegenNFTCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            GearboxDegenNFTCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<MintCall> for GearboxDegenNFTCalls {
        fn from(var: MintCall) -> Self {
            GearboxDegenNFTCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for GearboxDegenNFTCalls {
        fn from(var: NameCall) -> Self {
            GearboxDegenNFTCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for GearboxDegenNFTCalls {
        fn from(var: OwnerOfCall) -> Self {
            GearboxDegenNFTCalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<PauseCall> for GearboxDegenNFTCalls {
        fn from(var: PauseCall) -> Self {
            GearboxDegenNFTCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for GearboxDegenNFTCalls {
        fn from(var: PausedCall) -> Self {
            GearboxDegenNFTCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RemoveManagerCall> for GearboxDegenNFTCalls {
        fn from(var: RemoveManagerCall) -> Self {
            GearboxDegenNFTCalls::RemoveManager(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for GearboxDegenNFTCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            GearboxDegenNFTCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithDataCall> for GearboxDegenNFTCalls {
        fn from(var: SafeTransferFromWithDataCall) -> Self {
            GearboxDegenNFTCalls::SafeTransferFromWithData(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for GearboxDegenNFTCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            GearboxDegenNFTCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for GearboxDegenNFTCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            GearboxDegenNFTCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for GearboxDegenNFTCalls {
        fn from(var: SymbolCall) -> Self {
            GearboxDegenNFTCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for GearboxDegenNFTCalls {
        fn from(var: TokenURICall) -> Self {
            GearboxDegenNFTCalls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for GearboxDegenNFTCalls {
        fn from(var: TransferFromCall) -> Self {
            GearboxDegenNFTCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for GearboxDegenNFTCalls {
        fn from(var: UnpauseCall) -> Self {
            GearboxDegenNFTCalls::Unpause(var)
        }
    }
}
