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

use bridge_types::GenericNetworkId;
use bridge_types::LiberlandAssetId;
use frame_support::fail;
use frame_support::pallet_prelude::*;
use frame_support::traits::fungibles::{
    metadata::Mutate as MetadataMutate, Create, Inspect, InspectMetadata, Mutate, Transfer,
};
use frame_support::traits::Currency;
use frame_support::traits::ExistenceRequirement;
use frame_support::traits::WithdrawReasons;
pub use pallet::*;
use sp_core::H256;
use sp_io::hashing::blake2_256;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]
    use bridge_types::GenericNetworkId;
    use frame_support::pallet_prelude::{ValueQuery, *};
    use frame_system::pallet_prelude::*;

    pub type AssetIdOf<T> = <T as Config>::AssetId;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn tech_acc)]
    pub(super) type TechAccounts<T: Config> =
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

        type MinBalance: Get<<Self as pallet_assets::Config>::Balance>;

        type AssetId: Member
            + Parameter
            + Copy
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + From<<Self as pallet_assets::Config>::AssetId>;

        type Balances: frame_support::traits::Currency<Self::AccountId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AssetCreated(AssetIdOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        FailedToCreateAsset,
        NoTechAccFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub register_tech_accounts: Vec<(GenericNetworkId, T::AccountId)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                register_tech_accounts: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.register_tech_accounts.iter().for_each(|(k, v)| {
                TechAccounts::<T>::insert(k, v);
            });
        }
    }
}

impl<T: Config> bridge_types::traits::BridgeAssetRegistry<T::AccountId, LiberlandAssetId>
    for Pallet<T>
where
    <T as pallet_assets::Config>::AssetId: Into<u32> + From<u32>,
    AssetIdOf<T>: From<LiberlandAssetId>,
{
    type AssetName = Vec<u8>;
    type AssetSymbol = Vec<u8>;

    fn register_asset(
        network_id: GenericNetworkId,
        name: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, LiberlandAssetId>>::AssetName,
        symbol: <Self as bridge_types::traits::BridgeAssetRegistry<
            T::AccountId,
            LiberlandAssetId,
        >>::AssetSymbol,
    ) -> Result<LiberlandAssetId, DispatchError> {
        let nonce = Self::asset_nonce();
        AssetNonce::<T>::set(nonce + 1);
        let Some(tech_acc) = Self::tech_acc(network_id) else {
            fail!(Error::<T>::NoTechAccFound)
        };
        // let's take 3  itrations to create a new asset id, considering that collision can happen
        let iter = 3;
        for i in 0..iter {
            let hash = {
                let mut vector = name.clone();
                vector.extend_from_slice(&symbol);
                vector.extend_from_slice(&(nonce + i).encode());
                let hash = blake2_256(&vector);
                H256::from_slice(&hash)
            };
            let asset_id = {
                let arr: [u8; 4] = hash[..4].try_into().unwrap_or_default();
                u32::from_be_bytes(arr)
            };
            let res = <pallet_assets::Pallet<T> as Create<T::AccountId>>::create(
                asset_id.into(),
                tech_acc.clone(),
                true,
                T::MinBalance::get(),
            );
            if res.is_ok() {
                <pallet_assets::Pallet<T> as MetadataMutate<T::AccountId>>::set(
                    asset_id.into(),
                    &tech_acc,
                    name,
                    symbol,
                    18,
                )?;
                Self::deposit_event(Event::AssetCreated(
                    LiberlandAssetId::Asset(asset_id.into()).into(),
                ));
                return Ok(LiberlandAssetId::Asset(asset_id.into()));
            }
        }
        fail!(Error::<T>::FailedToCreateAsset)
    }

    fn manage_asset(_: GenericNetworkId, _: LiberlandAssetId) -> Result<(), DispatchError> {
        Ok(())
    }

    fn ensure_asset_exists(asset_id: LiberlandAssetId) -> bool {
        match asset_id {
            LiberlandAssetId::LLD => true,
            LiberlandAssetId::Asset(asset_id) => {
                <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::asset_exists(asset_id.into())
            }
        }
    }

    fn get_raw_info(asset_id: LiberlandAssetId) -> bridge_types::types::RawAssetInfo {
        match asset_id {
            LiberlandAssetId::LLD => bridge_types::types::RawAssetInfo {
                name: b"Liberland".to_vec(),
                symbol: b"LLD".to_vec(),
                precision: 12,
            },
            LiberlandAssetId::Asset(asset_id) => {
                let name = <pallet_assets::Pallet<T> as InspectMetadata<T::AccountId>>::name(
                    &asset_id.into(),
                );
                let symbol = pallet_assets::Pallet::<T>::symbol(&asset_id.into());
                let precision = pallet_assets::Pallet::<T>::decimals(&asset_id.into());
                bridge_types::types::RawAssetInfo {
                    name,
                    symbol,
                    precision,
                }
            }
        }
    }
}

