use crate::{PublicKey, VmResult, Vote};
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct VoteEntry {
    public_key: PublicKey,
    proposal: [u8; 32],
    signature: [u8; 64],
    vote: Vote,
}

impl VoteEntry {
    pub fn new(proposal_hash: [u8; 32]) -> Self {
        VoteEntry {
            public_key: [0u8; 32],
            proposal: proposal_hash,
            signature: [0u8; 64],
            vote: Vote::default(),
        }
    }

    pub fn add_public_key(&mut self, public_key: [u8; 32]) -> &mut Self {
        self.public_key = public_key;

        self
    }

    pub fn add_vote(&mut self, vote: Vote) -> &mut Self {
        self.vote = vote;

        self
    }

    pub fn prepare_bytes(&self) -> VmResult<Vec<u8>> {
        let mut buffer = Vec::<u8>::default();
        buffer.extend_from_slice(&self.proposal);
        buffer.extend_from_slice(&self.vote.try_to_vec()?);

        Ok(buffer)
    }

    pub fn add_signature(&mut self, signature: [u8; 64]) -> &mut Self {
        self.signature = signature;

        self
    }

    pub fn to_bytes(&self) -> VmResult<Vec<u8>> {
        Ok(self.try_to_vec()?)
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn proposal(&self) -> [u8; 32] {
        self.proposal
    }

    pub fn signature(&self) -> [u8; 64] {
        self.signature
    }

    pub fn vote(&self) -> Vote {
        self.vote
    }
}

impl fmt::Debug for VoteEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VoteEntry")
            .field("public_key", &hex::encode(&self.public_key))
            .field("proposal", &hex::encode(&self.proposal))
            .field("signature", &hex::encode(&self.signature))
            .field("vote", &self.vote)
            .finish()
    }
}
