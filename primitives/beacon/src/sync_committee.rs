use crate::prelude::*;
use crate::safe_arith::{ArithError, SafeArith};
use crate::typenum::Unsigned;
use crate::{EthSpec, FixedVector};
use bls::PublicKeyBytes;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArithError(ArithError),
    InvalidSubcommitteeRange {
        start_subcommittee_index: usize,
        end_subcommittee_index: usize,
        subcommittee_index: usize,
    },
}

impl From<ArithError> for Error {
    fn from(e: ArithError) -> Error {
        Error::ArithError(e)
    }
}

#[derive(
    Debug,
    PartialEq,
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
#[serde(bound = "T: EthSpec")]
#[scale_info(skip_type_params(T))]
pub struct SyncCommittee<T: EthSpec> {
    pub pubkeys: FixedVector<PublicKeyBytes, T::SyncCommitteeSize>,
    pub aggregate_pubkey: PublicKeyBytes,
}

impl<T: EthSpec> SyncCommittee<T> {
    /// Create a temporary sync committee that should *never* be included in a legitimate consensus object.
    pub fn temporary() -> Result<Self, ssz_types::Error> {
        Ok(Self {
            pubkeys: FixedVector::new(vec![
                PublicKeyBytes::empty();
                T::SyncCommitteeSize::to_usize()
            ])?,
            aggregate_pubkey: PublicKeyBytes::empty(),
        })
    }

    /// Return the pubkeys in this `SyncCommittee` for the given `subcommittee_index`.
    pub fn get_subcommittee_pubkeys(
        &self,
        subcommittee_index: usize,
    ) -> Result<Vec<PublicKeyBytes>, Error> {
        let start_subcommittee_index = subcommittee_index.safe_mul(T::sync_subcommittee_size())?;
        let end_subcommittee_index =
            start_subcommittee_index.safe_add(T::sync_subcommittee_size())?;
        self.pubkeys
            .get(start_subcommittee_index..end_subcommittee_index)
            .ok_or(Error::InvalidSubcommitteeRange {
                start_subcommittee_index,
                end_subcommittee_index,
                subcommittee_index,
            })
            .map(|s| s.to_vec())
    }

    /// Returns `true` if the pubkey exists in the `SyncCommittee`.
    pub fn contains(&self, pubkey: &PublicKeyBytes) -> bool {
        self.pubkeys.contains(pubkey)
    }
}
