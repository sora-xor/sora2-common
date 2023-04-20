// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Types for representing messages

use crate::H256;
use crate::{H160, U256};
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_beefy::mmr::{BeefyNextAuthoritySet, MmrLeafVersion};
use sp_runtime::{Digest, DigestItem};
use sp_std::vec::Vec;

use crate::GenericNetworkId;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub enum MessageDirection {
    Inbound,
    Outbound,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct MessageId {
    direction: MessageDirection,
    nonce: MessageNonce,
}

impl From<(MessageDirection, MessageNonce)> for MessageId {
    fn from((direction, nonce): (MessageDirection, MessageNonce)) -> Self {
        MessageId { direction, nonce }
    }
}

impl From<MessageId> for MessageNonce {
    fn from(id: MessageId) -> Self {
        id.nonce
    }
}

impl MessageId {
    pub fn inbound(nonce: MessageNonce) -> Self {
        MessageId::from((MessageDirection::Inbound, nonce))
    }

    pub fn outbound(nonce: MessageNonce) -> Self {
        MessageId::from((MessageDirection::Outbound, nonce))
    }
}

pub type MessageNonce = u64;

/// A message relayed from Ethereum.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct Message {
    /// The raw message data.
    pub data: Vec<u8>,
    /// Input to the message verifier
    pub proof: Proof,
}

/// A message relayed from Parachain.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct ParachainMessage<Balance> {
    pub payload: Vec<u8>,
    pub nonce: MessageNonce,
    pub timestamp: u64,
    pub fee: Balance,
}

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

#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct AuxiliaryDigest {
    pub logs: Vec<AuxiliaryDigestItem>,
}

impl From<Digest> for AuxiliaryDigest {
    fn from(digest: Digest) -> Self {
        Self {
            logs: digest
                .logs
                .into_iter()
                .filter_map(|log| AuxiliaryDigestItem::try_from(log).ok())
                .collect::<Vec<_>>(),
        }
    }
}

/// Auxiliary [`DigestItem`] to include in header digest.
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum AuxiliaryDigestItem {
    /// A batch of messages has been committed.
    Commitment(GenericNetworkId, H256),
}

impl From<AuxiliaryDigestItem> for DigestItem {
    fn from(item: AuxiliaryDigestItem) -> DigestItem {
        DigestItem::Other(item.encode())
    }
}

impl TryFrom<DigestItem> for AuxiliaryDigestItem {
    type Error = codec::Error;
    fn try_from(value: DigestItem) -> Result<Self, Self::Error> {
        match value {
            DigestItem::Other(data) => Ok(Decode::decode(&mut &*data)?),
            _ => Err(codec::Error::from("wrong digest item kind")),
        }
    }
}

/// Modified leaf data for SORA
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct MmrLeaf<BlockNumber, Hash, MerkleRoot, DigestHash> {
    /// Version of the leaf format.
    ///
    /// Can be used to enable future format migrations and compatibility.
    /// See [`MmrLeafVersion`] documentation for details.
    pub version: MmrLeafVersion,
    /// Current block parent number and hash.
    pub parent_number_and_hash: (BlockNumber, Hash),
    /// A merkle root of the next BEEFY authority set.
    pub beefy_next_authority_set: BeefyNextAuthoritySet<MerkleRoot>,
    /// Digest hash of previous block (because digest for current block can be incomplete)
    pub digest_hash: DigestHash,
}

/// A type of asset registered on a bridge.
///
/// - Thischain: a Sora asset.
/// - Sidechain: an Ethereum token.
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone,
    Copy,
    Encode,
    Decode,
    PartialEq,
    Eq,
    RuntimeDebug,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
pub enum AssetKind {
    Thischain,
    Sidechain,
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
pub enum MessageStatus {
    InQueue,
    Committed,
    Done,
    Failed,
    Refunded,
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
pub enum AppKind {
    EthApp,
    ERC20App,
    SidechainApp,
    SubstrateApp,
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
pub struct LeafExtraData<Hash, RandomSeed> {
    pub random_seed: RandomSeed,
    pub digest_hash: Hash,
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
pub struct BridgeAssetInfo<AssetId> {
    pub asset_id: AssetId,
    pub evm_address: Option<H160>,
    pub app_kind: AppKind,
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
pub struct BridgeAppInfo {
    pub evm_address: H160,
    pub app_kind: AppKind,
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
    Default,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CallOriginOutput<NetworkId, MessageId, Additional> {
    pub network_id: NetworkId,
    pub message_id: MessageId,
    pub timestamp: u64,
    pub additional: Additional,
}

impl<NetworkId, Additional> crate::traits::OriginOutput<NetworkId, Additional>
    for CallOriginOutput<NetworkId, H256, Additional>
{
    fn new(
        network_id: NetworkId,
        message_id: H256,
        timestamp: u64,
        additional: Additional,
    ) -> Self {
        Self {
            network_id,
            message_id,
            timestamp,
            additional,
        }
    }
}

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"trustless-evm-bridge";
pub const TECH_ACCOUNT_MAIN: &[u8] = b"main";
pub const TECH_ACCOUNT_FEES: &[u8] = b"fees";
pub const TECH_ACCOUNT_TREASURY_PREFIX: &[u8] = b"treasury";
