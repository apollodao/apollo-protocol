use apollo_asset::asset::AssetInfo;
use cosmwasm_std::{
    to_binary, Addr, Api, BankMsg, Binary, CanonicalAddr, CheckedFromRatioError, Coin,
    ConversionOverflowError, CosmosMsg, CustomQuery, Decimal, Decimal256, Deps, DepsMut, Empty,
    Env, Event, Fraction, MessageInfo, QuerierWrapper, QueryRequest, Response, StdError, StdResult,
    Uint128, Uint256, WasmMsg, WasmQuery,
};
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg, TokenInfoResponse};
use std::convert::{TryFrom, TryInto};

use crate::error::ContractError;

pub fn only_allow_human_addr(
    message_info: &MessageInfo,
    address: &str,
) -> Result<Empty, ContractError> {
    if address != message_info.sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(Empty {})
    }
}

pub fn only_allow_addresses(
    message_info: &MessageInfo,
    addresses: Vec<&str>,
) -> Result<Empty, ContractError> {
    for address in &addresses {
        if *address == &message_info.sender.to_string() {
            return Ok(Empty {});
        }
    }
    Err(ContractError::Std(StdError::generic_err(format!(
        "unauthorized - {}, required - {:?}",
        message_info.sender, addresses
    ))))
}

pub fn only_allow_canon_addr(
    api: &dyn Api,
    message_info: &MessageInfo,
    address: &CanonicalAddr,
) -> Result<Empty, ContractError> {
    let sender_address_raw = api.addr_canonicalize(message_info.sender.as_str())?;
    if address != &sender_address_raw {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Empty {})
}

const DECIMAL_FRACTIONAL: Uint128 = Uint128::new(1_000_000_000u128);

// TODO: remove decimal_division, reverse_decimal and decimal_multiplication
/// return a / b
pub fn decimal_division(a: Decimal, b: Decimal) -> Result<Decimal, CheckedFromRatioError> {
    Decimal::checked_from_ratio(DECIMAL_FRACTIONAL * a, b * DECIMAL_FRACTIONAL)
}

pub fn reverse_decimal(decimal: Decimal) -> Result<Decimal, CheckedFromRatioError> {
    Decimal::checked_from_ratio(DECIMAL_FRACTIONAL, decimal * DECIMAL_FRACTIONAL)
}

pub fn decimal_multiplication(a: Decimal, b: Decimal) -> Result<Decimal, CheckedFromRatioError> {
    Decimal::checked_from_ratio(a * DECIMAL_FRACTIONAL * b, DECIMAL_FRACTIONAL)
}

pub fn query_supply(querier: &QuerierWrapper, contract_addr: Addr) -> StdResult<Uint128> {
    // load price form the oracle
    // TODO: Should we use query_wasm_smart or query_wasm_raw?
    let token_info: TokenInfoResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
    }))?;

    Ok(token_info.total_supply)
}

pub fn round_half_to_even_128(a: Decimal) -> Uint128 {
    let numerator = a.numerator();
    let fraction_unit = Decimal::one().numerator();
    let truncated = (numerator / fraction_unit) * fraction_unit;
    let remainder = numerator - truncated;
    let result;

    //Round up if remainder is > 0.5 or if remainder is exactly 0.5 and truncated is odd
    //Else, round down
    if (remainder == fraction_unit / Uint128::new(2)
        && (numerator / fraction_unit) % Uint128::new(2) != Uint128::zero())
        || remainder > fraction_unit / Uint128::new(2)
    {
        //round up
        result = (truncated + fraction_unit) / fraction_unit;
    } else {
        //round down
        result = truncated / fraction_unit;
    }

    result
}

pub fn round_half_to_even_256(a: Decimal256) -> Uint256 {
    let numerator = a.numerator();
    let fraction_unit = Decimal256::one().numerator();
    let truncated = (numerator / fraction_unit) * fraction_unit;
    let remainder = numerator - truncated;
    let result;

    //Round up if remainder is > 0.5 or if remainder is exactly 0.5 and truncated is odd
    //Else, round down
    if (remainder == fraction_unit / Uint256::from_u128(2u128)
        && (numerator / fraction_unit) % Uint256::from_u128(2u128) != Uint256::zero())
        || remainder > fraction_unit / Uint256::from_u128(2u128)
    {
        //round up
        result = (truncated + fraction_unit) / fraction_unit;
    } else {
        //round down
        result = truncated / fraction_unit;
    }

    result
}

