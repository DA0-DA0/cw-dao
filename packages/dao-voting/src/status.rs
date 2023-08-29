use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Copy)]
pub enum Status {
    /// The proposal is open for voting.
    Open,
    /// The proposal has been rejected.
    Rejected,
    /// The proposal has been passed but has not been executed.
    Passed,
    /// The proposal has been passed and executed.
    Executed,
    /// The proposal has failed or expired and has been closed. A
    /// proposal deposit refund has been issued if applicable.
    Closed,
    /// The proposal's execution failed.
    ExecutionFailed,
    /// Timelocked proposals have delayed execution, this is only
    /// a potential status if timelock is configured on a proposal module.
    Timelocked,
    /// The proposal has been vetoed.
    Vetoed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open => write!(f, "open"),
            Status::Rejected => write!(f, "rejected"),
            Status::Passed => write!(f, "passed"),
            Status::Executed => write!(f, "executed"),
            Status::Closed => write!(f, "closed"),
            Status::ExecutionFailed => write!(f, "execution_failed"),
            Status::Timelocked => write!(f, "timelocked"),
            Status::Vetoed => write!(f, "vetoed"),
        }
    }
}
