use crate::{AttestationData, BitList, EthSpec};

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// An attestation that has been included in the state but not yet fully processed.
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
pub struct PendingAttestation<T: EthSpec> {
    pub aggregation_bits: BitList<T::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub inclusion_delay: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub proposer_index: u64,
}
