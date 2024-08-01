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

//! Autogenerated weights for jetton_app
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `X670`, CPU: `AMD Ryzen 9 7950X 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/framenode
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=jetton_app
// --extrinsic=*
// --steps=50
// --repeat=20
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for jetton_app.
pub trait WeightInfo {
	fn mint() -> Weight;
	fn register_network() -> Weight;
	fn register_network_with_existing_asset() -> Weight;
}

/// Weights for jetton_app using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: JettonApp AssetsByAddresses (r:1 w:0)
	/// Proof Skipped: JettonApp AssetsByAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetKinds (r:1 w:0)
	/// Proof Skipped: JettonApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AppInfo (r:1 w:0)
	/// Proof Skipped: JettonApp AppInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: JettonApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: JettonApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2607`
		//  Estimated: `376201`
		// Minimum execution time: 69_301_000 picoseconds.
		Weight::from_parts(70_591_000, 376201)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: JettonApp AppInfo (r:1 w:1)
	/// Proof Skipped: JettonApp AppInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:2)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp TokenAddresses (r:1 w:1)
	/// Proof Skipped: JettonApp TokenAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetsByAddresses (r:0 w:1)
	/// Proof Skipped: JettonApp AssetsByAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetKinds (r:0 w:1)
	/// Proof Skipped: JettonApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: JettonApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfosV2 (r:0 w:1)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn register_network() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2764`
		//  Estimated: `55905`
		// Minimum execution time: 92_921_000 picoseconds.
		Weight::from_parts(95_251_000, 55905)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(15_u64))
	}
	/// Storage: JettonApp AppInfo (r:1 w:1)
	/// Proof Skipped: JettonApp AppInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: JettonApp TokenAddresses (r:1 w:1)
	/// Proof Skipped: JettonApp TokenAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetsByAddresses (r:0 w:1)
	/// Proof Skipped: JettonApp AssetsByAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetKinds (r:0 w:1)
	/// Proof Skipped: JettonApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: JettonApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	fn register_network_with_existing_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1225`
		//  Estimated: `18970`
		// Minimum execution time: 30_381_000 picoseconds.
		Weight::from_parts(31_121_000, 18970)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: JettonApp AssetsByAddresses (r:1 w:0)
	/// Proof Skipped: JettonApp AssetsByAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetKinds (r:1 w:0)
	/// Proof Skipped: JettonApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AppInfo (r:1 w:0)
	/// Proof Skipped: JettonApp AppInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: JettonApp SidechainPrecision (r:1 w:0)
	/// Proof Skipped: JettonApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy LockedAssets (r:1 w:1)
	/// Proof Skipped: BridgeProxy LockedAssets (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: BridgeProxy Senders (r:0 w:1)
	/// Proof Skipped: BridgeProxy Senders (max_values: None, max_size: None, mode: Measured)
	/// Storage: BridgeProxy Transactions (r:0 w:1)
	/// Proof Skipped: BridgeProxy Transactions (max_values: None, max_size: None, mode: Measured)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2607`
		//  Estimated: `376201`
		// Minimum execution time: 69_301_000 picoseconds.
		Weight::from_parts(70_591_000, 376201)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: JettonApp AppInfo (r:1 w:1)
	/// Proof Skipped: JettonApp AppInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:2)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp TokenAddresses (r:1 w:1)
	/// Proof Skipped: JettonApp TokenAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetsByAddresses (r:0 w:1)
	/// Proof Skipped: JettonApp AssetsByAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetKinds (r:0 w:1)
	/// Proof Skipped: JettonApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: JettonApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfosV2 (r:0 w:1)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn register_network() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2764`
		//  Estimated: `55905`
		// Minimum execution time: 92_921_000 picoseconds.
		Weight::from_parts(95_251_000, 55905)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(15_u64))
	}
	/// Storage: JettonApp AppInfo (r:1 w:1)
	/// Proof Skipped: JettonApp AppInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: JettonApp TokenAddresses (r:1 w:1)
	/// Proof Skipped: JettonApp TokenAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetsByAddresses (r:0 w:1)
	/// Proof Skipped: JettonApp AssetsByAddresses (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp AssetKinds (r:0 w:1)
	/// Proof Skipped: JettonApp AssetKinds (max_values: None, max_size: None, mode: Measured)
	/// Storage: JettonApp SidechainPrecision (r:0 w:1)
	/// Proof Skipped: JettonApp SidechainPrecision (max_values: None, max_size: None, mode: Measured)
	fn register_network_with_existing_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1225`
		//  Estimated: `18970`
		// Minimum execution time: 30_381_000 picoseconds.
		Weight::from_parts(31_121_000, 18970)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}
