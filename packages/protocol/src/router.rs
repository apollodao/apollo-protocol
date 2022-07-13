use apollo_asset::pair::PairType;
use cosmwasm_std::{Addr, Binary, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use cw_asset::AssetInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Description
/// This structure describes the basic settings for creating a contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Apollo Factory contract address
    pub apollo_factory: String,
    /// Apollo Collector contract address
    pub apollo_collector: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Receives a message of type [`Cw20ReceiveMsg`] and processes it depending on the received
    /// template.
    Receive(Cw20ReceiveMsg),
    AddDexAdaptor {
        address: String,
    },
    AddPairs {
        pairs: Vec<PairInput>,
    },
    /// Swap where start asset is a native token
    SwapFromNative {
        to: AssetInfo,
        max_spread: Option<Decimal>,
        recipient: Option<String>,
        hook_msg: Option<Binary>,
        split: Option<bool>,
    },
    /// Internal use
    /// Swap all offer tokens to ask token
    SwapOperation {
        /// Swap Operation to execute
        swap_info: SwapInfo,
        /// Optional address to deposit target token
        recipient: Option<String>,
        /// Optional hook msg to send along with swap result to recipient
        hook_msg: Option<Binary>,
        max_spread: Option<Decimal>,
    },
    /// Internal use
    /// Check the swap amount is exceed minimum_receive
    AssertMinimumReceive {
        /// Asset info (Native or Token)
        target_asset: AssetInfo,
        /// Previous Balance before swap
        target_balance_before_swap: Uint128,
        /// Expected minimum to receive
        minimum_receive: Uint128,
        /// To Addr
        recipient: String,
    },
    SendTokens {
        token: AssetInfo,
        recipient: Addr,
        amount: Option<Uint128>,
        /// percentage of amount to send
        amount_pct: Option<Decimal>,
        hook_msg: Option<Binary>,
    },
    /// Query pair reserves and record price/twap
    CollectPrice {
        assets: [AssetInfo; 2],
        dex_id: u16,
    },
}

/// CW20 Hook
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Swap {
        to: AssetInfo,
        max_spread: Option<Decimal>,
        recipient: Option<String>,
        hook_msg: Option<Binary>,
        split: Option<bool>,
    },
}

/// ## Description
/// This structure describes the query messages of the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Config returns controls settings that specified in custom [`ConfigResponse`] structure
    Config {},
    /// Simulates multi-hop swap operations
    SimulateSwapOperations {
        /// the offer amount
        offer_amount: Uint128,
        /// operations for swap
        operations: Vec<SwapInfo>,
    },
    /// Get a list of all registered Dexes (store)
    Dexes {},
    /// Get a list of all pairs on router
    Pairs {
        limit: Option<u32>,
        start_after: Option<AssetInfo>,
    },
    /// Simulate an Operation (used by the aggregator)
    SimulateSwap {
        from: AssetInfo,
        to: AssetInfo,
        amount: Option<Uint128>,
    },
    /// Get price of a pair (use verbose flag to get more info)
    Price {
        assets: [AssetInfo; 2],
        /// Get more info on prices used to calculate the TWAP
        verbose: bool,
        /// Specify asset in pair to get the price of
        asset: Option<AssetInfo>,
    },
}

/// Dexes query response item
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexesResponseItem {
    /// Dex adaptor id
    pub id: usize,
    /// Dex adaptor address
    pub adaptor_addr: Addr,
}

/// Dexes query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexesResponse {
    pub dexes: Vec<DexesResponseItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInfoInput {
    pub dex_id: u16,
    pub pair_type: PairType,
    pub pair_info: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInput {
    pub info: PairInfoInput,
    pub asset_infos: Vec<AssetInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct SwapInfo {
    pub from: AssetInfo,
    pub to: AssetInfo,
    pub dex_id: u16,
}
