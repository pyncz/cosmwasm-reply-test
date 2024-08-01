#![cfg(test)]
use crate::utils::{
    instantiate_caller_with_reply, instantiate_caller_without_reply, instantiate_peer_with_reply,
    instantiate_peer_without_reply,
};
use cosmwasm_std::{to_json_binary, Addr};
use cw_multi_test::App;
use cw_multi_test::Executor;

/// Caller with reply, peer with reply - should pass
#[test]
fn caller_with_peer_with() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");

    let caller_with_reply_addr = instantiate_caller_with_reply(&mut app);
    let peer_with_reply_addr = instantiate_peer_with_reply(&mut app);

    // Expect ok response
    let resp = app.execute_contract(
        sender.clone(),
        caller_with_reply_addr.clone(),
        &caller::msg::ExecuteMsg::Ping {
            address: peer_with_reply_addr.clone(),
            msg: to_json_binary(&peer::msg::ExecuteMsg::Pong {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_ok());

    // Expect err response
    let resp = app.execute_contract(
        sender,
        caller_with_reply_addr,
        &caller::msg::ExecuteMsg::Ping {
            address: peer_with_reply_addr,
            msg: to_json_binary(&peer::msg::ExecuteMsg::JokesOnYou {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_err());
}

/// Caller with reply, peer without reply - should pass
#[test]
fn caller_with_peer_without() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");

    let caller_with_reply_addr = instantiate_caller_with_reply(&mut app);
    let peer_without_reply_addr = instantiate_peer_without_reply(&mut app);

    // Expect ok response
    let resp = app.execute_contract(
        sender.clone(),
        caller_with_reply_addr.clone(),
        &caller::msg::ExecuteMsg::Ping {
            address: peer_without_reply_addr.clone(),
            msg: to_json_binary(&peer::msg::ExecuteMsg::Pong {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_ok());

    // Expect err response
    let resp = app.execute_contract(
        sender,
        caller_with_reply_addr,
        &caller::msg::ExecuteMsg::Ping {
            address: peer_without_reply_addr,
            msg: to_json_binary(&peer::msg::ExecuteMsg::JokesOnYou {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_err());
}

/// Caller without reply, peer with reply - should fail
#[test]
fn caller_without_peer_with() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");

    let caller_without_reply_addr = instantiate_caller_without_reply(&mut app);
    let peer_with_reply_addr = instantiate_peer_with_reply(&mut app);

    // Expect panic because caller's `reply` is not implemented
    let resp = app.execute_contract(
        sender.clone(),
        caller_without_reply_addr.clone(),
        &caller::msg::ExecuteMsg::Ping {
            address: peer_with_reply_addr.clone(),
            msg: to_json_binary(&peer::msg::ExecuteMsg::Pong {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_err());

    // Expect panic because caller's `reply` is not implemented
    let resp = app.execute_contract(
        sender,
        caller_without_reply_addr,
        &caller::msg::ExecuteMsg::Ping {
            address: peer_with_reply_addr,
            msg: to_json_binary(&peer::msg::ExecuteMsg::JokesOnYou {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_err());
}

/// Caller without reply, peer without reply - should fail
#[test]
fn caller_without_peer_without() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");

    let caller_without_reply_addr = instantiate_caller_without_reply(&mut app);
    let peer_without_reply_addr = instantiate_peer_without_reply(&mut app);

    // Expect panic because caller's `reply` is not implemented
    let resp = app.execute_contract(
        sender.clone(),
        caller_without_reply_addr.clone(),
        &caller::msg::ExecuteMsg::Ping {
            address: peer_without_reply_addr.clone(),
            msg: to_json_binary(&peer::msg::ExecuteMsg::Pong {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_err());

    // Expect panic because caller's `reply` is not implemented
    let resp = app.execute_contract(
        sender,
        caller_without_reply_addr,
        &caller::msg::ExecuteMsg::Ping {
            address: peer_without_reply_addr,
            msg: to_json_binary(&peer::msg::ExecuteMsg::JokesOnYou {}).unwrap(),
        },
        &[],
    );
    assert!(resp.is_err());
}
