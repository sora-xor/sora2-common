#[cfg(not(feature = "std"))]
use crate::prelude::*;
use crate::safe_arith::{ArithError, SafeArith};
use crate::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    EpochTooLow { base: Epoch, other: Epoch },
    EpochTooHigh { base: Epoch, other: Epoch },
    ArithError(ArithError),
}

impl From<ArithError> for Error {
    fn from(e: ArithError) -> Self {
        Self::ArithError(e)
    }
}

/// Defines the epochs relative to some epoch. Most useful when referring to the committees prior
/// to and following some epoch.
///
/// Spec v0.12.1
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RelativeEpoch {
    /// The prior epoch.
    Previous,
    /// The current epoch.
    Current,
    /// The next epoch.
    Next,
}

impl RelativeEpoch {
    /// Returns the `epoch` that `self` refers to, with respect to the `base` epoch.
    ///
    /// Spec v0.12.1
    pub fn into_epoch(self, base: Epoch) -> Epoch {
        match self {
            // Due to saturating nature of epoch, check for current first.
            RelativeEpoch::Current => base,
            RelativeEpoch::Previous => base.saturating_sub(1u64),
            RelativeEpoch::Next => base.saturating_add(1u64),
        }
    }

    /// Converts the `other` epoch into a `RelativeEpoch`, with respect to `base`
    ///
    /// ## Errors
    /// Returns an error when:
    /// - `EpochTooLow` when `other` is more than 1 prior to `base`.
    /// - `EpochTooHigh` when `other` is more than 1 after `base`.
    ///
    /// Spec v0.12.1
    pub fn from_epoch(base: Epoch, other: Epoch) -> Result<Self, Error> {
        if other == base {
            Ok(RelativeEpoch::Current)
        } else if other.safe_add(1)? == base {
            Ok(RelativeEpoch::Previous)
        } else if other == base.safe_add(1)? {
            Ok(RelativeEpoch::Next)
        } else if other < base {
            Err(Error::EpochTooLow { base, other })
        } else {
            Err(Error::EpochTooHigh { base, other })
        }
    }

    /// Convenience function for `Self::from_epoch` where both slots are converted into epochs.
    pub fn from_slot(base: Slot, other: Slot, slots_per_epoch: u64) -> Result<Self, Error> {
        Self::from_epoch(base.epoch(slots_per_epoch), other.epoch(slots_per_epoch))
    }
}