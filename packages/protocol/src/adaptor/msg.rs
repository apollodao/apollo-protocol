use cosmwasm_std::{Addr, Binary, CosmosMsg, Decimal, Decimal256, Empty, Uint128};
use cw20::Cw20ReceiveMsg;
use cw_asset::PairType;
use cw_asset::{Asset, AssetInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type AdaptorExecuteMsg =
    BaseAdaptorExecuteMsg<BaseDexAdaptorExecuteMsg<()>, BaseStakingAdaptorExecuteMsg<()>>;
pub type AdaptorQueryMsg =
    BaseAdaptorQueryMsg<BaseDexAdaptorQueryMsg<()>, BaseStakingAdaptorQueryMsg<()>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseAdaptorExecuteMsg<
    D = BaseDexAdaptorExecuteMsg<()>,
    S = BaseStakingAdaptorExecuteMsg<()>,
> {
    Receive(Cw20ReceiveMsg),
    Dex(D),
    Staking(S),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseAdaptorQueryMsg<D = BaseDexAdaptorQueryMsg<()>, S = BaseStakingAdaptorQueryMsg<()>> {
    Dex(D),
    Staking(S),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseAdaptorCw20HookMsg<S = (), D = ()> {
    Staking(BaseStakingAdaptorCw20HookMsg<S>),
    Dex(BaseDexAdaptorCw20HookMsg<D>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseDexAdaptorExecuteMsg<C = ()> {
    ProvideLiquidity {
        assets: Vec<Asset>,
        pair_info: Binary,
        slippage_tolerance: Option<Decimal>,
        auto_stake: Option<bool>,
        recipient: Option<Addr>,
    },
    SendTokens {
        token: AssetInfo,
        recipient: Addr,
        amount: Option<Uint128>,
        amount_pct: Option<Decimal>,
        hook_msg: Option<Binary>,
    },
    AddPair {
        asset_infos: [AssetInfo; 2],
        pair_info: Binary,
    },
    Callback(C),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseStakingAdaptorExecuteMsg<A = ()> {
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
    Reserves {
        from: AssetInfo,
        to: AssetInfo,
    },
    Balance {
        asset: AssetInfo,
        account: String,
    },
    PairInfo {
        asset_infos: [AssetInfo; 2],
    },
    BuildSwapMsg {
        from: AssetInfo,
        to: AssetInfo,
        amount: Uint128,
        sender: String,
        max_spread: Option<Decimal>,
    },
    Adaptor(A),
}

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
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseDexAdaptorCw20HookMsg<A = ()> {
    WithdrawLiquidity { recipient: Option<Addr> },
    Adaptor(A),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum BaseStakingAdaptorCw20HookMsg<A = ()> {
    Deposit {},
    Adaptor(A),
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
#[schemars(deny_unknown_fields)]
pub struct PairInfo<I = ()> {
    pub pair_type: PairType,
    pub pair_info: I,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct PoolReserves {
    pub pair_type: PairType,
    pub from: Asset,
    pub to: Asset,
    pub amp: u64,
}

impl PoolReserves {
    pub fn empty() -> Self {
        Self {
            pair_type: PairType::None {},
            from: Asset::empty(),
            to: Asset::empty(),
            amp: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pair_type.is_none() && self.from.is_empty() && self.to.is_empty() && self.amp == 0
    }
}

// TODO - might be able to remove below, but keeping for now in case we can reuse
/**
 *  --------- Astroport specific messages ----------
 */

pub type AstroportAdaptorExecuteMsg = BaseAdaptorExecuteMsg<(), AstroportStakingAdaptorMsg>;

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

pub type AstroportAdaptorCw20HookMsg = BaseAdaptorCw20HookMsg<AstroportStakingCw20HookMsg>;

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
