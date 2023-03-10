use super::{BeaconBlockHeader, EthSpec, Slot, SyncAggregate};
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

/// A LightClientOptimisticUpdate is the update we send on each slot,
/// it is based off the current unfinalized epoch is verified only against BLS signature.
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
pub struct LightClientOptimisticUpdate<T: EthSpec> {
    /// The last `BeaconBlockHeader` from the last attested block by the sync committee.
    pub attested_header: BeaconBlockHeader,
    /// current sync aggreggate
    pub sync_aggregate: SyncAggregate<T>,
    /// Slot of the sync aggregated singature
    pub signature_slot: Slot,
}
