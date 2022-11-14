use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ConvexStrategyMsg {
    BalancePeg {},
}

#[cw_serde]
pub enum ConvexQuery {}
