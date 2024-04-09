// The code is used in tests but reported as dead code
// see https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]

use crate::{
    abc::{
        ClosedConfig, CommonsPhaseConfig, CurveType, HatchConfig, MinMax, OpenConfig, ReserveToken,
        SupplyToken,
    },
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    ContractError,
};

use cosmwasm_std::{Coin, Decimal, Uint128};
use dao_interface::token::{NewTokenInfo, TokenInfo};
use dao_testing::test_tube::cw_tokenfactory_issuer::TokenfactoryIssuer;
use osmosis_test_tube::{
    osmosis_std::types::cosmos::bank::v1beta1::QueryAllBalancesRequest,
    osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse, Account, Bank, Module,
    OsmosisTestApp, RunnerError, RunnerExecuteResult, SigningAccount, Wasm,
};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::path::PathBuf;

pub const DENOM: &str = "ucat";

// Needs to match what's configured for test-tube
pub const RESERVE: &str = "uosmo";

pub struct TestEnv<'a> {
    pub app: &'a OsmosisTestApp,
    pub abc: CwAbc<'a>,
    pub tf_issuer: TokenfactoryIssuer<'a>,
    pub accounts: Vec<SigningAccount>,
}

impl<'a> TestEnv<'a> {
    pub fn instantiate(
        &self,
        msg: &InstantiateMsg,
        signer: SigningAccount,
    ) -> Result<CwAbc, RunnerError> {
        CwAbc::<'a>::instantiate(self.app, self.abc.code_id, msg, &signer)
    }

    pub fn get_tf_issuer_code_id(&self) -> u64 {
        self.tf_issuer.code_id
    }

    pub fn bank(&self) -> Bank<'_, OsmosisTestApp> {
        Bank::new(self.app)
    }

    pub fn assert_account_balances(
        &self,
        account: SigningAccount,
        expected_balances: Vec<Coin>,
        ignore_denoms: Vec<&str>,
    ) {
        let account_balances: Vec<Coin> = Bank::new(self.app)
            .query_all_balances(&QueryAllBalancesRequest {
                address: account.address(),
                pagination: None,
            })
            .unwrap()
            .balances
            .into_iter()
            .map(|coin| Coin::new(coin.amount.parse().unwrap(), coin.denom))
            .filter(|coin| !ignore_denoms.contains(&coin.denom.as_str()))
            .collect();

        assert_eq!(account_balances, expected_balances);
    }

    pub fn assert_contract_balances(&self, expected_balances: &[Coin]) {
        let contract_balances: Vec<Coin> = Bank::new(self.app)
            .query_all_balances(&QueryAllBalancesRequest {
                address: self.abc.contract_addr.clone(),
                pagination: None,
            })
            .unwrap()
            .balances
            .into_iter()
            .map(|coin| Coin::new(coin.amount.parse().unwrap(), coin.denom))
            .collect();

        assert_eq!(contract_balances, expected_balances);
    }
}

pub struct TestEnvBuilder {}

impl TestEnvBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn default_setup(self, app: &'_ OsmosisTestApp) -> TestEnv<'_> {
        let accounts = app
            .init_accounts(&[Coin::new(1000000000000000u128, RESERVE)], 10)
            .unwrap();

        let issuer_id = TokenfactoryIssuer::upload(app, &accounts[0]).unwrap();

        let abc = CwAbc::deploy(
            app,
            &InstantiateMsg {
                funding_pool_forwarding: Some(accounts[0].address()),
                supply: SupplyToken {
                    token_info: TokenInfo::New(NewTokenInfo {
                        token_issuer_code_id: issuer_id,
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
            },
            &accounts[0],
        )
        .unwrap();

        let issuer_addr = CwAbc::query(&abc, &QueryMsg::TokenContract {}).unwrap();

        let tf_issuer = TokenfactoryIssuer::new_with_values(app, issuer_id, issuer_addr).unwrap();

        TestEnv {
            app,
            abc,
            tf_issuer,
            accounts,
        }
    }

    pub fn setup(
        self,
        app: &'_ OsmosisTestApp,
        mut msg: InstantiateMsg,
    ) -> Result<TestEnv<'_>, RunnerError> {
        let accounts = app
            .init_accounts(&[Coin::new(1000000000000000u128, RESERVE)], 10)
            .unwrap();

        let issuer_id = TokenfactoryIssuer::upload(app, &accounts[0])?;

        // Override issuer_id and fees_recipient
        if let TokenInfo::New(ref mut new_token_info) = msg.supply.token_info {
            new_token_info.token_issuer_code_id = issuer_id;
        }

        msg.funding_pool_forwarding = Some(accounts[0].address());

        let abc = CwAbc::deploy(app, &msg, &accounts[0])?;

        let issuer_addr = CwAbc::query(&abc, &QueryMsg::TokenContract {})?;

        let tf_issuer = TokenfactoryIssuer::new_with_values(app, issuer_id, issuer_addr)?;

        Ok(TestEnv {
            app,
            abc,
            tf_issuer,
            accounts,
        })
    }

    pub fn setup_with_token(
        self,
        app: &'_ OsmosisTestApp,
        mut msg: InstantiateMsg,
    ) -> Result<TestEnv<'_>, RunnerError> {
        let accounts = app
            .init_accounts(&[Coin::new(1000000000000000u128, RESERVE)], 10)
            .unwrap();

        let tf_issuer: TokenfactoryIssuer = TokenfactoryIssuer::new(
            app,
            &cw_tokenfactory_issuer::msg::InstantiateMsg::NewToken {
                subdenom: "subdenom".to_string(),
            },
            &accounts[0],
        )?;

        msg.funding_pool_forwarding = Some(accounts[0].address());

        msg.supply.token_info = TokenInfo::Existing {
            denom: format!("factory/{}/{}", tf_issuer.contract_addr, "subdenom"),
        };

        // Accounts[0] has minted some tokens to themselves
        tf_issuer.execute(
            &cw_tokenfactory_issuer::msg::ExecuteMsg::SetMinterAllowance {
                address: accounts[0].address().to_string(),
                allowance: Uint128::MAX,
            },
            &[],
            &accounts[0],
        )?;
        tf_issuer.execute(
            &cw_tokenfactory_issuer::msg::ExecuteMsg::Mint {
                to_address: accounts[0].address().to_string(),
                amount: Uint128::new(1_000_000u128),
            },
            &[],
            &accounts[0],
        )?;

        let abc = CwAbc::deploy(app, &msg, &accounts[0])?;

        Ok(TestEnv {
            app,
            abc,
            tf_issuer,
            accounts,
        })
    }

    pub fn upload_issuer(self, app: &'_ OsmosisTestApp, signer: &SigningAccount) -> u64 {
        TokenfactoryIssuer::upload(app, signer).unwrap()
    }
}

pub fn assert_contract_err(expected: ContractError, actual: RunnerError) {
    match actual {
        RunnerError::ExecuteError { msg } => {
            if !msg.contains(&expected.to_string()) {
                panic!(
                    "assertion failed:\n\n  must contain \t: \"{}\",\n  actual \t: \"{}\"\n",
                    expected, msg
                )
            }
        }
        _ => panic!("unexpected error, expect execute error but got: {}", actual),
    };
}

#[derive(Debug)]
pub struct CwAbc<'a> {
    pub app: &'a OsmosisTestApp,
    pub code_id: u64,
    pub contract_addr: String,
}

