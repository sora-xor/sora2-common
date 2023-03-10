use crate::prelude::*;
use crate::*;
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

/// Can be used to key (ID) the shuffling in some chain, in some epoch.
///
/// ## Reasoning
///
/// We say that the ID of some shuffling is always equal to a 2-tuple:
///
/// - The epoch for which the shuffling should be effective.
/// - A block root, where this is the root at the *last* slot of the penultimate epoch. I.e., the
/// final block which contributed a randao reveal to the seed for the shuffling.
///
/// The struct stores exactly that 2-tuple.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct AttestationShufflingId {
    pub shuffling_epoch: Epoch,
    pub shuffling_decision_block: Hash256,
}

impl AttestationShufflingId {
    pub fn from_components(shuffling_epoch: Epoch, shuffling_decision_block: Hash256) -> Self {
        Self {
            shuffling_epoch,
            shuffling_decision_block,
        }
    }
}
