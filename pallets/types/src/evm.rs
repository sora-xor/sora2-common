use crate::MainnetAssetId;
use crate::{H160, H256, U256};
use alloy_core::sol_types::SolValue;
use codec::{Decode, Encode};
use derivative::Derivative;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{Get, RuntimeDebug};
use sp_runtime::traits::Convert;
use sp_runtime::AccountId32;
use sp_runtime::{traits::Hash, BoundedVec};
use sp_std::prelude::*;

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// EVM contract kind
pub enum EVMAppKind {
    /// Used for native token transfers
    EthApp,
    /// Used for ERC20 tokens
    #[cfg_attr(feature = "std", serde(rename = "FaApp"))]
    FAApp,
    /// Legacy HASHI bridge contract
    HashiBridge,
    /// Legacy XOR master contract
    XorMaster,
    /// Legacy VAL master contract
    ValMaster,
}

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EVMAppInfo {
    pub evm_address: H160,
    pub app_kind: EVMAppKind,
}

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    Default,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AdditionalEVMOutboundData {
    pub max_gas: U256,
    pub target: H160,
}

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    Default,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AdditionalEVMInboundData {
    pub source: H160,
}

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Information about ERC-20 asset in EVM network
pub struct EVMAssetInfo {
    /// Thischain asset id
    pub asset_id: MainnetAssetId,
    /// Contract address
    pub evm_address: H160,
    /// Kind of contract
    pub app_kind: EVMAppKind,
    /// Sidechain asset precision
    pub precision: u8,
}

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// HASHI bridge asset info
/// Some data could not be provided by design
pub struct EVMLegacyAssetInfo {
    /// Thischain asset id
    pub asset_id: MainnetAssetId,
    /// Contract address
    pub evm_address: Option<H160>,
    /// Kind of contract
    pub app_kind: EVMAppKind,
    /// Sidechain asset precision
    pub precision: Option<u8>,
}

#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxMessages, MaxPayload))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub enum Commitment<MaxMessages: Get<u32>, MaxPayload: Get<u32>> {
    #[cfg_attr(feature = "std", serde(rename = "outbound"))]
    Outbound(OutboundCommitment<MaxMessages, MaxPayload>),
    #[cfg_attr(feature = "std", serde(rename = "inbound"))]
    Inbound(InboundCommitment<MaxPayload>),
    #[cfg_attr(feature = "std", serde(rename = "statusReport"))]
    StatusReport(StatusReport<MaxPayload>),
    #[cfg_attr(feature = "std", serde(rename = "statusReport"))]
    BaseFeeUpdate(BaseFeeUpdate),
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> Commitment<MaxMessages, MaxPayload> {
    pub fn hash(&self) -> H256 {
        match self {
            Commitment::Inbound(commitment) => commitment.hash(),
            Commitment::Outbound(commitment) => commitment.hash(),
            Commitment::StatusReport(commitment) => commitment.hash(),
            Commitment::BaseFeeUpdate(commitment) => commitment.hash(),
        }
    }

    pub fn nonce(&self) -> u64 {
        match self {
            Commitment::Inbound(commitment) => commitment.nonce,
            Commitment::Outbound(commitment) => commitment.nonce,
            Commitment::StatusReport(commitment) => commitment.nonce,
            Commitment::BaseFeeUpdate(_) => 0,
        }
    }
}

/// Wire-format for committed messages
#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxPayload))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub struct Message<MaxPayload: Get<u32>> {
    /// Target application on the Ethereum side.
    pub target: H160,
    /// Maximum gas this message can use on the Ethereum.
    pub max_gas: U256,
    /// Payload for target application.
    pub payload: BoundedVec<u8, MaxPayload>,
}

/// Wire-format for commitment
#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxMessages, MaxPayload))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub struct OutboundCommitment<MaxMessages: Get<u32>, MaxPayload: Get<u32>> {
    /// A batch nonce for replay protection and ordering.
    pub nonce: u64,
    /// Total maximum gas that can be used by all messages in the commit.
    /// Should be equal to sum of `max_gas`es of `messages`
    pub total_max_gas: U256,
    /// Messages passed through the channel in the current commit.
    pub messages: BoundedVec<Message<MaxPayload>, MaxMessages>,
}

