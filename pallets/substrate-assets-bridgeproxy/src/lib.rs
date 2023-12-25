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

pub use pallet::*;
use sp_core::H256;

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]
    use crate::AssetIdGenerator;
    use bridge_types::GenericNetworkId;
    use frame_support::fail;
    use frame_support::pallet_prelude::{ValueQuery, *};
    use frame_support::traits::fungibles::{
        metadata::Mutate as MetadataMutate, Create, Inspect, InspectMetadata, Transfer, Mutate,
    };
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_std::prelude::*;
    use sp_io::hashing::blake2_256;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // #[pallet::storage]
    // #[pallet::getter(fn tech_acc)]
    // pub(super) type TechAcc<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn tech_acc)]
    pub(super) type TechAccs<T: Config> =
        StorageMap<_, Identity, GenericNetworkId, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn asset_nonce)]
    pub(super) type AssetNonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// The module's configuration trait.
    #[pallet::config]
    #[pallet::disable_frame_system_supertrait_check]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // #[pallet::constant]
        // type TechAcc: Get<Self::AccountId>;

        type AssetIdGenerator: AssetIdGenerator<Self::AssetId>;

        type MinBalance: Get<Self::Balance>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AssetCreated(T::AssetId),
    }

    #[pallet::error]
    pub enum Error<T> {
        FailedToCreateAsset,
        NoTechAccFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    impl<T: Config> bridge_types::traits::BridgeAssetRegistry<T::AccountId, T::AssetId> for Pallet<T> {
        type AssetName = Vec<u8>;
        type AssetSymbol = Vec<u8>;

        fn register_asset(
            network_id: GenericNetworkId,
            name: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, T::AssetId>>::AssetName,
            symbol: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, T::AssetId>>::AssetSymbol,
        ) -> Result<T::AssetId, DispatchError> {
            let nonce = Self::asset_nonce();
            AssetNonce::<T>::set(nonce + 1);
            let Some(tech_acc) = Self::tech_acc(network_id) else {
                fail!(Error::<T>::NoTechAccFound)
            };
            let iter = 3;
            for i in 0..iter {
                let hash = {
                    let mut vector = name.clone();
                    vector.extend_from_slice(&symbol);
                    vector.extend_from_slice(&(nonce + i).encode());
                    let hash = blake2_256(&vector);
                    H256::from_slice(&hash)
                };
                let asset_id = T::AssetIdGenerator::generate_asset_id(hash);
                let res = <pallet_assets::Pallet<T> as Create<T::AccountId>>::create(
                    asset_id,
                    tech_acc.clone(),
                    true,
                    T::MinBalance::get(),
                );
                if res.is_ok() {
                    <pallet_assets::Pallet<T> as MetadataMutate<T::AccountId>>::set(
                        asset_id, &tech_acc, name, symbol, 18,
                    )?;
                    Self::deposit_event(Event::AssetCreated(asset_id));
                    return Ok(asset_id);
                }
            }
            fail!(Error::<T>::FailedToCreateAsset)
        }

        fn manage_asset(_: GenericNetworkId, _: T::AssetId) -> Result<(), DispatchError> {
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
            network_id: GenericNetworkId,
            asset_kind: bridge_types::types::AssetKind,
            who: &T::AccountId,
            asset_id: &Self::AssetId,
            amount: &Self::Balance,
        ) -> DispatchResult {
            // let ta = T::TechAcc::get();
            let Some(tech_acc) = Self::tech_acc(network_id) else {
                fail!(Error::<T>::NoTechAccFound)
            };
            match asset_kind {
                bridge_types::types::AssetKind::Thischain => {
                    <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
                        asset_id.clone(),
                        &who,
                        &tech_acc,
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
            network_id: GenericNetworkId,
            asset_kind: bridge_types::types::AssetKind,
            who: &T::AccountId,
            asset_id: &Self::AssetId,
            amount: &Self::Balance,
        ) -> DispatchResult {
            // let ta = T::TechAcc::get();
            let Some(tech_acc) = Self::tech_acc(network_id) else {
                fail!(Error::<T>::NoTechAccFound)
            };
            match asset_kind {
                bridge_types::types::AssetKind::Thischain => {
                    <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
                        asset_id.clone(),
                        &tech_acc,
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
}

pub trait AssetIdGenerator<T> {
    fn generate_asset_id(hash: H256) -> T;
}
