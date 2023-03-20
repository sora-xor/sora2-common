use crate::{SignedRoot, Slot};

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(
    Debug,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Hash,
    Encode,
    Decode,
    TreeHash,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct SyncAggregatorSelectionData {
    pub slot: Slot,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub subcommittee_index: u64,
}

impl SignedRoot for SyncAggregatorSelectionData {}
