use cosmwasm_std::{Addr, Empty, StdError, to_binary, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor, next_block};
use cw20::Cw20Coin;
use crate::ContractError;
use crate::msg::InstantiateMsg;

use cosmwasm_std::StdError::GenericErr;

const CREATOR_ADDR: &str = "creator";

fn distributor_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

fn cw20_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

fn staked_balances_voting_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        dao_voting_cw20_staked::contract::execute,
        dao_voting_cw20_staked::contract::instantiate,
        dao_voting_cw20_staked::contract::query,
    )
        .with_reply(dao_voting_cw20_staked::contract::reply);
    Box::new(contract)
}

fn cw20_staking_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_stake::contract::execute,
        cw20_stake::contract::instantiate,
        cw20_stake::contract::query,
    );
    Box::new(contract)
}

struct BaseTest {
    app: App,
    distributor_address: Addr,
    staking_address: Addr,
    token_address: Addr,
}

fn setup_test(initial_balances: Vec<Cw20Coin>) -> BaseTest {
    let mut app = App::default();
    let distributor_id = app.store_code(distributor_contract());
    let cw20_id = app.store_code(cw20_contract());
    let voting_id = app.store_code(staked_balances_voting_contract());
    let stake_cw20_id = app.store_code(cw20_staking_contract());

    let voting_address = app
        .instantiate_contract(
            voting_id,
            Addr::unchecked(CREATOR_ADDR),
            &dao_voting_cw20_staked::msg::InstantiateMsg {
                active_threshold: None,
                token_info: dao_voting_cw20_staked::msg::TokenInfo::New {
                    code_id: cw20_id,
                    label: "DAO DAO governance token.".to_string(),
                    name: "DAO DAO".to_string(),
                    symbol: "DAO".to_string(),
                    decimals: 6,
                    initial_balances: initial_balances.clone(),
                    marketing: None,
                    staking_code_id: stake_cw20_id,
                    unstaking_duration: None,
                    initial_dao_balance: None,
                },
            },
            &[],
            "voting contract",
            None,
        )
        .unwrap();

    let staking_contract: Addr = app
        .wrap()
        .query_wasm_smart(
            voting_address.clone(),
            &dao_voting_cw20_staked::msg::QueryMsg::StakingContract {},
        )
        .unwrap();

    let token_contract: Addr = app
        .wrap()
        .query_wasm_smart(
            voting_address.clone(),
            &dao_voting_cw20_staked::msg::QueryMsg::TokenContract {},
        )
        .unwrap();

    for Cw20Coin { address, amount } in initial_balances {
        app.execute_contract(
            Addr::unchecked(address),
            token_contract.clone(),
            &cw20_base::msg::ExecuteMsg::Send {
                contract: staking_contract.to_string(),
                amount,
                msg: to_binary(&cw20_stake::msg::ReceiveMsg::Stake {}).unwrap(),
            },
            &[],
        )
            .unwrap();
    }

    app.update_block(next_block);

    let distribution_contract = app
        .instantiate_contract(
            distributor_id,
            Addr::unchecked(CREATOR_ADDR),
            &InstantiateMsg {
                voting_contract: voting_address.to_string(),
            },
            &[],
            "distribution contract",
            None,
        )
        .unwrap();

    BaseTest {
        app,
        distributor_address: distribution_contract,
        staking_address: staking_contract,
        token_address: token_contract,
    }
}

