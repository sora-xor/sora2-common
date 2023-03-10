//! Identifies each sync committee subnet by an integer identifier.
use crate::consts::altair::SYNC_COMMITTEE_SUBNET_COUNT;
use crate::prelude::*;
use crate::safe_arith::{ArithError, SafeArith};
use crate::EthSpec;
use core::fmt::{self, Display};
use core::ops::{Deref, DerefMut};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use ssz_types::typenum::Unsigned;

lazy_static! {
    static ref SYNC_SUBNET_ID_TO_STRING: Vec<String> = {
        let mut v = Vec::with_capacity(SYNC_COMMITTEE_SUBNET_COUNT as usize);

        for i in 0..SYNC_COMMITTEE_SUBNET_COUNT {
            v.push(i.to_string());
        }
        v
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(transparent)]
pub struct SyncSubnetId(#[serde(with = "eth2_serde_utils::quoted_u64")] u64);

pub fn sync_subnet_id_to_string(i: u64) -> &'static str {
    if i < SYNC_COMMITTEE_SUBNET_COUNT {
        SYNC_SUBNET_ID_TO_STRING
            .get(i as usize)
            .expect("index below SYNC_COMMITTEE_SUBNET_COUNT")
    } else {
        "sync subnet id out of range"
    }
}

impl SyncSubnetId {
    pub fn new(id: u64) -> Self {
        id.into()
    }

    /// Compute required subnets to subscribe to given the sync committee indices.
    pub fn compute_subnets_for_sync_committee<T: EthSpec>(
        sync_committee_indices: &[u64],
    ) -> Result<BTreeSet<Self>, ArithError> {
        let subcommittee_size = T::SyncSubcommitteeSize::to_u64();

        sync_committee_indices
            .iter()
            .map(|index| index.safe_div(subcommittee_size).map(Self::new))
            .collect()
    }
}

impl Display for SyncSubnetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Deref for SyncSubnetId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SyncSubnetId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u64> for SyncSubnetId {
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl Into<u64> for SyncSubnetId {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<u64> for &SyncSubnetId {
    fn into(self) -> u64 {
        self.0
    }
}

impl AsRef<str> for SyncSubnetId {
    fn as_ref(&self) -> &str {
        sync_subnet_id_to_string(self.0)
    }
}
