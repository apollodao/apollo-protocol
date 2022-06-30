use crate::utils::execute_send_tokens;
use crate::{adaptor::msg::BaseDexAdaptorExecuteMsg, error::ContractError};
use cosmwasm_std::{CustomQuery, DepsMut, Env, MessageInfo, Response};

pub fn base_dex_execute<C, D: CustomQuery>(
    deps: DepsMut<D>,
    env: Env,
    info: MessageInfo,
    msg: BaseDexAdaptorExecuteMsg<C>,
) -> Result<Response, ContractError> {
    match msg {
        BaseDexAdaptorExecuteMsg::SendTokens {
            token,
            recipient,
            amount,
            amount_pct,
            hook_msg,
        } => execute_send_tokens(
            deps, env, info, token, amount, amount_pct, recipient, hook_msg,
        ),
        _ => todo!(),
    }
}
