use cosmwasm_std::{
    attr, Addr, DepsMut, Env, Reply, Response, StdError, StdResult, Storage, SubMsgResponse,
    SubMsgResult,
};

use crate::{error::ContractError, utils::parse_contract_addr_from_instantiate_event};

use super::state::STRATEGY_TOKEN;

/**
 * Submessage signals
 */
pub const REPLY_SAVE_STRATEGY_TOKEN_ADDR: u64 = 0;
pub const REPLY_STRATEGY_EXECUTE_GRACE_FAIL: u64 = 1;

/**
 * Base reply handler for strategy. Sets up strategy token and proxy addresses.
 */
pub fn base_strategy_reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_SAVE_STRATEGY_TOKEN_ADDR => match msg.result {
            SubMsgResult::Ok(subcall) => {
                reply_save_addr(deps, subcall, |s, addr| STRATEGY_TOKEN.save(s, &addr))
            }
            SubMsgResult::Err(_) => Err(ContractError::FailedToInitializeStrategyToken {}),
        },
        REPLY_STRATEGY_EXECUTE_GRACE_FAIL => match msg.result {
            SubMsgResult::Err(e) => {
                if e.contains("not optimal to execute") {
                    Ok(Response::new().add_attribute("autocompound", e))
                } else {
                    Err(ContractError::Std(StdError::generic_err(e)))
                }
            }
            SubMsgResult::Ok(_) => Err(ContractError::UnExpected {}),
        },
        _ => Err(ContractError::UnknownReply {}),
    }
}

/**
 * Parses the contract address of an initialized contract from the
 * submessage response and saves it using the `save_addr` function supplied.
 */
pub fn reply_save_addr(
    deps: DepsMut,
    subcall: SubMsgResponse,
    save_addr: fn(&mut dyn Storage, Addr) -> StdResult<()>,
) -> Result<Response, ContractError> {
    let addr = parse_contract_addr_from_instantiate_event(deps.as_ref(), subcall.events)?;
    save_addr(deps.storage, addr.clone())?;

    let attrs = vec![
        attr("action", "reply_save_addr"),
        attr("contract_addr", addr.to_string()),
    ];
    Ok(Response::new().add_attributes(attrs))
}