#[test]
fn test_instantiate_fails_given_invalid_voting_contract_address() {

    let mut app = App::default();
    let distributor_id = app.store_code(distributor_contract());
    let cw20_id = app.store_code(cw20_contract());
    let voting_id = app.store_code(staked_balances_voting_contract());
    let stake_cw20_id = app.store_code(cw20_staking_contract());

    let initial_balances = vec![
        Cw20Coin {
            address: "bekauz".to_string(),
            amount: Uint128::new(10),
        }
    ];

    let voting_address = app
        .instantiate_contract(
            voting_id,
            Addr::unchecked(CREATOR_ADDR),
            &dao_voting_cw20_staked::msg::InstantiateMsg {
                active_threshold: None,
                token_info: dao_voting_cw20_staked::msg::TokenInfo::New {
                    code_id: cw20_id,
                    label: "DAO DAO governance token.".to_string(),
                    name: "DAO DAO".to_string(),
                    symbol: "DAO".to_string(),
                    decimals: 6,
                    initial_balances: initial_balances.clone(),
                    marketing: None,
                    staking_code_id: stake_cw20_id,
                    unstaking_duration: None,
                    initial_dao_balance: None,
                },
            },
            &[],
            "voting contract",
            None,
        )
        .unwrap();

    let staking_contract: Addr = app
        .wrap()
        .query_wasm_smart(
            voting_address.clone(),
            &dao_voting_cw20_staked::msg::QueryMsg::StakingContract {},
        )
        .unwrap();

    let token_contract: Addr = app
        .wrap()
        .query_wasm_smart(
            voting_address.clone(),
            &dao_voting_cw20_staked::msg::QueryMsg::TokenContract {},
        )
        .unwrap();

    for Cw20Coin { address, amount } in initial_balances {
        app.execute_contract(
            Addr::unchecked(address),
            token_contract.clone(),
            &cw20_base::msg::ExecuteMsg::Send {
                contract: staking_contract.to_string(),
                amount,
                msg: to_binary(&cw20_stake::msg::ReceiveMsg::Stake {}).unwrap(),
            },
            &[],
        )
        .unwrap();
    }

    app.update_block(next_block);

    let expected_error: ContractError = app
        .instantiate_contract(
            distributor_id,
            Addr::unchecked(CREATOR_ADDR),
            &InstantiateMsg {
                voting_contract: "invalid address".to_string(),
            },
            &[],
            "distribution contract",
            None,
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert!(matches!(expected_error, ContractError::Std(GenericErr { .. })));
}

#[test]
fn test_instantiate_zero_voting_power_error() {

    let mut app = App::default();
    let distributor_id = app.store_code(distributor_contract());
    let cw20_id = app.store_code(cw20_contract());
    let voting_id = app.store_code(staked_balances_voting_contract());
    let stake_cw20_id = app.store_code(cw20_staking_contract());

    let initial_balances = vec![
        Cw20Coin {
            address: "bekauz".to_string(),
            amount: Uint128::new(10),
        }
    ];

    let voting_address = app
        .instantiate_contract(
            voting_id,
            Addr::unchecked(CREATOR_ADDR),
            &dao_voting_cw20_staked::msg::InstantiateMsg {
                active_threshold: None,
                token_info: dao_voting_cw20_staked::msg::TokenInfo::New {
                    code_id: cw20_id,
                    label: "DAO DAO governance token.".to_string(),
                    name: "DAO DAO".to_string(),
                    symbol: "DAO".to_string(),
                    decimals: 6,
                    initial_balances: initial_balances.clone(),
                    marketing: None,
                    staking_code_id: stake_cw20_id,
                    unstaking_duration: None,
                    initial_dao_balance: None,
                },
            },
            &[],
            "voting contract",
            None,
        )
        .unwrap();

    let staking_contract: Addr = app
        .wrap()
        .query_wasm_smart(
            voting_address.clone(),
            &dao_voting_cw20_staked::msg::QueryMsg::StakingContract {},
        )
        .unwrap();

    let token_contract: Addr = app
        .wrap()
        .query_wasm_smart(
            voting_address.clone(),
            &dao_voting_cw20_staked::msg::QueryMsg::TokenContract {},
        )
        .unwrap();

    app.update_block(next_block);

    let expected_error: ContractError = app
        .instantiate_contract(
            distributor_id,
            Addr::unchecked(CREATOR_ADDR),
            &InstantiateMsg {
                voting_contract: voting_address.to_string(),
            },
            &[],
            "distribution contract",
            None,
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert!(matches!(expected_error, ContractError::ZeroVotingPower {}));
}

#[test]
fn test_instantiate_cw_fund_distributor() {
    unimplemented!()
}