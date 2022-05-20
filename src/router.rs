use crate::graph::entities::Edge;
use apollo_asset::asset::{Asset, AssetInfo};
use cosmwasm_std::{Addr, Decimal, Uint128};
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

/// Dex information available for every Operation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexInfoMsg {
    /// Dex factory contract Addr
    pub factory_addr: String,
    /// Dex Router contract Addr
    pub router_addr: String,
}

/// Dexes query response item
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexesResponseItem {
    /// Dex adaptor id
    pub id: u8,
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

/// ## Description
/// This enum describes the swap operation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SwapOperation {
    /// Native swap
    NativeSwap {
        /// the offer denom
        offer_denom: String,
        /// the asks denom
        ask_denom: String,
    },
    /// Swap
    Swap {
        /// Dex id
        dex_adaptor_id: u8,
        /// the offer asset info
        offer_asset_info: AssetInfo,
        /// the asks asset info
        ask_asset_info: AssetInfo,
    },
}

impl From<Edge> for SwapOperation {
    fn from(edge: Edge) -> Self {
        if edge.pair_info.is_none() {
            Self::NativeSwap {
                offer_denom: edge.from.asset.to_string(),
                ask_denom: edge.to.asset.to_string(),
            }
        } else {
            Self::Swap {
                dex_adaptor_id: edge.pair_info.unwrap().dex_id,
                offer_asset_info: edge.from.into(),
                ask_asset_info: edge.to.into(),
            }
        }
    }
}

impl SwapOperation {
    /// Get the type asset info of the target swap operation
    pub fn get_target_asset_info(&self) -> AssetInfo {
        match self {
            SwapOperation::NativeSwap { ask_denom, .. } => AssetInfo::NativeToken {
                denom: ask_denom.clone(),
            },
            SwapOperation::Swap { ask_asset_info, .. } => ask_asset_info.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Receives a message of type [`Cw20ReceiveMsg`] and processes it depending on the received
    /// template.
    Receive(Cw20ReceiveMsg),
    AddPairs {
        pairs: Vec<Pair>,
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
        /// Operation to execute
        operation: SwapOperation,
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
        operations: Vec<SwapOperation>,
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
pub struct Pair {
    pub dex_adaptor: Option<Addr>,
    pub asset_infos: Vec<AssetInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairsResponse {
    pub pairs: Vec<PairsResponseItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairsResponseItem {
    pub asset: AssetInfo,
    pub pairs: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

/// ## Description
/// This structure describes the main controls configs of pair
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PairInfo {
    /// the type of asset infos available in [`AssetInfo`]
    pub asset_infos: [AssetInfo; 2],
    /// pair contract address
    pub contract_addr: Addr,
    /// pair liquidity token
    pub liquidity_token: Addr,
}
