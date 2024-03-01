#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for substrate_bridge_channel::outbound.
pub trait WeightInfo {
	fn submit() -> Weight;
	fn update_interval() -> Weight;
}

/// Weights for substrate_bridge_channel::outbound using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31`
		//  Estimated: `5012`
		// Minimum execution time: 7_800_000 picoseconds.
		Weight::from_parts(8_120_000, 5012)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SubstrateBridgeOutboundChannel Interval (r:0 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel Interval (max_values: Some(1), max_size: None, mode: Measured)
	fn update_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_920_000 picoseconds.
		Weight::from_parts(4_070_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31`
		//  Estimated: `5012`
		// Minimum execution time: 7_800_000 picoseconds.
		Weight::from_parts(8_120_000, 5012)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubstrateBridgeOutboundChannel Interval (r:0 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel Interval (max_values: Some(1), max_size: None, mode: Measured)
	fn update_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_920_000 picoseconds.
		Weight::from_parts(4_070_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}