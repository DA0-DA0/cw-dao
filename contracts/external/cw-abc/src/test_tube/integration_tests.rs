use crate::{
    abc::{
        ClosedConfig, CommonsPhase, CommonsPhaseConfig, CurveType, HatchConfig, MinMax, OpenConfig,
        ReserveToken, SupplyToken,
    },
    msg::{
        CommonsPhaseConfigResponse, CurveInfoResponse, DenomResponse, ExecuteMsg, InstantiateMsg,
        QueryMsg,
    },
    ContractError,
};

use super::test_env::{TestEnv, TestEnvBuilder, DENOM, RESERVE};

use cosmwasm_std::{coins, Decimal, Uint128};
use cw_tokenfactory_issuer::msg::QueryMsg as IssuerQueryMsg;
use dao_interface::token::{NewTokenInfo, TokenInfo};
use osmosis_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use osmosis_test_tube::{osmosis_std::types::cosmos::base::v1beta1::Coin, Account, OsmosisTestApp};

#[test]
fn test_happy_path() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();
    let env = builder.default_setup(&app);
    let TestEnv {
        ref abc,
        ref accounts,
        ref tf_issuer,
        ..
    } = env;

    // Buy tokens
    abc.execute(&ExecuteMsg::Buy {}, &coins(1000, RESERVE), &accounts[0])
        .unwrap();

    // Query denom
    let denom = tf_issuer
        .query::<DenomResponse>(&IssuerQueryMsg::Denom {})
        .unwrap()
        .denom;

    // Query balances
    let user_balance = env
        .bank()
        .query_balance(&QueryBalanceRequest {
            address: accounts[0].address(),
            denom: denom.clone(),
        })
        .unwrap();
    let contract_balance = env
        .bank()
        .query_balance(&QueryBalanceRequest {
            address: abc.contract_addr.to_string(),
            denom: RESERVE.to_string(),
        })
        .unwrap();

    // Check balances
    assert_eq!(
        user_balance.balance,
        Some(Coin {
            denom: denom.clone(),
            amount: "9000".to_string(),
        })
    );
    assert_eq!(
        contract_balance.balance,
        Some(Coin {
            denom: RESERVE.to_string(),
            amount: "900".to_string(), // Minus 10% to fees_recipient
        })
    );

    // Query curve
    let curve_info: CurveInfoResponse = abc.query(&QueryMsg::CurveInfo {}).unwrap();
    assert_eq!(
        curve_info,
        CurveInfoResponse {
            reserve: Uint128::new(900),
            supply: Uint128::new(9000),
            funding: Uint128::new(100),
            spot_price: Decimal::percent(10u64),
            reserve_denom: RESERVE.to_string(),
        }
    );

    // Query phase
    let phase: CommonsPhaseConfigResponse = abc.query(&QueryMsg::PhaseConfig {}).unwrap();
    assert_eq!(phase.phase, CommonsPhase::Hatch);
    assert_eq!(
        phase.phase_config,
        CommonsPhaseConfig {
            hatch: HatchConfig {
                contribution_limits: MinMax {
                    min: Uint128::from(10u128),
                    max: Uint128::from(1000000u128),
                },
                initial_raise: MinMax {
                    min: Uint128::from(10u128),
                    max: Uint128::from(1000000u128),
                },
                entry_fee: Decimal::percent(10u64),
                exit_fee: Decimal::percent(10u64),
            },
            open: OpenConfig {
                entry_fee: Decimal::percent(10u64),
                exit_fee: Decimal::percent(10u64),
            },
            closed: ClosedConfig {},
        }
    );

    // Sell
    abc.execute(
        &ExecuteMsg::Sell {},
        &coins(100, denom.clone()),
        &accounts[0],
    )
    .unwrap();

    // Query curve
    let curve_info: CurveInfoResponse = abc.query(&QueryMsg::CurveInfo {}).unwrap();
    assert_eq!(
        curve_info,
        CurveInfoResponse {
            reserve: Uint128::new(890),
            supply: Uint128::new(8900),
            funding: Uint128::new(101),
            spot_price: Decimal::percent(10u64),
            reserve_denom: RESERVE.to_string(),
        }
    );

    // Query balances
    let user_balance = env
        .bank()
        .query_balance(&QueryBalanceRequest {
            address: accounts[0].address(),
            denom: denom.clone(),
        })
        .unwrap();
    let contract_balance = env
        .bank()
        .query_balance(&QueryBalanceRequest {
            address: abc.contract_addr.to_string(),
            denom: RESERVE.to_string(),
        })
        .unwrap();

    // Check balances
    assert_eq!(
        user_balance.balance,
        Some(Coin {
            denom: denom.clone(),
            amount: "8900".to_string(),
        })
    );
    assert_eq!(
        contract_balance.balance,
        Some(Coin {
            denom: RESERVE.to_string(),
            amount: "890".to_string(),
        })
    );

    // Buy enough tokens to end the hatch phase
    abc.execute(&ExecuteMsg::Buy {}, &coins(999999, RESERVE), &accounts[1])
        .unwrap();

    // Contract is now in open phase
    let phase: CommonsPhaseConfigResponse = abc.query(&QueryMsg::PhaseConfig {}).unwrap();
    assert_eq!(phase.phase, CommonsPhase::Open);
}

