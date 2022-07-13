use std::convert::TryInto;

use crate::vault::execute::calculate_shares;

use crate::vault::state::{BASE_DENOM, BASE_TOKEN, ORACLE};

use crate::{
    adaptor::msg::QueryMsg as AdaptorQueryMsg, oracle::query_oracle_price,
    utils::calculate_user_bonds,
};
use cosmwasm_std::{to_binary, Deps, Env, QueryRequest, StdResult, Uint128, WasmQuery};
use cosmwasm_std::{Decimal, Uint256};

use cw20_base::contract::{query_balance, query_token_info};

use super::state::{BaseVaultConfig, UserInfo, VaultInfo, ADAPTOR};

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

/// Query core info about the vault.
///
/// Returns:
/// * Vault info as a [`VaultInfo`] struct.
pub fn query_strategy_info(deps: Deps) -> StdResult<VaultInfo> {
    let proxy_addr = ADAPTOR.load(deps.storage)?;

    let total_bond_amount: Uint128 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: proxy_addr.to_string(),
        msg: to_binary(&AdaptorQueryMsg::TotalBondAmount {})?,
    }))?;

    let token_info = query_token_info(deps)?;

    Ok(VaultInfo {
        total_bond_amount,
        total_shares: token_info.total_supply,
        global_index: Default::default(),
    })
}

/// Query info about a users position in the vault. Returns info as a [`UserInfo`] struct.
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

// TODO: Should we move the TVL and APR calculations to the specific implementations, i.e. autocompound?
//         That way the vault does not depend on the oracle. Instead the vault could handle the total_bond_amount
//         and reward_rate queries. But not converting to APR or TVL.
pub fn query_tvl(deps: Deps, _env: Env) -> StdResult<Uint128> {
    let oracle_addr = ORACLE.load(deps.storage)?;
    let asset_token = BASE_TOKEN.load(deps.storage)?;
    let proxy_addr = ADAPTOR.load(deps.storage)?;

    let asset_token_price = query_oracle_price(
        &deps.querier,
        oracle_addr,
        "uusd".to_string(), // TODO: UST used here...
        asset_token.to_string(),
        None, // TODO: Figure out where to store price age limit
    )?;

    // Query the proxy contract for the total bond amount
    let amount: Uint128 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: proxy_addr.to_string(),
        msg: to_binary(&AdaptorQueryMsg::TotalBondAmount {})?,
    }))?;

    let tvl = Uint256::from(amount) * asset_token_price;

    Ok(tvl.try_into()?)
}

pub fn query_apr(deps: Deps) -> StdResult<Decimal> {
    let adaptor_addr = ADAPTOR.load(deps.storage)?;

    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: adaptor_addr.to_string(),
        msg: to_binary(&AdaptorQueryMsg::Apr {
            price_age_limit: 86400u64, // TODO: Figure out where to store price age limit
            oracle: None,
        })?,
    }))
}

pub fn query_should_execute(_deps: Deps, _cost: Uint128) -> StdResult<bool> {
    Ok(false)
}

// TODO - remove after migration
pub fn query_base_config(deps: Deps) -> StdResult<BaseVaultConfig> {
    Ok(BaseVaultConfig {
        base_token: BASE_TOKEN.load(deps.storage)?,
        base_denom: BASE_DENOM.load(deps.storage)?,
        adaptor: ADAPTOR.load(deps.storage)?,
    })
}
