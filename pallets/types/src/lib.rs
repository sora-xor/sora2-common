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

#![cfg_attr(not(feature = "std"), no_std)]

pub mod channel_abi;
pub mod difficulty;
pub mod ethashdata;
pub mod ethashproof;
pub mod header;
pub mod log;
mod mpt;
pub mod network_config;
pub mod receipt;
pub mod substrate;
pub mod traits;
pub mod types;

#[cfg(any(feature = "test", test))]
pub mod test_utils;
pub mod utils;

use codec::{Decode, Encode};
pub use ethereum_types::{Address, H128, H160, H256, H512, H64, U256};
use frame_support::RuntimeDebug;
use sp_std::vec;
use sp_std::vec::Vec;

pub use header::{Header, HeaderId};
pub use log::Log;
pub use receipt::Receipt;

#[derive(Debug)]
pub enum DecodeError {
    // Unexpected RLP data
    InvalidRLP(rlp::DecoderError),
    // Data does not match expected ABI
    InvalidABI(ethabi::Error),
    // Invalid message payload
    InvalidPayload,
}

impl From<rlp::DecoderError> for DecodeError {
    fn from(err: rlp::DecoderError) -> Self {
        DecodeError::InvalidRLP(err)
    }
}

impl From<ethabi::Error> for DecodeError {
    fn from(err: ethabi::Error) -> Self {
        DecodeError::InvalidABI(err)
    }
}

pub type EVMChainId = U256;

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
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum SubNetworkId {
    Mainnet,
    Kusama,
    Polkadot,
    Rococo,
    Custom(u32),
}

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
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum GenericNetworkId {
    #[cfg_attr(feature = "std", serde(with = "serde_u256", rename = "evm"))]
    EVM(EVMChainId),
    Sub(SubNetworkId),
    #[cfg_attr(feature = "std", serde(rename = "evmLegacy"))]
    EVMLegacy(u32),
}

impl GenericNetworkId {
    pub fn evm(self) -> Option<EVMChainId> {
        match self {
            Self::EVM(chain_id) => Some(chain_id),
            _ => None,
        }
    }

    pub fn sub(self) -> Option<SubNetworkId> {
        match self {
            Self::Sub(network_id) => Some(network_id),
            _ => None,
        }
    }

    pub fn evm_legacy(self) -> Option<u32> {
        match self {
            Self::EVMLegacy(network_id) => Some(network_id),
            _ => None,
        }
    }
}

impl From<EVMChainId> for GenericNetworkId {
    fn from(id: EVMChainId) -> Self {
        GenericNetworkId::EVM(id)
    }
}

impl From<SubNetworkId> for GenericNetworkId {
    fn from(id: SubNetworkId) -> Self {
        GenericNetworkId::Sub(id)
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub enum GenericAccount<AccountId> {
    EVM(H160),
    Sora(AccountId),
    Parachain(xcm::VersionedMultiLocation),
    Unknown,
    Root,
}

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
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum GenericTimepoint {
    #[cfg_attr(feature = "std", serde(rename = "evm"))]
    EVM(u64),
    Sora(u32),
    Parachain(u32),
    Pending,
    Unknown,
}

impl Default for GenericTimepoint {
    fn default() -> Self {
        GenericTimepoint::Unknown
    }
}

pub const CHANNEL_INDEXING_PREFIX: &[u8] = b"commitment";

pub fn import_digest(network_id: &EVMChainId, header: &Header) -> Vec<u8>
where
    EVMChainId: Encode,
    Header: Encode,
{
    let mut digest = vec![];
    network_id.encode_to(&mut digest);
    header.encode_to(&mut digest);
    digest
}

#[cfg(feature = "std")]
mod serde_u256 {
    use serde::{Deserialize, Serialize};
    use sp_core::U256;

    pub fn deserialize<'de, D>(deserializer: D) -> core::result::Result<U256, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let network_id = String::deserialize(deserializer)?;
        if network_id.starts_with("0x") {
            let network_id =
                U256::from_str_radix(&network_id[2..], 16).map_err(serde::de::Error::custom)?;
            Ok(network_id)
        } else {
            let network_id =
                U256::from_str_radix(&network_id, 10).map_err(serde::de::Error::custom)?;
            Ok(network_id)
        }
    }

    pub fn serialize<S>(value: &U256, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value.0.serialize(serializer)
    }
}
