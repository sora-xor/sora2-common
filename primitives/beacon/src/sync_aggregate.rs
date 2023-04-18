use crate::prelude::*;
use crate::safe_arith::ArithError;
use crate::{AggregateSignature, BitVector, EthSpec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq)]
pub enum Error {
    SszTypesError(ssz_types::Error),
    ArithError(ArithError),
}

impl From<ArithError> for Error {
    fn from(e: ArithError) -> Error {
        Error::ArithError(e)
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    Derivative,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[derivative(PartialEq, Hash(bound = "T: EthSpec"))]
#[serde(bound = "T: EthSpec")]
#[scale_info(skip_type_params(T))]
pub struct SyncAggregate<T: EthSpec> {
    pub sync_committee_bits: BitVector<T::SyncCommitteeSize>,
    pub sync_committee_signature: AggregateSignature,
}

impl<T: EthSpec> SyncAggregate<T> {
    /// New aggregate to be used as the seed for aggregating other signatures.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            sync_committee_bits: BitVector::default(),
            sync_committee_signature: AggregateSignature::infinity(),
        }
    }

    /// Empty aggregate to be used at genesis.
    ///
    /// Contains an empty signature and should *not* be used as the starting point for aggregation,
    /// use `new` instead.
    pub fn empty() -> Self {
        Self {
            sync_committee_bits: BitVector::default(),
            sync_committee_signature: AggregateSignature::empty(),
        }
    }

    /// Returns how many bits are `true` in `self.sync_committee_bits`.
    pub fn num_set_bits(&self) -> usize {
        self.sync_committee_bits.num_set_bits()
    }
}