#[test]
fn test_contribution_limits_enforced() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();
    let env = builder.default_setup(&app);
    let TestEnv {
        ref abc,
        ref accounts,
        ..
    } = env;

    // Buy more tokens then the max contribution limit errors
    let err = abc
        .execute(
            &ExecuteMsg::Buy {},
            &coins(1000000000, RESERVE),
            &accounts[0],
        )
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::ContributionLimit {
            min: Uint128::from(10u128),
            max: Uint128::from(1000000u128),
        })
    );

    // Buy less tokens then the min contribution limit errors
    let err = abc
        .execute(&ExecuteMsg::Buy {}, &coins(1, RESERVE), &accounts[0])
        .unwrap_err();

    assert_eq!(
        err,
        abc.execute_error(ContractError::ContributionLimit {
            min: Uint128::from(10u128),
            max: Uint128::from(1000000u128),
        })
    );
}

#[test]
fn test_max_supply() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();
    let env = builder.default_setup(&app);
    let TestEnv {
        ref abc,
        ref accounts,
        ..
    } = env;

    // Buy enough tokens to end the hatch phase
    abc.execute(&ExecuteMsg::Buy {}, &coins(1000000, RESERVE), &accounts[0])
        .unwrap();

    // Buy enough tokens to trigger a max supply error
    let err = abc
        .execute(
            &ExecuteMsg::Buy {},
            &coins(10000000000000, RESERVE),
            &accounts[0],
        )
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::CannotExceedMaxSupply {
            max: Uint128::from(1000000000u128)
        })
    );

    // Only owner can update the max supply
    let err = abc
        .execute(
            &ExecuteMsg::UpdateMaxSupply { max_supply: None },
            &[],
            &accounts[1],
        )
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::Ownership(
            cw_ownable::OwnershipError::NotOwner
        ))
    );

    // Update the max supply to no limit
    abc.execute(
        &ExecuteMsg::UpdateMaxSupply { max_supply: None },
        &[],
        &accounts[0],
    )
    .unwrap();

    // Purchase large amount of coins succeeds
    abc.execute(
        &ExecuteMsg::Buy {},
        &coins(10000000000000, RESERVE),
        &accounts[0],
    )
    .unwrap();
}

#[test]
fn test_allowlist() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();
    let instantiate_msg = InstantiateMsg {
        funding_pool_forwarding: Some("replaced to accounts[0]".to_string()),
        supply: SupplyToken {
            token_info: TokenInfo::New(NewTokenInfo {
                token_issuer_code_id: 0,
                subdenom: DENOM.to_string(),
                metadata: None,
                initial_balances: vec![],
                initial_dao_balance: None,
            }),
            decimals: 6,
            max_supply: Some(Uint128::from(1000000000u128)),
        },
        reserve: ReserveToken {
            denom: RESERVE.to_string(),
            decimals: 6,
        },
        phase_config: CommonsPhaseConfig {
            hatch: HatchConfig {
                contribution_limits: MinMax {
                    min: Uint128::from(10u128),
                    max: Uint128::from(1000000u128),
                },
                initial_raise: MinMax {
                    min: Uint128::from(10u128),
                    max: Uint128::from(1000000u128),
                },
                entry_fee: Decimal::percent(10u64),
                exit_fee: Decimal::percent(10u64),
            },
            open: OpenConfig {
                entry_fee: Decimal::percent(10u64),
                exit_fee: Decimal::percent(10u64),
            },
            closed: ClosedConfig {},
        },
        hatcher_allowlist: None,
        curve_type: CurveType::Constant {
            value: Uint128::one(),
            scale: 1,
        },
    };
    let env = builder.setup(&app, instantiate_msg).unwrap();
    let TestEnv {
        ref abc,
        ref accounts,
        ..
    } = env;

    // Only owner can update hatch list
    let err = abc
        .execute(
            &ExecuteMsg::UpdateHatchAllowlist {
                to_add: vec![accounts[0].address(), accounts[1].address()],
                to_remove: vec![],
            },
            &[],
            &accounts[1],
        )
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::Ownership(
            cw_ownable::OwnershipError::NotOwner
        ))
    );

    // Enable the allow list, normally this would be passed in through
    // instantiation.
    abc.execute(
        &ExecuteMsg::UpdateHatchAllowlist {
            to_add: vec![accounts[0].address(), accounts[1].address()],
            to_remove: vec![],
        },
        &[],
        &accounts[0],
    )
    .unwrap();

    // Account not on the hatch allowlist can't purchase
    let err = abc
        .execute(&ExecuteMsg::Buy {}, &coins(1000, RESERVE), &accounts[3])
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::SenderNotAllowlisted {
            sender: accounts[3].address()
        })
    );

    // Account on allowlist can purchase
    abc.execute(&ExecuteMsg::Buy {}, &coins(1000, RESERVE), &accounts[1])
        .unwrap();
}

