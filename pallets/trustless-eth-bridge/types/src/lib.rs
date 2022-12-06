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
pub enum GenericNetworkId {
    EVM(EVMChainId),
    Sub(SubNetworkId),
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
