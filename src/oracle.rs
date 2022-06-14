use std::str::FromStr;

use apollo_asset::asset::AssetInfo;
use cosmwasm_std::{Addr, Api, Decimal256, QuerierWrapper, StdError, StdResult, Uint256};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::utils::query_supply;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
    },
    RegisterFeeder {
        asset: String,
        feeder: String,
    },
    RemoveFeeder {
        asset: String,
        feeder: String,
    },
    FeedPrice {
        prices: Vec<(String, Decimal256)>, // (asset, price)
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum QueryMsg {
    Config {},
    Feeders {
        asset: String,
    },
    Price {
        base: String,
        quote: String,
    },
    LpPrice {
        base: String,
        asset0: AssetInfo,
        asset1: AssetInfo,
        pair: String,
        lp_token: String,
    },
    Prices {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct MigrateMsg {}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub base_asset: String,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FeedersResponse {
    pub asset: String,
    pub feeders: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub rate: Decimal256,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PricesResponseElem {
    pub asset: String,
    pub price: Decimal256,
    pub last_updated_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PricesResponse {
    pub prices: Vec<PricesResponseElem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceInfo {
    pub value: Decimal256,
    pub last_updated_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub base_asset: String,
}

pub const PRICES: Map<&str, PriceInfo> = Map::new("prices");
pub const CONFIG: Item<Config> = Item::new("config");

// Remote Raw contract call
pub fn query_config(querier: &QuerierWrapper, oracle: &Addr) -> StdResult<Config> {
    CONFIG.query(querier, oracle.clone())
}

pub fn query_price(
    querier: &QuerierWrapper,
    oracle: &Addr,
    quote: &String,
) -> StdResult<PriceInfo> {
    PRICES
        .query(querier, oracle.clone(), quote.as_str())?
        .ok_or_else(|| StdError::generic_err("No price data for the specified asset exist"))
}

pub fn query_oracle_price(
    querier: &QuerierWrapper,
    oracle: Addr,
    base: String,
    quote: String,
    oldest_acceptable_price: Option<u64>,
) -> StdResult<Decimal256> {
    let config = query_config(querier, &oracle)?;

    let quote_price = if config.base_asset == quote {
        PriceInfo {
            value: Decimal256::one(),
            last_updated_time: 9999999999,
        }
    } else {
        query_price(querier, &oracle, &quote)?
    };

    let base_price = if config.base_asset == base {
        PriceInfo {
            value: Decimal256::one(),
            last_updated_time: 9999999999,
        }
    } else {
        query_price(querier, &oracle, &base)?
    };

    let rate = quote_price.value;

    if let Some(age_limit) = oldest_acceptable_price {
        if quote_price.last_updated_time < age_limit {
            Err(StdError::generic_err(format!(
                "Oracle quote price too old - {}",
                quote
            )))
        } else if base_price.last_updated_time < age_limit {
            Err(StdError::generic_err(format!(
                "Oracle base price too old - {}",
                base
            )))
        } else {
            Ok(rate)
        }
    } else {
        Ok(rate)
    }
}

pub fn calculate_lp_price(
    querier: &QuerierWrapper,
    _api: &dyn Api,
    asset0: AssetInfo,
    asset1: AssetInfo,
    asset0_price: Decimal256,
    asset1_price: Decimal256,
    pair: Addr,
    lp_token: Addr,
) -> StdResult<Decimal256> {
    //Calculate the LP token price
    //Use Alpha Finance "Fair LP token pricing": https://blog.alphafinance.io/fair-lp-token-pricing/
    let pool_asset0_balance = asset0.query_balance(querier, pair.clone())?;
    let pool_asset1_balance = asset1.query_balance(querier, pair.clone())?;

    //Convert from cosmwasm_std Uint256 to cosmwasm_bignumber Uint256... :(
    //TODO: Make PR for cosmwasm_std to have ops::Mul, ops::Div, and rounding so we can just use that instead
    let constant_product = Uint256::from_str(
        &pool_asset0_balance
            .full_mul(pool_asset1_balance)
            .to_string(),
    )?;
    let lp_token_supply = query_supply(querier, lp_token)?;
    let lp_token_price = Decimal256::from_ratio(2u8, 1u8)
        * (asset0_price * asset1_price * Decimal256::from_atomics(constant_product, 18).unwrap())
            .sqrt()
        / Decimal256::from_atomics(lp_token_supply, 18).unwrap();

    Ok(lp_token_price)
}

pub fn query_oracle_lp_price(
    querier: &QuerierWrapper,
    api: &dyn Api,
    oracle: Addr,
    base: String,
    asset0: AssetInfo,
    asset1: AssetInfo,
    pair: Addr,
    lp_token: Addr,
    oldest_acceptable_price: Option<u64>,
) -> StdResult<Decimal256> {
    //Fetch the quote token price

    let asset0_price = query_oracle_price(
        querier,
        oracle.clone(),
        base.clone(),
        asset0.to_string(),
        oldest_acceptable_price,
    )?;
    let asset1_price = query_oracle_price(
        querier,
        oracle,
        base,
        asset1.to_string(),
        oldest_acceptable_price,
    )?;

    calculate_lp_price(
        querier,
        api,
        asset0,
        asset1,
        asset0_price,
        asset1_price,
        pair,
        lp_token,
    )
}
