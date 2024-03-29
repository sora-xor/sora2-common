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

//! Autogenerated weights for substrate_bridge_app
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `TRX40`, CPU: `AMD Ryzen Threadripper 3960X 24-Core Processor`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// ./target/release/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=substrate_bridge_app
// --extrinsic=*
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=../sora2-common/pallets/substrate-app/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for substrate_bridge_app.
pub trait WeightInfo {
	fn register_sidechain_asset() -> Weight;
	fn update_transaction_status() -> Weight;
	fn incoming_thischain_asset_registration() -> Weight;
	fn mint() -> Weight;
	fn burn() -> Weight;
	fn finalize_asset_registration() -> Weight;
}

/// Weights for substrate_bridge_app using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: LiberlandBridgeApp ThischainAssetId (r:1 w:0)
	/// Proof Skipped: LiberlandBridgeApp ThischainAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn register_sidechain_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2146`
		//  Estimated: `42046`
		// Minimum execution time: 76_000 nanoseconds.
		Weight::from_parts(80_000_000, 42046)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp ThischainAssetId (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp ThischainAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp AssetKinds (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainAssetId (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp SidechainAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	fn incoming_thischain_asset_registration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1991`
		//  Estimated: `34760`
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_parts(39_000_000, 34760)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: LiberlandBridgeApp ThischainAssetId (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp ThischainAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp AssetKinds (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainAssetId (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp SidechainAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: LiberlandBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	fn finalize_asset_registration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000)
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: LiberlandBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: LiberlandBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: LiberlandBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2578`
		//  Estimated: `48352`
		// Minimum execution time: 64_000 nanoseconds.
		Weight::from_parts(67_000_000, 48352)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: LiberlandBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: LiberlandBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: LiberlandBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: LiberlandBridgeApp SidechainAssetId (r:1 w:0)
	/// Proof Skipped: LiberlandBridgeApp SidechainAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LimitedAssets (r:1 w:0)
	/// Proof Skipped: BridgeProxy LimitedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2152`
		//  Estimated: `53772`
		// Minimum execution time: 68_000 nanoseconds.
		Weight::from_parts(74_000_000, 53772)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: BridgeProxy Senders (r:1 w:0)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	fn update_transaction_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2479`
		// Minimum execution time: 7_000 nanoseconds.
		Weight::from_parts(8_000_000, 2479)
			.saturating_add(T::DbWeight::get().reads(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp AllowedParachainAssets (r:100 w:100)
	/// Proof Skipped: SubstrateBridgeApp AllowedParachainAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: SubstrateBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 100]`.
	fn register_sidechain_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_860_000 picoseconds.
		Weight::from_parts(1_931_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SubstrateBridgeApp BridgeTransferLimit (r:0 w:1)
	/// Proof Skipped: SubstrateBridgeApp BridgeTransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	fn incoming_thischain_asset_registration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_860_000 picoseconds.
		Weight::from_parts(1_931_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: BridgeProxy Senders (r:1 w:0)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	fn update_transaction_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2479`
		// Minimum execution time: 5_760_000 picoseconds.
		Weight::from_parts(6_040_000, 2479)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: SubstrateBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2422`
		//  Estimated: `37154`
		// Minimum execution time: 52_452_000 picoseconds.
		Weight::from_parts(52_983_000, 37154)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: SubstrateBridgeApp BridgeTransferLimit (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp BridgeTransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp RelaychainAsset (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp RelaychainAsset (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp AssetKinds (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LimitedAssets (r:1 w:0)
	/// Proof Skipped: BridgeProxy LimitedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: SubstrateBridgeOutboundChannel MessageQueues (r:1 w:1)
	/// Proof Skipped: SubstrateBridgeOutboundChannel MessageQueues (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeOutboundChannel ChannelNonces (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeOutboundChannel ChannelNonces (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2305`
		//  Estimated: `58255`
		// Minimum execution time: 64_983_000 picoseconds.
		Weight::from_parts(65_613_000, 58255)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: SubstrateBridgeApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: SubstrateBridgeApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: SubstrateBridgeApp AssetKinds (r:0 w:1)
	/// Proof Skipped: SubstrateBridgeApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	fn finalize_asset_registration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `2889`
		// Minimum execution time: 7_201_000 picoseconds.
		Weight::from_parts(7_381_000, 2889)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
