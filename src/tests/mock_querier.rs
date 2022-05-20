use crate::tests::state::{TokenQuerier, WasmMockQuerier};
use anchor_token::staking::StakerInfoResponse;
use cosmwasm_std::{ContractResult, Empty, SystemResult};
use cw20::Cw20QueryMsg;
use std::collections::HashMap;

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Api, CanonicalAddr, Coin, Decimal, OwnedDeps,
    Querier, QuerierResult, QueryRequest, SystemError, Uint128, WasmQuery,
};
use cosmwasm_storage::to_length_prefixed;
use cw20::TokenInfoResponse;
use terra_cosmwasm::{TaxCapResponse, TaxRateResponse, TerraQuery, TerraQueryWrapper, TerraRoute};
use terraswap::{
    asset::Asset, asset::AssetInfo, asset::PairInfo, factory::QueryMsg as FactoryQueryMsg,
    pair::PoolResponse, pair::QueryMsg as PairQueryMsg,
};

use crate::legacy_strategy::msg::{
    StrategyConfigResponse, StrategyInfo, StrategyQueryMsg, StrategyUserInfoResponse, TvlResponse,
};
use anchor_token::staking::QueryMsg as PylonStakingQueryMsg;
use mirror_protocol::staking::{QueryMsg as MirrorStakingQueryMsg, RewardInfoResponse};

