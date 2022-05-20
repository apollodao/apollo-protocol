use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Addr, CanonicalAddr, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum StrategyExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Execute {},
    Withdraw { user_addr: String, amount: Uint128 },
    EmergencyWithdraw { receiver: String },
    ZapIn { depositor_addr: String },
    ZapOut { user_addr: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum StrategyQueryMsg {
    UserInfo { address: String },
    StrategyInfo {},
    Config {},
    ShouldExecute { cost: Uint128 },
    Tvl {},
    Apr {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// config struct stored in / read from cw_4626 storage
pub struct StrategyConfig<C> {
    //Strategies must have these three fields in their config, other fields are allowed.
    pub apollo_factory: CanonicalAddr,
    pub apollo_collector: CanonicalAddr,
    pub base_token: CanonicalAddr,
    pub performance_fee: Decimal,
    pub strategy_config: C,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// config struct stored in / read from cw_4626 storage
pub struct BaseStrategyConfig {
    //Strategies must have these three fields in their config, other fields are allowed.
    pub apollo_factory: CanonicalAddr,
    pub apollo_collector: CanonicalAddr,
    pub base_token: CanonicalAddr,
    pub performance_fee: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// config struct used as input to update config
pub struct StrategyConfigOptions<C> {
    pub apollo_collector: Option<String>,
    pub performance_fee: Option<Decimal>,
    pub strategy_config: C,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// strategy info struct stored in / read from cw_4626 storage
pub struct StrategyInfo {
    pub global_index: Decimal,
    pub total_bond_amount: Uint128,
    pub total_shares: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// user info struct stored in / read from cw_4626 storage
pub struct UserInfo {
    pub index: Decimal,
    pub shares: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// user info struct returned by queries
pub struct StrategyUserInfoResponse {
    pub base_token_balance: Uint128,
    pub shares: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ShouldExecuteResponse {
    pub should_execute: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
#[schemars(deny_unknown_fields)]
pub struct TvlResponse {
    pub tvl: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct AprResponse {
    pub apr: Decimal256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum StrategyCw20HookMsg {
    Deposit { depositor_addr: String },
}
