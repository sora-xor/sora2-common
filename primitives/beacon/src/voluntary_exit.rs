use crate::{ChainSpec, Domain, Epoch, Fork, Hash256, SecretKey, SignedRoot, SignedVoluntaryExit};

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// An exit voluntarily submitted a validator who wishes to withdraw.
///
/// Spec v0.12.1
#[derive(
    Debug,
    PartialEq,
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
pub struct VoluntaryExit {
    /// Earliest epoch when voluntary exit can be processed.
    pub epoch: Epoch,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub validator_index: u64,
}

impl SignedRoot for VoluntaryExit {}

impl VoluntaryExit {
    pub fn sign(
        self,
        secret_key: &SecretKey,
        fork: &Fork,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> SignedVoluntaryExit {
        let domain = spec.get_domain(
            self.epoch,
            Domain::VoluntaryExit,
            fork,
            genesis_validators_root,
        );
        let message = self.signing_root(domain);
        SignedVoluntaryExit {
            message: self,
            signature: secret_key.sign(message),
        }
    }
}
