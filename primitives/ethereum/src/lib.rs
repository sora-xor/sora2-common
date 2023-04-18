#![cfg_attr(not(feature = "std"), no_std)]

pub mod difficulty;
pub mod ethashdata;
pub mod ethashproof;
pub mod header;
pub mod log;
pub mod mpt;
pub mod network_config;
pub mod receipt;
#[cfg(feature = "std")]
pub mod serde_utils;
#[cfg(any(feature = "test", test))]
pub mod test_utils;

use codec::Encode;
pub use ethereum_types::{Address as EthAddress, H128, H160, H256, H512, H64, U256};
pub use header::Header;
pub use header::HeaderId;
pub use log::Log;
pub use receipt::Receipt;
use sp_std::prelude::*;

pub type EVMChainId = U256;

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