#[test]
fn test_close_curve() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();
    let env = builder.default_setup(&app);
    let TestEnv {
        ref abc,
        ref accounts,
        ref tf_issuer,
        ..
    } = env;

    // Query denom
    let denom = tf_issuer
        .query::<DenomResponse>(&IssuerQueryMsg::Denom {})
        .unwrap()
        .denom;

    // Buy enough tokens to end the hatch phase
    abc.execute(&ExecuteMsg::Buy {}, &coins(1000000, RESERVE), &accounts[0])
        .unwrap();

    // Only owner can close the curve
    let err = abc
        .execute(&ExecuteMsg::Close {}, &[], &accounts[1])
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::Ownership(
            cw_ownable::OwnershipError::NotOwner
        ))
    );

    // Owner closes curve
    abc.execute(&ExecuteMsg::Close {}, &[], &accounts[0])
        .unwrap();

    // Can no longer buy
    let err = abc
        .execute(&ExecuteMsg::Buy {}, &coins(1000, RESERVE), &accounts[0])
        .unwrap_err();
    assert_eq!(err, abc.execute_error(ContractError::CommonsClosed {}));

    // Can sell
    abc.execute(&ExecuteMsg::Sell {}, &coins(100, denom), &accounts[0])
        .unwrap();
}

// TODO maybe we don't allow for updating the curve in the MVP as it could lead
// to weird edge cases?
#[test]
fn test_update_curve() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();
    let env = builder.default_setup(&app);
    let TestEnv {
        ref abc,
        ref accounts,
        ref tf_issuer,
        ..
    } = env;

    // Query denom
    let denom = tf_issuer
        .query::<DenomResponse>(&IssuerQueryMsg::Denom {})
        .unwrap()
        .denom;

    // Buy enough tokens to end the hatch phase
    abc.execute(&ExecuteMsg::Buy {}, &coins(1000000, RESERVE), &accounts[0])
        .unwrap();

    // Only owner can update the curve
    let err = abc
        .execute(
            &ExecuteMsg::UpdateCurve {
                curve_type: CurveType::Linear {
                    slope: Uint128::new(2),
                    scale: 5,
                },
            },
            &[],
            &accounts[1],
        )
        .unwrap_err();
    assert_eq!(
        err,
        abc.execute_error(ContractError::Ownership(
            cw_ownable::OwnershipError::NotOwner
        ))
    );

    // Owner updates curve
    abc.execute(
        &ExecuteMsg::UpdateCurve {
            curve_type: CurveType::Linear {
                slope: Uint128::new(2),
                scale: 5,
            },
        },
        &[],
        &accounts[0],
    )
    .unwrap();

    // All tokens are sold successfully
    let user_balance = env
        .bank()
        .query_balance(&QueryBalanceRequest {
            address: accounts[0].address(),
            denom: denom.clone(),
        })
        .unwrap();
    assert_eq!(
        user_balance.balance,
        Some(Coin {
            denom: denom.clone(),
            amount: "9000000".to_string(),
        })
    );

    abc.execute(
        &ExecuteMsg::Sell {},
        &coins(9000000, denom.clone()),
        &accounts[0],
    )
    .unwrap();

    // No money is left over in the contract
    let contract_balance = env
        .bank()
        .query_balance(&QueryBalanceRequest {
            address: abc.contract_addr.to_string(),
            denom: RESERVE.to_string(),
        })
        .unwrap();
    assert_eq!(
        contract_balance.balance,
        Some(Coin {
            denom: RESERVE.to_string(),
            amount: "0".to_string(),
        })
    );
}

