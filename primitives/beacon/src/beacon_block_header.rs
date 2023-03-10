use crate::*;

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

/// A header of a `BeaconBlock`.
///
/// Spec v0.12.1
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
pub struct BeaconBlockHeader {
    pub slot: Slot,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body_root: Hash256,
}

impl SignedRoot for BeaconBlockHeader {}

impl BeaconBlockHeader {
    /// Returns the `tree_hash_root` of the header.
    ///
    /// Spec v0.12.1
    pub fn canonical_root(&self) -> Hash256 {
        Hash256::from_slice(&self.tree_hash_root()[..])
    }

    /// Signs `self`, producing a `SignedBeaconBlockHeader`.
    pub fn sign<E: EthSpec>(
        self,
        secret_key: &SecretKey,
        fork: &Fork,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> SignedBeaconBlockHeader {
        let epoch = self.slot.epoch(E::slots_per_epoch());
        let domain = spec.get_domain(epoch, Domain::BeaconProposer, fork, genesis_validators_root);
        let message = self.signing_root(domain);
        let signature = secret_key.sign(message);
        SignedBeaconBlockHeader {
            message: self,
            signature,
        }
    }
}
