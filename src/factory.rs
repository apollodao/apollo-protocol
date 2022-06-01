use cosmwasm_std::{
    Addr, Binary, CanonicalAddr, Decimal, Decimal256, Order, QuerierWrapper, Record, StdError,
    StdResult, Storage, Uint128, Uint256,
};
use cw20::Cw20ReceiveMsg;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static APOLLO_CONTRACTS: Item<ApolloContracts> = Item::new("apollo_contracts");
pub static APOLLO_DEX_ADAPTORS: Map<u8, Addr> = Map::new("apollo_dex_adaptors");
pub const APOLLO_DEX_COUNT: Item<u8> = Item::new("apollo_dex_count");

pub fn next_dex_id(store: &mut dyn Storage) -> StdResult<u8> {
    let id: u8 = APOLLO_DEX_COUNT.may_load(store)?.unwrap_or_default() + 1;
    APOLLO_DEX_COUNT.save(store, &id)?;
    Ok(id)
}

pub fn add_dex(storage: &mut dyn Storage, dex_addr: &Addr) -> StdResult<u8> {
    let id = next_dex_id(storage)?;
    APOLLO_DEX_ADAPTORS.save(storage, id.into(), dex_addr)?;
    Ok(id)
}

pub fn remove_dex(storage: &mut dyn Storage, dex_id: u8) -> StdResult<()> {
    APOLLO_DEX_ADAPTORS.remove(storage, dex_id.into());
    Ok(())
}

pub fn get_apollo_dex_adaptor_by_addr(
    storage: &dyn Storage,
    dex_adaptor_addr: &Addr,
) -> StdResult<Addr> {
    match APOLLO_DEX_ADAPTORS
        .range(storage, None, None, Order::Ascending)
        .find(|p| &p.as_ref().unwrap().1 == dex_adaptor_addr)
    {
        None => Err(StdError::generic_err(format!(
            "dex adaptor not whitelisted with factory - {:?}",
            dex_adaptor_addr
        ))),
        Some(found) => Ok(found?.1),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum ExecuteMsg {
    AddStrategy {
        strategy: String,
        is_apollo: bool,
        receives_rewards: bool,
        strategy_token: Option<String>,
    },
    RemoveStrategy {
        strategy_id: u64,
    },
    UpdateStrategy {
        strategy_id: u64,
        address: Option<String>,
        execution_paused: Option<bool>,
        deposits_paused: Option<bool>,
        withdrawals_paused: Option<bool>,
        rewards_paused: Option<bool>,
    },
    ExecuteStrategy {
        strategy_id: u64,
    },
    WithdrawFromStrategy {
        strategy_id: u64,
        amount: Uint128,
    },
    Receive(Cw20ReceiveMsg),
    EmergencyWithdraw {
        strategy_id: u64,
    },
    UpdateConfig {
        owner: Option<String>,
        warchest: Option<String>,
        distribution_schedule: Option<Vec<(u64, u64, Uint128)>>,
        genesis_time: Option<u64>,
        apollo_token: Option<String>,
        apollo_reward_percentage: Option<Decimal>,
    },
    ZapIntoStrategy {
        strategy_id: u64,
    },
    ZapOutOfStrategy {
        strategy_id: u64,
        amount: Uint128,
    },
    PassMessage {
        contract_addr: String,
        msg: Binary,
    },
    UpdateRewardWeights {},
    ClaimRewards {
        strategy_id: Option<u64>,
    },
    UpdateRewardInfos {
        rewards: Vec<CfeRewards>,
    },
    UpdateUserPendingRewardsForStrategy {
        user_addr: String,
        strategy_id: u64,
    },
    AddDex {
        dex_addr: String,
    },
    RemoveDex {
        dex_id: u8,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum QueryMsg {
    GetStrategies {
        limit: Option<u32>,
        start_from: Option<u64>,
    },
    GetStrategy {
        id: u64,
    },
    GetUserStrategies {
        user: String,
        limit: Option<u32>,
        start_from: Option<u64>,
    },
    GetConfig {},
    GetStrategyTvl {
        id: u64,
    },
    GetTotalTvl {},
    GetTotalCollectedFees {},
    GetExtensionTotalCollectedFees {},
    GetTotalRewardWeight {},
    GetTotalCfeRewardsByUser {
        address: String,
    },
    GetStakerInfo {
        staker: String,
        strategy_id: u64,
        time: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryConfig {
    pub owner: CanonicalAddr,
    pub warchest: CanonicalAddr,
    // ( initial time , token distribution period, total distribution amount)
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    pub genesis_time: u64,
    pub apollo_token: CanonicalAddr,
    pub apollo_reward_percentage: Decimal, //Percentage of rewards going to apollo strategies
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ApolloContracts {
    pub oracle: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ApolloContractsResponse {
    pub contracts: ApolloContracts,
    pub dex_adaptors: Vec<(u8, Addr)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryStrategyInfoResponse {
    pub id: u64,
    pub address: Addr,
    pub deprecated: bool,
    // pub global_index: Decimal,
    pub execution_paused: bool,
    pub withdrawals_paused: bool,
    pub deposits_paused: bool,
    pub total_bond_amount: Uint128,
    pub base_token: Addr,
    pub tvl: Uint128,
    pub performance_fee: Decimal,
    pub reward_index: Decimal256,
    pub extension_reward_index: Decimal256,
    pub lm_reward_index: Decimal256,
    pub last_distributed: u64,
    pub total_shares: Uint128,
    pub reward_weight: Uint256,
    pub strategy_token: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetStrategiesResponse {
    pub strategies: Vec<FactoryStrategyInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryUserInfoResponse {
    pub id: u64,
    pub base_token_balance: Uint128,
    pub shares: Uint128,
    pub lm_pending_reward: Uint128,
    pub lm_reward_index: Decimal256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetUserStrategiesResponse {
    pub strategies: Vec<FactoryUserInfoResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetConfigResponse {
    pub owner: Addr,
    pub warchest: Addr,
    pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    pub genesis_time: u64,
    pub apollo_token: Addr,
    pub apollo_reward_percentage: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTvlResponse {
    pub tvl: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTotalCollectedFeesResponse {
    pub total_collected_fees: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetExtensionTotalCollectedFeesResponse {
    pub extension_total_collected_fees: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTotalRewardWeightResponse {
    pub total_reward_weight: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct GetTotalCfeRewardsResponse {
    pub pending_reward: Uint128,
    pub extension_pending_reward: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum Cw20HookMsg {
    Deposit { strategy_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfoResponse {
    pub staker: String,
    pub reward_index: Decimal,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct CfeRewards {
    pub user: String,
    pub strategy_id: u64,
    pub pending_reward: Uint128,
    pub extension_pending_reward: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryStrategyConfig {
    //Strategies must have these three fields in their config, other fields are allowed.
    pub base_token: CanonicalAddr,
    pub performance_fee: Decimal,
}

// TODO - used for backward compatibility, remove after migration to v2
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct FactoryStrategyConfigResponse {
    pub config: FactoryStrategyConfig,
}
