use apollo_asset::asset::Asset;
use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct InstantiateMsg {
    pub owner: String,
    pub distribution_contract: String, //Warchest contract
    pub terraswap_factory: String,
    pub base_denom: String,
    pub aust_token: String,
    pub target_assets: Vec<Asset>,
    pub max_spread: Decimal,
    pub anchor_market: String, //Anchor MoneyMarket Market contract
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum ExecuteMsg {
    Deposit {},
    Distribute {},
    UpdateConfig {
        owner: Option<String>,
        distribution_contract: Option<String>,
        terraswap_factory: Option<String>,
        base_denom: Option<String>,
        target_assets: Option<Vec<Asset>>,
        aust_token: Option<String>,
        max_spread: Option<Decimal>,
        anchor_market: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum QueryMsg {
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct ConfigResponse {
    pub owner: Addr,
    pub distribution_contract: Addr,
    pub terraswap_factory: Addr,
    pub base_denom: String,
    pub target_assets: Vec<Asset>,
    pub aust_token: Addr,
    pub max_spread: Decimal,
    pub total_weight: u64,
    pub anchor_market: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct MigrateMsg {
    pub target_assets: Vec<Asset>,
}
