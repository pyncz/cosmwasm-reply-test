use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Ping { address: Addr, msg: Binary },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
