use apollo_asset::asset::Asset;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal};

#[cw_serde]
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

#[cw_serde]
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

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
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

#[cw_serde]
pub struct MigrateMsg {
    pub target_assets: Vec<Asset>,
}
