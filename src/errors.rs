use borsh::{BorshDeserialize, BorshSerialize};

pub type VmResult<T> = Result<T, VmError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]

pub enum VmError {
    ///The proposal hash provided by the user does not match the proposal hash
    ProposalHashMismatch,
    /// An error occured from serializing or deserializing
    /// some bytes using `borsh` crate
    BorshError,
    /// The bytes provided to create a `ed25519_dalek::PublicKey` are invalid
    InvalidBytesForPublicKey,
    /// The bytes provided to create a `ed25519_dalek::Signature` are invalid
    InvalidBytesForSignature,
    /// The signature did not match the message
    InvalidSignature,
    /// The public key is not allowed to vote on this proposal
    PublicKeyNotFoundInRecords,
}

impl From<borsh::maybestd::io::Error> for VmError {
    fn from(_: borsh::maybestd::io::Error) -> Self {
        VmError::BorshError
    }
}
