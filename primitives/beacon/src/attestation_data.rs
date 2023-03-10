use crate::{Checkpoint, Hash256, SignedRoot, Slot};

use crate::prelude::*;
use crate::slot_data::SlotData;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// The data upon which an attestation is based.
///
/// Spec v0.12.1
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Hash,
    Encode,
    Decode,
    TreeHash,
    Default,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct AttestationData {
    pub slot: Slot,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub index: u64,

    // LMD GHOST vote
    pub beacon_block_root: Hash256,

    // FFG Vote
    pub source: Checkpoint,
    pub target: Checkpoint,
}

impl SignedRoot for AttestationData {}

impl SlotData for AttestationData {
    fn get_slot(&self) -> Slot {
        self.slot
    }
}
