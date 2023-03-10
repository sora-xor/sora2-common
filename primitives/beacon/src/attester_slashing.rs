use crate::{EthSpec, IndexedAttestation};

use crate::prelude::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// Two conflicting attestations.
///
/// Spec v0.12.1
#[derive(
    Derivative,
    Debug,
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
#[derivative(PartialEq, Eq, Hash(bound = "T: EthSpec"))]
#[serde(bound = "T: EthSpec")]
#[scale_info(skip_type_params(T))]
pub struct AttesterSlashing<T: EthSpec> {
    pub attestation_1: IndexedAttestation<T>,
    pub attestation_2: IndexedAttestation<T>,
}
