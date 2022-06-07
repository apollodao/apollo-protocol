use crate::adaptor::msg::{BaseAdaptorExecuteMsg, BaseDexAdaptorExecuteMsg};
use crate::utils::execute_send_tokens;
use cosmwasm_std::{CustomQuery, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw20_base::ContractError;

pub fn base_dex_execute<C, D: CustomQuery>(
    deps: DepsMut<D>,
    env: Env,
    info: MessageInfo,
    msg: BaseDexAdaptorExecuteMsg<C>,
) -> StdResult<Response> {
    match msg {
        BaseDexAdaptorExecuteMsg::SendTokens {
            token,
            recipient,
            amount,
            hook_msg,
        } => execute_send_tokens(deps, env, info, token, amount, recipient, hook_msg),
        _ => todo!(),
    }
}
