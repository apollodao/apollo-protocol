use cosmwasm_schema::cw_serde;
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_init_msg: Cw20InstantiateMsg,
    pub apollo_factory: String,
    pub strategy_id: u64,
}

/// Allow Migration
#[cw_serde]
pub struct MigrateMsg {}
