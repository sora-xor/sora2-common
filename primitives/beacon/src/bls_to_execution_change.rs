use crate::prelude::*;
use crate::*;
use bls::PublicKeyBytes;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

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
pub struct BlsToExecutionChange {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub validator_index: u64,
    pub from_bls_pubkey: PublicKeyBytes,
    pub to_execution_address: Address,
}

impl SignedRoot for BlsToExecutionChange {}

impl BlsToExecutionChange {
    pub fn sign(
        self,
        secret_key: &SecretKey,
        genesis_validators_root: Hash256,
        spec: &ChainSpec,
    ) -> SignedBlsToExecutionChange {
        let domain = spec.compute_domain(
            Domain::BlsToExecutionChange,
            spec.genesis_fork_version,
            genesis_validators_root,
        );
        let message = self.signing_root(domain);
        SignedBlsToExecutionChange {
            message: self,
            signature: secret_key.sign(message),
        }
    }
}
