use crate::*;

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::FixedVector;
use tree_hash_derive::TreeHash;

/// Historical block and state roots.
///
/// Spec v0.12.1
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct HistoricalBatch<T: EthSpec> {
    pub block_roots: FixedVector<Hash256, T::SlotsPerHistoricalRoot>,
    pub state_roots: FixedVector<Hash256, T::SlotsPerHistoricalRoot>,
}
