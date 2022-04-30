#[doc = "`MultiCall(address,bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct MultiCall {
    pub target: ethers::core::types::Address,
    pub call_data: ethers::core::types::Bytes,
}
