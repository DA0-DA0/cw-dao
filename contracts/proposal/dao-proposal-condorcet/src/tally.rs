use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::{
    m::{Stats, M},
    vote::Vote,
};

/// Stores the state of a ranked choice election by wrapping a `M`
/// matrix and maintaining:
///
/// LM[x][y] = |x > y| - |y > x|
///
/// Or in english "the number of times x has beaten y" minus "the
/// number of times y has beaten x". This construction provides that
/// if a column holds all positive, non-zero values then the
/// corresponding candidate is the Condorcet winner. A Condorcet
/// winner is undisputed if it's smallest margin of victory is larger
/// than the outstanding voting power.
#[cw_serde]
pub struct Tally {
    m: M,

    /// Amount of voting power that has yet to vote in this tally.
    pub power_outstanding: Uint128,

    /// The current winner. Always up to date and updated on vote.
    pub winner: Winner,
}

#[cw_serde]
#[derive(Copy)]
pub enum Winner {
    Never,
    None,
    Some(u32),
    Undisputed(u32),
}

impl Tally {
    pub fn new(candidates: u32, total_power: Uint128) -> Self {
        Self {
            m: M::new(candidates as usize),
            power_outstanding: total_power,
            winner: Winner::None,
        }
    }

    pub fn candidates(&self) -> u32 {
        self.m.n as u32
    }

    /// Records a vote in the tally.
    ///
    ///  - `vote` a list of candidates sorted in order from most to
    ///    least favored
    ///  - `power` the voting power of the voter
    pub fn add_vote(&mut self, vote: Vote, power: Uint128) {
        for (index, preference) in vote.iter().enumerate() {
            // an interesting property of the symetry of M is that in
            // recording all the defeats, we also record all of the
            // victories.
            for defeat in 0..index {
                self.m
                    .decrement((*preference as usize, vote[defeat] as usize), power)
            }
        }
        self.power_outstanding -= power;
        self.winner = self.winner();
    }

    fn winner(&self) -> Winner {
        match self.m.stats() {
            Stats::PositiveColumn { col, min_margin } => {
                if min_margin > self.power_outstanding {
                    Winner::Undisputed(col as u32)
                } else {
                    Winner::Some(col as u32)
                }
            }
            Stats::NoPositiveColumn {
                min_col_distance_from_positivity,
                max_negative_in_min_col,
            } => {
                if min_col_distance_from_positivity
                    > self.power_outstanding.full_mul((self.m.n - 1) as u32)
                    || max_negative_in_min_col >= self.power_outstanding
                {
                    Winner::Never
                } else {
                    Winner::None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_election() {
        let candidates = 2;
        let mut tally = Tally::new(candidates, Uint128::new(3));

        tally.add_vote(Vote::new(vec![0, 1], candidates).unwrap(), Uint128::one());
        tally.add_vote(Vote::new(vec![1, 0], candidates).unwrap(), Uint128::one());
        tally.add_vote(Vote::new(vec![1, 0], candidates).unwrap(), Uint128::one());

        let winner = tally.winner();
        assert_eq!(winner, Winner::Undisputed(1));
    }

    #[test]
    fn test_triplet_election() {
        let candidates = 3;
        let mut tally = Tally::new(candidates, Uint128::new(3));

        tally.add_vote(
            Vote::new(vec![0, 1, 2], candidates).unwrap(),
            Uint128::one(),
        );

        let winner = tally.winner();
        assert_eq!(winner, Winner::Some(0));

        tally.add_vote(
            Vote::new(vec![0, 2, 1], candidates).unwrap(),
            Uint128::one(),
        );
        tally.add_vote(
            Vote::new(vec![2, 0, 1], candidates).unwrap(),
            Uint128::one(),
        );

        let winner = tally.winner();
        assert_eq!(winner, Winner::Undisputed(0));
    }

    #[test]
    fn test_condorcet_paradox() {
        let candidates = 3;
        let mut tally = Tally::new(candidates, Uint128::new(6));

        tally.add_vote(
            Vote::new(vec![0, 2, 1], candidates).unwrap(),
            Uint128::one(),
        );
        tally.add_vote(
            Vote::new(vec![1, 0, 2], candidates).unwrap(),
            Uint128::one(),
        );
        tally.add_vote(
            Vote::new(vec![2, 1, 0], candidates).unwrap(),
            Uint128::one(),
        );
        tally.add_vote(
            Vote::new(vec![1, 0, 2], candidates).unwrap(),
            Uint128::one(),
        );
        tally.add_vote(
            Vote::new(vec![0, 2, 1], candidates).unwrap(),
            Uint128::one(),
        );
        tally.add_vote(
            Vote::new(vec![2, 0, 1], candidates).unwrap(),
            Uint128::one(),
        );

        // sequence of ballots cast:
        //
        // 0 > 2 > 1
        // 1 > 0 > 2
        // 2 > 1 > 0
        // 1 > 0 > 2
        // 0 > 2 > 1
        // 2 > 0 > 1
        //
        // produces a M matrix:
        //
        // ```
        //   \  0 -2
        //   0  \  2
        //   2 -2  \
        // ```
        //
        // the "condorcet paradox" 0 > 2, 2 > 1, 0 !> 1.
        assert_eq!(tally.winner(), Winner::Never)
    }
}
