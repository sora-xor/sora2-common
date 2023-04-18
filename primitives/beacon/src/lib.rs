//! Ethereum 2.0 types

#![cfg_attr(not(feature = "std"), no_std)]
// Required for big type-level numbers
#![recursion_limit = "128"]
// Clippy lint set up
#![cfg_attr(
    not(test),
    deny(
        clippy::integer_arithmetic,
        clippy::disallowed_methods,
        clippy::indexing_slicing
    )
)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod prelude {
    pub use alloc::borrow::Cow;
    pub use alloc::collections::{BTreeMap, BTreeSet};
    pub use alloc::format;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::{vec, vec::Vec};
    pub use codec::{Decode as ScaleDecode, Encode as ScaleEncode, MaxEncodedLen};
    pub use core::fmt::Debug;
    pub use core::hash::Hash;
    pub use derivative::Derivative;
    pub use scale_info::TypeInfo;
    pub use ssz_derive::{Decode, Encode};
    pub use superstruct::superstruct;
    pub use tree_hash::TreeHash;
    pub use tree_hash_derive::TreeHash;
}

#[cfg(feature = "std")]
mod prelude {
    pub use codec::{Decode as ScaleDecode, Encode as ScaleEncode, MaxEncodedLen};
    pub use derivative::Derivative;
    pub use scale_info::TypeInfo;
    pub use ssz_derive::{Decode, Encode};
    pub use std::borrow::Cow;
    pub use std::collections::{BTreeMap, BTreeSet};
    pub use superstruct::superstruct;
    pub use tree_hash::TreeHash;
    pub use tree_hash_derive::TreeHash;
}

pub mod attestation;
pub mod attestation_data;
pub mod attester_slashing;
pub mod beacon_block;
pub mod beacon_block_body;
pub mod beacon_block_header;
pub mod beacon_state;
pub mod bls_to_execution_change;
pub mod checkpoint;
pub mod consts;
pub mod deposit;
pub mod deposit_data;
pub mod eth1_data;
pub mod eth_spec;
pub mod execution_block_hash;
pub mod execution_payload;
pub mod execution_payload_header;
pub mod fork;
pub mod fork_data;
pub mod fork_name;
pub mod fork_versioned_response;
pub mod graffiti;
pub mod indexed_attestation;
pub mod light_client_bootstrap;
pub mod light_client_update;
pub mod proposer_slashing;
pub mod signed_beacon_block;
pub mod signed_beacon_block_header;
pub mod signed_bls_to_execution_change;
pub mod signed_voluntary_exit;
pub mod signing_data;
pub mod voluntary_exit;
#[macro_use]
pub mod slot_epoch_macros;
pub mod int_to_bytes;
pub mod payload;
pub mod slot_epoch;
pub mod sync_aggregate;
pub mod sync_committee;
pub mod withdrawal;

pub mod slot_data;

pub mod beacon_config;
pub mod light_client_header;
pub mod safe_arith;

use ethereum_types::{H160, H256};

pub use crate::attestation::{Attestation, Error as AttestationError};
pub use crate::attestation_data::AttestationData;
pub use crate::attester_slashing::AttesterSlashing;
pub use crate::beacon_block::{
    BeaconBlock, BeaconBlockAltair, BeaconBlockBase, BeaconBlockCapella, BeaconBlockMerge,
    BeaconBlockRef, BeaconBlockRefMut, BlindedBeaconBlock,
};
pub use crate::beacon_block_body::{
    BeaconBlockBody, BeaconBlockBodyAltair, BeaconBlockBodyBase, BeaconBlockBodyCapella,
    BeaconBlockBodyMerge, BeaconBlockBodyRef, BeaconBlockBodyRefMut,
};
pub use crate::beacon_block_header::BeaconBlockHeader;
pub use crate::beacon_state::{Error as BeaconStateError, *};
pub use crate::bls_to_execution_change::BlsToExecutionChange;
pub use crate::checkpoint::Checkpoint;
pub use crate::deposit::{Deposit, DEPOSIT_TREE_DEPTH};
pub use crate::deposit_data::DepositData;
pub use crate::eth1_data::Eth1Data;
pub use crate::eth_spec::EthSpecId;
pub use crate::eth_spec::{EthSpec, GnosisEthSpec, MainnetEthSpec, MinimalEthSpec};
pub use crate::execution_block_hash::ExecutionBlockHash;
pub use crate::execution_payload::{
    ExecutionPayload, ExecutionPayloadCapella, ExecutionPayloadMerge, ExecutionPayloadRef,
    Transaction, Transactions, Withdrawals,
};
pub use crate::execution_payload_header::{
    ExecutionPayloadHeader, ExecutionPayloadHeaderCapella, ExecutionPayloadHeaderMerge,
    ExecutionPayloadHeaderRef, ExecutionPayloadHeaderRefMut,
};
pub use crate::fork::Fork;
pub use crate::fork_data::ForkData;
pub use crate::fork_name::{ForkName, InconsistentFork};
#[cfg(feature = "std")]
pub use crate::fork_versioned_response::ForkVersionDeserialize;
pub use crate::fork_versioned_response::{
    ExecutionOptimisticForkVersionedResponse, ForkVersionedResponse,
};
pub use crate::graffiti::{Graffiti, GRAFFITI_BYTES_LEN};
pub use crate::indexed_attestation::IndexedAttestation;
pub use crate::light_client_header::{
    LightClientHeader, LightClientHeaderCapella, LightClientHeaderMerge,
};
pub use crate::payload::{
    AbstractExecPayload, BlindedPayload, BlindedPayloadCapella, BlindedPayloadMerge,
    BlindedPayloadRef, BlockType, ExecPayload, FullPayload, FullPayloadCapella, FullPayloadMerge,
    FullPayloadRef, OwnedExecPayload,
};
pub use crate::proposer_slashing::ProposerSlashing;
pub use crate::signed_beacon_block::{
    SignedBeaconBlock, SignedBeaconBlockAltair, SignedBeaconBlockBase, SignedBeaconBlockCapella,
    SignedBeaconBlockHash, SignedBeaconBlockMerge, SignedBlindedBeaconBlock,
};
pub use crate::signed_beacon_block_header::SignedBeaconBlockHeader;
pub use crate::signed_bls_to_execution_change::SignedBlsToExecutionChange;
pub use crate::signed_voluntary_exit::SignedVoluntaryExit;
pub use crate::signing_data::{SignedRoot, SigningData};
pub use crate::slot_epoch::{Epoch, Slot};
pub use crate::sync_aggregate::SyncAggregate;
pub use crate::sync_committee::SyncCommittee;
pub use crate::voluntary_exit::VoluntaryExit;
pub use crate::withdrawal::Withdrawal;
pub use beacon_config::{ConsensusConfig, ForkInfo, ForkSchedule};

pub type CommitteeIndex = u64;
pub type Hash256 = H256;
pub type Uint256 = ethereum_types::U256;
pub type Address = H160;
pub type ForkVersion = [u8; 4];
pub type BLSFieldElement = Uint256;
// pub type Blob<T> = FixedVector<BLSFieldElement, <T as EthSpec>::FieldElementsPerBlob>;
pub type VersionedHash = Hash256;
pub type Hash64 = ethereum_types::H64;

pub use bls::{
    AggregatePublicKey, AggregateSignature, Keypair, PublicKey, PublicKeyBytes, SecretKey,
    Signature, SignatureBytes,
};
pub use ssz_types::{typenum, typenum::Unsigned, BitList, BitVector, FixedVector, VariableList};
pub use superstruct::superstruct;
