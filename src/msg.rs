use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{self, EthData};
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddEntry { data: EthData },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(state::EthData)]
    Entry {txid: String},
}
