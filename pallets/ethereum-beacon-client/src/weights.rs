#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for ethereum_beacon_client.
pub trait WeightInfo {
	fn initialize() -> Weight;
	fn sync_committee_period_update() -> Weight;
	fn import_finalized_header() -> Weight;
	fn import_execution_header() -> Weight;
	fn unblock_bridge() -> Weight;
}

/// Weights for ethereum_beacon_client using the Snowbridge node and recommended hardware.
pub struct SnowbridgeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SnowbridgeWeight<T> {
	fn initialize() -> Weight {
		Default::default()
	}
	fn sync_committee_period_update() -> Weight {
		Default::default()
	}
	fn import_finalized_header() -> Weight {
		Default::default()
	}
	fn import_execution_header() -> Weight {
		Default::default()
	}
	fn unblock_bridge() -> Weight {
		Default::default()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn initialize() -> Weight {
		Default::default()
	}
	fn sync_committee_period_update() -> Weight {
		Default::default()
	}
	fn import_finalized_header() -> Weight {
		Default::default()
	}
	fn import_execution_header() -> Weight {
		Default::default()
	}
	fn unblock_bridge() -> Weight {
		Default::default()
	}
}
