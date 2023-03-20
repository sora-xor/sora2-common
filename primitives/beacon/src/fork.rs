use crate::Epoch;

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

/// Specifies a fork of the `BeaconChain`, to prevent replay attacks.
///
/// Spec v0.12.1
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
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
pub struct Fork {
    #[serde(with = "eth2_serde_utils::bytes_4_hex")]
    pub previous_version: [u8; 4],
    #[serde(with = "eth2_serde_utils::bytes_4_hex")]
    pub current_version: [u8; 4],
    pub epoch: Epoch,
}

impl Fork {
    /// Return the fork version of the given ``epoch``.
    ///
    /// Spec v0.12.1
    pub fn get_fork_version(&self, epoch: Epoch) -> [u8; 4] {
        if epoch < self.epoch {
            return self.previous_version;
        }
        self.current_version
    }
}
