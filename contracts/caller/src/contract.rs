#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg, SubMsgResult, WasmMsg,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Ping {
            address,
            msg: ping_msg,
        } => {
            let submsg = CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
                contract_addr: address.into(),
                msg: ping_msg,
                funds: vec![],
            });
            Ok(Response::new().add_submessage(SubMsg::reply_always(submsg, 69420)))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, StdError> {
    match msg.result {
        SubMsgResult::Ok(_) => Ok(Response::default()),
        SubMsgResult::Err(_) => Err(StdError::generic_err("RIP".to_string())),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
