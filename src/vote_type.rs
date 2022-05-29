use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum Vote {
    Accept,
    Reject,
}

impl Default for Vote {
    fn default() -> Self {
        Vote::Reject
    }
}