pub use alloy_core::primitives::Address as EvmAddress;
pub use alloy_core::primitives::B256 as EvmB256;
pub use alloy_core::primitives::U256 as EvmU256;

pub struct EvmConverter;

impl Convert<U256, EvmU256> for EvmConverter {
    fn convert(a: U256) -> EvmU256 {
        let mut bytes = [0u8; 32];
        a.to_little_endian(&mut bytes);
        EvmU256::from_le_bytes(bytes)
    }
}

impl Convert<H160, EvmAddress> for EvmConverter {
    fn convert(a: H160) -> EvmAddress {
        a.0.into()
    }
}

impl Convert<H256, EvmB256> for EvmConverter {
    fn convert(a: H256) -> EvmB256 {
        a.0.into()
    }
}

impl Convert<AccountId32, EvmB256> for EvmConverter {
    fn convert(a: AccountId32) -> EvmB256 {
        EvmB256::new(*a.as_ref())
    }
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> OutboundCommitment<MaxMessages, MaxPayload> {
    pub fn hash(&self) -> H256 {
        let batch = crate::channel_abi::Batch {
            nonce: EvmU256::from(self.nonce),
            total_max_gas: EvmConverter::convert(self.total_max_gas),
            messages: self
                .messages
                .iter()
                .map(|m| crate::channel_abi::Message {
                    target: m.target.0.into(),
                    max_gas: EvmConverter::convert(m.max_gas),
                    payload: m.payload.to_vec().into(),
                })
                .collect(),
        };
        let input = batch.abi_encode();
        sp_runtime::traits::Keccak256::hash(&input)
    }
}

#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxPayload))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub struct InboundCommitment<MaxPayload: Get<u32>> {
    /// Channel contract address.
    pub channel: H160,
    /// Source contract of the message.
    pub source: H160,
    /// Batch nonce for replay protection and ordering.
    pub nonce: u64,
    /// Block number at which the message was committed.
    pub block_number: u64,
    /// Message payload.
    pub payload: BoundedVec<u8, MaxPayload>,
}

impl<MaxPayload: Get<u32>> InboundCommitment<MaxPayload> {
    pub fn hash(&self) -> H256 {
        ("evm-inbound", self).using_encoded(|encoded| sp_runtime::traits::Keccak256::hash(encoded))
    }
}

#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxMessages))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub struct StatusReport<MaxMessages: Get<u32>> {
    /// Channel contract address.
    pub channel: H160,
    /// Block number at which the event was emitted.
    pub block_number: u64,
    /// Relayer which submitted the messages.
    pub relayer: H160,
    /// Batch nonce for replay protection and ordering.
    pub nonce: u64,
    /// Message payload.
    pub results: BoundedVec<bool, MaxMessages>,
    /// Gas spent by the relayer.
    pub gas_spent: U256,
    /// Base fee paid by the relayer.
    pub base_fee: U256,
}

impl<MaxMessages: Get<u32>> StatusReport<MaxMessages> {
    pub fn hash(&self) -> H256 {
        ("evm-status-report", self)
            .using_encoded(|encoded| sp_runtime::traits::Keccak256::hash(encoded))
    }
}

#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = "")
)]
#[scale_info(skip_type_params(MaxMessages))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub struct BaseFeeUpdate {
    /// Updated base fee
    pub new_base_fee: U256,
    /// EVM block number of base fee
    pub evm_block_number: u64,
}

impl BaseFeeUpdate {
    pub fn hash(&self) -> H256 {
        ("base-fee-update", self)
            .using_encoded(|encoded| sp_runtime::traits::Keccak256::hash(encoded))
    }
}

#[test]
fn test_commitment_hash() {
    use hex_literal::hex;

    pub type MaxU32 = sp_runtime::traits::ConstU32<{ u32::MAX }>;

    let commitment: OutboundCommitment<MaxU32, MaxU32> = OutboundCommitment {
        nonce: 1,
        total_max_gas: 123.into(),
        messages: BoundedVec::default(),
    };

    // Value calculated on Ethereum contract with Remix IDE
    let expected = H256::from(hex!(
        "fe5da6b743707a6d3f8974111079fe7fb466bfed7a703d659e593c9120353bb1"
    ));
    assert_eq!(commitment.hash(), expected);
}
