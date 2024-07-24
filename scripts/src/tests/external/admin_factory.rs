use cw_orch::{anyhow, prelude::*};

use crate::{
    external::AdminFactorySuite,
    tests::{ADMIN, PREFIX},
};

#[test]
fn test_admin() -> anyhow::Result<()> {
    let mock = MockBech32::new(PREFIX);
    let admin = mock.addr_make(ADMIN);
    let _app = AdminFactorySuite::deploy_on(mock.clone(), admin.clone())?;
    mock.next_block().unwrap();
    Ok(())
}
