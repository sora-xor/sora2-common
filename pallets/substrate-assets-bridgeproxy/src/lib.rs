// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! A BEEFY+MMR pallet combo.
//!
//! While both BEEFY and Merkle Mountain Range (MMR) can be used separately,
//! these tools were designed to work together in unison.
//!
//! The pallet provides a standardized MMR Leaf format that is can be used
//! to bridge BEEFY+MMR-based networks (both standalone and polkadot-like).
//!
//! The MMR leaf contains:
//! 1. Block number and parent block hash.
//! 2. Merkle Tree Root Hash of next BEEFY validator set.
//! 3. Merkle Tree Root Hash of current parachain heads state.
//!
//! and thanks to versioning can be easily updated in the future.

pub use pallet::*;
use sp_core::H256;

/// Subject for randomness.
// pub const RANDOMNESS_SUBJECT: &[u8] = b"beefy-leaf-extra";

/// A type that is able to return current list of parachain heads that end up in the MMR leaf.

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]
    use crate::AssetIdGenerator;
    use bridge_types::types::RawAssetInfo;
    use bridge_types::GenericNetworkId;
    use frame_support::pallet_prelude::{ValueQuery, *};
    use frame_support::traits::fungibles::{
        metadata::Mutate as MetadataMutate, Create, Inspect, InspectMetadata, Transfer, Mutate,
    };
    use frame_system::{pallet_prelude::*, Origin};
    use sp_core::H256;
    use sp_runtime::traits::StaticLookup;
    use sp_std::prelude::*;
    use sp_io::hashing::blake2_256;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn tech_acc)]
    pub(super) type TechAcc<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_nonce)]
    pub(super) type AssetNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The module's configuration trait.
    #[pallet::config]
    #[pallet::disable_frame_system_supertrait_check]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // type Randomness: Randomness<RandomnessOutputOf<Self>, Self::BlockNumber>;
        type TechAcc: Get<Self::AccountId>;

        type AssetIdGenerator: AssetIdGenerator<Self::AssetId>;

        type MinBalance: Get<Self::Balance>;
    }

    #[pallet::event]
    pub enum Event<T: Config> {
        AssetCreated(T::AssetId),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_tech_account(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            TechAcc::<T>::set(Some(account_id));
            Ok(())
        }
    }

    impl<T: Config> bridge_types::traits::BridgeAssetRegistry<T::AccountId, T::AssetId> for Pallet<T> {
        type AssetName = Vec<u8>;
        type AssetSymbol = Vec<u8>;

        fn register_asset(
            _: GenericNetworkId,
            name: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, T::AssetId>>::AssetName,
            symbol: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, T::AssetId>>::AssetSymbol,
        ) -> Result<T::AssetId, DispatchError> {
            // Err(DispatchError::Other("NOT AVAILIBLE"))
            let nonce = Self::asset_nonce();
            AssetNonce::<T>::set(nonce + 1);
            // let ta = T::TechAcc::get();
            // let ta = account_id();
            
            let ta = Self::tech_acc().unwrap();
            let mut i = 0;
            let iter = 4;
            while i <= iter {
                let hash = {
                    let mut vector = name.clone();
                    vector.extend_from_slice(&symbol);
                    vector.extend_from_slice(&(nonce + i).encode());
                    // let hash = sp_core::hashing::blake2_256(&vector);
                    let hash = blake2_256(&vector);
                    H256::from_slice(&hash)
                };
                let asset_id = T::AssetIdGenerator::generate_asset_id(hash);
                // let asset_id = (10000 + nonce).into();

                let res = <pallet_assets::Pallet<T> as Create<T::AccountId>>::create(
                    asset_id,
                    ta.clone(),
                    true,
                    T::MinBalance::get(),
                );
                // let res = <pallet_assets::Pallet<T> as Create<T::AccountId>>::create(
                //     (10000 + nonce).into(),
                //     ta.clone(),
                //     true,
                //     T::MinBalance::get(),
                // );
                if res.is_ok() {
                    <pallet_assets::Pallet<T> as MetadataMutate<T::AccountId>>::set(
                        asset_id, &ta, name, symbol, 18,
                    )?;
                    return Ok(asset_id);
                }
                if i == iter {
                    res?;
                }
                i += 1;
            }
            Err(DispatchError::Other("FAILED TO CREATE ASSET"))
        }

        fn manage_asset(_: GenericNetworkId, _: T::AssetId) -> Result<(), DispatchError> {
            // Err(DispatchError::Other("NOT AVAILIBLE"))
            Ok(())
        }

        fn ensure_asset_exists(asset_id: T::AssetId) -> bool {
            <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::asset_exists(asset_id)
        }

        fn get_raw_info(asset_id: T::AssetId) -> bridge_types::types::RawAssetInfo {
            let name = <pallet_assets::Pallet<T> as InspectMetadata<T::AccountId>>::name(&asset_id);
            let symbol = pallet_assets::Pallet::<T>::symbol(&asset_id);
            let precision = pallet_assets::Pallet::<T>::decimals(&asset_id);
            bridge_types::types::RawAssetInfo {
                name,
                symbol,
                precision,
            }
        }
    }

    impl<T: Config> bridge_types::traits::BridgeAssetLocker<T::AccountId> for Pallet<T> {
        type AssetId = T::AssetId;
        type Balance = T::Balance;

        fn lock_asset(
            _network_id: GenericNetworkId,
            asset_kind: bridge_types::types::AssetKind,
            who: &T::AccountId,
            asset_id: &Self::AssetId,
            amount: &Self::Balance,
        ) -> DispatchResult {
            // let ta = T::TechAcc::get();
            let ta = Self::tech_acc().unwrap();
            // let ta = Self::account_id();

            match asset_kind {
                bridge_types::types::AssetKind::Thischain => {
                    <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
                        asset_id.clone(),
                        &who,
                        &ta,
                        amount.clone(),
                        true,
                    )?;
                },
                bridge_types::types::AssetKind::Sidechain => {
                    <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::burn_from(
                        asset_id.clone(),
                        &who,
                        amount.clone(),
                    )?;
                },
            }

            Ok(())
        }

        fn unlock_asset(
            _network_id: GenericNetworkId,
            asset_kind: bridge_types::types::AssetKind,
            who: &T::AccountId,
            asset_id: &Self::AssetId,
            amount: &Self::Balance,
        ) -> DispatchResult {
            // let ta = T::TechAcc::get();
            // let ta = Self::account_id();
            let ta = Self::tech_acc().unwrap();
            match asset_kind {
                bridge_types::types::AssetKind::Thischain => {
                    <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
                        asset_id.clone(),
                        &ta,
                        &who,
                        amount.clone(),
                        true,
                    )?;
                },
                bridge_types::types::AssetKind::Sidechain => {
                    <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
                        asset_id.clone(),
                        &who,
                        amount.clone(),
                    )?;
                },
            }

            Ok(())
        }
    }

    // impl<T: Config> Pallet<T> { 
    //     pub fn account_id() -> T::AccountId {
    //         use frame_support::PalletId;
    //         use sp_runtime::traits::AccountIdConversion;
        
    //         const PALLET_ID: PalletId = PalletId(*b"brdgprxy");
    //         // Convert the PalletId into an AccountId using the AccountIdConversion trait
    //         PALLET_ID.into_account()
    //     }
    // }


}

pub trait AssetIdGenerator<T> {
    fn generate_asset_id(hash: H256) -> T;
}
