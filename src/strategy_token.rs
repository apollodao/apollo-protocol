use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub base_init_msg: Cw20InstantiateMsg,
    pub apollo_factory: String,
    pub strategy_id: u64,
}

/// Allow Migration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct MigrateMsg {}
