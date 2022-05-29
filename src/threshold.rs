use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize, Clone, Copy)]
pub enum ThresholdStatus {
    All,
    Third,
    Half,
    TwoThirds,
}

impl ThresholdStatus {
    pub fn calculate_threshold(&self, group_size: usize) -> usize {
        match self {
            Self::All => group_size,
            Self::Third => (group_size as f32 * (1.0 / 3.0)).ceil() as usize,
            Self::Half => (group_size as f32 * (1.0 / 2.0)).ceil() as usize,
            Self::TwoThirds => (group_size as f32 * (2.0 / 3.0)).ceil() as usize,
        }
    }
}
