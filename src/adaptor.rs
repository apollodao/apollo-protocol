use apollo_asset::asset::{Asset, AssetInfo};
use cosmwasm_std::{Addr, Binary, CosmosMsg, Decimal, Decimal256, Empty, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum DexAdaptorExecuteMsg<C = ()> {
    ProvideLiquidity {
        assets: [Asset; 2],
        slippage_tolerance: Option<Decimal>,
        auto_stake: Option<bool>,
        recipient: Option<Addr>,
    },
    SendTokens {
        token: Addr,
        recipient: Addr,
        amount: Option<Uint128>,
        hook_msg: Option<Binary>,
    },
    Callback(C),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum StakingAdaptorExecuteMsg<A = ()> {
    // required proxy methods regardless of type
    Withdraw {
        token: Addr,
        amount: Uint128,
        recipient: String,
    },
    Bond {
        token: Addr,
        user_addr: Addr,
        amount: Option<Uint128>,
    },
    ClaimRewards {
        token: Addr,
        fee_rates: Vec<(Addr, Decimal)>,
    },
    SendFee {
        fee_amounts: Vec<(Addr, Addr, Uint128)>,
    },
    Adaptor(A),
}

pub type DexAdaptorQueryMsg = BaseDexAdaptorQueryMsg<()>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseDexAdaptorQueryMsg<A = ()> {
    /// query dex factory address
    Factory {},
    // TODO: Move SimulateSwap to router?
    SimulateSwap {
        from: AssetInfo,
        to: AssetInfo,
        amount: Uint128,
    },
    QueryBalance {
        asset: AssetInfo,
        account: String,
    },
    QueryPairInfo {
        asset_infos: Vec<AssetInfo>,
    },
    BuildSwapMsg {
        from: AssetInfo,
        to: AssetInfo,
        amount: Uint128,
        max_spread: Option<Decimal>,
        recipient: Option<String>,
    },
    Adaptor(A),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum DexAdaptorCw20HookMsg<A = ()> {
    WithdrawLiquidity { recipient: Option<Addr> },
    Adaptor(A),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum StakingAdaptorCw20HookMsg<A = ()> {
    Deposit {},
    Adaptor(A),
}

pub type StakingAdaptorQueryMsg = BaseStakingAdaptorQueryMsg<()>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseStakingAdaptorQueryMsg<A = ()> {
    Apr {
        oracle: Addr,
        price_age_limit: u64,
        token: Option<Addr>,
    },
    PendingRewards {
        lp_token: String,
    },
    TotalBondAmount {
        staker_addr: Option<Addr>,
        token: Option<Addr>,
    },
    Adaptor(A),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Config<P = ()> {
    pub dex: DexConfig,
    pub staking: StakingConfig,
    pub private: P,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct DexConfig {
    pub factory: Addr,
    pub router: Addr,
    pub generator: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct StakingConfig {
    pub staking_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct SimulateSwapResponse {
    pub return_amount: Uint128,
    pub spread_amount: Uint128,
    pub commission_amount: Uint128,
    pub execute_swap_msg: CosmosMsg,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct PendingRewardsResponse {
    pub pending_rewards: Uint128,
    pub claim_rewards_msg: CosmosMsg<Empty>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct AprResponse {
    pub apr: Decimal256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]

pub enum AdaptorExecuteMsg<D = (), S = ()> {
    Receive(Cw20ReceiveMsg),
    Dex(DexAdaptorExecuteMsg<D>),
    Staking(StakingAdaptorExecuteMsg<S>),
}

pub type AdaptorQueryMsg = BaseAdaptorQueryMsg<(), ()>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseAdaptorQueryMsg<D = (), S = ()> {
    Dex(BaseDexAdaptorQueryMsg<D>),
    Staking(BaseStakingAdaptorQueryMsg<S>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum AdaptorCw20HookMsg<S = (), D = ()> {
    Staking(StakingAdaptorCw20HookMsg<S>),
    Dex(DexAdaptorCw20HookMsg<D>),
}

/**
 *  --------- Astroport specific messages ----------
 */

pub type AstroportAdaptorExecuteMsg = AdaptorExecuteMsg<(), AstroportStakingAdaptorMsg>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum AstroportStakingAdaptorMsg {
    // required proxy methods regardless of type
    ExtendLockTime {},
    BoostLp {},
    AstroGovernance(Binary),
    ClaimRewards {
        recipient: Option<String>,
        fee_rate: (Addr, Decimal),
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum AstroportStakingCw20HookMsg {
    // required proxy methods regardless of type
    VoteLock {},
}

pub type AstroportAdaptorCw20HookMsg = AdaptorCw20HookMsg<AstroportStakingCw20HookMsg>;

pub type ExecuteMsg = BaseExecuteMsg<()>;
pub type LpExecuteMsg = BaseExecuteMsg<BaseLpExecuteMsg>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseExecuteMsg<P = ()> {
    // required proxy methods regardless of type
    Receive(Cw20ReceiveMsg),
    ZapIn {},
    ZapOut {
        recipient: Option<String>,
        to_asset: Option<AssetInfo>,
        amount: Uint128,
    },
    Withdraw {
        amount: Uint128,
        recipient: String,
    },
    Bond {},
    UnBond {},
    ClaimRewards {},
    SellRewards {
        max_spread: Option<Decimal>,
    },
    SendFee {
        fee_rates: Vec<(Addr, Decimal)>,
    },

    // other proxy methods that depend on type of proxy (eg. single asset vs lp)
    Proxy(P),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseLpExecuteMsg {
    Stake {},
    ProvideLiquidity {
        auto_stake: bool,
    },
    WithdrawLiquidity {
        amount: Uint128,
    },
    BuyAssetsForPair {},
    SellAssetsAndReturn {
        recipient: Option<String>,
        to_asset: AssetInfo,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum QueryMsg {
    Apr { oracle: Addr, price_age_limit: u64 },
    PendingRewards {},
    TotalBondAmount {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum Cw20HookMsg {
    Deposit {},
    ZapOut {
        recipient: Option<String>,
        to_asset: Option<AssetInfo>,
    },
}
