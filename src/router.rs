use apollo_asset::asset::AssetInfo;
use apollo_asset::pair::PairType;
use cosmwasm_std::{Addr, Binary, Decimal, Uint128};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Description
/// This structure describes the basic settings for creating a contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The apollo Factory contract address
    pub apollo_factory: String,
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
        split: Option<bool>,
    },
    /// Internal use
    /// Swap all offer tokens to ask token
    SwapOperation {
        /// Swap Operation to execute
        swap_info: SwapInfo,
        /// Optional address to deposit target token
        recipient: Option<String>,
        max_spread: Option<Decimal>,
        allo: Option<Decimal>,
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
        hook_msg: Option<Binary>,
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// Dexes query response item
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexesResponseItem {
    /// Dex adaptor id
    pub id: u16,
    /// Dex adaptor address
    pub adaptor_addr: Addr,
    /// Dex factory address
    pub factory_addr: Addr,
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