#[test]
fn test_existing_token_failures() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();

    // The tokenfactory token does not exist - fails in supply query
    let mut instantiate_msg = InstantiateMsg {
        funding_pool_forwarding: Some("replaced to accounts[0]".to_string()),
        supply: SupplyToken {
            token_info: TokenInfo::Existing {
                denom: "factory/address/nonexistent".to_string(),
            },
            decimals: 6,
            max_supply: Some(Uint128::from(1000000000u128)),
        },
        reserve: ReserveToken {
            denom: RESERVE.to_string(),
            decimals: 6,
        },
        phase_config: CommonsPhaseConfig {
            hatch: HatchConfig {
                contribution_limits: MinMax {
                    min: Uint128::from(10u128),
                    max: Uint128::from(1000000u128),
                },
                initial_raise: MinMax {
                    min: Uint128::from(10u128),
                    max: Uint128::from(1000000u128),
                },
                entry_fee: Decimal::percent(10u64),
                exit_fee: Decimal::percent(10u64),
            },
            open: OpenConfig {
                entry_fee: Decimal::percent(10u64),
                exit_fee: Decimal::percent(10u64),
            },
            closed: ClosedConfig {},
        },
        hatcher_allowlist: None,
        curve_type: CurveType::Constant {
            value: Uint128::one(),
            scale: 1,
        },
    };
    let result = builder.setup(&app, instantiate_msg.clone());
    assert!(result.is_err());

    // A subdenom is required
    let builder = TestEnvBuilder::new();
    instantiate_msg.supply.token_info = TokenInfo::Existing {
        denom: "factory/malformed".to_string(),
    };
    let result = builder.setup(&app, instantiate_msg.clone());
    assert!(result.is_err());

    // Only tokenfactory tokens are supported
    let builder = TestEnvBuilder::new();
    instantiate_msg.supply.token_info = TokenInfo::Existing {
        denom: "junox".to_string(),
    };
    let result = builder.setup(&app, instantiate_msg.clone());
    assert!(result.is_err());
}

#[test]
fn test_existing_token() {
    let app = OsmosisTestApp::new();
    let builder = TestEnvBuilder::new();

    let result = builder.setup_with_token(
        &app,
        InstantiateMsg {
            funding_pool_forwarding: Some("replaced to accounts[0]".to_string()),
            supply: SupplyToken {
                token_info: TokenInfo::Existing {
                    denom: "replaced".to_string(),
                },
                decimals: 6,
                max_supply: None,
            },
            reserve: ReserveToken {
                denom: RESERVE.to_string(),
                decimals: 6,
            },
            phase_config: CommonsPhaseConfig {
                hatch: HatchConfig {
                    contribution_limits: MinMax {
                        min: Uint128::from(10u128),
                        max: Uint128::from(1000000u128),
                    },
                    initial_raise: MinMax {
                        min: Uint128::from(10u128),
                        max: Uint128::from(1000000u128),
                    },
                    entry_fee: Decimal::percent(10u64),
                    exit_fee: Decimal::percent(10u64),
                },
                open: OpenConfig {
                    entry_fee: Decimal::percent(10u64),
                    exit_fee: Decimal::percent(10u64),
                },
                closed: ClosedConfig {},
            },
            hatcher_allowlist: None,
            curve_type: CurveType::Constant {
                value: Uint128::one(),
                scale: 1,
            },
        },
    );
    assert!(result.is_ok());
    let TestEnv {
        ref abc,
        ref accounts,
        ref tf_issuer,
        ..
    } = result.unwrap();

    let denom_response: DenomResponse = tf_issuer
        .query(&cw_tokenfactory_issuer::msg::QueryMsg::Denom {})
        .unwrap();
    let _denom = format!(
        "factory/{}/{}",
        tf_issuer.contract_addr, denom_response.denom
    );

    // Allow the ABC to modify the supply
    let result = tf_issuer.execute(
        &cw_tokenfactory_issuer::msg::ExecuteMsg::SetMinterAllowance {
            address: abc.contract_addr.clone(),
            allowance: Uint128::MAX,
        },
        &[],
        &accounts[0],
    );
    assert!(result.is_ok());

    let result = tf_issuer.execute(
        &cw_tokenfactory_issuer::msg::ExecuteMsg::SetBurnerAllowance {
            address: abc.contract_addr.clone(),
            allowance: Uint128::MAX,
        },
        &[],
        &accounts[0],
    );
    assert!(result.is_ok());
}