impl<'a> CwAbc<'a> {
    pub fn deploy(
        app: &'a OsmosisTestApp,
        instantiate_msg: &InstantiateMsg,
        signer: &SigningAccount,
    ) -> Result<Self, RunnerError> {
        let wasm = Wasm::new(app);
        let token_creation_fee = Coin::new(10000000, RESERVE);

        let code_id = wasm
            .store_code(&Self::get_wasm_byte_code(), None, signer)?
            .data
            .code_id;

        let contract_addr = wasm
            .instantiate(
                code_id,
                &instantiate_msg,
                Some(&signer.address()),
                None,
                &[token_creation_fee],
                signer,
            )?
            .data
            .address;

        Ok(Self {
            app,
            code_id,
            contract_addr,
        })
    }

    pub fn new_with_values(
        app: &'a OsmosisTestApp,
        code_id: u64,
        contract_addr: String,
    ) -> Result<Self, RunnerError> {
        Ok(Self {
            app,
            code_id,
            contract_addr,
        })
    }

    /// uploads contract and returns a code ID
    pub fn upload(app: &OsmosisTestApp, signer: &SigningAccount) -> Result<u64, RunnerError> {
        let wasm = Wasm::new(app);

        let code_id = wasm
            .store_code(&Self::get_wasm_byte_code(), None, signer)?
            .data
            .code_id;

        Ok(code_id)
    }

    pub fn instantiate(
        app: &'a OsmosisTestApp,
        code_id: u64,
        instantiate_msg: &InstantiateMsg,
        signer: &SigningAccount,
    ) -> Result<Self, RunnerError> {
        let wasm = Wasm::new(app);
        let contract_addr = wasm
            .instantiate(
                code_id,
                &instantiate_msg,
                Some(&signer.address()),
                None,
                &[],
                signer,
            )?
            .data
            .address;

        Ok(Self {
            app,
            code_id,
            contract_addr,
        })
    }

    // executes
    pub fn execute(
        &self,
        execute_msg: &ExecuteMsg,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse> {
        let wasm = Wasm::new(self.app);
        wasm.execute(&self.contract_addr, execute_msg, funds, signer)
    }

    // queries
    pub fn query<T>(&self, query_msg: &QueryMsg) -> Result<T, RunnerError>
    where
        T: DeserializeOwned,
    {
        let wasm = Wasm::new(self.app);
        wasm.query(&self.contract_addr, query_msg)
    }

    // pub fn migrate(
    //     &self,
    //     testdata: &str,
    //     signer: &SigningAccount,
    // ) -> RunnerExecuteResult<MsgMigrateContractResponse> {
    //     let wasm = Wasm::new(self.app);
    //     let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     let wasm_byte_code =
    //         std::fs::read(manifest_path.join("tests").join("testdata").join(testdata)).unwrap();

    //     let code_id = wasm.store_code(&wasm_byte_code, None, signer)?.data.code_id;
    //     self.app.execute(
    //         MsgMigrateContract {
    //             sender: signer.address(),
    //             contract: self.contract_addr.clone(),
    //             code_id,
    //             msg: serde_json::to_vec(&MigrateMsg {}).unwrap(),
    //         },
    //         "/cosmwasm.wasm.v1.MsgMigrateContract",
    //         signer,
    //     )
    // }

    fn get_wasm_byte_code() -> Vec<u8> {
        let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let byte_code = std::fs::read(
            manifest_path
                .join("..")
                .join("..")
                .join("..")
                .join("artifacts")
                .join("cw_abc.wasm"),
        );
        match byte_code {
            Ok(byte_code) => byte_code,
            // On arm processors, the above path is not found, so we try the following path
            Err(_) => std::fs::read(
                manifest_path
                    .join("..")
                    .join("..")
                    .join("..")
                    .join("artifacts")
                    .join("cw_abc-aarch64.wasm"),
            )
            .unwrap(),
        }
    }

    pub fn execute_error(&self, err: ContractError) -> RunnerError {
        RunnerError::ExecuteError {
            msg: format!(
                "failed to execute message; message index: 0: {}: execute wasm contract failed",
                err
            ),
        }
    }
}