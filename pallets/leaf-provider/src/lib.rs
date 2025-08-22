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
#![doc = include_str!("../README.md")]
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
pub const RANDOMNESS_SUBJECT: &[u8] = b"beefy-leaf-extra";

/// A type that is able to return current list of parachain heads that end up in the MMR leaf.

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]

    use bridge_types::traits::AuxiliaryDigestHandler;
    use bridge_types::types::{AuxiliaryDigest, AuxiliaryDigestItem, LeafExtraData};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use frame_system::pallet_prelude::*;
    use sp_beefy::mmr::BeefyDataProvider;
    use sp_runtime::traits;
    use sp_runtime::traits::Hash;
    use sp_std::prelude::*;

    use crate::RANDOMNESS_SUBJECT;

    type HashOf<T> = <T as Config>::Hash;
    type RandomnessOutputOf<T> = <T as frame_system::Config>::Hash;

    /// BEEFY-MMR pallet.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Latest digest
    #[pallet::storage]
    #[pallet::getter(fn latest_digest)]
    pub(super) type LatestDigest<T: Config> =
        StorageValue<_, Vec<AuxiliaryDigestItem>, OptionQuery>;

    /// The module's configuration trait.
    #[pallet::config]
    #[pallet::disable_frame_system_supertrait_check]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Hashing: traits::Hash<Output = <Self as Config>::Hash>;
        type Hash: traits::Member
            + traits::MaybeSerializeDeserialize
            + sp_std::fmt::Debug
            + sp_std::hash::Hash
            + AsRef<[u8]>
            + AsMut<[u8]>
            + Copy
            + Default
            + codec::Codec
            + codec::EncodeLike
            + scale_info::TypeInfo
            + MaxEncodedLen;

        type Randomness: Randomness<RandomnessOutputOf<Self>, Self::BlockNumber>;
    }

    #[pallet::event]
    pub enum Event<T: Config> {}

    impl<T: Config> AuxiliaryDigestHandler for Pallet<T> {
        fn add_item(item: AuxiliaryDigestItem) {
            LatestDigest::<T>::append(item);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Clear the latest digest. This pallet should be placed before any other pallets which is use AuxiliaryDigestHandler.
        fn on_initialize(_now: T::BlockNumber) -> Weight {
            LatestDigest::<T>::kill();
            <T as frame_system::Config>::DbWeight::get().writes(1)
        }
    }

    impl<T: Config> BeefyDataProvider<LeafExtraData<HashOf<T>, RandomnessOutputOf<T>>> for Pallet<T> {
        fn extra_data() -> LeafExtraData<HashOf<T>, RandomnessOutputOf<T>> {
            let digest = AuxiliaryDigest {
                logs: LatestDigest::<T>::get().unwrap_or_default(),
            };
            let digest_encoded = digest.encode();
            let (random_seed, _) = T::Randomness::random(RANDOMNESS_SUBJECT);
            let digest_hash = <T as Config>::Hashing::hash(&digest_encoded);
            LeafExtraData {
                random_seed,
                digest_hash,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridge_types::{types::AuxiliaryDigestItem, GenericNetworkId, H256};
    use frame_support::{parameter_types, traits::Randomness};
    use frame_system as system;
    use sp_core::H256 as Hash256;
    use sp_runtime::traits::BlakeTwo256;

    type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
            LeafProvider: pallet::{Pallet, Storage, Event<T>},
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    pub struct TestRandomness;
    impl frame_support::traits::Randomness<Hash256, u64> for TestRandomness {
        fn random(_subject: &[u8]) -> (Hash256, u64) {
            (Hash256::repeat_byte(42), 0)
        }
    }

    impl system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = Hash256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
        type Header = sp_runtime::testing::Header;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type DbWeight = ();
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
    }

    impl pallet::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Hashing = BlakeTwo256;
        type Hash = Hash256;
        type Randomness = TestRandomness;
    }

    fn new_ext() -> sp_io::TestExternalities {
        let t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    #[test]
    fn on_initialize_clears_latest_digest() {
        new_ext().execute_with(|| {
            // Seed with a digest item
            pallet::LatestDigest::<Test>::put(vec![AuxiliaryDigestItem::Commitment(
                GenericNetworkId::Sub(Default::default()),
                H256::repeat_byte(1),
            )]);
            // Call hook
            <pallet::Pallet<Test> as frame_support::traits::Hooks<u64>>::on_initialize(1);
            assert_eq!(pallet::LatestDigest::<Test>::get(), None);
        });
    }

    #[test]
    fn add_item_appends_to_latest_digest() {
        new_ext().execute_with(|| {
            let item = AuxiliaryDigestItem::Commitment(
                GenericNetworkId::Sub(Default::default()),
                H256::repeat_byte(2),
            );
            <pallet::Pallet<Test> as bridge_types::traits::AuxiliaryDigestHandler>::add_item(item);
            let stored = pallet::LatestDigest::<Test>::get().unwrap();
            assert_eq!(stored.len(), 1);
            assert_eq!(stored[0], item);
        });
    }

    #[test]
    fn extra_data_produces_expected_hash_and_seed() {
        new_ext().execute_with(|| {
            // Seed two items
            let i1 = AuxiliaryDigestItem::Commitment(
                GenericNetworkId::Sub(Default::default()),
                H256::repeat_byte(3),
            );
            let i2 = AuxiliaryDigestItem::Commitment(
                GenericNetworkId::Sub(Default::default()),
                H256::repeat_byte(4),
            );
            pallet::LatestDigest::<Test>::put(vec![i1, i2]);

            let extra = <pallet::Pallet<Test> as sp_beefy::mmr::BeefyDataProvider<
                bridge_types::types::LeafExtraData<Hash256, Hash256>,
            >>::extra_data();

            // Compute expected digest hash
            use codec::Encode;
            let digest = bridge_types::types::AuxiliaryDigest { logs: vec![i1, i2] };
            let expected_hash = <BlakeTwo256 as sp_runtime::traits::Hash>::hash(&digest.encode());
            assert_eq!(extra.digest_hash, expected_hash);

            // Randomness is deterministic in this test
            let (seed, _bn) = <Test as pallet::Config>::Randomness::random(RANDOMNESS_SUBJECT);
            assert_eq!(extra.random_seed, seed);
        });
    }
}
