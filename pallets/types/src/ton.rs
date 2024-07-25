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

use codec::{Decode, Encode, MaxEncodedLen};
use derivative::Derivative;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{Get, RuntimeDebug, H256};
use sp_runtime::{traits::Hash, BoundedVec};

use crate::MainnetAssetId;

#[derive(
    Encode,
    Decode,
    Copy,
    Clone,
    PartialEq,
    Eq,
    RuntimeDebug,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
    Default,
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum TonNetworkId {
    #[default]
    Mainnet,
    Testnet,
}

// We use u128 encoding in our contracts
pub type TonBalance = u128;

#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    scale_info::TypeInfo,
    MaxEncodedLen,
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TonAddress {
    pub workchain: i32,
    pub hash_part: [u8; 32],
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
    MaxEncodedLen,
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TonTransactionId {
    pub lt: i64,
    pub hash: [u8; 32],
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
    MaxEncodedLen,
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TonAddressWithPrefix {
    pub prefix: u8,
    pub workchain: i32,
    pub hash_part: [u8; 32],
}

impl TonAddressWithPrefix {
    pub fn address(&self) -> Option<TonAddress> {
        Some(TonAddress {
            workchain: self.workchain,
            hash_part: self.hash_part,
        })
    }
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
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AdditionalTONInboundData {
    pub source: TonAddress,
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
/// Information about Jetton in TON network
pub struct TonAssetInfo {
    /// Thischain asset id
    pub asset_id: MainnetAssetId,
    /// Contract address
    pub address: TonAddress,
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
pub struct TonAppInfo {
    pub address: TonAddress,
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
pub enum Commitment<MaxPayload: Get<u32>> {
    #[cfg_attr(feature = "std", serde(rename = "inbound"))]
    Inbound(InboundCommitment<MaxPayload>),
}

impl<MaxPayload: Get<u32>> Commitment<MaxPayload> {
    pub fn hash(&self) -> H256 {
        match self {
            Commitment::Inbound(commitment) => commitment.hash(),
        }
    }

    pub fn nonce(&self) -> u64 {
        match self {
            Commitment::Inbound(commitment) => commitment.nonce,
        }
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
    pub channel: TonAddress,
    /// Source contract of the message.
    pub source: TonAddress,
    /// Batch nonce for replay protection and ordering.
    pub nonce: u64,
    /// Transaction at which the message was committed.
    pub transaction_id: TonTransactionId,
    /// Message payload.
    pub payload: BoundedVec<u8, MaxPayload>,
}

impl<MaxPayload: Get<u32>> InboundCommitment<MaxPayload> {
    pub fn hash(&self) -> H256 {
        ("ton-inbound", self).using_encoded(|encoded| sp_runtime::traits::Keccak256::hash(encoded))
    }
}
