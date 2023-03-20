use super::Hash256;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// Contains data obtained from the Eth1 chain.
///
/// Spec v0.12.1
#[derive(
    Debug,
    PartialEq,
    Clone,
    Default,
    Eq,
    Hash,
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
pub struct Eth1Data {
    pub deposit_root: Hash256,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub deposit_count: u64,
    pub block_hash: Hash256,
}