impl<T: Config> bridge_types::traits::BridgeAssetLocker<T::AccountId> for Pallet<T>
    where <T as pallet_assets::Config>::AssetId: Into<u32> + From<u32>,
    <T as pallet_assets::Config>::Balance: Into<<<T as pallet::Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance>,
{
    type AssetId = LiberlandAssetId;
    type Balance = <T as pallet_assets::Config>::Balance;

    fn lock_asset(
        network_id: GenericNetworkId,
        asset_kind: bridge_types::types::AssetKind,
        who: &T::AccountId,
        asset_id: &Self::AssetId,
        amount: &Self::Balance,
    ) -> DispatchResult {
        let Some(tech_acc) = Self::tech_acc(network_id) else {
            fail!(Error::<T>::NoTechAccFound)
        };
        match asset_id {
                LiberlandAssetId::LLD => {
                match asset_kind {
                    bridge_types::types::AssetKind::Thischain => {
                        T::Balances::transfer(
                            who,
                            &tech_acc,
                            amount.clone().into(),
                            ExistenceRequirement::AllowDeath,
                        )?;
                    },
                    bridge_types::types::AssetKind::Sidechain => {
                        T::Balances::withdraw(
                            who,
                            amount.clone().into(),
                            WithdrawReasons::RESERVE,
                            ExistenceRequirement::AllowDeath,
                        )?;
                    },
                }
            },
            LiberlandAssetId::Asset(asset) => {
                match asset_kind {
                    bridge_types::types::AssetKind::Thischain => {
                        <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
                            asset.clone().into(),
                            &who,
                            &tech_acc,
                            amount.clone(),
                            true,
                        )?;
                    },
                    bridge_types::types::AssetKind::Sidechain => {
                        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::burn_from(
                            asset.clone().into(),
                            &who,
                            amount.clone(),
                        )?;
                    },
                }
            }
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
        let Some(tech_acc) = Self::tech_acc(network_id) else {
            fail!(Error::<T>::NoTechAccFound)
        };
        match asset_id {
            LiberlandAssetId::LLD => {
                match asset_kind {
                    bridge_types::types::AssetKind::Thischain => {
                        T::Balances::transfer(
                            &tech_acc,
                            who,
                            amount.clone().into(),
                            ExistenceRequirement::AllowDeath,
                        )?;
                    },
                    bridge_types::types::AssetKind::Sidechain => {
                        T::Balances::deposit_into_existing(
                            who,
                            amount.clone().into(),
                        )?;
                    },
                }
            },
            LiberlandAssetId::Asset(asset) => {
                match asset_kind {
                    bridge_types::types::AssetKind::Thischain => {
                        <pallet_assets::Pallet<T> as Transfer<T::AccountId>>::transfer(
                            asset.clone().into(),
                            &tech_acc,
                            &who,
                            amount.clone(),
                            true,
                        )?;
                    },
                    bridge_types::types::AssetKind::Sidechain => {
                        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::mint_into(
                            asset.clone().into(),
                            &who,
                            amount.clone(),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}