// pub fn simulate_routed_swap(
//     querier: &QuerierWrapper,
//     from: String,
//     to: String,
//     amount: Uint128,
//     osmosis_router: Addr,
//     self_address: Addr,
//     pool_id: u64,
// ) -> StdResult<Uint128> {
//     println!("dog1 = {}", amount);
//     querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//         contract_addr: osmosis_router.to_string(),
//         msg: to_binary(&OsmosisQuery::EstimateSwap {
//             sender: self_address.to_string(),
//             first: Swap::new(pool_id, from, to),
//             route: vec![],
//             amount: SwapAmount::In(amount),
//         })
//         .unwrap(),
//     }))
// }

// pub fn create_routed_swap_msg(
//     querier: &QuerierWrapper,
//     from: AssetInfo,
//     to: AssetInfo,
//     amount: Uint128,
//     max_slippage: Decimal,
//     terraswap_router: Addr,
// ) -> StdResult<CosmosMsg> {
//     let estimated_tokens_per_base = simulate_routed_swap(
//         querier,
//         from.clone(),
//         to.clone(),
//         amount,
//         terraswap_router.clone(),
//     )?;
//     println!("cat1");
//     let minimum_recieve = amount
//         * decimal_multiplication(
//             Decimal::from_ratio(estimated_tokens_per_base, 1000000u128),
//             Decimal::from_ratio(1u128, 1u128) - max_slippage,
//         );

//     println!("minimum_recieve = {}", minimum_recieve);
//     println!("est_price = {}", estimated_tokens_per_base);
//     println!("max_slippage = {}", max_slippage);

//     match from {
//         AssetInfo::NativeToken { denom } => Ok(CosmosMsg::Wasm(WasmMsg::Execute {
//             contract_addr: terraswap_router.to_string(),
//             msg: to_binary(&RouterCw20HookMsg::ExecuteSwapOperations {
//                 operations: vec![SwapOperation::TerraSwap {
//                     offer_asset_info: AssetInfo::NativeToken {
//                         denom: denom.clone(),
//                     },
//                     ask_asset_info: to,
//                 }],
//                 minimum_receive: Some(minimum_recieve),
//                 to: None,
//             })?,
//             funds: vec![Coin { amount, denom }],
//         })),
//         AssetInfo::Token { contract_addr } => Ok(CosmosMsg::Wasm(WasmMsg::Execute {
//             contract_addr: contract_addr.to_string(),
//             msg: to_binary(&Cw20ExecuteMsg::Send {
//                 amount,
//                 contract: terraswap_router.to_string(),
//                 msg: to_binary(&RouterCw20HookMsg::ExecuteSwapOperations {
//                     operations: vec![SwapOperation::TerraSwap {
//                         offer_asset_info: AssetInfo::Token { contract_addr },
//                         ask_asset_info: to,
//                     }],
//                     minimum_receive: Some(minimum_recieve),
//                     to: None,
//                 })?,
//             })
//             .unwrap(),
//             funds: vec![],
//         })),
//     }
// }

pub fn calculate_user_bonds(
    shares: Uint128,
    total_shares: Uint128,
    total_bond_amount: Uint128,
) -> Result<Uint128, ConversionOverflowError> {
    if total_shares.is_zero() {
        return Ok(Uint128::zero());
    }
    Ok(Uint128::try_from(
        (Decimal256::raw(total_bond_amount.u128())
            * Decimal256::from_ratio(shares.u128(), total_shares.u128()))
        .atomics(),
    )?)
}

// // generate swap messages for sending all sent funds to denom
// pub fn swap_funds_for(
//     querier: &QuerierWrapper,
//     env: &Env,
//     funds: &Vec<Coin>,
//     denom: &str,
// ) -> StdResult<Vec<AstroSwapOperation>> {
//     let operations = funds
//         .iter()
//         .filter(|f| {
//             f.denom != denom
//                 && (!f.amount.is_zero()
//                     || !AstroAssetInfo::NativeToken {
//                         denom: f.denom.to_string(),
//                     }
//                     .query_pool(querier, env.contract.address.clone())
//                     .unwrap_or(Uint128::zero())
//                     .is_zero())
//         })
//         .map(|f| AstroSwapOperation::NativeSwap {
//             offer_denom: f.denom.to_string(),
//             ask_denom: denom.to_string(),
//         })
//         .collect::<Vec<AstroSwapOperation>>();
//     let denom_balance = AstroAssetInfo::NativeToken {
//         denom: denom.to_string(),
//     }
//     .query_pool(querier, env.contract.address.clone())
//     .unwrap_or(Uint128::zero());
//     if denom_balance.is_zero() && operations.len() == 0 {
//         return Err(StdError::generic_err("no funds provided to swap"));
//     }
//     Ok(operations)
// }

