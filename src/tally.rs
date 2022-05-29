use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, BorshDeserialize, BorshSerialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoteTally {
    accepted: usize,
    rejected: usize,
}

impl VoteTally {
    pub fn new() -> Self {
        VoteTally {
            accepted: 0,
            rejected: 0,
        }
    }

    pub fn accept(&mut self) -> &mut Self {
        self.accepted += 1;

        self
    }

    pub fn reject(&mut self) -> &mut Self {
        self.rejected += 1;

        self
    }

    pub fn compile(&self) -> (usize, usize) {
        (self.accepted, self.rejected)
    }

    pub fn outcome(&self) -> VoteOutcome {
        if self.accepted > self.rejected {
            VoteOutcome::Accepted
        } else if self.accepted == self.rejected {
            VoteOutcome::Equal
        } else {
            VoteOutcome::Rejected
        }
    }
}

#[derive(Debug, Clone, Copy, BorshDeserialize, BorshSerialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum VoteOutcome {
    Accepted,
    Rejected,
    Equal,
}
