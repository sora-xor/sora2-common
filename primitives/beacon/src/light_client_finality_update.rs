use super::{BeaconBlockHeader, EthSpec, FixedVector, Hash256, Slot, SyncAggregate};
use crate::light_client_update::*;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

/// A LightClientFinalityUpdate is the update lightclient request or received by a gossip that
/// signal a new finalized beacon block header for the light client sync protocol.
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
pub struct LightClientFinalityUpdate<T: EthSpec> {
    /// The last `BeaconBlockHeader` from the last attested block by the sync committee.
    pub attested_header: BeaconBlockHeader,
    /// The last `BeaconBlockHeader` from the last attested finalized block (end of epoch).
    pub finalized_header: BeaconBlockHeader,
    /// Merkle proof attesting finalized header.
    pub finality_branch: FixedVector<Hash256, FinalizedRootProofLen>,
    /// current sync aggreggate
    pub sync_aggregate: SyncAggregate<T>,
    /// Slot of the sync aggregated singature
    pub signature_slot: Slot,
}
