pub use geartoken_mod::*;
#[allow(clippy::too_many_arguments)]
mod geartoken_mod {
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
    #[doc = "GearToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GEARTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"fromDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DelegateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"previousBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DelegateVotesChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MinerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"TransferAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DELEGATION_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PERMIT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"allowTransfers\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkpoints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"fromBlock\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint96\",\"name\":\"votes\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegateBySig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegates\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentVotes\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPriorVotes\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"miner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numCheckpoints\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_miner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMiner\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rawAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transfersAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static GEARTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b506040516200222038038062002220833981016040819052620000349162000122565b6001600160a01b0381166200008f5760405162461bcd60e51b815260206004820152601b60248201527f5a65726f2061646472657373206973206e6f7420616c6c6f7765640000000000604482015260640160405180910390fd5b6001600160a01b03811660008181526001602052604080822080546001600160601b0319166b204fce5e3e2502611000000090811790915590517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91620000f99190815260200190565b60405180910390a350600680546001600160a81b031916336101000260ff191617905562000154565b6000602082840312156200013557600080fd5b81516001600160a01b03811681146200014d57600080fd5b9392505050565b6120bc80620001646000396000f3fe608060405234801561001057600080fd5b50600436106101a95760003560e01c806370a08231116100f9578063b4b5ea5711610097578063dd62ed3e11610071578063dd62ed3e14610479578063e7a324dc146104b9578063f1127ed8146104e0578063f2fde38b1461054757600080fd5b8063b4b5ea5714610440578063c3cda52014610453578063d505accf1461046657600080fd5b806395d89b41116100d357806395d89b41146103ea5780639742ca461461040d578063a9059cbb14610420578063b0660c3d1461043357600080fd5b806370a082311461036d578063782d6fe11461039f5780637ecebe00146103ca57600080fd5b806330adf81f11610166578063481c6a7511610140578063481c6a75146102de578063587cde1e146102f65780635c19a95c1461031f5780636fcfff451461033257600080fd5b806330adf81f14610272578063313ce56714610299578063349dc329146102b357600080fd5b806306fdde03146101ae578063095ea7b3146101ea57806318160ddd1461020d57806320606b701461022e5780632185810b1461025557806323b872dd1461025f575b600080fd5b6101d46040518060400160405280600781526020016608ecac2e4c4def60cb1b81525081565b6040516101e19190611b91565b60405180910390f35b6101fd6101f8366004611c02565b61055a565b60405190151581526020016101e1565b6102206b204fce5e3e2502611000000081565b6040519081526020016101e1565b6102207f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a86681565b61025d610619565b005b6101fd61026d366004611c2c565b610689565b6102207f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c981565b6102a1601281565b60405160ff90911681526020016101e1565b6007546102c6906001600160a01b031681565b6040516001600160a01b0390911681526020016101e1565b6006546102c69061010090046001600160a01b031681565b6102c6610304366004611c68565b6002602052600090815260409020546001600160a01b031681565b61025d61032d366004611c68565b6107ca565b610358610340366004611c68565b60046020526000908152604090205463ffffffff1681565b60405163ffffffff90911681526020016101e1565b61022061037b366004611c68565b6001600160a01b03166000908152600160205260409020546001600160601b031690565b6103b26103ad366004611c02565b6107d7565b6040516001600160601b0390911681526020016101e1565b6102206103d8366004611c68565b60056020526000908152604090205481565b6101d46040518060400160405280600481526020016323a2a0a960e11b81525081565b61025d61041b366004611c68565b610a5e565b6101fd61042e366004611c02565b610b2d565b6006546101fd9060ff1681565b6103b261044e366004611c68565b610b69565b61025d610461366004611c94565b610be7565b61025d610474366004611cec565b610ed2565b610220610487366004611d56565b6001600160a01b039182166000908152602081815260408083209390941682529190915220546001600160601b031690565b6102207fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf81565b6105236104ee366004611d89565b600360209081526000928352604080842090915290825290205463ffffffff811690600160201b90046001600160601b031682565b6040805163ffffffff90931683526001600160601b039091166020830152016101e1565b61025d610555366004611c68565b6112c0565b60008060001983141561057557506001600160601b0361059a565b61059783604051806060016040528060258152602001611ef2602591396113ac565b90505b336000818152602081815260408083206001600160a01b0389168085529083529281902080546001600160601b0319166001600160601b03871690811790915590519081529192917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a360019150505b92915050565b60065461010090046001600160a01b031633146106515760405162461bcd60e51b815260040161064890611dc9565b60405180910390fd5b6006805460ff191660011790556040517f795b0e16c8da9807b0a215f3749bd6dbcc49fc0472183f4e446abb7dcbd9d00790600090a1565b6001600160a01b0383166000908152602081815260408083203380855290835281842054825160608101909352602580845291936001600160601b039091169285926106df9288929190611ef2908301396113ac565b9050866001600160a01b0316836001600160a01b03161415801561070c57506001600160601b0382811614155b156107b257600061073683836040518060600160405280603d8152602001612014603d91396113db565b6001600160a01b03898116600081815260208181526040808320948a168084529482529182902080546001600160601b0319166001600160601b0387169081179091559151918252939450919290917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505b6107bd878783611425565b5060019695505050505050565b6107d433826116f7565b50565b60004382106108385760405162461bcd60e51b815260206004820152602760248201527f476561723a3a6765745072696f72566f7465733a206e6f742079657420646574604482015266195c9b5a5b995960ca1b6064820152608401610648565b6001600160a01b03831660009081526004602052604090205463ffffffff1680610866576000915050610613565b6001600160a01b0384166000908152600360205260408120849161088b600185611e16565b63ffffffff908116825260208201929092526040016000205416116108fe576001600160a01b0384166000908152600360205260408120906108ce600184611e16565b63ffffffff168152602081019190915260400160002054600160201b90046001600160601b031691506106139050565b6001600160a01b038416600090815260036020908152604080832083805290915290205463ffffffff16831015610939576000915050610613565b600080610947600184611e16565b90505b8163ffffffff168163ffffffff161115610a19576000600261096c8484611e16565b6109769190611e3b565b6109809083611e16565b6001600160a01b038816600090815260036020908152604080832063ffffffff858116855290835292819020815180830190925254928316808252600160201b9093046001600160601b0316918101919091529192508714156109ed576020015194506106139350505050565b805163ffffffff16871115610a0457819350610a12565b610a0f600183611e16565b92505b505061094a565b506001600160a01b038516600090815260036020908152604080832063ffffffff909416835292905220546001600160601b03600160201b9091041691505092915050565b60065461010090046001600160a01b03163314610a8d5760405162461bcd60e51b815260040161064890611dc9565b6001600160a01b038116610ae35760405162461bcd60e51b815260206004820152601b60248201527f5a65726f2061646472657373206973206e6f7420616c6c6f77656400000000006044820152606401610648565b600780546001600160a01b0319166001600160a01b0383169081179091556040517f2f834d1c8c4b956018fff5faca4d99868ae635487424d9c265c257ccbc698c6a90600090a250565b600080610b5283604051806060016040528060268152602001611f47602691396113ac565b9050610b5f338583611425565b5060019392505050565b6001600160a01b03811660009081526004602052604081205463ffffffff1680610b94576000610be0565b6001600160a01b038316600090815260036020526040812090610bb8600184611e16565b63ffffffff168152602081019190915260400160002054600160201b90046001600160601b03165b9392505050565b604080518082018252600781526608ecac2e4c4def60cb1b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f028ef9f797075f74ac647c65fde04fb0f128c2d59fd40f45732269917642fd4681840152466060820152306080808301919091528351808303909101815260a0820184528051908301207fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60c08301526001600160a01b038a1660e083015261010082018990526101208083018990528451808403909101815261014083019094528351939092019290922061190160f01b6101608401526101628301829052610182830181905290916000906101a20160408051601f198184030181528282528051602091820120600080855291840180845281905260ff8a169284019290925260608301889052608083018790529092509060019060a0016020604051602081039080840390855afa158015610d69573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116610ddb5760405162461bcd60e51b815260206004820152602660248201527f476561723a3a64656c656761746542795369673a20696e76616c6964207369676044820152656e617475726560d01b6064820152608401610648565b6001600160a01b0381166000908152600560205260408120805491610dff83611e6c565b919050558914610e5c5760405162461bcd60e51b815260206004820152602260248201527f476561723a3a64656c656761746542795369673a20696e76616c6964206e6f6e604482015261636560f01b6064820152608401610648565b87421115610ebb5760405162461bcd60e51b815260206004820152602660248201527f476561723a3a64656c656761746542795369673a207369676e617475726520656044820152651e1c1a5c995960d21b6064820152608401610648565b610ec5818b6116f7565b505050505b505050505050565b6000600019861415610eec57506001600160601b03610f11565b610f0e86604051806060016040528060248152602001611fbc602491396113ac565b90505b604080518082018252600781526608ecac2e4c4def60cb1b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f028ef9f797075f74ac647c65fde04fb0f128c2d59fd40f45732269917642fd4681840152466060820152306080808301919091528351808303909101815260a090910183528051908201206001600160a01b038b166000908152600590925291812080547f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9918c918c918c919086610fee83611e6c565b909155506040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810188905260e0016040516020818303038152906040528051906020012090506000828260405160200161106d92919061190160f01b81526002810192909252602282015260420190565b60408051601f198184030181528282528051602091820120600080855291840180845281905260ff8b169284019290925260608301899052608083018890529092509060019060a0016020604051602081039080840390855afa1580156110d8573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811661113b5760405162461bcd60e51b815260206004820152601f60248201527f476561723a3a7065726d69743a20696e76616c6964207369676e6174757265006044820152606401610648565b8b6001600160a01b0316816001600160a01b03161461119c5760405162461bcd60e51b815260206004820152601a60248201527f476561723a3a7065726d69743a20756e617574686f72697a65640000000000006044820152606401610648565b884211156111ec5760405162461bcd60e51b815260206004820152601f60248201527f476561723a3a7065726d69743a207369676e61747572652065787069726564006044820152606401610648565b846000808e6001600160a01b03166001600160a01b0316815260200190815260200160002060008d6001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a8154816001600160601b0302191690836001600160601b031602179055508a6001600160a01b03168c6001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925876040516112aa91906001600160601b0391909116815260200190565b60405180910390a3505050505050505050505050565b60065461010090046001600160a01b031633146112ef5760405162461bcd60e51b815260040161064890611dc9565b6001600160a01b0381166113455760405162461bcd60e51b815260206004820152601b60248201527f5a65726f2061646472657373206973206e6f7420616c6c6f77656400000000006044820152606401610648565b6006546040516001600160a01b0380841692610100900416907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a3600680546001600160a01b0390921661010002610100600160a81b0319909216919091179055565b600081600160601b84106113d35760405162461bcd60e51b81526004016106489190611b91565b509192915050565b6000836001600160601b0316836001600160601b0316111582906114125760405162461bcd60e51b81526004016106489190611b91565b5061141d8385611e87565b949350505050565b60065460ff1680611445575060065461010090046001600160a01b031633145b8061145a57506007546001600160a01b031633145b6114a65760405162461bcd60e51b815260206004820152601d60248201527f476561723a3a7472616e73666572732061726520666f7262696464656e0000006044820152606401610648565b6001600160a01b0383166115225760405162461bcd60e51b815260206004820152603c60248201527f476561723a3a5f7472616e73666572546f6b656e733a2063616e6e6f7420747260448201527f616e736665722066726f6d20746865207a65726f2061646472657373000000006064820152608401610648565b6001600160a01b03821661159e5760405162461bcd60e51b815260206004820152603a60248201527f476561723a3a5f7472616e73666572546f6b656e733a2063616e6e6f7420747260448201527f616e7366657220746f20746865207a65726f20616464726573730000000000006064820152608401610648565b6001600160a01b0383166000908152600160209081526040918290205482516060810190935260368084526115e9936001600160601b039092169285929190612051908301396113db565b6001600160a01b03848116600090815260016020908152604080832080546001600160601b0319166001600160601b039687161790559286168252908290205482516060810190935260308084526116519491909116928592909190611f1790830139611781565b6001600160a01b0383811660008181526001602090815260409182902080546001600160601b0319166001600160601b03968716179055905193851684529092918616917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a36001600160a01b038084166000908152600260205260408082205485841683529120546116f2929182169116836117ce565b505050565b6001600160a01b03808316600081815260026020818152604080842080546001845282862054949093528787166001600160a01b031984168117909155905191909516946001600160601b039092169391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a461177b8284836117ce565b50505050565b60008061178e8486611ea7565b9050846001600160601b0316816001600160601b0316101583906117c55760405162461bcd60e51b81526004016106489190611b91565b50949350505050565b816001600160a01b0316836001600160a01b0316141580156117f957506000816001600160601b0316115b156116f2576001600160a01b038316156118be576001600160a01b03831660009081526004602052604081205463ffffffff169081611839576000611885565b6001600160a01b03851660009081526003602052604081209061185d600185611e16565b63ffffffff168152602081019190915260400160002054600160201b90046001600160601b03165b905060006118ac8285604051806060016040528060288152602001611f6d602891396113db565b90506118ba86848484611976565b5050505b6001600160a01b038216156116f2576001600160a01b03821660009081526004602052604081205463ffffffff1690816118f9576000611945565b6001600160a01b03841660009081526003602052604081209061191d600185611e16565b63ffffffff168152602081019190915260400160002054600160201b90046001600160601b03165b9050600061196c8285604051806060016040528060278152602001611f9560279139611781565b9050610eca858484845b600061199a43604051806060016040528060348152602001611fe060349139611b6e565b905060008463ffffffff161180156119f457506001600160a01b038516600090815260036020526040812063ffffffff8316916119d8600188611e16565b63ffffffff908116825260208201929092526040016000205416145b15611a68576001600160a01b03851660009081526003602052604081208391611a1e600188611e16565b63ffffffff168152602081019190915260400160002080546001600160601b0392909216600160201b026fffffffffffffffffffffffff0000000019909216919091179055611b19565b60408051808201825263ffffffff80841682526001600160601b0380861660208085019182526001600160a01b038b166000908152600382528681208b8616825290915294909420925183549451909116600160201b026fffffffffffffffffffffffffffffffff19909416911617919091179055611ae8846001611ed2565b6001600160a01b0386166000908152600460205260409020805463ffffffff191663ffffffff929092169190911790555b604080516001600160601b038086168252841660208201526001600160a01b038716917fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724910160405180910390a25050505050565b600081600160201b84106113d35760405162461bcd60e51b815260040161064891905b600060208083528351808285015260005b81811015611bbe57858101830151858201604001528201611ba2565b81811115611bd0576000604083870101525b50601f01601f1916929092016040019392505050565b80356001600160a01b0381168114611bfd57600080fd5b919050565b60008060408385031215611c1557600080fd5b611c1e83611be6565b946020939093013593505050565b600080600060608486031215611c4157600080fd5b611c4a84611be6565b9250611c5860208501611be6565b9150604084013590509250925092565b600060208284031215611c7a57600080fd5b610be082611be6565b803560ff81168114611bfd57600080fd5b60008060008060008060c08789031215611cad57600080fd5b611cb687611be6565b95506020870135945060408701359350611cd260608801611c83565b92506080870135915060a087013590509295509295509295565b600080600080600080600060e0888a031215611d0757600080fd5b611d1088611be6565b9650611d1e60208901611be6565b95506040880135945060608801359350611d3a60808901611c83565b925060a0880135915060c0880135905092959891949750929550565b60008060408385031215611d6957600080fd5b611d7283611be6565b9150611d8060208401611be6565b90509250929050565b60008060408385031215611d9c57600080fd5b611da583611be6565b9150602083013563ffffffff81168114611dbe57600080fd5b809150509250929050565b6020808252601f908201527f476561723a3a63616c6c6572206973206e6f7420746865206d616e6167657200604082015260600190565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff83811690831681811015611e3357611e33611e00565b039392505050565b600063ffffffff80841680611e6057634e487b7160e01b600052601260045260246000fd5b92169190910492915050565b6000600019821415611e8057611e80611e00565b5060010190565b60006001600160601b0383811690831681811015611e3357611e33611e00565b60006001600160601b03808316818516808303821115611ec957611ec9611e00565b01949350505050565b600063ffffffff808316818516808303821115611ec957611ec9611e0056fe476561723a3a617070726f76653a20616d6f756e7420657863656564732039362062697473476561723a3a5f7472616e73666572546f6b656e733a207472616e7366657220616d6f756e74206f766572666c6f7773476561723a3a7472616e736665723a20616d6f756e7420657863656564732039362062697473476561723a3a5f6d6f7665566f7465733a20766f746520616d6f756e7420756e646572666c6f7773476561723a3a5f6d6f7665566f7465733a20766f746520616d6f756e74206f766572666c6f7773476561723a3a7065726d69743a20616d6f756e7420657863656564732039362062697473476561723a3a5f7772697465436865636b706f696e743a20626c6f636b206e756d62657220657863656564732033322062697473476561723a3a7472616e7366657246726f6d3a207472616e7366657220616d6f756e742065786365656473207370656e64657220616c6c6f77616e6365476561723a3a5f7472616e73666572546f6b656e733a207472616e7366657220616d6f756e7420657863656564732062616c616e6365a264697066735822122028fcc76e6493d30baaa906842f47baa7679988038c82e7bfec7228ccdaab03cf64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct GearToken<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for GearToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for GearToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(GearToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> GearToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GEARTOKEN_ABI.clone(), client).into()
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
                GEARTOKEN_ABI.clone(),
                GEARTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `DELEGATION_TYPEHASH` (0xe7a324dc) function"]
        pub fn delegation_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([231, 163, 36, 220], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function"]
        pub fn domain_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowTransfers` (0x2185810b) function"]
        pub fn allow_transfers(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 133, 129, 11], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            account: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (account, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, raw_amount))
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
        #[doc = "Calls the contract's `checkpoints` (0xf1127ed8) function"]
        pub fn checkpoints(
            &self,
            p0: ethers::core::types::Address,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, (u32, u128)> {
            self.0
                .method_hash([241, 18, 126, 216], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegate` (0x5c19a95c) function"]
        pub fn delegate(
            &self,
            delegatee: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 25, 169, 92], delegatee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegateBySig` (0xc3cda520) function"]
        pub fn delegate_by_sig(
            &self,
            delegatee: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 205, 165, 32], (delegatee, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegates` (0x587cde1e) function"]
        pub fn delegates(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([88, 124, 222, 30], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentVotes` (0xb4b5ea57) function"]
        pub fn get_current_votes(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([180, 181, 234, 87], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriorVotes` (0x782d6fe1) function"]
        pub fn get_prior_votes(
            &self,
            account: ethers::core::types::Address,
            block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([120, 45, 111, 225], (account, block_number))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `manager` (0x481c6a75) function"]
        pub fn manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `miner` (0x349dc329) function"]
        pub fn miner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([52, 157, 195, 41], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numCheckpoints` (0x6fcfff45) function"]
        pub fn num_checkpoints(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 207, 255, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, raw_amount, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMiner` (0x9742ca46) function"]
        pub fn set_miner(
            &self,
            miner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 66, 202, 70], miner)
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
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            src: ethers::core::types::Address,
            dst: ethers::core::types::Address,
            raw_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (src, dst, raw_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfersAllowed` (0xb0660c3d) function"]
        pub fn transfers_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([176, 102, 12, 61], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DelegateChanged` event"]
        pub fn delegate_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DelegateChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DelegateVotesChanged` event"]
        pub fn delegate_votes_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DelegateVotesChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinerSet` event"]
        pub fn miner_set_filter(&self) -> ethers::contract::builders::Event<M, MinerSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAllowed` event"]
        pub fn transfer_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferAllowedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, GearTokenEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for GearToken<M> {
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
        pub amount: ethers::core::types::U256,
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
    #[ethevent(
        name = "DelegateChanged",
        abi = "DelegateChanged(address,address,address)"
    )]
    pub struct DelegateChangedFilter {
        #[ethevent(indexed)]
        pub delegator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_delegate: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_delegate: ethers::core::types::Address,
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
    #[ethevent(
        name = "DelegateVotesChanged",
        abi = "DelegateVotesChanged(address,uint256,uint256)"
    )]
    pub struct DelegateVotesChangedFilter {
        #[ethevent(indexed)]
        pub delegate: ethers::core::types::Address,
        pub previous_balance: ethers::core::types::U256,
        pub new_balance: ethers::core::types::U256,
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
    #[ethevent(name = "MinerSet", abi = "MinerSet(address)")]
    pub struct MinerSetFilter {
        #[ethevent(indexed)]
        pub miner: ethers::core::types::Address,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "TransferAllowed", abi = "TransferAllowed()")]
    pub struct TransferAllowedFilter();
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GearTokenEvents {
        ApprovalFilter(ApprovalFilter),
        DelegateChangedFilter(DelegateChangedFilter),
        DelegateVotesChangedFilter(DelegateVotesChangedFilter),
        MinerSetFilter(MinerSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TransferFilter(TransferFilter),
        TransferAllowedFilter(TransferAllowedFilter),
    }
    impl ethers::contract::EthLogDecode for GearTokenEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(GearTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DelegateChangedFilter::decode_log(log) {
                return Ok(GearTokenEvents::DelegateChangedFilter(decoded));
            }
            if let Ok(decoded) = DelegateVotesChangedFilter::decode_log(log) {
                return Ok(GearTokenEvents::DelegateVotesChangedFilter(decoded));
            }
            if let Ok(decoded) = MinerSetFilter::decode_log(log) {
                return Ok(GearTokenEvents::MinerSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(GearTokenEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(GearTokenEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferAllowedFilter::decode_log(log) {
                return Ok(GearTokenEvents::TransferAllowedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for GearTokenEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GearTokenEvents::ApprovalFilter(element) => element.fmt(f),
                GearTokenEvents::DelegateChangedFilter(element) => element.fmt(f),
                GearTokenEvents::DelegateVotesChangedFilter(element) => element.fmt(f),
                GearTokenEvents::MinerSetFilter(element) => element.fmt(f),
                GearTokenEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                GearTokenEvents::TransferFilter(element) => element.fmt(f),
                GearTokenEvents::TransferAllowedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DELEGATION_TYPEHASH`function with signature `DELEGATION_TYPEHASH()` and selector `[231, 163, 36, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DELEGATION_TYPEHASH", abi = "DELEGATION_TYPEHASH()")]
    pub struct DelegationTypehashCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_TYPEHASH`function with signature `DOMAIN_TYPEHASH()` and selector `[32, 96, 107, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    #[doc = "Container type for all input parameters for the `PERMIT_TYPEHASH`function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    #[doc = "Container type for all input parameters for the `allowTransfers`function with signature `allowTransfers()` and selector `[33, 133, 129, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "allowTransfers", abi = "allowTransfers()")]
    pub struct AllowTransfersCall;
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
    pub struct AllowanceCall {
        pub account: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
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
        pub spender: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
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
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `checkpoints`function with signature `checkpoints(address,uint32)` and selector `[241, 18, 126, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "checkpoints", abi = "checkpoints(address,uint32)")]
    pub struct CheckpointsCall(pub ethers::core::types::Address, pub u32);
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
    #[doc = "Container type for all input parameters for the `delegate`function with signature `delegate(address)` and selector `[92, 25, 169, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delegate", abi = "delegate(address)")]
    pub struct DelegateCall {
        pub delegatee: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `delegateBySig`function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[195, 205, 165, 32]`"]
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
        name = "delegateBySig",
        abi = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegateBySigCall {
        pub delegatee: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `delegates`function with signature `delegates(address)` and selector `[88, 124, 222, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "delegates", abi = "delegates(address)")]
    pub struct DelegatesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getCurrentVotes`function with signature `getCurrentVotes(address)` and selector `[180, 181, 234, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCurrentVotes", abi = "getCurrentVotes(address)")]
    pub struct GetCurrentVotesCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPriorVotes`function with signature `getPriorVotes(address,uint256)` and selector `[120, 45, 111, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPriorVotes", abi = "getPriorVotes(address,uint256)")]
    pub struct GetPriorVotesCall {
        pub account: ethers::core::types::Address,
        pub block_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `manager`function with signature `manager()` and selector `[72, 28, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    #[doc = "Container type for all input parameters for the `miner`function with signature `miner()` and selector `[52, 157, 195, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "miner", abi = "miner()")]
    pub struct MinerCall;
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
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `numCheckpoints`function with signature `numCheckpoints(address)` and selector `[111, 207, 255, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "numCheckpoints", abi = "numCheckpoints(address)")]
    pub struct NumCheckpointsCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `permit`function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setMiner`function with signature `setMiner(address)` and selector `[151, 66, 202, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setMiner", abi = "setMiner(address)")]
    pub struct SetMinerCall {
        pub miner: ethers::core::types::Address,
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
    pub struct TransferCall {
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
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
        pub src: ethers::core::types::Address,
        pub dst: ethers::core::types::Address,
        pub raw_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_manager: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transfersAllowed`function with signature `transfersAllowed()` and selector `[176, 102, 12, 61]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transfersAllowed", abi = "transfersAllowed()")]
    pub struct TransfersAllowedCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GearTokenCalls {
        DelegationTypehash(DelegationTypehashCall),
        DomainTypehash(DomainTypehashCall),
        PermitTypehash(PermitTypehashCall),
        AllowTransfers(AllowTransfersCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Checkpoints(CheckpointsCall),
        Decimals(DecimalsCall),
        Delegate(DelegateCall),
        DelegateBySig(DelegateBySigCall),
        Delegates(DelegatesCall),
        GetCurrentVotes(GetCurrentVotesCall),
        GetPriorVotes(GetPriorVotesCall),
        Manager(ManagerCall),
        Miner(MinerCall),
        Name(NameCall),
        Nonces(NoncesCall),
        NumCheckpoints(NumCheckpointsCall),
        Permit(PermitCall),
        SetMiner(SetMinerCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        TransfersAllowed(TransfersAllowedCall),
    }
    impl ethers::core::abi::AbiDecode for GearTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DelegationTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::DelegationTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowTransfersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::AllowTransfers(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CheckpointsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Checkpoints(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Delegate(decoded));
            }
            if let Ok(decoded) =
                <DelegateBySigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::DelegateBySig(decoded));
            }
            if let Ok(decoded) =
                <DelegatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Delegates(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::GetCurrentVotes(decoded));
            }
            if let Ok(decoded) =
                <GetPriorVotesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::GetPriorVotes(decoded));
            }
            if let Ok(decoded) =
                <ManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Manager(decoded));
            }
            if let Ok(decoded) = <MinerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Miner(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(GearTokenCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <NumCheckpointsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::NumCheckpoints(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <SetMinerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::SetMiner(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransfersAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GearTokenCalls::TransfersAllowed(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GearTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GearTokenCalls::DelegationTypehash(element) => element.encode(),
                GearTokenCalls::DomainTypehash(element) => element.encode(),
                GearTokenCalls::PermitTypehash(element) => element.encode(),
                GearTokenCalls::AllowTransfers(element) => element.encode(),
                GearTokenCalls::Allowance(element) => element.encode(),
                GearTokenCalls::Approve(element) => element.encode(),
                GearTokenCalls::BalanceOf(element) => element.encode(),
                GearTokenCalls::Checkpoints(element) => element.encode(),
                GearTokenCalls::Decimals(element) => element.encode(),
                GearTokenCalls::Delegate(element) => element.encode(),
                GearTokenCalls::DelegateBySig(element) => element.encode(),
                GearTokenCalls::Delegates(element) => element.encode(),
                GearTokenCalls::GetCurrentVotes(element) => element.encode(),
                GearTokenCalls::GetPriorVotes(element) => element.encode(),
                GearTokenCalls::Manager(element) => element.encode(),
                GearTokenCalls::Miner(element) => element.encode(),
                GearTokenCalls::Name(element) => element.encode(),
                GearTokenCalls::Nonces(element) => element.encode(),
                GearTokenCalls::NumCheckpoints(element) => element.encode(),
                GearTokenCalls::Permit(element) => element.encode(),
                GearTokenCalls::SetMiner(element) => element.encode(),
                GearTokenCalls::Symbol(element) => element.encode(),
                GearTokenCalls::TotalSupply(element) => element.encode(),
                GearTokenCalls::Transfer(element) => element.encode(),
                GearTokenCalls::TransferFrom(element) => element.encode(),
                GearTokenCalls::TransferOwnership(element) => element.encode(),
                GearTokenCalls::TransfersAllowed(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GearTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GearTokenCalls::DelegationTypehash(element) => element.fmt(f),
                GearTokenCalls::DomainTypehash(element) => element.fmt(f),
                GearTokenCalls::PermitTypehash(element) => element.fmt(f),
                GearTokenCalls::AllowTransfers(element) => element.fmt(f),
                GearTokenCalls::Allowance(element) => element.fmt(f),
                GearTokenCalls::Approve(element) => element.fmt(f),
                GearTokenCalls::BalanceOf(element) => element.fmt(f),
                GearTokenCalls::Checkpoints(element) => element.fmt(f),
                GearTokenCalls::Decimals(element) => element.fmt(f),
                GearTokenCalls::Delegate(element) => element.fmt(f),
                GearTokenCalls::DelegateBySig(element) => element.fmt(f),
                GearTokenCalls::Delegates(element) => element.fmt(f),
                GearTokenCalls::GetCurrentVotes(element) => element.fmt(f),
                GearTokenCalls::GetPriorVotes(element) => element.fmt(f),
                GearTokenCalls::Manager(element) => element.fmt(f),
                GearTokenCalls::Miner(element) => element.fmt(f),
                GearTokenCalls::Name(element) => element.fmt(f),
                GearTokenCalls::Nonces(element) => element.fmt(f),
                GearTokenCalls::NumCheckpoints(element) => element.fmt(f),
                GearTokenCalls::Permit(element) => element.fmt(f),
                GearTokenCalls::SetMiner(element) => element.fmt(f),
                GearTokenCalls::Symbol(element) => element.fmt(f),
                GearTokenCalls::TotalSupply(element) => element.fmt(f),
                GearTokenCalls::Transfer(element) => element.fmt(f),
                GearTokenCalls::TransferFrom(element) => element.fmt(f),
                GearTokenCalls::TransferOwnership(element) => element.fmt(f),
                GearTokenCalls::TransfersAllowed(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DelegationTypehashCall> for GearTokenCalls {
        fn from(var: DelegationTypehashCall) -> Self {
            GearTokenCalls::DelegationTypehash(var)
        }
    }
    impl ::std::convert::From<DomainTypehashCall> for GearTokenCalls {
        fn from(var: DomainTypehashCall) -> Self {
            GearTokenCalls::DomainTypehash(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for GearTokenCalls {
        fn from(var: PermitTypehashCall) -> Self {
            GearTokenCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<AllowTransfersCall> for GearTokenCalls {
        fn from(var: AllowTransfersCall) -> Self {
            GearTokenCalls::AllowTransfers(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for GearTokenCalls {
        fn from(var: AllowanceCall) -> Self {
            GearTokenCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for GearTokenCalls {
        fn from(var: ApproveCall) -> Self {
            GearTokenCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for GearTokenCalls {
        fn from(var: BalanceOfCall) -> Self {
            GearTokenCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CheckpointsCall> for GearTokenCalls {
        fn from(var: CheckpointsCall) -> Self {
            GearTokenCalls::Checkpoints(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for GearTokenCalls {
        fn from(var: DecimalsCall) -> Self {
            GearTokenCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DelegateCall> for GearTokenCalls {
        fn from(var: DelegateCall) -> Self {
            GearTokenCalls::Delegate(var)
        }
    }
    impl ::std::convert::From<DelegateBySigCall> for GearTokenCalls {
        fn from(var: DelegateBySigCall) -> Self {
            GearTokenCalls::DelegateBySig(var)
        }
    }
    impl ::std::convert::From<DelegatesCall> for GearTokenCalls {
        fn from(var: DelegatesCall) -> Self {
            GearTokenCalls::Delegates(var)
        }
    }
    impl ::std::convert::From<GetCurrentVotesCall> for GearTokenCalls {
        fn from(var: GetCurrentVotesCall) -> Self {
            GearTokenCalls::GetCurrentVotes(var)
        }
    }
    impl ::std::convert::From<GetPriorVotesCall> for GearTokenCalls {
        fn from(var: GetPriorVotesCall) -> Self {
            GearTokenCalls::GetPriorVotes(var)
        }
    }
    impl ::std::convert::From<ManagerCall> for GearTokenCalls {
        fn from(var: ManagerCall) -> Self {
            GearTokenCalls::Manager(var)
        }
    }
    impl ::std::convert::From<MinerCall> for GearTokenCalls {
        fn from(var: MinerCall) -> Self {
            GearTokenCalls::Miner(var)
        }
    }
    impl ::std::convert::From<NameCall> for GearTokenCalls {
        fn from(var: NameCall) -> Self {
            GearTokenCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for GearTokenCalls {
        fn from(var: NoncesCall) -> Self {
            GearTokenCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<NumCheckpointsCall> for GearTokenCalls {
        fn from(var: NumCheckpointsCall) -> Self {
            GearTokenCalls::NumCheckpoints(var)
        }
    }
    impl ::std::convert::From<PermitCall> for GearTokenCalls {
        fn from(var: PermitCall) -> Self {
            GearTokenCalls::Permit(var)
        }
    }
    impl ::std::convert::From<SetMinerCall> for GearTokenCalls {
        fn from(var: SetMinerCall) -> Self {
            GearTokenCalls::SetMiner(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for GearTokenCalls {
        fn from(var: SymbolCall) -> Self {
            GearTokenCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for GearTokenCalls {
        fn from(var: TotalSupplyCall) -> Self {
            GearTokenCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for GearTokenCalls {
        fn from(var: TransferCall) -> Self {
            GearTokenCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for GearTokenCalls {
        fn from(var: TransferFromCall) -> Self {
            GearTokenCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for GearTokenCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            GearTokenCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TransfersAllowedCall> for GearTokenCalls {
        fn from(var: TransfersAllowedCall) -> Self {
            GearTokenCalls::TransfersAllowed(var)
        }
    }
}
