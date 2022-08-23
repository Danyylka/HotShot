//! Error type for `HotShot`
//!
//! This module provides [`HotShotError`], which is an enum representing possible faults that can
//! occur while interacting with this crate.

use crate::{data::ViewNumber, traits::storage::StorageError};
use async_std::future::TimeoutError;
use snafu::Snafu;

/// Error type for `HotShot`
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum HotShotError {
    /// Failed to Message the leader in the given stage
    #[snafu(display("Failed to message leader with error: {source}"))]
    FailedToMessageLeader {
        /// The underlying network fault
        source: crate::traits::network::NetworkError,
    },
    /// Failed to broadcast a message on the network
    #[snafu(display("Failed to broadcast a message"))]
    FailedToBroadcast {
        /// The underlying network fault
        source: crate::traits::network::NetworkError,
    },
    /// Bad or forged quorum certificate
    #[snafu(display("Bad or forged QC"))]
    BadOrForgedQC {
        /// The bad quorum certificate
        bad_qc: crate::data::VecQuorumCertificate,
    },
    /// A block failed verification
    #[snafu(display("Failed verification of block"))]
    BadBlock {},
    /// A block was not consistent with the existing state
    #[snafu(display("Inconsistent block"))]
    InconsistentBlock {},
    /// Failure in networking layer
    #[snafu(display("Failure in networking layer: {source}"))]
    NetworkFault {
        /// Underlying network fault
        source: crate::traits::network::NetworkError,
    },
    /// Item was not present in storage
    ItemNotFound {
        /// Name of the hash type that was not found.
        /// Can be easily obtained with `std::any::type_name::<T>()`
        type_name: &'static str,
        /// Hash of the missing item
        hash: Vec<u8>,
    },
    /// Error accesing storage
    StorageError {
        /// Underlying error
        source: StorageError,
    },
    /// Invalid state machine state
    #[snafu(display("Invalid state machine state: {}", context))]
    InvalidState {
        /// Context
        context: String,
    },
    /// HotShot timed out waiting for msgs
    TimeoutError {
        /// source of error
        source: TimeoutError,
    },
    /// HotShot timed out during round
    ViewTimeoutError {
        /// view number
        view_number: ViewNumber,
        /// The state that the round was in when it timed out
        state: RoundTimedoutState,
    },
    /// Not enough valid signatures for a quorum
    #[snafu(display("Insufficient number of valid signatures: the threshold is {}, but only {} signatures were valid", threshold, num_valid_signatures))]
    InsufficientValidSignatures {
        /// Number of valid signatures
        num_valid_signatures: usize,
        /// Threshold of signatures needed for a quorum
        threshold: usize,
    },
    /// Miscelaneous error
    /// TODO fix this with
    /// #181 <https://github.com/EspressoSystems/HotShot/issues/181>
    Misc {
        /// source of error
        context: String,
    },
    /// Internal value used to drive the state machine
    Continue,
}

/// Contains information about what the state of the hotshot-consensus was when a round timed out
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RoundTimedoutState {
    /// Leader is in a Prepare phase and is waiting for a HighQC
    LeaderWaitingForHighQC,
    /// Leader is in a Prepare phase and timed out before the round min time is reached
    LeaderMinRoundTimeNotReached,
    /// Leader is waiting for prepare votes
    LeaderWaitingForPrepareVotes,
    /// Leader is waiting for precommit votes
    LeaderWaitingForPreCommitVotes,
    /// Leader is waiting for commit votes
    LeaderWaitingForCommitVotes,

    /// Replica is waiting for a prepare message
    ReplicaWaitingForPrepare,
    /// Replica is waiting for a pre-commit message
    ReplicaWaitingForPreCommit,
    /// Replica is waiting for a commit message
    ReplicaWaitingForCommit,
    /// Replica is waiting for a decide message
    ReplicaWaitingForDecide,

    /// HotShot-testing tried to collect round events, but it timed out
    TestCollectRoundEventsTimedOut,
}
