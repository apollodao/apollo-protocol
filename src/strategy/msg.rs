use apollo_asset::asset::{Asset, AssetInfo};
use cosmwasm_std::{Addr, Binary, Decimal, Decimal256, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

pub type QueryMsg = BaseStrategyQueryMsg<()>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseStrategyQueryMsg<S> {
    UserInfo {
        address: String,
        token: Option<Addr>,
    },
    StrategyInfo {
        token: Option<Addr>,
    },
    Config {},
    ShouldExecute {
        cost: Uint128,
    },
    Tvl {},
    Apr {},
    Strategy(S),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct StrategyInfo {
    pub total_bond_amount: Uint128,
    pub total_shares: Uint128,
    pub global_index: Decimal, // TODO - remove this after migration, legacy
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// user info struct returned by queries
pub struct UserInfo {
    pub base_token_balance: Uint128,
    pub shares: Uint128,
    pub index: Decimal, // TODO - remove this after migration, legacy
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ShouldExecuteResponse {
    pub should_execute: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct TvlResponse {
    pub tvl: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct AprResponse {
    pub apr: Decimal256,
}

// TODO - move these to a proxy folder
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct PendingRewardsResponse {
    pub pending_rewards: Vec<PendingRewardsItem>,
    pub total_value: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct PendingRewardsItem {
    pub reward_token: Asset,
    pub value: Uint128,
}

pub type ExecuteMsg = BaseStrategyExecuteMsg<()>;

// TODO: Switch to using Addr
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseStrategyExecuteMsg<S> {
    ExecuteStrategy {
        executor: Option<String>, // recipient for execution fee (defaults to sender)
        cost: Option<Uint128>, // provide cost amount to query shouldExecute() with before executing
    },
    SendTokens {
        token: Addr,
        recipient: Addr,
        amount: Option<Uint128>,
        hook_msg: Option<Binary>,
    },
    Strategy(S),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct InstantiateMsg {
    pub base_denom: AssetInfo,
    pub apollo_factory: String, //The Apollo Factory contract
    pub asset_token: AssetInfo, //Apollo token
    pub oracle_contract: String,
    pub strategy_token_code_id: u64,
    pub strategy_token_symbol: String,
    pub strategy_token_name: String,
    pub adaptor_addr: String,
    pub strategy_id: u64,
}
