#[cfg(not(feature = "std"))]
use crate::prelude::*;
use crate::safe_arith::ArithError;
use crate::*;

use core::fmt;
use core::hash::Hash;

// #[macro_use]
// mod committee_cache;
// mod clone_config;
// mod exit_cache;
// mod iter;
// mod pubkey_cache;
// mod tests;
// mod tree_hash_cache;

pub const CACHED_EPOCHS: usize = 3;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// A state for a different hard-fork was required -- a severe logic error.
    IncorrectStateVariant,
    EpochOutOfBounds,
    SlotOutOfBounds,
    UnknownValidator(usize),
    UnableToDetermineProducer,
    InvalidBitfield,
    ValidatorIsWithdrawable,
    ValidatorIsInactive {
        val_index: usize,
    },
    UnableToShuffle,
    ShuffleIndexOutOfBounds(usize),
    IsAggregatorOutOfBounds,
    BlockRootsOutOfBounds(usize),
    StateRootsOutOfBounds(usize),
    SlashingsOutOfBounds(usize),
    BalancesOutOfBounds(usize),
    RandaoMixesOutOfBounds(usize),
    CommitteeCachesOutOfBounds(usize),
    ParticipationOutOfBounds(usize),
    InactivityScoresOutOfBounds(usize),
    TooManyValidators,
    InsufficientValidators,
    InsufficientRandaoMixes,
    InsufficientBlockRoots,
    InsufficientIndexRoots,
    InsufficientAttestations,
    InsufficientCommittees,
    InsufficientStateRoots,
    NoCommittee {
        slot: Slot,
        index: CommitteeIndex,
    },
    ZeroSlotsPerEpoch,
    PubkeyCacheInconsistent,
    PubkeyCacheIncomplete {
        cache_len: usize,
        registry_len: usize,
    },
    PreviousCommitteeCacheUninitialized,
    CurrentCommitteeCacheUninitialized,
    TotalActiveBalanceCacheUninitialized,
    TotalActiveBalanceCacheInconsistent {
        initialized_epoch: Epoch,
        current_epoch: Epoch,
    },
    ExitCacheUninitialized,
    SyncCommitteeCacheUninitialized,
    BlsError(bls::Error),
    SszTypesError(ssz_types::Error),
    TreeHashCacheNotInitialized,
    NonLinearTreeHashCacheHistory,
    TreeHashCacheSkippedSlot {
        cache: Slot,
        state: Slot,
    },
    TreeHashError(tree_hash::Error),
    InvalidValidatorPubkey(ssz::DecodeError),
    ValidatorRegistryShrunk,
    TreeHashCacheInconsistent,
    InvalidDepositState {
        deposit_count: u64,
        deposit_index: u64,
    },
    /// Attestation slipped through block processing with a non-matching source.
    IncorrectAttestationSource,
    /// An arithmetic operation occurred which would have overflowed or divided by 0.
    ///
    /// This represents a serious bug in either the spec or Lighthouse!
    ArithError(ArithError),
    MissingBeaconBlock(SignedBeaconBlockHash),
    MissingBeaconState(BeaconStateHash),
    PayloadConversionLogicFlaw,
    SyncCommitteeNotKnown {
        current_epoch: Epoch,
        epoch: Epoch,
    },
    IndexNotSupported(usize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BeaconStateHash(Hash256);

impl fmt::Debug for BeaconStateHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BeaconStateHash({:?})", self.0)
    }
}

impl fmt::Display for BeaconStateHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Hash256> for BeaconStateHash {
    fn from(hash: Hash256) -> BeaconStateHash {
        BeaconStateHash(hash)
    }
}

impl From<BeaconStateHash> for Hash256 {
    fn from(beacon_state_hash: BeaconStateHash) -> Hash256 {
        beacon_state_hash.0
    }
}

impl From<ArithError> for Error {
    fn from(value: ArithError) -> Self {
        Self::ArithError(value)
    }
}
