use crate::{
    ProposalHash, PublicKey, ThresholdStatus, VmError, VmResult, Vote, VoteEntry, VoteTally,
};
use core::fmt;
use std::collections::HashMap;
use tai64::Tai64N;

pub struct VoteMachine {
    proposal: ProposalHash,
    votes: HashMap<PublicKey, Vote>,
    records: Vec<PublicKey>,
    threshold: ThresholdStatus,
    expiry: Tai64N, //TODO Create an async timer
}

impl VoteMachine {
    pub fn new(proposal: [u8; 32]) -> Self {
        VoteMachine {
            proposal: proposal.into(),
            votes: HashMap::default(),
            records: Vec::default(),
            threshold: ThresholdStatus::All,
            expiry: Tai64N::now(),
        }
    }

    pub fn add_record(&mut self, public_key: PublicKey) -> &mut Self {
        self.records.push(public_key);

        self
    }

    pub fn add_threshold(&mut self, threshold: ThresholdStatus) -> &mut Self {
        self.threshold = threshold;

        self
    }

    pub fn add_vote(&mut self, entry: VoteEntry) -> VmResult<&mut Self> {
        self.proposal_hashes_match(entry.proposal())?;

        self.record_exists(entry.public_key())?;

        self.verify_hash(entry)?;

        self.votes.insert(entry.public_key(), entry.vote());

        Ok(self)
    }

    pub fn proposal_hashes_match(&self, value: [u8; 32]) -> VmResult<blake3::Hash> {
        let user_proposal_hash: blake3::Hash = value.into();

        if self.proposal != user_proposal_hash {
            Err(VmError::ProposalHashMismatch)
        } else {
            Ok(user_proposal_hash)
        }
    }

    pub fn record_exists(&self, public_key: PublicKey) -> VmResult<bool> {
        match self.records.iter().find(|record| **record == public_key) {
            Some(_) => Ok(true),
            None => Err(VmError::PublicKeyNotFoundInRecords),
        }
    }

    pub fn verify_hash(&self, entry: VoteEntry) -> VmResult<bool> {
        use ed25519_dalek::{
            PublicKey as Ed25519PublicKey, Signature as Ed25519Signature, Verifier,
        };
        let public_key: Ed25519PublicKey = match Ed25519PublicKey::from_bytes(&entry.public_key()) {
            Ok(public_key) => public_key,
            Err(_) => return Err(VmError::InvalidBytesForPublicKey),
        };
        let signature: Ed25519Signature = match Ed25519Signature::from_bytes(&entry.signature()) {
            Ok(signature) => signature,
            Err(_) => return Err(VmError::InvalidBytesForSignature),
        };

        let message = entry.prepare_bytes()?;

        match public_key.verify(&message, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Err(VmError::InvalidSignature),
        }
    }

    pub fn tally(&self) -> VoteTally {
        let mut tally = VoteTally::new();

        self.votes.values().for_each(|vote| {
            if *vote == Vote::Accept {
                tally.accept();
            } else {
                tally.reject();
            }
        });

        tally
    }
}

impl Default for VoteMachine {
    fn default() -> Self {
        VoteMachine::new([0u8; 32])
    }
}

impl fmt::Debug for VoteMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut votes = HashMap::<String, Vote>::new();
        self.votes.iter().for_each(|value| {
            votes.insert(hex::encode(value.0), *value.1);
        });

        let mut records = Vec::<String>::new();
        self.records.iter().for_each(|record| {
            records.push(hex::encode(record));
        });

        f.debug_struct("VoteMachine")
            .field("proposal", &self.proposal.to_hex().as_str())
            .field("votes", &votes)
            .field("records", &records)
            .field("threshold", &self.threshold)
            .field("expiry", &self.expiry)
            .finish()
    }
}
