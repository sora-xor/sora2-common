use crate::prelude::*;
use crate::{AggregateSignature, AttestationData, EthSpec, VariableList};
use core::hash::{Hash, Hasher};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz::Encode;
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// Details an attestation that can be slashable.
///
/// To be included in an `AttesterSlashing`.
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
#[derivative(PartialEq, Eq)] // to satisfy Clippy's lint about `Hash`
#[serde(bound = "T: EthSpec")]
#[scale_info(skip_type_params(T))]
pub struct IndexedAttestation<T: EthSpec> {
    /// Lists validator registry indices, not committee indices.
    #[serde(with = "quoted_variable_list_u64")]
    pub attesting_indices: VariableList<u64, T::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub signature: AggregateSignature,
}

impl<T: EthSpec> IndexedAttestation<T> {
    /// Check if ``attestation_data_1`` and ``attestation_data_2`` have the same target.
    ///
    /// Spec v0.12.1
    pub fn is_double_vote(&self, other: &Self) -> bool {
        self.data.target.epoch == other.data.target.epoch && self.data != other.data
    }

    /// Check if ``attestation_data_1`` surrounds ``attestation_data_2``.
    ///
    /// Spec v0.12.1
    pub fn is_surround_vote(&self, other: &Self) -> bool {
        self.data.source.epoch < other.data.source.epoch
            && other.data.target.epoch < self.data.target.epoch
    }
}

/// Implementation of non-crypto-secure `Hash`, for use with `HashMap` and `HashSet`.
///
/// Guarantees `att1 == att2 -> hash(att1) == hash(att2)`.
///
/// Used in the operation pool.
impl<T: EthSpec> Hash for IndexedAttestation<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attesting_indices.hash(state);
        self.data.hash(state);
        self.signature.as_ssz_bytes().hash(state);
    }
}

/// Serialize a variable list of `u64` such that each int is quoted. Deserialize a variable
/// list supporting both quoted and un-quoted ints.
///
/// E.g.,`["0", "1", "2"]`
mod quoted_variable_list_u64 {
    use super::*;
    use crate::Unsigned;
    use eth2_serde_utils::quoted_u64_vec::{QuotedIntVecVisitor, QuotedIntWrapper};
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S, T>(value: &VariableList<u64, T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Unsigned,
    {
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for &int in value.iter() {
            seq.serialize_element(&QuotedIntWrapper { int })?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<VariableList<u64, T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Unsigned,
    {
        deserializer
            .deserialize_any(QuotedIntVecVisitor)
            .and_then(|vec| {
                VariableList::new(vec)
                    .map_err(|e| serde::de::Error::custom(format!("invalid length: {:?}", e)))
            })
    }
}
