// use cosmwasm_std::{
//     to_binary, Binary, Deps, DepsMut, Env, MessageInfo, ReplyOn, Response, StdError, StdResult,
//     SubMsg, WasmMsg,
// };
// use cw20::MinterResponse;
//
// use super::{
//     msg::InstantiateMsg,
//     reply::REPLY_SAVE_STRATEGY_TOKEN_ADDR,
//     state::{BASE_TOKEN, FACTORY, STAKING_ADAPTOR},
// };
//
// use crate::strategy::msg::{BaseQueryMsg, BaseStrategyExecuteMsg, ExecuteMsg, QueryMsg};
// use crate::strategy::querier::{
//     query_apr, query_base_config, query_should_execute, query_strategy_info, query_tvl,
//     query_user_info,
// };
// use crate::strategy::state::{BASE_DENOM, ORACLE};
// use crate::strategy_token::InstantiateMsg as StrategyTokenInstantiateMsg;
// use crate::{
//     strategy::execute::{
//         base_execute_deposit, base_receive_cw20, execute_emergency_withdraw, execute_withdraw,
//     },
//     utils::execute_send_tokens,
// };
// use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
//
// pub fn base_strategy_instantiate(
//     deps: DepsMut,
//     env: Env,
//     msg: InstantiateMsg,
// ) -> StdResult<Response> {
//     if msg.asset_token.is_native_token() {
//         return Err(StdError::generic_err(
//             "Native asset token is not supported for single asset autocompound strategy",
//         ));
//     }
//
//     BASE_TOKEN.save(deps.storage, &msg.asset_token)?;
//     FACTORY.save(deps.storage, &deps.api.addr_validate(&msg.apollo_factory)?)?;
//     ORACLE.save(deps.storage, &deps.api.addr_validate(&msg.oracle_contract)?)?;
//     BASE_DENOM.save(deps.storage, &msg.base_denom)?;
//     STAKING_ADAPTOR.save(deps.storage, &deps.api.addr_validate(&msg.adaptor_addr)?)?;
//
//     let init_strategy_token = SubMsg {
//         // Instantiate strategy token
//         msg: WasmMsg::Instantiate {
//             admin: Some(msg.apollo_factory.to_string()),
//             code_id: msg.strategy_token_code_id,
//             msg: to_binary(&StrategyTokenInstantiateMsg {
//                 base_init_msg: Cw20InstantiateMsg {
//                     name: msg.strategy_token_name,
//                     symbol: msg.strategy_token_symbol,
//                     decimals: 6,
//                     initial_balances: vec![],
//                     mint: Some(MinterResponse {
//                         minter: env.contract.address.to_string(),
//                         cap: None,
//                     }),
//                     marketing: None,
//                 },
//                 apollo_factory: msg.apollo_factory,
//                 strategy_id: msg.strategy_id,
//             })?,
//             funds: vec![],
//             label: "".to_string(),
//         }
//         .into(),
//         gas_limit: None,
//         id: REPLY_SAVE_STRATEGY_TOKEN_ADDR,
//         reply_on: ReplyOn::Always,
//         // TODO: Will the entire instantiate fail if the submessage fails then?
//     };
//
//     Ok(Response::new().add_submessages(vec![init_strategy_token]))
// }
//
// pub fn base_strategy_execute<S>(
//     deps: DepsMut,
//     env: Env,
//     message_info: MessageInfo,
//     msg: BaseStrategyExecuteMsg<S>,
// ) -> StdResult<Response> {
//     match msg {
//         // BaseStrategyExecuteMsg::Receive(msg) => execute_receive_cw20(deps, env, message_info, msg),
//         BaseStrategyExecuteMsg::ExecuteStrategy { .. } => todo!(),
//         BaseStrategyExecuteMsg::Strategy(_) => todo!(),
//         BaseStrategyExecuteMsg::SendTokens { .. } => todo!()
//     }
// }
//
// pub fn base_strategy_query<S>(deps: Deps, env: Env, msg: BaseQueryMsg<S>) -> StdResult<Binary> {
//     match msg {
//         BaseQueryMsg::Config {} => to_binary(&query_base_config(deps)?),
//         BaseQueryMsg::UserInfo { address, token } => {
//             to_binary(&query_user_info(deps, env, address, token)?)
//         }
//         BaseQueryMsg::StrategyInfo { token } => to_binary(&query_strategy_info(deps, env, token)?),
//         BaseQueryMsg::ShouldExecute { cost } => to_binary(&query_should_execute(deps, cost)?),
//         BaseQueryMsg::Tvl {} => to_binary(&query_tvl(deps, env)?),
//         BaseQueryMsg::Apr {} => to_binary(&query_apr(deps)?),
//         _ => unimplemented!(),
//     }
// }
