use apollo_asset::asset::AssetInfo;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_controllers::Claims;
use cw_storage_plus::Item;
use cw_utils::Duration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const ORACLE: Item<Addr> = Item::new("oracle"); // TODO - raw query factory instead?
pub const ADAPTOR: Item<Addr> = Item::new("adaptor");
pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token"); // TODO - should this be here or implemented by the strategy?
pub const BASE_DENOM: Item<AssetInfo> = Item::new("base_denom"); // TODO - needed?
pub const VAULT_TOKEN: Item<AssetInfo> = Item::new("vault_token");

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const WITHDRAWALS_ALLOWED: Item<bool> = Item::new("withdrawals_allowed");
pub const DEPOSITS_ALLOWED: Item<bool> = Item::new("deposits_allowed");
pub const UNBONDING_PERIOD: Item<Duration> = Item::new("unbonding_period");

pub const CLAIMS: Claims = Claims::new("claims");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// config struct stored in / read from cw_4626 storage
pub struct BaseVaultConfig {
    pub base_token: AssetInfo,
    pub base_denom: AssetInfo,
    pub adaptor: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct VaultInfo {
    pub total_bond_amount: Uint128,
    pub total_shares: Uint128,
    pub global_index: Decimal, // TODO - remove this after migration, legacy
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
// user info struct returned by queries
pub struct UserInfo {
    pub base_token_balance: Uint128,
    pub shares: Uint128,
    pub index: Decimal, // TODO - remove this after migration, legacy
}
