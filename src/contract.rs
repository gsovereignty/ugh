#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:mantra-migrate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{self, Config, EthData, CONFIG, PROCESSED},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let _ = CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender.to_string(),
        },
    );
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    let cfg = CONFIG.load(deps.storage)?;
    if cfg.owner != info.sender.as_str() {
        return Err(StdError::generic_err("Called by non-admin"));
    }

    match msg {
        ExecuteMsg::AddEntry { data } => execute_add_entry(deps, data),
    }
}

fn execute_add_entry(deps: DepsMut, data: EthData) -> StdResult<Response> {
    if PROCESSED.has(deps.storage, &data.txid) {
        return Err(StdError::generic_err("Already Processed"));
    }
    PROCESSED.save(deps.storage, &data.txid, &data)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Entry { txid } => query_txid(deps, txid),
    }
}

pub fn query_txid(deps: Deps, txid: String) -> StdResult<Binary> {
    let result = PROCESSED.may_load(deps.storage, &txid);
    let result = match result {
        Ok(entry) => entry,
        Err(_) => return to_json_binary(&"No entry"),
    };
    match result {
        Some(entry) => return to_json_binary(&entry),
        None => return to_json_binary(&"No entry"),
    }
}
