use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use cw_ownable::cw_ownable;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub staking_addr: String,
    pub reward_rate: Uint128,
    pub reward_token: String,
}

#[cw_ownable]
#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        staking_addr: String,
        reward_rate: Uint128,
        reward_token: String,
    },
    Distribute {},
    Withdraw {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(InfoResponse)]
    Info {},

    #[returns(::cw_ownable::Ownership<::cosmwasm_std::Addr>)]
    Ownership {},
}

#[cw_serde]
pub struct InfoResponse {
    pub config: Config,
    pub last_payment_block: u64,
    pub balance: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {}
