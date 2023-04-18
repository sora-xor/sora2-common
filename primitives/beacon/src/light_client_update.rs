use super::{EthSpec, FixedVector, Hash256, Slot, SyncAggregate, SyncCommittee};
use crate::beacon_state;
use crate::light_client_header::LightClientHeaderRef;
use crate::prelude::*;
use crate::safe_arith::ArithError;
use crate::LightClientHeaderCapella;
use crate::LightClientHeaderMerge;
use serde::{Deserialize, Serialize};
use ssz_types::typenum::U4;
use ssz_types::typenum::{U5, U6};

pub const FINALIZED_ROOT_INDEX: usize = 41;
pub const CURRENT_SYNC_COMMITTEE_INDEX: usize = 22;
pub const NEXT_SYNC_COMMITTEE_INDEX: usize = 23;
pub const EXECUTION_PAYLOAD_INDEX: usize = 9;

pub type FinalizedRootProofLen = U6;
pub type CurrentSyncCommitteeProofLen = U5;
pub type NextSyncCommitteeProofLen = U5;
pub type ExecutionPayloadProofLen = U4;

pub const FINALIZED_ROOT_PROOF_LEN: usize = 6;
pub const CURRENT_SYNC_COMMITTEE_PROOF_LEN: usize = 5;
pub const NEXT_SYNC_COMMITTEE_PROOF_LEN: usize = 5;
pub const EXECUTION_PAYLOAD_PROOF_LEN: usize = 4;

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
#[superstruct(
    variants(Merge, Capella),
    variant_attributes(
        derive(
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Derivative,
            ScaleEncode,
            ScaleDecode,
            TypeInfo,
            MaxEncodedLen,
        ),
        derivative(PartialEq),
        serde(bound = "T: EthSpec", deny_unknown_fields),
        scale_info(skip_type_params(T))
    ),
    ref_attributes(derive(PartialEq))
)]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Derivative,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[derivative(PartialEq)]
#[serde(bound = "T: EthSpec", untagged)]
#[scale_info(skip_type_params(T))]
pub struct LightClientUpdate<T: EthSpec> {
    /// The last `BeaconBlockHeader` from the last attested block by the sync committee.
    #[superstruct(only(Merge), partial_getter(rename = "attested_header_merge"))]
    pub attested_header: LightClientHeaderMerge<T>,
    #[superstruct(only(Capella), partial_getter(rename = "attested_header_capella"))]
    pub attested_header: LightClientHeaderCapella<T>,
    /// The `SyncCommittee` used in the next period.
    #[serde(default)]
    pub next_sync_committee: Option<SyncCommittee<T>>,
    /// Merkle proof for next sync committee
    #[serde(default)]
    pub next_sync_committee_branch: Option<FixedVector<Hash256, NextSyncCommitteeProofLen>>,
    /// The last `BeaconBlockHeader` from the last attested finalized block (end of epoch).
    #[superstruct(only(Merge), partial_getter(rename = "finalized_header_merge"))]
    #[serde(default)]
    pub finalized_header: Option<LightClientHeaderMerge<T>>,
    #[superstruct(only(Capella), partial_getter(rename = "finalized_header_capella"))]
    #[serde(default)]
    pub finalized_header: Option<LightClientHeaderCapella<T>>,
    /// Merkle proof attesting finalized header.
    #[serde(default)]
    pub finality_branch: Option<FixedVector<Hash256, FinalizedRootProofLen>>,
    /// current sync aggreggate
    pub sync_aggregate: SyncAggregate<T>,
    /// Slot of the sync aggregated singature
    #[superstruct(getter(copy))]
    pub signature_slot: Slot,
}

impl<T: EthSpec> LightClientUpdate<T> {
    pub fn is_sync_committee_update(&self) -> bool {
        self.next_sync_committee().is_some() && self.next_sync_committee_branch().is_some()
    }

    pub fn is_finality_update(&self) -> bool {
        self.finalized_header().is_some() && self.finality_branch().is_some()
    }

    pub fn finalized_header(&self) -> Option<LightClientHeaderRef<T>> {
        match self {
            Self::Merge(update) => update
                .finalized_header
                .as_ref()
                .map(|x| LightClientHeaderRef::Merge(x)),
            Self::Capella(update) => update
                .finalized_header
                .as_ref()
                .map(|x| LightClientHeaderRef::Capella(x)),
        }
    }

    pub fn attested_header(&self) -> LightClientHeaderRef<T> {
        match self {
            Self::Merge(update) => LightClientHeaderRef::Merge(&update.attested_header),
            Self::Capella(update) => LightClientHeaderRef::Capella(&update.attested_header),
        }
    }
}

#[cfg(feature = "std")]
impl<T: EthSpec> crate::ForkVersionDeserialize for LightClientUpdate<T> {
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: crate::ForkName,
    ) -> Result<Self, D::Error> {
        let convert_err = |e| {
            serde::de::Error::custom(format!(
                "ExecutionPayloadHeader failed to deserialize: {:?}",
                e
            ))
        };

        Ok(match fork_name {
            crate::ForkName::Merge => {
                Self::Merge(serde_json::from_value(value).map_err(convert_err)?)
            }
            crate::ForkName::Capella => {
                Self::Capella(serde_json::from_value(value).map_err(convert_err)?)
            }
            crate::ForkName::Base | crate::ForkName::Altair => {
                return Err(serde::de::Error::custom(format!(
                    "ExecutionPayloadHeader failed to deserialize: unsupported fork '{}'",
                    fork_name
                )));
            }
        })
    }
}
