use apollo_asset::asset::AssetInfo;
use cosmwasm_std::{Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum VaultExecuteMsg {
    ProvideLiquidity {
        base_token_balance_before: Option<Uint128>,
        asset_token_balance_before: Option<Uint128>,
        base_token: AssetInfo,
        asset_token: AssetInfo,
        asset_token_pair: String,
        slippage_tolerance: Decimal,
    },
    WithdrawLiquidity {
        base_token: String,
        asset_token_pair: String,
        amount: Uint128,
    },
    SellAsset {
        user_addr: String,
        asset_token: AssetInfo,
        asset_token_pair: String,
        asset_token_balance_before: Uint128,
        max_spread: Decimal,
    },
    ReturnBaseDenom {
        user_addr: String,
        base_denom: String,
        base_denom_balance_before: Uint128,
    },
    SellRewards {
        asset_token: AssetInfo,
        reward_token: AssetInfo,
        reward_token_pair: String,
        max_spread: Decimal,
    },
    BuyAsset {
        zapped_in: Option<Uint128>,
        base_denom: String,
        asset_token_pair: String,
        max_spread: Decimal,
        swap_commission: Decimal,
    },
    SendRewards {
        base_denom: String,
        asset_token: AssetInfo,
        reward_token: AssetInfo,
    },
    ReturnLp {
        user_addr: String,
        base_token: String,
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardInfoResponse<C> {
    pub staker_addr: String,
    pub reward_infos: Vec<RewardInfoResponseItem<C>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardInfoResponseItem<C> {
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
    pub other: Option<C>,
}
