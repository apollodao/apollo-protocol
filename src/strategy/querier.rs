use std::convert::TryFrom;

use cosmwasm_std::{
    to_binary, Addr, Deps, Env, QueryRequest, StdResult, Uint128, Uint256, WasmQuery,
};

use super::{
    msg::{AprResponse, ShouldExecuteResponse, StrategyInfo, TvlResponse, UserInfo},
    state::{BASE_TOKEN, ORACLE, STAKING_ADAPTOR, STRATEGY_TOKEN},
};

use crate::adaptor::msg::{
    AdaptorExecuteMsg, AdaptorQueryMsg, BaseAdaptorQueryMsg, BaseStakingAdaptorQueryMsg,
};
use crate::{
    oracle::query_oracle_price,
    utils::{calculate_user_bonds, query_token_balance},
};
use crate::{
    querier::query_cw20_token_info,
    strategy::state::{BaseConfig, BASE_DENOM, FACTORY},
};

pub fn query_total_bond_amount(deps: Deps, env: &Env, token: Option<Addr>) -> StdResult<Uint128> {
    let adaptor_addr = STAKING_ADAPTOR.load(deps.storage)?;

    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: adaptor_addr.to_string(),
        msg: to_binary(&AdaptorQueryMsg::Staking(
            BaseStakingAdaptorQueryMsg::TotalBondAmount {
                token,
                staker_addr: Some(env.contract.address.clone()),
            },
        ))?,
    }))
}

//Query the total shares in the Strategy and total base_token amount in the Strategy
pub fn query_strategy_info(deps: Deps, env: Env, token: Option<Addr>) -> StdResult<StrategyInfo> {
    let adaptor_addr = STAKING_ADAPTOR.load(deps.storage)?;
    let strategy_token = STRATEGY_TOKEN.load(deps.storage)?;

    let total_bond_amount = query_total_bond_amount(deps, &env, token)?;

    let total_shares = query_cw20_token_info(&deps.querier, strategy_token)?.total_supply;

    Ok(StrategyInfo {
        total_bond_amount,
        total_shares,
        global_index: Default::default(),
    })
}

pub fn query_user_info(
    deps: Deps,
    env: Env,
    user_addr: String,
    token: Option<Addr>,
) -> StdResult<UserInfo> {
    let strategy_token = STRATEGY_TOKEN.load(deps.storage)?;
    let user_addr = deps.api.addr_validate(&user_addr)?;

    // TODO: change to raw query
    let user_shares = query_token_balance(&deps.querier, strategy_token, user_addr)?;

    let strategy_info = query_strategy_info(deps, env, token)?;

    let user_bonds = calculate_user_bonds(
        user_shares,
        strategy_info.total_shares,
        strategy_info.total_bond_amount,
    )?;

    Ok(UserInfo {
        base_token_balance: user_bonds,
        shares: user_shares,
        index: Default::default(),
    })
}

pub fn query_tvl(deps: Deps, env: Env) -> StdResult<TvlResponse> {
    let oracle_addr = ORACLE.load(deps.storage)?;
    let asset_token = BASE_TOKEN.load(deps.storage)?;
    let proxy_addr = STAKING_ADAPTOR.load(deps.storage)?;

    let asset_token_price = query_oracle_price(
        &deps.querier,
        oracle_addr,
        "uusd".to_string(), // TODO: Pass in value?
        asset_token.to_string(),
        None, // TODO: Figure out where to store price age limit
    )?;

    // Query the proxy contract for the total bond amount
    let amount: Uint128 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: proxy_addr.to_string(),
        msg: to_binary(&AdaptorQueryMsg::Staking(
            BaseStakingAdaptorQueryMsg::TotalBondAmount {
                staker_addr: None,
                token: None,
            },
        ))?,
    }))?;

    let tvl = Uint256::from(amount) * asset_token_price;

    Ok(TvlResponse {
        tvl: Uint128::try_from(tvl)?,
    })
}

pub fn query_apr(deps: Deps) -> StdResult<AprResponse> {
    let proxy_addr = STAKING_ADAPTOR.load(deps.storage)?;
    let oracle = ORACLE.load(deps.storage)?;

    let res: AprResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: proxy_addr.to_string(),
        msg: to_binary(&AdaptorQueryMsg::Staking(BaseStakingAdaptorQueryMsg::Apr {
            oracle,
            price_age_limit: 86400u64, // TODO: Figure out where to store price age limit
            token: None,
        }))?,
    }))?;

    Ok(res)
}

pub fn query_should_execute(_deps: Deps, _cost: Uint128) -> StdResult<ShouldExecuteResponse> {
    Ok(ShouldExecuteResponse {
        should_execute: true,
    })
}

// TODO - remove after migration
pub fn query_base_config(deps: Deps) -> StdResult<BaseConfig> {
    Ok(BaseConfig {
        base_token: BASE_TOKEN.load(deps.storage)?,
        base_denom: BASE_DENOM.load(deps.storage)?,
        strategy_token: STRATEGY_TOKEN.load(deps.storage)?,
        proxy: STAKING_ADAPTOR.load(deps.storage)?,
        factory: FACTORY.load(deps.storage)?,
        oracle: ORACLE.load(deps.storage)?,
    })
}
