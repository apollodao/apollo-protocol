use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CanonicalAddr, Decimal, Decimal256, Uint128};
use cw20::Cw20ReceiveMsg;

use crate::oracle::Config;

#[cw_serde]
pub enum StrategyExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Execute {},
    Withdraw { user_addr: String, amount: Uint128 },
    EmergencyWithdraw { receiver: String },
    ZapIn { depositor_addr: String },
    ZapOut { user_addr: String, amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum StrategyQueryMsg {
    #[returns(UserInfo)]
    /// UserInfo
    UserInfo { address: String },
    #[returns(StrategyUserInfoResponse)]
    /// StrategyInfo
    StrategyInfo {},
    #[returns(Config)]
    /// Config
    Config {},
    #[returns(ShouldExecuteResponse)]
    /// ShouldExecute
    ShouldExecute { cost: Uint128 },
    #[returns(TvlResponse)]
    /// Tvl
    Tvl {},
    #[returns(AprResponse)]
    /// Apr
    Apr {},
}

#[cw_serde]
// config struct stored in / read from cw_4626 storage
pub struct StrategyConfig<C> {
    //Strategies must have these three fields in their config, other fields are allowed.
    pub apollo_factory: CanonicalAddr,
    pub apollo_collector: CanonicalAddr,
    pub base_token: CanonicalAddr,
    pub performance_fee: Decimal,
    pub strategy_config: C,
}

#[cw_serde]
// config struct stored in / read from cw_4626 storage
pub struct BaseStrategyConfig {
    //Strategies must have these three fields in their config, other fields are allowed.
    pub apollo_factory: CanonicalAddr,
    pub apollo_collector: CanonicalAddr,
    pub base_token: CanonicalAddr,
    pub performance_fee: Decimal,
}

#[cw_serde]
// config struct returned by queries
pub struct StrategyConfigResponse<C> {
    pub apollo_factory: Addr,
    pub apollo_collector: Addr,
    pub base_token: Addr,
    pub performance_fee: Decimal,
    pub terraswap_router: Addr,
    pub use_strategy_token: bool,
    pub strategy_config: C, // strategy implementation specific config
}

#[cw_serde]
// config struct used as input to update config
pub struct StrategyConfigOptions<C> {
    pub apollo_collector: Option<String>,
    pub performance_fee: Option<Decimal>,
    pub strategy_config: C,
}

#[cw_serde]
// strategy info struct stored in / read from cw_4626 storage
pub struct StrategyInfo {
    pub global_index: Decimal,
    pub total_bond_amount: Uint128,
    pub total_shares: Uint128,
}

#[cw_serde]
// user info struct stored in / read from cw_4626 storage
pub struct UserInfo {
    pub index: Decimal,
    pub shares: Uint128,
}

#[cw_serde]
// user info struct returned by queries
pub struct StrategyUserInfoResponse {
    pub base_token_balance: Uint128,
    pub shares: Uint128,
}

#[cw_serde]
pub struct ShouldExecuteResponse {
    pub should_execute: bool,
}

#[cw_serde]
pub struct TvlResponse {
    pub tvl: Uint128,
}

#[cw_serde]
pub struct AprResponse {
    pub apr: Decimal256,
}

#[cw_serde]
pub enum StrategyCw20HookMsg {
    Deposit { depositor_addr: String },
}
