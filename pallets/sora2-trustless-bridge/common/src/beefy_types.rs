use crate::bitfield::BitField;
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use sp_core::H160;

pub type EthAddress = H160;

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct Commitment {
    pub payload_prefix: Vec<u8>,
    pub payload: [u8; 32],
    pub payload_suffix: Vec<u8>,
    pub block_number: u32,
    pub validator_set_id: u64,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo)]
pub struct ValidatorProof {
    pub validator_claims_bitfield: BitField,
    pub signatures: Vec<Vec<u8>>,
    pub positions: Vec<u128>,
    pub public_keys: Vec<EthAddress>,
    pub public_key_merkle_proofs: Vec<Vec<[u8; 32]>>,
}

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct BeefyMMRLeaf {
    pub version: u8,
    pub parent_number: u32,
    pub next_authority_set_id: u64,
    pub next_authority_set_len: u32,
    pub parent_hash: [u8; 32],
    pub next_authority_set_root: [u8; 32],
    pub random_seed: [u8; 32],
    pub digest_hash: [u8; 32],
}

#[derive(
    Encode,
    Decode,
    Clone,
    RuntimeDebug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    scale_info::TypeInfo,
    Default,
)]
pub struct ValidatorSet {
    pub id: u128,
    pub length: u128,
    pub root: [u8; 32],
}
