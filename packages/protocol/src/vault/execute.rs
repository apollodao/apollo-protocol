use std::convert::TryInto;

use crate::strategy::msg::PendingRewardsResponse;
use crate::utils::round_half_to_even_256;
use crate::{oracle::query_oracle_price, strategy::msg::StrategyInfo};
use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, Decimal256, DepsMut, Env, MessageInfo, QuerierWrapper,
    QueryRequest, Response, StdError, StdResult, Storage, Uint128, WasmMsg, WasmQuery,
};

use super::{
    error::ContractError,
    msg::Cw4626ConfigOptions,
    query::query_simulate_withdraw,
    state::{
        ADAPTOR, BASE_TOKEN, CLAIMS, DEPOSITS_ALLOWED, ORACLE, UNBONDING_PERIOD,
        WITHDRAWALS_ALLOWED,
    },
};
use crate::adaptor::msg::{ExecuteMsg as AdaptorExecuteMsg, QueryMsg as AdaptorQueryMsg};

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    _message_info: MessageInfo,
    config_options: Cw4626ConfigOptions,
) -> Result<Response, ContractError> {
    if let Some(withdrawals_allowed) = config_options.withdrawals_allowed {
        WITHDRAWALS_ALLOWED.save(deps.storage, &withdrawals_allowed)?;
    }
    if let Some(deposits_allowed) = config_options.deposits_allowed {
        DEPOSITS_ALLOWED.save(deps.storage, &deposits_allowed)?;
    }

    Ok(Response::default())
}

/// ## Description
/// Unbond `amount` of lp_tokens from the strategy contract.
/// Creates a [`cw-controllers::Claim`] that expires after the unbond period.
pub fn execute_unbond(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let base_token_amount = query_simulate_withdraw(deps.as_ref(), &env, amount)?;
    let expiration = UNBONDING_PERIOD.load(deps.storage)?.after(&env.block);
    CLAIMS.create_claim(deps.storage, &info.sender, base_token_amount, expiration)?;

    let adaptor_addr = ADAPTOR.load(deps.storage)?;

    // TODO: Burn vault tokens

    // Perform withdraw on adaptor
    let unbond = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: adaptor_addr.to_string(),
        msg: to_binary(&AdaptorExecuteMsg {
            amount,
            recipient: env.contract.address.to_string(),
        })?,
        funds: vec![],
    });

    Ok(Response::new().add_message(unbond))
}

pub fn calculate_shares(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    _env: &Env,
    strategy_info: &StrategyInfo,
    bonds: Uint128,
) -> StdResult<Uint128> {
    if strategy_info.total_bond_amount.is_zero() {
        return Ok(Uint128::zero());
    }

    let proxy_addr = ADAPTOR.load(storage)?;
    let oracle_addr = ORACLE.load(storage)?;
    let base_token = BASE_TOKEN.load(storage)?;

    // Query accrued rewards
    let pending_rewards_in_base_denom = querier
        .query::<PendingRewardsResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: proxy_addr.to_string(),
            msg: to_binary(&AdaptorQueryMsg::PendingRewards {})?,
        }))?
        .total_value;

    // Query token prices
    let asset_token_price = query_oracle_price(
        querier,
        oracle_addr,
        "uusd".to_string(), // TODO: Pass in value?
        base_token.to_string(),
        None, // TODO: Figure out where to store price age limit
    )?;

    // Calculate pending rewards in asset tokens
    let pending_rewards_in_assets =
        Decimal256::from_ratio(pending_rewards_in_base_denom.u128(), 1u128) / asset_token_price;

    Ok(round_half_to_even_256(
        Decimal256::from_ratio(strategy_info.total_shares, 1u128)
            * Decimal256::from_ratio(bonds, 1u128)
            / (Decimal256::from_ratio(strategy_info.total_bond_amount, 1u128)
                + pending_rewards_in_assets),
    )
    .try_into()?)
}
