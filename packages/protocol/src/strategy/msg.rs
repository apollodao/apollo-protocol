use apollo_asset::asset::{Asset, AssetInfo};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};

pub type QueryMsg = BaseStrategyQueryMsg<()>;

#[cw_serde]
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

#[cw_serde]
pub struct StrategyInfo {
    pub total_bond_amount: Uint128,
    pub total_shares: Uint128,
    pub global_index: Decimal, // TODO - remove this after migration, legacy
}

#[cw_serde]
// user info struct returned by queries
pub struct UserInfo {
    pub base_token_balance: Uint128,
    pub shares: Uint128,
    pub index: Decimal, // TODO - remove this after migration, legacy
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

// TODO - move these to a proxy folder
#[cw_serde]
pub struct PendingRewardsResponse {
    pub pending_rewards: Vec<PendingRewardsItem>,
    pub total_value: Uint128,
}

#[cw_serde]
pub struct PendingRewardsItem {
    pub reward_token: Asset,
    pub value: Uint128,
}

pub type ExecuteMsg = BaseStrategyExecuteMsg<()>;

// TODO: Switch to using Addr
#[cw_serde]
pub enum BaseStrategyExecuteMsg<S> {
    ExecuteStrategy {
        executor: Option<String>, // recipient for execution fee (defaults to sender)
        cost: Option<Uint128>, // provide cost amount to query shouldExecute() with before executing
    },
    Strategy(S),
}

#[cw_serde]
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
