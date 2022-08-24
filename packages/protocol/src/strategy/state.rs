use apollo_asset::asset::AssetInfo;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token");
pub const BASE_DENOM: Item<AssetInfo> = Item::new("base_denom");
pub const STRATEGY_TOKEN: Item<Addr> = Item::new("strategy_token");
pub const STAKING_ADAPTOR: Item<Addr> = Item::new("staking_adaptor");
pub const FACTORY: Item<Addr> = Item::new("factory");
pub const ORACLE: Item<Addr> = Item::new("oracle");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// config struct stored in / read from cw_4626 storage
pub struct BaseConfig {
    pub base_token: AssetInfo,
    pub base_denom: AssetInfo,
    pub strategy_token: Addr,
    pub proxy: Addr,
    pub factory: Addr,
    pub oracle: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// config response
pub struct ConfigResponse<C> {
    pub base_token: AssetInfo,
    pub base_denom: AssetInfo,
    pub strategy_token: Addr,
    pub proxy: Addr,
    pub factory: Addr,
    pub oracle: Addr,
    pub config: C,
}
