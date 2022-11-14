use crate::legacy_strategy::msg::StrategyConfigOptions;
use crate::legacy_vault::msg::VaultExecuteMsg;
use apollo_asset::asset::AssetInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CanonicalAddr, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct AutoCompoundInstantiateMsg {
    pub apollo_factory: String,        //The Apollo Factory contract
    pub staking_contract: String,      //The Mirror staking contract
    pub lp_token: String,              //The mAsset LP token for this strategy
    pub reward_token: AssetInfo,       //MIR token
    pub asset_token: AssetInfo,        //The mAsset token
    pub base_token: Option<AssetInfo>, //Cw20 address if base token is not same as base_denom else None
    pub base_denom: String,            //Native currency to trade against. Usually "uusd".
    pub max_spread: Decimal, //Max spread in percent (enter 0.01 for 1%) when trading on Terraswap
    pub reward_token_pair: String, //The Terraswap reward_token-base_denom pair contract
    pub asset_token_pair: String, //The Terraswap asset_token-base_denom pair contract
    pub apollo_collector: String,
    pub performance_fee: Decimal,
    pub swap_commission: Decimal,
    pub oracle_contract: String,
    pub farm_factory_contract: String,
    pub terraswap_router: String,
}

#[cw_serde]
pub enum AutoCompoundExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Execute {},
    Farm {},
    Stake {},
    Withdraw {
        user_addr: String,
        amount: Uint128,
    },
    EmergencyWithdraw {
        receiver: String,
    },
    UpdateConfig {
        update: StrategyConfigOptions<AutoCompoundConfigOptions>,
    },
    ZapIn {
        depositor_addr: String,
    },
    AddSharesAndStake {
        depositor_addr: String,
    },
    ZapOut {
        user_addr: String,
        amount: Uint128,
    },
    Vault(VaultExecuteMsg),
}

#[cw_serde]
pub struct AutoCompoundMigrateMsg {
    pub oracle_contract: String,
    pub farm_factory_contract: String,
    pub terraswap_router: String,
}

#[cw_serde]
// config struct stored in / read from cw_4626 storage
pub struct AutoCompoundConfig {
    pub asset_token: AssetInfo, // X token in X-Y pair
    pub base_token: AssetInfo,  // Y token in X-Y pair
    pub asset_token_pair: CanonicalAddr,
    pub base_denom: String, //Native currency to trade against. Usually "uusd".
    pub reward_token: AssetInfo, //MIR token
    pub reward_token_pair: CanonicalAddr,
    pub staking_contract: CanonicalAddr, //The underlying farm staking contract
    pub max_spread: Decimal, //Max spread in percent (enter 0.01 for 1%) when trading on Terraswap
    pub swap_commission: Decimal, //The Terraswap commission (0.3% by default)
    pub slippage_tolerance: Decimal, //Max slippage tolerance in percent when adding liquidity to Terraswap pools
    pub oracle_contract: CanonicalAddr, //Apollo oracle contract
    pub price_age_limit: u64,        //Age limit on oracle price before we throw an error
    pub farm_factory_contract: CanonicalAddr,
}

#[cw_serde]
// used in response from config query
pub struct AutoCompoundConfigResponse {
    pub asset_token: AssetInfo,
    pub asset_token_pair: Addr,
    pub base_denom: String, // base denomination of assets (eg. "uusd")
    pub base_token: AssetInfo,
    pub reward_token: AssetInfo, // reward token produced from staking in vault (eg. 'MIR')
    pub reward_token_pair: Addr, // reward token pair (eg. 'MIR-UST')
    pub staking_contract: Addr,  // staking contract for vault
    pub max_spread: Decimal,     // max spread for vault in percent decimal (eg. '0.1' for 10%)
    pub swap_commission: Decimal,
    pub slippage_tolerance: Decimal,
    pub oracle_contract: Addr,
    pub price_age_limit: u64,
    pub farm_factory_contract: Addr,
}

#[cw_serde]
// used as input to update config
pub struct AutoCompoundConfigOptions {
    pub asset_token: Option<String>,
    pub asset_token_pair: Option<String>,
    pub max_spread: Option<Decimal>, // max spread for vault in percent decimal (eg. '0.1' for 10%)
    pub swap_commission: Option<Decimal>,
    pub slippage_tolerance: Option<Decimal>,
    pub price_age_limit: Option<u64>,
}
