use crate::prelude::*;
use crate::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Hash,
    Clone,
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
pub struct Withdrawal {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub index: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub validator_index: u64,
    pub address: Address,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub amount: u64,
}
