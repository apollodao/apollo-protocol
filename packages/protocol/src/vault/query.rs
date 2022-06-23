use std::convert::TryInto;

use crate::vault::execute::calculate_shares;
use crate::vault::msg::{
    AprResponse, BaseConfig, ShouldExecuteResponse, StrategyInfo, TvlResponse, UserInfo,
};
use crate::vault::state::{BASE_DENOM, BASE_TOKEN, ORACLE};

use apollo_protocol::{
    adaptor::msg::QueryMsg as ProxyQueryMsg, oracle::query_oracle_price,
    utils::calculate_user_bonds,
};
use cosmwasm_std::Uint256;
use cosmwasm_std::{to_binary, Deps, Env, QueryRequest, StdResult, Uint128, WasmQuery};

use cw20_base::contract::{query_balance, query_token_info};

use super::state::ADAPTOR;

/// Simulates a deposit into the strategy.
///
/// Arguments:
/// * `deps`
/// * `env`
/// * `amount` - Amount of tokens to deposit
///
/// Returns:
/// * `Ok(Uint128)` - Amount of vault shares to receive
pub fn query_simulate_deposit(deps: Deps, env: Env, amount: Uint128) -> StdResult<Uint128> {
    let strategy_info = query_strategy_info(deps)?;

    // Calculate how many strategy tokens to mint
    let tokens_to_mint = if strategy_info.total_shares.is_zero() {
        // If all tokens in the contract where sent as part of this deposit then return that amount of shares
        amount
    } else {
        calculate_shares(deps.storage, &deps.querier, &env, &strategy_info, amount)?
    };

    Ok(tokens_to_mint)
}

/// Simulates a withdraw from the strategy.
///
/// Arguments:
/// * `deps`
/// * `env`
/// * `amount` - Amount of vault shares to liquidate
///
/// Returns:
/// * `Ok(Uint128)` - Amount of underlying base tokens to receive
pub fn query_simulate_withdraw(deps: Deps, _env: &Env, amount: Uint128) -> StdResult<Uint128> {
    let strategy_info = query_strategy_info(deps)?;

    let base_tokens_to_withdraw = calculate_user_bonds(
        amount,
        strategy_info.total_shares,
        strategy_info.total_bond_amount,
    )?;

    Ok(base_tokens_to_withdraw)
}

//Query the total shares in the Strategy and total base_token amount in the Strategy
pub fn query_strategy_info(deps: Deps) -> StdResult<StrategyInfo> {
    let proxy_addr = ADAPTOR.load(deps.storage)?;

    let total_bond_amount: Uint128 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: proxy_addr.to_string(),
        msg: to_binary(&ProxyQueryMsg::TotalBondAmount {})?,
    }))?;

    let token_info = query_token_info(deps)?;

    Ok(StrategyInfo {
        total_bond_amount,
        total_shares: token_info.total_supply,
        global_index: Default::default(),
    })
}

pub fn query_user_info(deps: Deps, address: String) -> StdResult<UserInfo> {
    let _user_addr = deps.api.addr_validate(address.as_str())?;

    let user_shares = query_balance(deps, address)?.balance;

    let strategy_info = query_strategy_info(deps)?;

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

pub fn query_tvl(deps: Deps, _env: Env) -> StdResult<TvlResponse> {
    let oracle_addr = ORACLE.load(deps.storage)?;
    let asset_token = BASE_TOKEN.load(deps.storage)?;
    let proxy_addr = ADAPTOR.load(deps.storage)?;

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
        msg: to_binary(&ProxyQueryMsg::TotalBondAmount {})?,
    }))?;

    let tvl = Uint256::from(amount) * asset_token_price;

    Ok(TvlResponse {
        tvl: tvl.try_into()?,
    })
}

pub fn query_apr(deps: Deps) -> StdResult<AprResponse> {
    let proxy_addr = ADAPTOR.load(deps.storage)?;
    let oracle = ORACLE.load(deps.storage)?;

    let res: AprResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: proxy_addr.to_string(),
        msg: to_binary(&ProxyQueryMsg::Apr {
            oracle,
            price_age_limit: 86400u64, // TODO: Figure out where to store price age limit
        })?,
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
        proxy: ADAPTOR.load(deps.storage)?,
        oracle: ORACLE.load(deps.storage)?,
    })
}
