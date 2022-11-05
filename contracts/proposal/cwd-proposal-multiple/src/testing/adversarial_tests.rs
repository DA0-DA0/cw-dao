use cosmwasm_std::{Addr, CosmosMsg};
use cw_multi_test::{App, Executor, next_block};
use cwd_voting::multiple_choice::{MultipleChoiceOption, MultipleChoiceOptions, MultipleChoiceVote};
use cwd_voting::status::Status;
use cwd_voting::voting::MultipleChoiceVotes;
use crate::ContractError;
use crate::msg::ExecuteMsg;
use crate::testing::execute::{make_proposal, mint_cw20s};
use crate::testing::instantiate::{_get_default_token_dao_proposal_module_instantiate, instantiate_with_multiple_staked_balances_governance, instantiate_with_staked_balances_governance};
use crate::testing::queries::{query_dao_token, query_multiple_proposal_module, query_proposal};
use crate::testing::tests::{ALTERNATIVE_ADDR, CREATOR_ADDR};

struct CommonTest {
    app: App,
    core_addr: Addr,
    proposal_module: Addr,
    gov_token: Addr,
    proposal_id: u64,
}
fn setup_test(messages: Vec<CosmosMsg>) -> CommonTest {
    let mut app = App::default();
    let instantiate = _get_default_token_dao_proposal_module_instantiate(&mut app);
    let core_addr = instantiate_with_multiple_staked_balances_governance(&mut app, instantiate, None);
    let proposal_module = query_multiple_proposal_module(&app, &core_addr);
    let gov_token = query_dao_token(&app, &core_addr);

    // Mint some tokens to pay the proposal deposit.
    mint_cw20s(&mut app, &gov_token, &core_addr, CREATOR_ADDR, 10_000_000);

    let options = vec![
        MultipleChoiceOption {
            description: "multiple choice option 1".to_string(),
            msgs: None,
        },
        MultipleChoiceOption {
            description: "multiple choice option 2".to_string(),
            msgs: None,
        },
    ];

    let mc_options = MultipleChoiceOptions { options };

    let proposal_id = make_proposal(&mut app, &proposal_module, CREATOR_ADDR, mc_options);

    CommonTest {
        app,
        core_addr,
        proposal_module,
        gov_token,
        proposal_id,
    }
}

#[test]
fn test_execute_proposal_open() {
    let CommonTest {
        mut app,
        core_addr: _,
        proposal_module,
        gov_token: _,
        proposal_id,
    } = setup_test(vec![]);

    app.update_block(next_block);

    // assert proposal is open
    let prop = query_proposal(&app, &proposal_module, proposal_id);
    assert_eq!(prop.proposal.status, Status::Open);

    // attempt to execute and assert that it fails
    let err = app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module,
        &ExecuteMsg::Execute { proposal_id },
        &[],
    )
    .unwrap_err()
    .downcast()
    .unwrap();

    assert!(matches!(err, ContractError::NotPassed {}))
}

#[test]
fn test_execute_proposal_rejected_closed() {
    let CommonTest {
        mut app,
        core_addr: _,
        proposal_module,
        gov_token: _,
        proposal_id,
    } = setup_test(vec![]);

    app.update_block(next_block);

    // assert proposal is open
    let prop = query_proposal(&app, &proposal_module, proposal_id);
    assert_eq!(prop.proposal.status, Status::Open);

    // Vote on both options to reject the proposal
    let vote = MultipleChoiceVote {
        option_id: 0,
    };
    app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Vote {
            proposal_id,
            vote
        },
        &[],
    )
    .unwrap();

    let vote = MultipleChoiceVote {
        option_id: 1,
    };
    app.execute_contract(
        Addr::unchecked(ALTERNATIVE_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Vote {
            proposal_id,
            vote
        },
        &[],
    )
    .unwrap();

    app.update_block(next_block);

    let prop = query_proposal(&app, &proposal_module, proposal_id);
    assert_eq!(prop.proposal.status, Status::Rejected);

    // attempt to execute and assert that it fails
    let err = app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Execute { proposal_id },
        &[],
    )
    .unwrap_err()
    .downcast()
    .unwrap();

    assert!(matches!(err, ContractError::NotPassed {}));

    app.update_block(next_block);

    // close the proposal
    app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Close { proposal_id },
        &[],
    )
    .unwrap();

    // assert prop is closed and attempt to execute it
    let prop = query_proposal(&app, &proposal_module, proposal_id);
    assert_eq!(prop.proposal.status, Status::Closed);

    let err = app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module,
        &ExecuteMsg::Execute { proposal_id },
        &[],
    )
    .unwrap_err()
    .downcast()
    .unwrap();
    assert!(matches!(err, ContractError::NotPassed {}));
}

#[test]
fn test_execute_proposal_more_than_once() {
    let CommonTest {
        mut app,
        core_addr: _,
        proposal_module,
        gov_token: _,
        proposal_id,
    } = setup_test(vec![]);

    app.update_block(next_block);

    // get the proposal to pass
    let vote = MultipleChoiceVote {
        option_id: 0,
    };
    app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Vote {
            proposal_id,
            vote
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked(ALTERNATIVE_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Vote {
            proposal_id,
            vote
        },
        &[],
    )
    .unwrap();

    app.update_block(next_block);

    let prop = query_proposal(&app, &proposal_module, proposal_id);
    assert_eq!(prop.proposal.status, Status::Passed);

    // execute the proposal
    app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module.clone(),
        &ExecuteMsg::Execute { proposal_id },
        &[],
    )
    .unwrap();

    app.update_block(next_block);

    let prop = query_proposal(&app, &proposal_module, proposal_id);
    assert_eq!(prop.proposal.status, Status::Executed);

    let err = app.execute_contract(
        Addr::unchecked(CREATOR_ADDR),
        proposal_module,
        &ExecuteMsg::Execute { proposal_id },
        &[],
    )
    .unwrap_err()
    .downcast()
    .unwrap();
    assert!(matches!(err, ContractError::NotPassed {}));
}