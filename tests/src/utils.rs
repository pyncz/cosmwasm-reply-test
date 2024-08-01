#![cfg(test)]
use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};
use serde::Serialize;

fn instantiate(
    router: &mut App,
    code_id: u64,
    msg: impl Serialize,
    label: impl Into<String>,
) -> Addr {
    let admin = Addr::unchecked("notdrew");
    router
        .instantiate_contract(code_id, admin, &msg, &[], label, None)
        .unwrap()
}

pub fn instantiate_caller_with_reply(router: &mut App) -> Addr {
    let code = ContractWrapper::new(
        caller::contract::execute,
        caller::contract::instantiate,
        caller::contract::query,
    )
    .with_reply(caller::contract::reply);

    let code_id = router.store_code(Box::new(code));

    instantiate(
        router,
        code_id,
        caller::msg::InstantiateMsg {},
        "caller-with-reply",
    )
}

pub fn instantiate_caller_without_reply(router: &mut App) -> Addr {
    let code = ContractWrapper::new(
        caller::contract::execute,
        caller::contract::instantiate,
        caller::contract::query,
    );

    let code_id = router.store_code(Box::new(code));

    instantiate(
        router,
        code_id,
        caller::msg::InstantiateMsg {},
        "caller-without-reply",
    )
}

pub fn instantiate_peer_with_reply(router: &mut App) -> Addr {
    let code = ContractWrapper::new(
        peer::contract::execute,
        peer::contract::instantiate,
        peer::contract::query,
    )
    .with_reply(peer::contract::reply);

    let code_id = router.store_code(Box::new(code));

    instantiate(
        router,
        code_id,
        peer::msg::InstantiateMsg {},
        "peer-with-reply",
    )
}

pub fn instantiate_peer_without_reply(router: &mut App) -> Addr {
    let code = ContractWrapper::new(
        peer::contract::execute,
        peer::contract::instantiate,
        peer::contract::query,
    );

    let code_id = router.store_code(Box::new(code));

    instantiate(
        router,
        code_id,
        peer::msg::InstantiateMsg {},
        "peer-without-reply",
    )
}
