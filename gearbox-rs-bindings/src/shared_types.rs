#[doc = "`ExactOutputParams(bytes,address,uint256,uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ExactOutputParams {
    pub path: ethers::core::types::Bytes,
    pub recipient: ethers::core::types::Address,
    pub deadline: ethers::core::types::U256,
    pub amount_out: ethers::core::types::U256,
    pub amount_in_maximum: ethers::core::types::U256,
}
#[doc = "`ExactOutputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ExactOutputSingleParams {
    pub token_in: ethers::core::types::Address,
    pub token_out: ethers::core::types::Address,
    pub fee: u32,
    pub recipient: ethers::core::types::Address,
    pub deadline: ethers::core::types::U256,
    pub amount_out: ethers::core::types::U256,
    pub amount_in_maximum: ethers::core::types::U256,
    pub sqrt_price_limit_x96: ethers::core::types::U256,
}
#[doc = "`TokenInfo(address,string,uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct TokenInfo {
    pub addr: ethers::core::types::Address,
    pub symbol: String,
    pub decimals: u8,
}
#[doc = "`PoolData(address,bool,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct PoolData {
    pub addr: ethers::core::types::Address,
    pub is_weth: bool,
    pub underlying: ethers::core::types::Address,
    pub diesel_token: ethers::core::types::Address,
    pub linear_cumulative_index: ethers::core::types::U256,
    pub available_liquidity: ethers::core::types::U256,
    pub expected_liquidity: ethers::core::types::U256,
    pub expected_liquidity_limit: ethers::core::types::U256,
    pub total_borrowed: ethers::core::types::U256,
    pub deposit_apy_ray: ethers::core::types::U256,
    pub borrow_apy_ray: ethers::core::types::U256,
    pub diesel_rate_ray: ethers::core::types::U256,
    pub withdraw_fee: ethers::core::types::U256,
    pub cumulative_index_ray: ethers::core::types::U256,
    pub timestamp_lu: ethers::core::types::U256,
    pub version: u8,
}
#[doc = "`ExactAllInputSingleParams(address,address,uint24,uint256,uint256,uint160)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ExactAllInputSingleParams {
    pub token_in: ethers::core::types::Address,
    pub token_out: ethers::core::types::Address,
    pub fee: u32,
    pub deadline: ethers::core::types::U256,
    pub rate_min_ray: ethers::core::types::U256,
    pub sqrt_price_limit_x96: ethers::core::types::U256,
}
#[doc = "`ExactInputParams(bytes,address,uint256,uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ExactInputParams {
    pub path: ethers::core::types::Bytes,
    pub recipient: ethers::core::types::Address,
    pub deadline: ethers::core::types::U256,
    pub amount_in: ethers::core::types::U256,
    pub amount_out_minimum: ethers::core::types::U256,
}
#[doc = "`PoolInfo(address,address,address,address,address,bool)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct PoolInfo {
    pub lptoken: ethers::core::types::Address,
    pub token: ethers::core::types::Address,
    pub gauge: ethers::core::types::Address,
    pub crv_rewards: ethers::core::types::Address,
    pub stash: ethers::core::types::Address,
    pub shutdown: bool,
}
#[doc = "`CreditManagerData(address,bool,address,bool,bool,uint256,uint256,uint256,uint256,uint256,address[],(address,address)[],uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct CreditManagerData {
    pub addr: ethers::core::types::Address,
    pub has_account: bool,
    pub underlying: ethers::core::types::Address,
    pub is_weth: bool,
    pub can_borrow: bool,
    pub borrow_rate: ethers::core::types::U256,
    pub min_amount: ethers::core::types::U256,
    pub max_amount: ethers::core::types::U256,
    pub max_leverage_factor: ethers::core::types::U256,
    pub available_liquidity: ethers::core::types::U256,
    pub allowed_tokens: Vec<ethers::core::types::Address>,
    pub adapters: ::std::vec::Vec<ContractAdapter>,
    pub version: u8,
}
#[doc = "`TokenBalance(address,uint256,bool)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct TokenBalance {
    pub token: ethers::core::types::Address,
    pub balance: ethers::core::types::U256,
    pub is_allowed: bool,
}
#[doc = "`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ExactInputSingleParams {
    pub token_in: ethers::core::types::Address,
    pub token_out: ethers::core::types::Address,
    pub fee: u32,
    pub recipient: ethers::core::types::Address,
    pub deadline: ethers::core::types::U256,
    pub amount_in: ethers::core::types::U256,
    pub amount_out_minimum: ethers::core::types::U256,
    pub sqrt_price_limit_x96: ethers::core::types::U256,
}
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
#[doc = "`ExactAllInputParams(bytes,uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ExactAllInputParams {
    pub path: ethers::core::types::Bytes,
    pub deadline: ethers::core::types::U256,
    pub rate_min_ray: ethers::core::types::U256,
}
#[doc = "`CreditAccountData(address,address,bool,address,address,uint256,uint256,uint256,uint256,(address,uint256,bool)[],uint256,uint256,bool,uint256,uint256,uint256,uint8)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct CreditAccountData {
    pub addr: ethers::core::types::Address,
    pub borrower: ethers::core::types::Address,
    pub in_use: bool,
    pub credit_manager: ethers::core::types::Address,
    pub underlying: ethers::core::types::Address,
    pub borrowed_amount_plus_interest: ethers::core::types::U256,
    pub total_value: ethers::core::types::U256,
    pub health_factor: ethers::core::types::U256,
    pub borrow_rate: ethers::core::types::U256,
    pub balances: ::std::vec::Vec<TokenBalance>,
    pub repay_amount: ethers::core::types::U256,
    pub liquidation_amount: ethers::core::types::U256,
    pub can_be_closed: bool,
    pub borrowed_amount: ethers::core::types::U256,
    pub cumulative_index_at_open: ethers::core::types::U256,
    pub since: ethers::core::types::U256,
    pub version: u8,
}
#[doc = "`ContractAdapter(address,address)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ContractAdapter {
    pub allowed_contract: ethers::core::types::Address,
    pub adapter: ethers::core::types::Address,
}
