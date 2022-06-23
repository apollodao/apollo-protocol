use apollo_asset::asset::AssetInfo;
use cosmwasm_std::Addr;
use cw_controllers::Claims;
use cw_storage_plus::Item;
use cw_utils::Duration;

pub const ORACLE: Item<Addr> = Item::new("oracle"); // TODO - raw query factory instead?
pub const ADAPTOR: Item<Addr> = Item::new("adaptor");
pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token"); // TODO - should this be here or implemented by the strategy?
pub const BASE_DENOM: Item<AssetInfo> = Item::new("base_denom"); // TODO - needed?

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const WITHDRAWALS_ALLOWED: Item<bool> = Item::new("withdrawals_allowed");
pub const DEPOSITS_ALLOWED: Item<bool> = Item::new("deposits_allowed");
pub const UNBONDING_PERIOD: Item<Duration> = Item::new("unbonding_period");

pub const CLAIMS: Claims = Claims::new("claims");
