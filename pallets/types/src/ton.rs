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

use bitvec::{
    order::Msb0,
    vec::BitVec,
    view::{AsBits, BitView},
};
use codec::{Decode, Encode, MaxEncodedLen};
use derivative::Derivative;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{Get, RuntimeDebug, H256};
use sp_runtime::{traits::Hash, BoundedVec};
use sp_std::prelude::*;

use crate::{MainnetAccountId, MainnetAssetId, MainnetBalance};

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

// TON encodes integers as big-endian and we use uint128 in our contracts
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, scale_info::TypeInfo, Default)]
pub struct TonBalance([u8; 16]);

impl TonBalance {
    pub fn new(balance: u128) -> Self {
        TonBalance(balance.to_be_bytes())
    }

    pub fn balance(&self) -> MainnetBalance {
        u128::from_be_bytes(self.0)
    }
}

impl core::fmt::Debug for TonBalance {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TonBalance").field(&self.balance()).finish()
    }
}

impl From<u128> for TonBalance {
    fn from(value: u128) -> Self {
        TonBalance::new(value)
    }
}

impl From<u64> for TonBalance {
    fn from(value: u64) -> Self {
        TonBalance::new(value as u128)
    }
}

impl From<u32> for TonBalance {
    fn from(value: u32) -> Self {
        TonBalance::new(value as u128)
    }
}

impl From<TonBalance> for u128 {
    fn from(value: TonBalance) -> Self {
        value.balance()
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
    MaxEncodedLen,
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TonAddress {
    pub workchain: u8,
    pub address: H256,
}

impl TonAddress {
    pub const fn new(workchain: u8, address: H256) -> Self {
        Self { workchain, address }
    }

    pub const fn empty() -> Self {
        Self::new(0, H256::zero())
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::empty()
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
    MaxEncodedLen,
    Default,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TonTransactionId {
    pub lt: i64,
    pub hash: H256,
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
    prefix: u8,
    address: TonAddress,
}

impl TonAddressWithPrefix {
    pub fn new(prefix: u8, address: TonAddress) -> Self {
        Self { prefix, address }
    }
    pub fn address(&self) -> Option<TonAddress> {
        if self.prefix == 4 || (self.address == TonAddress::empty() && self.prefix == 0) {
            Some(self.address)
        } else {
            None
        }
    }
}

impl From<TonAddress> for TonAddressWithPrefix {
    fn from(value: TonAddress) -> Self {
        Self::new(4, value)
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
    pub target: TonAddress,
    /// Maximum gas this message can use on the Ethereum.
    pub max_fee: MainnetBalance,
    /// Payload for target application.
    pub payload: BoundedVec<u8, MaxPayload>,
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
    Default,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AdditionalTONOutboundData {
    pub max_fee: u128,
    pub target: TonAddress,
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
#[scale_info(skip_type_params(MaxPayload, MaxMessages))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub enum Commitment<MaxMessages: Get<u32>, MaxPayload: Get<u32>> {
    #[cfg_attr(feature = "std", serde(rename = "inbound"))]
    Inbound(InboundCommitment<MaxPayload>),
    #[cfg_attr(feature = "std", serde(rename = "outbound"))]
    Outbound(OutboundCommitment<MaxMessages, MaxPayload>),
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> Commitment<MaxMessages, MaxPayload> {
    pub fn hash(&self) -> H256 {
        match self {
            Commitment::Inbound(commitment) => commitment.hash(),
            Commitment::Outbound(commitment) => commitment.hash(),
        }
    }

    pub fn nonce(&self) -> u64 {
        match self {
            Commitment::Inbound(commitment) => commitment.nonce,
            Commitment::Outbound(commitment) => commitment.nonce,
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
    pub total_max_fee: u128,
    /// Messages passed through the channel in the current commit.
    pub messages: BoundedVec<Message<MaxPayload>, MaxMessages>,
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> OutboundCommitment<MaxMessages, MaxPayload> {
    pub fn hash(&self) -> H256 {
        ("ton-outbound", self).using_encoded(|encoded| sp_runtime::traits::Keccak256::hash(encoded))
    }
}

#[derive(Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Derivative)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Default(bound = "")
)]
#[scale_info(skip_type_params(MaxPayload, MaxMessages))]
#[cfg_attr(feature = "std", serde(bound = ""))]
pub struct MessageQueue<MaxMessages: Get<u32>, MaxPayload: Get<u32>> {
    /// Queue total gas amount
    pub total_fee: MainnetBalance,
    /// Messages queue
    pub queue: BoundedVec<Message<MaxPayload>, MaxMessages>,
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> MessageQueue<MaxMessages, MaxPayload> {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Message<MaxPayload>> {
        self.queue.iter()
    }

    pub fn try_push(&mut self, message: Message<MaxPayload>) -> Result<(), Message<MaxPayload>> {
        let message_fee = message.max_fee;
        self.queue.try_push(message)?;
        self.total_fee = self.total_fee.saturating_add(message_fee);
        Ok(())
    }
}

pub struct PayloadBuilder {
    inner: BitVec<u8, Msb0>,
}

impl PayloadBuilder {
    pub fn new() -> Self {
        Self {
            inner: BitVec::new(),
        }
    }

    pub fn write_id(mut self, id: u32) -> Self {
        self.inner.extend_from_bitslice(id.view_bits::<Msb0>());
        self
    }

    pub fn write_u8(mut self, value: u8) -> Self {
        self.inner.extend_from_bitslice(value.view_bits::<Msb0>());
        self
    }

    pub fn write_bytes(mut self, value: &[u8]) -> Self {
        self.inner.extend_from_raw_slice(value);
        self
    }

    pub fn write_address(mut self, address: TonAddress) -> Self {
        if address.is_empty() {
            // 0x00 - Null
            self.inner
                .extend_from_bitslice(bitvec::bits![u8, Msb0; 0, 0]);
        } else {
            // 0x10 - Std + anycast: (Maybe Anycast)
            self.inner
                .extend_from_bitslice(bitvec::bits![u8, Msb0; 1, 0, 0]);
            // workchain_id: int8
            self.inner
                .extend_from_bitslice(address.workchain.view_bits::<Msb0>());
            // address: bits256
            self.inner
                .extend_from_bitslice(address.address.as_bytes().view_bits::<Msb0>());
        }
        self
    }

    pub fn write_account_id(mut self, account: MainnetAccountId) -> Self {
        self.inner.extend_from_bitslice(account.as_bits::<Msb0>());
        self
    }

    pub fn write_balance(mut self, amount: TonBalance) -> Self {
        self.inner.extend_from_bitslice(amount.0.as_bits::<Msb0>());
        self
    }

    /// Fails if payload exceed 1023 bits, which is limit for Cell in TVM
    pub fn build(self) -> Option<Vec<u8>> {
        if self.inner.len() >= 1023 {
            return None;
        }
        Some(self.inner.encode())
    }
}

impl Default for PayloadBuilder {
    fn default() -> Self {
        Self::new()
    }
}
