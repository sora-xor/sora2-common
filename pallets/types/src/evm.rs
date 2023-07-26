use crate::MainnetAssetId;
use codec::{Decode, Encode};
use derivative::Derivative;
use ethabi::Token;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{Get, RuntimeDebug, H160, H256, U256};
use sp_runtime::{traits::Hash, BoundedVec};
use sp_std::prelude::*;

/// Verification input for the message verifier.
///
/// This data type allows us to support multiple verification schemes. In the near future,
/// A light-client scheme will be added too.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct Proof {
    // The block hash of the block in which the receipt was included.
    pub block_hash: H256,
    // The index of the transaction (and receipt) within the block.
    // !!! Untrusted value used just for logging purposes.
    pub tx_index: u32,
    // Proof values
    pub data: Vec<Vec<u8>>,
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
/// EVM contract kind
pub enum EVMAppKind {
    /// Used for native token transfers
    EthApp,
    /// Used for ERC20 tokens
    #[cfg_attr(feature = "std", serde(rename = "erc20App"))]
    ERC20App,
    /// Used for this chain native tokens
    SidechainApp,
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
pub struct Commitment<MaxMessages: Get<u32>, MaxPayload: Get<u32>> {
    /// A batch nonce for replay protection and ordering.
    pub nonce: u64,
    /// Total maximum gas that can be used by all messages in the commit.
    /// Should be equal to sum of `max_gas`es of `messages`
    pub total_max_gas: U256,
    /// Messages passed through the channel in the current commit.
    pub messages: BoundedVec<Message<MaxPayload>, MaxMessages>,
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> Commitment<MaxMessages, MaxPayload> {
    pub fn hash(&self) -> H256 {
        // Batch(uint256,(address,uint64,uint256,uint256,bytes)[])
        let messages: Vec<Token> = self
            .messages
            .iter()
            .map(|message| {
                Token::Tuple(vec![
                    Token::Address(message.target),
                    Token::Uint(message.max_gas.into()),
                    Token::Bytes(message.payload.clone().into()),
                ])
            })
            .collect();
        let commitment: Vec<Token> = vec![
            Token::Uint(self.nonce.into()),
            Token::Uint(self.total_max_gas),
            Token::Array(messages),
        ];
        // Structs are represented as tuples in ABI
        // https://docs.soliditylang.org/en/v0.8.15/abi-spec.html#mapping-solidity-to-abi-types
        let input = ethabi::encode(&vec![Token::Tuple(commitment)]);
        sp_runtime::traits::Keccak256::hash(&input)
    }
}

#[test]
fn test_commitment_hash() {
    use hex_literal::hex;

    pub type MaxU32 = sp_runtime::traits::ConstU32<{ u32::MAX }>;

    let commitment: Commitment<MaxU32, MaxU32> = Commitment {
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
