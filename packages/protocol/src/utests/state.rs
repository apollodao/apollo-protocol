// use crate::legacy_strategy::msg::{StrategyConfigResponse, StrategyInfo, StrategyUserInfoResponse};
// use anchor_token::staking::StakerInfoResponse;
// use cosmwasm_std::testing::MockQuerier;
// use cosmwasm_std::{Addr, Decimal, Empty, Uint128};
// use mirror_protocol::staking::RewardInfoResponse;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use terra_cosmwasm::TerraQueryWrapper;
// use terraswap::asset::Asset;

// #[derive(Serialize, Deserialize, Clone, Debug, Default)]
// pub struct TokenQuerier {
//     pub balances: HashMap<String, HashMap<String, Uint128>>,
// }

// pub struct WasmMockQuerier {
//     // terraswap client
//     pub base: MockQuerier<TerraQueryWrapper>,
//     pub token_querier: TokenQuerier,
//     // LP addr contract
//     pub pair_addr: Addr,
//     // LP normally asset-ust
//     pub pool_assets: [Asset; 2],
//     pub tax: (Decimal, Uint128),
//     // strategy information
//     pub strategy_info: StrategyInfo,
//     // user balance
//     pub user_info: StrategyUserInfoResponse,
//     // strategy config
//     pub strategy_config: StrategyConfigResponse<Empty>,
//     pub mirror_reward_info: RewardInfoResponse,
//     pub pylon_reward_info: StakerInfoResponse,
//     pub strategy_tvl: Uint128,
// }

// #[derive(Clone, Default)]
// pub struct TerraswapFactoryQuerier {
//     pairs: HashMap<String, String>,
// }
