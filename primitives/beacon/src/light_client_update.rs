use super::{BeaconBlockHeader, EthSpec, FixedVector, Hash256, Slot, SyncAggregate, SyncCommittee};
use crate::beacon_state;
use crate::prelude::*;
use crate::safe_arith::ArithError;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::typenum::{U5, U6};

pub const FINALIZED_ROOT_INDEX: usize = 105;
pub const CURRENT_SYNC_COMMITTEE_INDEX: usize = 54;
pub const NEXT_SYNC_COMMITTEE_INDEX: usize = 55;

pub type FinalizedRootProofLen = U6;
pub type CurrentSyncCommitteeProofLen = U5;
pub type NextSyncCommitteeProofLen = U5;

pub const FINALIZED_ROOT_PROOF_LEN: usize = 6;
pub const CURRENT_SYNC_COMMITTEE_PROOF_LEN: usize = 5;
pub const NEXT_SYNC_COMMITTEE_PROOF_LEN: usize = 5;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    SszTypesError(ssz_types::Error),
    BeaconStateError(beacon_state::Error),
    ArithError(ArithError),
    AltairForkNotActive,
    NotEnoughSyncCommitteeParticipants,
    MismatchingPeriods,
    InvalidFinalizedBlock,
}

impl From<ssz_types::Error> for Error {
    fn from(e: ssz_types::Error) -> Error {
        Error::SszTypesError(e)
    }
}

impl From<beacon_state::Error> for Error {
    fn from(e: beacon_state::Error) -> Error {
        Error::BeaconStateError(e)
    }
}

impl From<ArithError> for Error {
    fn from(e: ArithError) -> Error {
        Error::ArithError(e)
    }
}

/// A LightClientUpdate is the update we request solely to either complete the bootstraping process,
/// or to sync up to the last committee period, we need to have one ready for each ALTAIR period
/// we go over, note: there is no need to keep all of the updates from [ALTAIR_PERIOD, CURRENT_PERIOD].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[serde(bound = "T: EthSpec")]
#[scale_info(skip_type_params(T))]
pub struct LightClientUpdate<T: EthSpec> {
    /// The last `BeaconBlockHeader` from the last attested block by the sync committee.
    pub attested_header: BeaconBlockHeader,
    /// The `SyncCommittee` used in the next period.
    pub next_sync_committee: SyncCommittee<T>,
    /// Merkle proof for next sync committee
    pub next_sync_committee_branch: FixedVector<Hash256, NextSyncCommitteeProofLen>,
    /// The last `BeaconBlockHeader` from the last attested finalized block (end of epoch).
    pub finalized_header: BeaconBlockHeader,
    /// Merkle proof attesting finalized header.
    pub finality_branch: FixedVector<Hash256, FinalizedRootProofLen>,
    /// current sync aggreggate
    pub sync_aggregate: SyncAggregate<T>,
    /// Slot of the sync aggregated singature
    pub signature_slot: Slot,
}
