use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cwd_interface::voting::InfoResponse;
use cwd_interface::voting::TotalPowerAtHeightResponse;
use cwd_interface::voting::VotingPowerAtHeightResponse;
use cwd_macros::{info_query, voting_query};

#[cw_serde]
pub struct InstantiateMsg {
    pub cw4_group_code_id: u64,
    pub initial_members: Vec<cw4::Member>,
}

#[cw_serde]
pub enum ExecuteMsg {
    MemberChangedHook { diffs: Vec<cw4::MemberDiff> },
}

#[voting_query]
#[info_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Addr)]
    GroupContract {},
    #[returns(Addr)]
    Dao {},
}

#[cw_serde]
pub struct MigrateMsg {}
