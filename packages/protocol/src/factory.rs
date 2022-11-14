use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Decimal, Order, StdError, StdResult, Storage, Uint128};
use cw20::Cw20ReceiveMsg;
use cw_storage_plus::{Item, Map};

use crate::collector::ConfigResponse;

pub static APOLLO_CONTRACTS: Item<ApolloContracts> = Item::new("apollo_contracts");
pub static APOLLO_DEX_ADAPTORS: Map<u8, Addr> = Map::new("apollo_dex_adaptors");
pub const APOLLO_DEX_COUNT: Item<u8> = Item::new("apollo_dex_count");

// TODO: Is this an iterator? If not we should define as increment instead of next
pub fn next_dex_id(store: &mut dyn Storage) -> StdResult<u8> {
    let id: u8 = APOLLO_DEX_COUNT.may_load(store)?.unwrap_or_default() + 1;
    APOLLO_DEX_COUNT.save(store, &id)?;
    Ok(id)
}

pub fn add_dex(storage: &mut dyn Storage, dex_addr: &Addr) -> StdResult<u8> {
    let id = next_dex_id(storage)?;
    APOLLO_DEX_ADAPTORS.save(storage, id, dex_addr)?;
    Ok(id)
}

pub fn remove_dex(storage: &mut dyn Storage, dex_id: u8) -> StdResult<()> {
    APOLLO_DEX_ADAPTORS.remove(storage, dex_id);
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

#[cw_serde]
pub enum ExecuteMsg {
    AddStrategy {
        strategy: String,
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
    AddDex {
        dex_addr: String,
    },
    RemoveDex {
        dex_id: u8,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<GetStrategiesResponse>)]
    /// GetStrategies
    GetStrategies {
        limit: Option<u32>,
        start_from: Option<u64>,
    },
    #[returns(GetStrategiesResponse)]
    /// GetStrategy
    GetStrategy { id: u64 },
    #[returns(Vec<GetUserStrategiesResponse>)]
    /// GetUserStrategies
    GetUserStrategies {
        user: String,
        limit: Option<u32>,
        start_from: Option<u64>,
    },
    // TODO: check this
    #[returns(ConfigResponse)]
    /// GetUserStrategies
    GetConfig {},
    #[returns(Decimal)]
    /// GetStrategyTvl
    GetStrategyTvl { id: u64 },
    #[returns(Decimal)]
    /// GetTotalTvl
    GetTotalTvl {},
    #[returns(StakerInfoResponse)]
    /// GetStakerInfo
    GetStakerInfo { staker: String, strategy_id: u64 },
}

#[cw_serde]
pub struct FactoryConfig {
    pub owner: Addr,
    pub warchest: Addr,
}

#[cw_serde]
pub struct ApolloContracts {
    pub oracle: Addr,
}

#[cw_serde]
pub struct ApolloContractsResponse {
    pub contracts: ApolloContracts,
    pub dex_adaptors: Vec<(u8, Addr)>,
}

#[cw_serde]
pub struct FactoryStrategyInfoResponse {
    pub id: u64,
    pub address: Addr,
    pub deprecated: bool,
    pub execution_paused: bool,
    pub withdrawals_paused: bool,
    pub deposits_paused: bool,
    pub total_bond_amount: Uint128,
    pub base_token: Addr,
    pub tvl: Uint128,
    pub performance_fee: Decimal,
    pub total_shares: Uint128,
    pub strategy_token: Option<Addr>,
}

#[cw_serde]
pub struct GetStrategiesResponse {
    pub strategies: Vec<FactoryStrategyInfoResponse>,
}

#[cw_serde]
pub struct FactoryUserInfoResponse {
    pub id: u64,
    pub base_token_balance: Uint128,
    pub shares: Uint128,
}

#[cw_serde]
pub struct GetUserStrategiesResponse {
    pub strategies: Vec<FactoryUserInfoResponse>,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub owner: Addr,
    pub warchest: Addr,
}

#[cw_serde]
pub struct GetTvlResponse {
    pub tvl: Uint128,
}

#[cw_serde]
pub enum Cw20HookMsg {
    Deposit { strategy_id: u64 },
}

#[cw_serde]
pub struct StakerInfoResponse {
    pub staker: String,
    pub bond_amount: Uint128,
}

#[cw_serde]
pub struct FactoryStrategyConfig {
    //Strategies must have these three fields in their config, other fields are allowed.
    pub base_token: Addr,
    pub performance_fee: Decimal,
}

// TODO - used for backward compatibility, remove after migration to v2
#[cw_serde]
pub struct FactoryStrategyConfigResponse {
    pub config: FactoryStrategyConfig,
}