// pub fn calculate_minimum_receive(
//     querier: &QuerierWrapper,
//     dex_router: &Addr,
//     offer_amount: Uint128,
//     operations: &Vec<AstroSwapOperation>,
//     max_spread: Option<Decimal>,
// ) -> StdResult<Option<Uint128>> {
//     let minimum_receive = match max_spread {
//         Some(max_spread) => {
//             let simulation_response: SimulateSwapOperationsResponse =
//                 querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//                     contract_addr: dex_router.to_string(),
//                     msg: to_binary(&AstroRouterQueryMsg::SimulateSwapOperations {
//                         offer_amount,
//                         operations: operations.clone(),
//                     })?,
//                 }))?;
//             Some((Decimal::one() - max_spread) * simulation_response.amount)
//         }
//         None => None,
//     };
//     Ok(minimum_receive)
// }

pub fn query_token_balance(
    querier: &QuerierWrapper,
    contract_addr: Addr,
    account_addr: Addr,
) -> StdResult<Uint128> {
    let res: BalanceResponse = querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: account_addr.to_string(),
            })?,
        }))
        .unwrap_or_else(|_| BalanceResponse {
            balance: Uint128::zero(),
        });

    // load balance form the token contract
    Ok(res.balance)
}

pub fn execute_send_tokens<D: CustomQuery, T>(
    deps: DepsMut<D>,
    env: Env,
    info: MessageInfo,
    token: AssetInfo,
    amount: Option<Uint128>,
    recipient: Addr,
    hook_msg: Option<Binary>,
) -> Result<Response<T>, ContractError> {
    only_allow_human_addr(&info, env.contract.address.as_str())?;

    let amount = amount.unwrap_or_else(|| {
        token
            .query_balance(&deps.querier, env.contract.address.clone())
            .unwrap_or_default()
    });

    let funds = if token.is_native_token() {
        vec![Coin::new(amount.u128(), token.to_string())]
    } else {
        vec![]
    };

    let send = match hook_msg {
        Some(cw20_hook_msg) => {
            if token.is_native_token() {
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: recipient.to_string(),
                    msg: cw20_hook_msg,
                    funds,
                })
            } else {
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: token.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::Send {
                        contract: recipient.to_string(),
                        amount,
                        msg: cw20_hook_msg,
                    })?,
                    funds,
                })
            }
        }
        None => CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient.to_string(),
            amount: funds,
        }),
    };

    Ok(Response::new().add_message(send))
}

pub fn parse_u8_key(data: &[u8]) -> Result<u8, ContractError> {
    match data[0..8].try_into() {
        Ok(bytes) => Ok(u8::from_be_bytes(bytes)),
        Err(_) => Err(ContractError::CorruptedData {}),
    }
}

pub fn parse_contract_addr_from_instantiate_event(
    deps: Deps,
    events: Vec<Event>,
) -> Result<Addr, ContractError> {
    Ok(deps.api.addr_validate(
        &events
            .into_iter()
            .find(|e| e.ty == "instantiate_contract")
            .and_then(|ev| {
                ev.attributes
                    .into_iter()
                    .find(|a| a.key == "contract_address")
            })
            .unwrap()
            .value,
    )?)
}

pub fn decimal256_to_decimal(decimal: Decimal256) -> Result<Decimal, ContractError> {
    let atomics: Uint128 = decimal
        .atomics()
        .try_into()
        .map_err(|e| ContractError::ConversionOverflowError(e))?;
    Ok(Decimal::from_atomics(atomics, decimal.decimal_places())
        .map_err(|e| ContractError::DecimalRangeExceeded(e))?)
}

pub fn decimal_to_decimal256(decimal: Decimal) -> Result<Decimal256, ContractError> {
    let atomics: Uint128 = decimal.atomics();
    Ok(Decimal256::from_atomics(atomics, decimal.decimal_places())
        .map_err(|e| ContractError::Decimal256RangeExceeded(e))?)
}