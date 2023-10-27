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

/// Subject for randomness.
// pub const RANDOMNESS_SUBJECT: &[u8] = b"beefy-leaf-extra";

/// A type that is able to return current list of parachain heads that end up in the MMR leaf.

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]
    use frame_support::pallet_prelude::*;
    use frame_system::{pallet_prelude::*, Origin};
    // use pallet_assets::Metadata;
    // // use sp_beefy::mmr::BeefyDataProvider;
    // use sp_runtime::traits;
    // use sp_runtime::traits::Hash;
    use sp_std::prelude::*;
    use bridge_types::types::RawAssetInfo;
    use bridge_types::GenericNetworkId;
    use sp_runtime::traits::StaticLookup;

    // use crate::RANDOMNESS_SUBJECT;

    // type HashOf<T> = <T as Config>::Hash;
    // type RandomnessOutputOf<T> = <T as frame_system::Config>::Hash;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // /// Latest digest
    // #[pallet::storage]
    // #[pallet::getter(fn latest_digest)]
    // pub(super) type LatestDigest<T: Config> =
    //     StorageValue<_, Vec<AuxiliaryDigestItem>, OptionQuery>;

    /// The module's configuration trait.
    #[pallet::config]
    #[pallet::disable_frame_system_supertrait_check]
    pub trait Config: frame_system::Config + pallet_assets::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // type Hashing: traits::Hash<Output = <Self as Config>::Hash>;
        // type Hash: traits::Member
        //     + traits::MaybeSerializeDeserialize
        //     + sp_std::fmt::Debug
        //     + sp_std::hash::Hash
        //     + AsRef<[u8]>
        //     + AsMut<[u8]>
        //     + Copy
        //     + Default
        //     + codec::Codec
        //     + codec::EncodeLike
        //     + scale_info::TypeInfo
        //     + MaxEncodedLen;

        // type Randomness: Randomness<RandomnessOutputOf<Self>, Self::BlockNumber>;
        type TechAcc: Get<Self::AccountId>;
    }

    #[pallet::event]
    pub enum Event<T: Config> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    impl<T: Config> Pallet<T> {
        fn get_asset_info() -> RawAssetInfo {
            todo!()
        }
    }

    // impl<T: Config> bridge_types::traits::BridgeAssetRegistry<T::AccountId, u32> for Pallet<T> {
    //     type AssetName = ();
    //     type AssetSymbol = ();

    //     fn register_asset(
    //         _: GenericNetworkId, 
    //         _: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, u32>>::AssetName, 
    //         _: <Self as bridge_types::traits::BridgeAssetRegistry<T::AccountId, u32>>::AssetSymbol
    //     ) -> Result<u32, DispatchError> { 
    //         Err(DispatchError::Other("NOT AVAILIBLE"))
    //     }
    
    //     fn manage_asset(
    //         _: GenericNetworkId, 
    //         _: u32
    //     ) -> Result<(), DispatchError> { 
    //         Err(DispatchError::Other("NOT AVAILIBLE"))
    //     }
    
    //     fn get_raw_info(
    //         asset_id: u32
    //     ) -> bridge_types::types::RawAssetInfo { 
    //         // let a = <crate::Assets as crate::Runtime>::Metadata::get(asset_id);
    //         // let a = crate::Assets::Metadata::get(asset_id);
    //         // let a = crate::Asset;
    //         // todo!();
    //         // bridge_types::types::RawAssetInfo {
    //         //     name: Vec::new(),
    //         //     symbol: Vec::new(),
    //         //     precision: 0,
    //         // }\
    //         // let asset_metadata = <T as pallet_assets>::Metadata::get(asset_id);
    //         // let asset_metadata = <T as pallet_assets>::Metadata::get(asset_id);
    //         let a = pallet_assets::Pallet::<T>::metadata();
    //         todo!()
    //     }
    // }

    impl<T: Config> bridge_types::traits::BridgeAssetLocker<T::AccountId> for Pallet<T> {
        type AssetId = T::AssetId;
        type Balance = T::Balance;
    
        fn lock_asset(
                _network_id: GenericNetworkId,
                _asset_kind: bridge_types::types::AssetKind,
                who: &T::AccountId,
                asset_id: &Self::AssetId,
                amount: &Self::Balance,
            ) -> DispatchResult {
            // todo!()
            let ta = T::TechAcc::get();
            // let origin: frame_system::RawOrigin<<T as Config>::AccountId> = Origin::Signed(who.clone());
            let origin: frame_system::RawOrigin<T::AccountId> = Origin::<T>::Signed(who.clone());
            let ta_lookup = <T::Lookup as StaticLookup>::unlookup(ta);
            pallet_assets::Pallet::<T>::transfer(origin.into(), asset_id.clone().into(), ta_lookup, amount.clone())?;
            Ok(())
        }
    
        fn unlock_asset(
                _network_id: GenericNetworkId,
                _asset_kind: bridge_types::types::AssetKind,
                who: &T::AccountId,
                asset_id: &Self::AssetId,
                amount: &Self::Balance,
            ) -> DispatchResult {
            // todo!()
            let ta = T::TechAcc::get();
            let origin: frame_system::RawOrigin<T::AccountId> = Origin::<T>::Signed(ta);
            let ta_lookup = <T::Lookup as StaticLookup>::unlookup(who.clone());
            pallet_assets::Pallet::<T>::transfer(origin.into(), asset_id.clone().into(), ta_lookup, amount.clone())?;
            Ok(())
        }
    }

}
