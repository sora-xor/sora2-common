use crate::consts::altair::{
    SYNC_COMMITTEE_SUBNET_COUNT, TARGET_AGGREGATORS_PER_SYNC_SUBCOMMITTEE,
};
#[cfg(not(feature = "std"))]
use crate::prelude::*;
use crate::safe_arith::{ArithError, SafeArith};
use crate::{
    ChainSpec, Domain, EthSpec, Fork, Hash256, PublicKey, SecretKey, Signature, SignedRoot, Slot,
    SyncAggregatorSelectionData,
};
use core::cmp;
use core::convert::TryInto;
use eth2_hashing::hash;
use ssz::Encode;
use ssz_types::typenum::Unsigned;

#[derive(PartialEq, Debug, Clone)]
pub struct SyncSelectionProof(Signature);

impl SyncSelectionProof {
    pub fn new<T: EthSpec>(
        slot: Slot,
        subcommittee_index: u64,
        secret_key: &SecretKey,
        fork: &Fork,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> Self {
        let domain = spec.get_domain(
            slot.epoch(T::slots_per_epoch()),
            Domain::SyncCommitteeSelectionProof,
            fork,
            genesis_validators_root,
        );
        let message = SyncAggregatorSelectionData {
            slot,
            subcommittee_index,
        }
        .signing_root(domain);

        Self(secret_key.sign(message))
    }

    /// Returns the "modulo" used for determining if a `SyncSelectionProof` elects an aggregator.
    pub fn modulo<T: EthSpec>() -> Result<u64, ArithError> {
        Ok(cmp::max(
            1,
            (T::SyncCommitteeSize::to_u64())
                .safe_div(SYNC_COMMITTEE_SUBNET_COUNT)?
                .safe_div(TARGET_AGGREGATORS_PER_SYNC_SUBCOMMITTEE)?,
        ))
    }

    pub fn is_aggregator<T: EthSpec>(&self) -> Result<bool, ArithError> {
        self.is_aggregator_from_modulo(Self::modulo::<T>()?)
    }

    pub fn is_aggregator_from_modulo(&self, modulo: u64) -> Result<bool, ArithError> {
        let signature_hash = hash(&self.0.as_ssz_bytes());
        let signature_hash_int = u64::from_le_bytes(
            signature_hash
                .get(0..8)
                .expect("hash is 32 bytes")
                .try_into()
                .expect("first 8 bytes of signature should always convert to fixed array"),
        );

        signature_hash_int.safe_rem(modulo).map(|rem| rem == 0)
    }

    pub fn verify<T: EthSpec>(
        &self,
        slot: Slot,
        subcommittee_index: u64,
        pubkey: &PublicKey,
        fork: &Fork,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> bool {
        let domain = spec.get_domain(
            slot.epoch(T::slots_per_epoch()),
            Domain::SyncCommitteeSelectionProof,
            fork,
            genesis_validators_root,
        );
        let message = SyncAggregatorSelectionData {
            slot,
            subcommittee_index,
        }
        .signing_root(domain);

        self.0.verify(pubkey, message)
    }
}

impl Into<Signature> for SyncSelectionProof {
    fn into(self) -> Signature {
        self.0
    }
}

impl From<Signature> for SyncSelectionProof {
    fn from(sig: Signature) -> Self {
        Self(sig)
    }
}
