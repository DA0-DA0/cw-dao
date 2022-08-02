use cosm_orc::config::key::Key;
use cosm_orc::orchestrator::cosm_orc::CosmOrc;
use cosm_orc::{
    config::cfg::Config, config::key::SigningKey, profilers::gas_profiler::GasProfiler,
};
use ctor::ctor;
use once_cell::sync::OnceCell;
use rand::Rng;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use test_context::TestContext;

static CONFIG: OnceCell<Cfg> = OnceCell::new();

#[derive(Debug)]
pub struct Cfg {
    cfg: Config,
    gas_report_dir: String,
}

pub struct Chain {
    pub cfg: Config,
    pub orc: CosmOrc,
    pub key: SigningKey,
}

// TODO: Make tests run in parallel
// Im getting the following cosmos-sdk error when running in parallel right now:
//   `account sequence mismatch, expected 92, got 91: incorrect account sequence`

impl TestContext for Chain {
    fn setup() -> Self {
        let cfg = CONFIG.get().unwrap().cfg.clone();
        let orc = CosmOrc::new(cfg.clone())
            .unwrap()
            .add_profiler(Box::new(GasProfiler::new()));

        Self {
            cfg,
            orc,
            key: test_account(),
        }
    }

    fn teardown(self) {
        let cfg = CONFIG.get().unwrap();
        save_gas_report(&self.orc, &cfg.gas_report_dir);
    }
}

fn test_account() -> SigningKey {
    // TODO: Make this configurable + bootstrap the local env with many test accounts
    SigningKey {
        name: "localval".to_string(),
        key: Key::Mnemonic("siren window salt bullet cream letter huge satoshi fade shiver permit offer happy immense wage fitness goose usual aim hammer clap about super trend".to_string()),
        derivation_path : "m/44'/118'/0'/0/0".to_string(),
    }
}

// global_setup() runs once before anything else
#[ctor]
fn global_setup() {
    env_logger::init();
    let config = env::var("CONFIG").expect("missing yaml CONFIG env var");
    let gas_report_dir = env::var("GAS_OUT_DIR").expect("missing GAS_OUT_DIR env var");

    let mut cfg = Config::from_yaml(&config).unwrap();
    let mut orc = CosmOrc::new(cfg.clone())
        .unwrap()
        .add_profiler(Box::new(GasProfiler::new()));

    let key = test_account();

    let skip_storage = env::var("SKIP_CONTRACT_STORE").unwrap_or_else(|_| "false".to_string());
    if !skip_storage.parse::<bool>().unwrap() {
        let contract_dir = env::var("CONTRACT_DIR").expect("missing CONTRACT_DIR env var");
        orc.store_contracts(&contract_dir, &key).unwrap();
        save_gas_report(&orc, &gas_report_dir);
        // persist stored code_ids in CONFIG, so we can reuse for all tests
        cfg.code_ids = orc
            .contract_map
            .deploy_info()
            .iter()
            .map(|(k, v)| (k.clone(), v.code_id))
            .collect();
    }

    let config = Cfg {
        cfg,
        gas_report_dir,
    };
    CONFIG.set(config).expect("error initializing Config");
}

fn save_gas_report(orc: &CosmOrc, gas_report_dir: &str) {
    let reports = orc
        .profiler_reports()
        .expect("error fetching profile reports");

    let j: Value = serde_json::from_slice(&reports[0].json_data).unwrap();

    let p = Path::new(gas_report_dir);
    if !p.exists() {
        fs::create_dir(p).unwrap();
    }

    let mut rng = rand::thread_rng();
    let file_name = format!("test-{}.json", rng.gen::<u32>());
    fs::write(p.join(file_name), j.to_string()).unwrap();
}
