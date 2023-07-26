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
#![allow(clippy::large_enum_variant)]

use codec::{Decode, Encode};
use derivative::Derivative;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{ecdsa, Get, H256};
use sp_runtime::{traits::Hash, BoundedVec, RuntimeDebug};
use sp_std::prelude::*;

use crate::{types::AssetKind, GenericTimepoint, MainnetAccountId, MainnetAssetId, MainnetBalance};

pub use xcm::v3::{Junction, Junctions};
pub use xcm::VersionedMultiLocation;

pub type ParachainAccountId = VersionedMultiLocation;

pub type ParachainAssetId = xcm::v3::AssetId;

pub const PARENT_PARACHAIN_ASSET: ParachainAssetId =
    ParachainAssetId::Concrete(xcm::v3::MultiLocation::parent());

pub trait SubstrateBridgeMessageEncode {
    fn prepare_message(self) -> Vec<u8>;
}

/// Message to SubstrateApp pallet
#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum SubstrateAppCall {
    Transfer {
        asset_id: MainnetAssetId,
        sender: Option<ParachainAccountId>,
        recipient: MainnetAccountId,
        amount: MainnetBalance,
    },
    FinalizeAssetRegistration {
        asset_id: MainnetAssetId,
        asset_kind: AssetKind,
    },
    ReportXCMTransferResult {
        message_id: H256,
        transfer_status: XCMAppTransferStatus,
    },
}

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum XCMAppTransferStatus {
    Success,
    XCMTransferError,
}

impl SubstrateBridgeMessageEncode for SubstrateAppCall {
    fn prepare_message(self) -> Vec<u8> {
        BridgeCall::SubstrateApp(self).encode()
    }
}

/// Message to XCMApp pallet
#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum XCMAppCall {
    Transfer {
        asset_id: MainnetAssetId,
        sender: MainnetAccountId,
        recipient: ParachainAccountId,
        amount: MainnetBalance,
    },
    RegisterAsset {
        asset_id: MainnetAssetId,
        sidechain_asset: ParachainAssetId,
        asset_kind: AssetKind,
        minimal_xcm_amount: MainnetBalance,
    },
    SetAssetMinAmount {
        asset_id: MainnetAssetId,
        minimal_xcm_amount: MainnetBalance,
    },
}

impl SubstrateBridgeMessageEncode for XCMAppCall {
    fn prepare_message(self) -> Vec<u8> {
        BridgeCall::XCMApp(self).encode()
    }
}

/// Message to DataSigner pallet
#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum DataSignerCall {
    AddPeer { peer: ecdsa::Public },
    RemovePeer { peer: ecdsa::Public },
}

impl SubstrateBridgeMessageEncode for DataSignerCall {
    fn prepare_message(self) -> Vec<u8> {
        BridgeCall::DataSigner(self).encode()
    }
}

/// Message to MultisigVerifier pallet
#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum MultisigVerifierCall {
    AddPeer { peer: ecdsa::Public },
    RemovePeer { peer: ecdsa::Public },
}

impl SubstrateBridgeMessageEncode for MultisigVerifierCall {
    fn prepare_message(self) -> Vec<u8> {
        BridgeCall::MultisigVerifier(self).encode()
    }
}

/// Substrate bridge message payload
#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum BridgeCall {
    SubstrateApp(SubstrateAppCall),
    XCMApp(XCMAppCall),
    DataSigner(DataSignerCall),
    MultisigVerifier(MultisigVerifierCall),
}

impl SubstrateBridgeMessageEncode for BridgeCall {
    fn prepare_message(self) -> Vec<u8> {
        self.encode()
    }
}

/// Substrate bridge message.
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
pub struct BridgeMessage<MaxPayload: Get<u32>> {
    pub payload: BoundedVec<u8, MaxPayload>,
    pub timepoint: GenericTimepoint,
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
/// Substrate bridge asset info
pub struct SubAssetInfo {
    /// Thischain asset info
    pub asset_id: MainnetAssetId,
    pub asset_kind: AssetKind,
    pub precision: u8,
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
    /// Messages passed through the channel in the current commit.
    pub messages: BoundedVec<BridgeMessage<MaxPayload>, MaxMessages>,
    pub nonce: u64,
}

impl<MaxMessages: Get<u32>, MaxPayload: Get<u32>> Commitment<MaxMessages, MaxPayload> {
    pub fn hash(&self) -> H256 {
        sp_runtime::traits::Keccak256::hash_of(self)
    }
}
