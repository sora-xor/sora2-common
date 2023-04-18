#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for ethereum_beacon_client.
pub trait WeightInfo {
	fn initialize() -> Weight;
	fn import_update() -> Weight;
}

pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	fn initialize() -> Weight {
		Default::default()
	}
	fn import_update() -> Weight {
		Default::default()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn initialize() -> Weight {
		Default::default()
	}
	fn import_update() -> Weight {
		Default::default()
	}
}
