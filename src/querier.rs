use cosmwasm_std::{
    to_binary, Addr, Binary, CanonicalAddr, Decimal, Empty, Order, QuerierWrapper, QueryRequest,
    Record, StdError, StdResult, Uint128, Uint256, WasmQuery,
};
use cosmwasm_storage::to_length_prefixed;

use crate::{
    factory::FactoryConfig,
    legacy_strategy::msg::{StrategyConfig, StrategyInfo, UserInfo},
};

///Contains functions to query the store of Apollo contracts. This is to optimize gas usage when querying between contracts of Apollo Protocol.

pub fn query_strategy_user_info(
    querier: &QuerierWrapper,
    strategy: &Addr,
    user: &CanonicalAddr,
) -> UserInfo {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Raw {
            contract_addr: strategy.to_string(),
            key: Binary::from(concat(&to_length_prefixed(b"user"), user.as_slice())),
        }))
        .unwrap_or_else(|_x| {
            let strategy_info = query_strategy_strategy_info(querier, strategy);
            UserInfo {
                index: strategy_info.global_index,
                shares: Uint128::zero(),
            }
        })
}

pub fn query_strategy_strategy_info(querier: &QuerierWrapper, strategy: &Addr) -> StrategyInfo {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Raw {
            contract_addr: strategy.to_string(),
            key: Binary::from(to_length_prefixed(b"strategy")),
        }))
        .unwrap_or({
            StrategyInfo {
                global_index: Decimal::one(),
                total_bond_amount: Uint128::zero(),
                total_shares: Uint128::zero(),
            }
        })
}

use crate::factory::{ApolloContracts, APOLLO_CONTRACTS, APOLLO_DEX_ADAPTORS, APOLLO_DEX_COUNT};
use crate::legacy_strategy::msg::{BaseStrategyConfig, StrategyQueryMsg};
use crate::strategy::state::ConfigResponse;
use cw20_base::state::TOKEN_INFO;

pub fn query_cw20_token_info(
    querier: &QuerierWrapper,
    token_addr: Addr,
) -> StdResult<cw20_base::state::TokenInfo> {
    TOKEN_INFO.query(querier, token_addr)
}

use super::strategy::state::STRATEGY_TOKEN;
pub fn query_strategy_strategy_token(querier: &QuerierWrapper, strategy: Addr) -> StdResult<Addr> {
    STRATEGY_TOKEN.query(querier, strategy)
}

pub fn query_strategy_config(
    querier: &QuerierWrapper,
    strategy: &Addr,
) -> StdResult<StrategyConfig<Empty>> {
    match querier.query(&QueryRequest::Wasm(WasmQuery::Raw {
        contract_addr: strategy.to_string(),
        key: Binary::from(to_length_prefixed(b"config")),
    })) {
        Ok(cfg) => Ok(cfg),
        Err(_) => {
            // TODO - change single asset to be more backward compat (move canons to root)
            let res = querier
                .query::<ConfigResponse<BaseStrategyConfig>>(&QueryRequest::Wasm(
                    WasmQuery::Smart {
                        contract_addr: strategy.to_string(),
                        msg: to_binary(&StrategyQueryMsg::Config {})?,
                    },
                ))?
                .config;
            Ok(StrategyConfig {
                apollo_factory: res.apollo_factory,
                apollo_collector: res.apollo_collector,
                base_token: res.base_token,
                performance_fee: res.performance_fee,
                strategy_config: Empty {},
            })
        }
    }
}

pub fn query_factory_config(querier: &QuerierWrapper, factory: &Addr) -> StdResult<FactoryConfig> {
    querier.query(&QueryRequest::Wasm(WasmQuery::Raw {
        contract_addr: factory.to_string(),
        key: Binary::from(to_length_prefixed(b"config")),
    }))
}

pub fn query_factory_total_reward_weight(querier: &QuerierWrapper, factory: &Addr) -> Uint256 {
    querier
        .query(&QueryRequest::Wasm(WasmQuery::Raw {
            contract_addr: factory.to_string(),
            key: Binary::from(b"total_reward_weight"),
        }))
        .unwrap_or_else(|_x| Uint256::zero())
}

pub fn query_apollo_contracts(
    querier: &QuerierWrapper,
    apollo_factory: &Addr,
) -> StdResult<ApolloContracts> {
    APOLLO_CONTRACTS.query(querier, apollo_factory.clone())
}

pub fn query_apollo_dex_adaptor_by_id(
    querier: &QuerierWrapper,
    apollo_factory: &Addr,
    dex_adaptor_id: u8,
) -> Option<Addr> {
    APOLLO_DEX_ADAPTORS
        .query(querier, apollo_factory.clone(), dex_adaptor_id.into())
        .ok()?
}

// TODO - might be cleaner to just pass the dex id instead of the address, and create a query on factory for all available dexes
pub fn query_apollo_dex_adaptor_by_addr(
    querier: &QuerierWrapper,
    apollo_factory_addr: &Addr,
    dex_adaptor_addr: &Addr,
) -> StdResult<u8> {
    let dex_count = APOLLO_DEX_COUNT.query(querier, apollo_factory_addr.clone())?;
    for i in 1..=dex_count {
        if let Some(found) =
            APOLLO_DEX_ADAPTORS.query(querier, apollo_factory_addr.clone(), i.into())?
        {
            if &found == dex_adaptor_addr {
                return Ok(i);
            }
        }
    }
    Err(StdError::generic_err(format!(
        "dex adaptor not whitelisted with factory - {:?}",
        dex_adaptor_addr
    )))
}

#[inline]
fn concat(namespace: &[u8], key: &[u8]) -> Vec<u8> {
    let mut k = namespace.to_vec();
    k.extend_from_slice(key);
    k
}
