mod entry;
pub use entry::*;

mod errors;
pub use errors::*;

mod tally;
pub use tally::*;

mod threshold;
pub use threshold::*;

mod types;
pub use types::*;

mod state_machine;
pub use state_machine::*;

mod vote_type;
pub use vote_type::*;

#[cfg(test)]
mod sanity_tests {
    use crate::*;
    use ed25519_dalek::{Keypair as Ed25519Keypair, Signer};
    use rand::rngs::OsRng;

    #[test]
    fn threshold_tests() {
        let one_third = ThresholdStatus::Third;
        assert_eq!(1, one_third.calculate_threshold(2));

        let half = ThresholdStatus::Half;
        assert_eq!(1, half.calculate_threshold(2));

        let two_thirds = ThresholdStatus::TwoThirds;
        assert_eq!(2, two_thirds.calculate_threshold(2));

        let full = ThresholdStatus::All;
        assert_eq!(2, full.calculate_threshold(2));

        let one_third = ThresholdStatus::Third;
        assert_eq!(4, one_third.calculate_threshold(10));

        let half = ThresholdStatus::Half;
        assert_eq!(5, half.calculate_threshold(10));

        let two_thirds = ThresholdStatus::TwoThirds;
        assert_eq!(7, two_thirds.calculate_threshold(10));

        let full = ThresholdStatus::All;
        assert_eq!(10, full.calculate_threshold(10));
    }

    #[test]
    fn proposal_rejected() {
        let proposal_content =
            b"A Party for Financial Inclusion of Billions of Users from the Developing World";
        let proposal_hash = blake3::hash(proposal_content);

        let mut csprng = OsRng {};
        let kp0 = Ed25519Keypair::generate(&mut csprng);
        let kp1 = Ed25519Keypair::generate(&mut csprng);
        let kp2 = Ed25519Keypair::generate(&mut csprng);
        let kp3 = Ed25519Keypair::generate(&mut csprng);
        let kp4 = Ed25519Keypair::generate(&mut csprng);
        let kp5 = Ed25519Keypair::generate(&mut csprng);
        let kp6 = Ed25519Keypair::generate(&mut csprng);
        let kp7 = Ed25519Keypair::generate(&mut csprng);
        let kp8 = Ed25519Keypair::generate(&mut csprng);
        let kp9 = Ed25519Keypair::generate(&mut csprng);

        let keypair_holder: Vec<&Ed25519Keypair> =
            vec![&kp0, &kp1, &kp2, &kp3, &kp4, &kp5, &kp6, &kp7, &kp8, &kp9];

        let mut vote_machine = VoteMachine::new(*proposal_hash.as_bytes());
        vote_machine.add_threshold(ThresholdStatus::Third);
        keypair_holder.iter().for_each(|keypair| {
            vote_machine.add_record(keypair.public.to_bytes());
        });

        let mut vote_entry = VoteEntry::new(*proposal_hash.as_bytes());
        keypair_holder
            .iter()
            .enumerate()
            .for_each(|(index, keypair)| {
                if index < 4 {
                    vote_entry.add_public_key(keypair.public.to_bytes());
                    vote_entry.add_vote(Vote::Accept);
                } else {
                    vote_entry.add_public_key(keypair.public.to_bytes());
                    vote_entry.add_vote(Vote::Reject);
                }

                let vote_bytes = vote_entry.prepare_bytes();
                assert!(&vote_bytes.is_ok());
                let signature = keypair.sign(&vote_bytes.unwrap());
                vote_entry.add_signature(signature.to_bytes());

                let add_vote = vote_machine.add_vote(vote_entry);

                assert!(add_vote.is_ok());
            });

        let tally = vote_machine.tally();

        assert_eq!((4, 6), tally.compile());
        assert_eq!(VoteOutcome::Rejected, tally.outcome());
    }
}
