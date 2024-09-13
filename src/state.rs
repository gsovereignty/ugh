use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct EthData {
    pub txid: String,      //EVM txid
    pub tx_origin: String, //tx.origin from evm tx
    pub tx_recipient: String,
    pub balance: Uint128,     //balance of OM in this account before sending
    pub send_amount: Uint128, //amount being sent
    pub block: Uint128,       //block that tx can be found in
}

pub const PROCESSED: Map<&String, EthData> = Map::new("processed");

#[cw_serde]
pub struct Config {
    pub owner: String,
}
pub const CONFIG: Item<Config> = Item::new("config");