pub fn mock_dependencies_with_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    // Length of canonical addresses created with MockApi. Contracts should not make any assumtions
    // what this value is.
    // TODO evaluate if we need canonical_length on MockApi
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(contract_addr, contract_balance)]));

    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
    }
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {:?}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(TerraQueryWrapper { route, query_data }) => match route {
                TerraRoute::Treasury {} => match query_data {
                    TerraQuery::TaxRate {} => {
                        let res = TaxRateResponse { rate: self.tax.0 };
                        SystemResult::Ok(ContractResult::from(to_binary(&res)))
                    }
                    TerraQuery::TaxCap { .. } => {
                        let res = TaxCapResponse { cap: self.tax.1 };
                        SystemResult::Ok(ContractResult::from(to_binary(&res)))
                    }
                    _ => {
                        return SystemResult::Err(SystemError::UnsupportedRequest {
                            kind: "query_data type Not Found".to_string(),
                        })
                    }
                },
                _ => {
                    return SystemResult::Err(SystemError::UnsupportedRequest {
                        kind: "route type Not Found".to_string(),
                    })
                }
            },
            QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: _,
                msg,
            }) => match from_binary(msg).unwrap() {
                SystemResult::Ok(FactoryQueryMsg::Pair { asset_infos }) => {
                    let response = PairInfo {
                        asset_infos: asset_infos.clone(),
                        contract_addr: self.pair_addr.to_string(),
                        liquidity_token: "lptoken".to_string(),
                    };

                    SystemResult::Ok(ContractResult::from(to_binary(&response)))
                }
                _ => match from_binary(&msg).unwrap() {
                    SystemResult::Ok(PairQueryMsg::Pool {}) => {
                        let response = PoolResponse {
                            assets: self.pool_assets.clone(),
                            total_share: Uint128::zero(),
                        };

                        SystemResult::Ok(ContractResult::from(to_binary(&response)))
                    }
                    _ => match from_binary(&msg).unwrap() {
                        SystemResult::Ok(StrategyQueryMsg::StrategyInfo {}) => {
                            let response = StrategyInfo {
                                total_bond_amount: self.strategy_info.total_bond_amount,
                                global_index: self.strategy_info.global_index,
                                total_shares: self.strategy_info.total_shares,
                            };

                            SystemResult::Ok(ContractResult::from(to_binary(&response)))
                        }
                        _ => match from_binary(&msg).unwrap() {
                            SystemResult::Ok(StrategyQueryMsg::UserInfo { address: _ }) => {
                                let response = StrategyUserInfoResponse {
                                    base_token_balance: self.user_info.base_token_balance,
                                    shares: self.user_info.shares,
                                };

                                SystemResult::Ok(ContractResult::from(to_binary(&response)))
                            }
                            _ => match from_binary(&msg).unwrap() {
                                SystemResult::Ok(StrategyQueryMsg::Config {}) => SystemResult::Ok(
                                    ContractResult::from(to_binary(&self.strategy_config)),
                                ),
                                _ => match from_binary(&msg).unwrap() {
                                    SystemResult::Ok(StrategyQueryMsg::Tvl {}) => {
                                        let response = TvlResponse {
                                            tvl: self.strategy_tvl,
                                        };

                                        SystemResult::Ok(ContractResult::from(to_binary(&response)))
                                    }
                                    _ => match from_binary(&msg).unwrap() {
                                        SystemResult::Ok(MirrorStakingQueryMsg::RewardInfo {
                                            asset_token: _,
                                            staker_addr: _,
                                        }) => SystemResult::Ok(ContractResult::from(to_binary(
                                            &self.mirror_reward_info,
                                        ))),
                                        _ => match from_binary(&msg).unwrap() {
                                            SystemResult::Ok(Cw20QueryMsg::Balance {
                                                address: _,
                                            }) => SystemResult::Ok(ContractResult::from(
                                                to_binary(&self.token_querier.balances),
                                            )),
                                            _ => match from_binary(&msg).unwrap() {
                                                SystemResult::Ok(
                                                    PylonStakingQueryMsg::StakerInfo {
                                                        staker: _,
                                                        block_height: None,
                                                    },
                                                ) => SystemResult::Ok(ContractResult::from(
                                                    to_binary(&self.pylon_reward_info),
                                                )),
                                                _ => panic!("DO NOT ENTER HERE"),
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
            QueryRequest::Wasm(WasmQuery::Raw { contract_addr, key }) => {
                let key: &[u8] = key.as_slice();

                let prefix_token_info = to_length_prefixed(b"token_info").to_vec();
                let prefix_balance = to_length_prefixed(b"balance").to_vec();

                let balances: &HashMap<String, Uint128> =
                    match self.token_querier.balances.get(contract_addr) {
                        Some(balances) => balances,
                        None => {
                            return SystemResult::Err(SystemError::InvalidRequest {
                                error: format!(
                                    "No balance info exists for the contract {}",
                                    contract_addr
                                ),
                                request: key.into(),
                            })
                        }
                    };

                if key.to_vec() == prefix_token_info {
                    let mut total_supply = Uint128::zero();

                    for balance in balances {
                        total_supply += *balance.1;
                    }

                    SystemResult::Ok(ContractResult::from(to_binary(&TokenInfoResponse {
                        name: "mAPPL".to_string(),
                        symbol: "mAPPL".to_string(),
                        decimals: 6,
                        total_supply: total_supply,
                    })))
                } else if key[..prefix_balance.len()].to_vec() == prefix_balance {
                    let key_address: &[u8] = &key[prefix_balance.len()..];
                    let address_raw: CanonicalAddr = CanonicalAddr::from(key_address);
                    //let api: MockApi = MockApi::new(self.canonical_length);
                    let api: MockApi = MockApi::default();
                    let address: Addr = match api.addr_humanize(&address_raw) {
                        Ok(v) => v,
                        Err(e) => {
                            return SystemResult::Err(SystemError::InvalidRequest {
                                error: format!("Parsing query request: {:?}", e),
                                request: key.into(),
                            })
                        }
                    };
                    let balance = match balances.get(&address.into_string()) {
                        Some(v) => v,
                        None => {
                            return SystemResult::Err(SystemError::InvalidRequest {
                                error: "Balance not found".to_string(),
                                request: key.into(),
                            })
                        }
                    };
                    SystemResult::Ok(ContractResult::from(to_binary(&balance)))
                } else {
                    panic!("DO NOT ENTER HERE")
                }
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<TerraQueryWrapper>) -> Self {
        WasmMockQuerier {
            base,
            token_querier: TokenQuerier::default(),
            // canonical_length, remove since now its not possible to set it
            pair_addr: Addr::unchecked(""),
            pool_assets: [
                Asset {
                    info: AssetInfo::NativeToken {
                        denom: "uusd".to_string(),
                    },
                    amount: Uint128::zero(),
                },
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: "asset".to_string(),
                    },
                    amount: Uint128::zero(),
                },
            ],
            // TODO Please check this
            tax: (Decimal::percent(1), Uint128::new(1000000)),
            strategy_info: StrategyInfo {
                total_bond_amount: Uint128::zero(),
                global_index: Decimal::one(),
                total_shares: Uint128::zero(),
            },
            user_info: StrategyUserInfoResponse {
                base_token_balance: Uint128::zero(),
                shares: Uint128::zero(),
            },
            strategy_config: StrategyConfigResponse {
                apollo_factory: Addr::unchecked("factory"),
                apollo_collector: Addr::unchecked("collector"),
                base_token: Addr::unchecked("base_token"),
                performance_fee: Decimal::percent(20),
                terraswap_router: Addr::unchecked("terraswap_router"),
                use_strategy_token: false,
                strategy_config: Empty {},
            },
            mirror_reward_info: RewardInfoResponse {
                reward_infos: vec![],
                staker_addr: "staker_addr".to_string(),
            },
            strategy_tvl: Uint128::zero(),
            pylon_reward_info: StakerInfoResponse {
                staker: "staker".to_string(),
                bond_amount: Uint128::zero(),
                pending_reward: Uint128::zero(),
                reward_index: Decimal::zero(),
            },
        }
    }

    /*

    pub fn with_pair_info(&mut self, pair_addr: Addr) {
        self.pair_addr = pair_addr;
    }

    pub fn with_pool_assets(&mut self, pool_assets: [Asset; 2]) {
        self.pool_assets = pool_assets;
    }

    pub fn with_token_balances(&mut self, balances: &[(&String, &[(&String, &Uint128)])]) {
        self.token_querier = TokenQuerier::new(balances);
    }

    pub fn with_strategy_info(&mut self, strategy_info: StrategyInfo) {
        self.strategy_info = strategy_info;
    }

    pub fn with_strategy_user_info(&mut self, user_info: StrategyUserInfoResponse) {
        self.user_info = user_info;
    }

    pub fn with_strategy_config(&mut self, strategy_config: StrategyConfig<Empty>) {
        self.strategy_config = strategy_config;
    }

    pub fn with_mirror_reward_info(&mut self, reward_info: RewardInfoResponse) {
        self.mirror_reward_info = reward_info;
    }

    pub fn with_pylon_reward_info(&mut self, reward_info: StakerInfoResponse) {
        self.pylon_reward_info = reward_info;
    }

    pub fn with_strategy_tvl(&mut self, tvl: Uint128) {
        self.strategy_tvl = tvl;
    }

    */
}
