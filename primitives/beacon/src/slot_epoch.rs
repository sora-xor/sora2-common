//! The `Slot` and `Epoch` types are defined as new types over u64 to enforce type-safety between
//! the two types.
//!
//! `Slot` and `Epoch` have implementations which permit conversion, comparison and math operations
//! between each and `u64`, however specifically not between each other.
//!
//! All math operations on `Slot` and `Epoch` are saturating, they never wrap.
//!
//! It would be easy to define `PartialOrd` and other traits generically across all types which
//! implement `Into<u64>`, however this would allow operations between `Slots` and `Epochs` which
//! may lead to programming errors which are not detected by the compiler.

use crate::{EthSpec, SignedRoot};

use crate::prelude::*;
use crate::safe_arith::{ArithError, SafeArith};
use core::fmt;
use core::hash::Hash;
use core::iter::Iterator;
use serde::{Deserialize, Serialize};
use ssz::{Decode, DecodeError, Encode};
use ssz_types::typenum::Unsigned;

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[serde(transparent)]
pub struct Slot(#[serde(with = "eth2_serde_utils::quoted_u64")] u64);

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[serde(transparent)]
pub struct Epoch(#[serde(with = "eth2_serde_utils::quoted_u64")] u64);

impl_common!(Slot);
impl_common!(Epoch);

impl Slot {
    pub const fn new(slot: u64) -> Slot {
        Slot(slot)
    }

    pub fn epoch(self, slots_per_epoch: u64) -> Epoch {
        Epoch::new(self.0)
            .safe_div(slots_per_epoch)
            .expect("slots_per_epoch is not 0")
    }

    pub fn epoch_with_spec<T: EthSpec>(self) -> Epoch {
        self.epoch(T::slots_per_epoch())
    }

    pub fn max_value() -> Slot {
        Slot(u64::max_value())
    }
}

impl Epoch {
    pub const fn new(slot: u64) -> Epoch {
        Epoch(slot)
    }

    pub fn max_value() -> Epoch {
        Epoch(u64::max_value())
    }

    /// The first slot in the epoch.
    pub fn start_slot(self, slots_per_epoch: u64) -> Slot {
        Slot::from(self.0.saturating_mul(slots_per_epoch))
    }

    /// The last slot in the epoch.
    pub fn end_slot(self, slots_per_epoch: u64) -> Slot {
        Slot::from(
            self.0
                .saturating_mul(slots_per_epoch)
                .saturating_add(slots_per_epoch.saturating_sub(1)),
        )
    }

    /// Position of some slot inside an epoch, if any.
    ///
    /// E.g., the first `slot` in `epoch` is at position `0`.
    pub fn position(self, slot: Slot, slots_per_epoch: u64) -> Option<usize> {
        let start = self.start_slot(slots_per_epoch);
        let end = self.end_slot(slots_per_epoch);

        if slot >= start && slot <= end {
            slot.as_usize().checked_sub(start.as_usize())
        } else {
            None
        }
    }

    /// Compute the sync committee period for an epoch.
    pub fn sync_committee_period(
        &self,
        epochs_per_sync_committee_period: Epoch,
    ) -> Result<u64, ArithError> {
        Ok(self.safe_div(epochs_per_sync_committee_period)?.as_u64())
    }

    /// Compute the sync committee period for an epoch.
    pub fn sync_committee_period_with_spec<T: EthSpec>(&self) -> Result<u64, ArithError> {
        Ok(self
            .safe_div(Epoch::new(T::EpochsPerSyncCommitteePeriod::to_u64()))?
            .as_u64())
    }

    pub fn slot_iter(&self, slots_per_epoch: u64) -> SlotIter {
        SlotIter {
            current_iteration: 0,
            epoch: self,
            slots_per_epoch,
        }
    }
}

pub struct SlotIter<'a> {
    current_iteration: u64,
    epoch: &'a Epoch,
    slots_per_epoch: u64,
}

impl<'a> Iterator for SlotIter<'a> {
    type Item = Slot;

    fn next(&mut self) -> Option<Slot> {
        if self.current_iteration >= self.slots_per_epoch {
            None
        } else {
            let start_slot = self.epoch.start_slot(self.slots_per_epoch);
            let previous = self.current_iteration;
            self.current_iteration = self.current_iteration.checked_add(1)?;
            start_slot.safe_add(previous).ok()
        }
    }
}
