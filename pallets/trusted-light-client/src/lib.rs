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

#![cfg_attr(not(feature = "std"), no_std)]

use bridge_common::simplified_proof::*;
use bridge_common::{beefy_types::*, bitfield, simplified_proof::Proof};
use bridge_types::types::AuxiliaryDigest;
use bridge_types::types::AuxiliaryDigestItem;
use bridge_types::{GenericNetworkId, SubNetworkId};
use codec::Decode;
use codec::Encode;
use frame_support::ensure;
use frame_support::fail;
use frame_support::log;
use frame_support::pallet_prelude::*;
use frame_support::traits::Randomness;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::prelude::vec::Vec;
use sp_core::H256;
use sp_core::{Get, RuntimeDebug};
use sp_io::hashing::keccak_256;
use sp_runtime::traits::Hash;
use sp_runtime::traits::Keccak256;
use sp_runtime::DispatchError;
use sp_std::collections::vec_deque::VecDeque;

pub use bitfield::BitField;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(any(test, feature = "runtime-benchmarks"))]
// mod fixtures;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub struct ProvedSubstrateBridgeMessage<Message, Proof> {
    pub message: Message,
    pub proof: Proof,
}

fn recover_signature(sig: &[u8; 65], msg_hash: &H256) -> Option<EthAddress> {
    use sp_io::crypto::secp256k1_ecdsa_recover;

    secp256k1_ecdsa_recover(sig, &msg_hash.0)
        .map(|pubkey| EthAddress::from(H256::from_slice(&keccak_256(&pubkey))))
        .ok()
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_types::SubNetworkId;
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_support::pallet_prelude::OptionQuery;
    use frame_support::{fail, Twox64Concat};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Randomness: frame_support::traits::Randomness<Self::Hash, Self::BlockNumber>;
        type Message: Parameter;
        type Proof: Parameter;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_peer_keys)]
    pub type PeerKeys<T> =
        StorageMap<_, Twox64Concat, SubNetworkId, Vec<EthAddress>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_treshold)]
    pub type Treshold<T> =
        StorageMap<_, Twox64Concat, SubNetworkId, u32, OptionQuery>;


    #[pallet::type_value]
    pub fn DefaultForThisNetworkId() -> SubNetworkId {
        SubNetworkId::Mainnet
    }

    #[pallet::storage]
    #[pallet::getter(fn this_network_id)]
    pub type ThisNetworkId<T> = StorageValue<_, SubNetworkId, ValueQuery, DefaultForThisNetworkId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        VerificationSuccessful(SubNetworkId, T::AccountId, u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidInitParams,
        NetworkNotInitialized,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        /// Network id for current network
        pub network_id: SubNetworkId,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                network_id: SubNetworkId::Mainnet,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            ThisNetworkId::<T>::put(self.network_id);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn initialize(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            keys_treshold: u32,
            keys: Vec<EthAddress>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(keys_treshold > 0, Error::<T>::InvalidInitParams);
            ensure!(keys.len() > 0, Error::<T>::InvalidInitParams);
            ensure!(keys.len() >= keys_treshold as usize, Error::<T>::InvalidInitParams);
            PeerKeys::<T>::set(network_id, Some(keys));
            Treshold::<T>::set(network_id, Some(keys_treshold));
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn verify(
            network_id: SubNetworkId,
            message: &T::Message,
            signatures: Vec<[u8; 65]>,
        ) -> DispatchResultWithPostInfo {
            let Some(peers) = PeerKeys::<T>::get(network_id) else {
                fail!(Error::<T>::NetworkNotInitialized)
            };

            for peer in peers {

            }

            Ok(().into())
        }
    }
}

// impl<T: Config>
//     bridge_types::traits::Verifier<SubNetworkId, ProvedSubstrateBridgeMessage<T::Message>>
//     for Pallet<T>
// {
//     type Result = T::Message;
//     fn verify(
//         network_id: SubNetworkId,
//         message: &ProvedSubstrateBridgeMessage<T::Message>,
//     ) -> Result<Self::Result, DispatchError> {


//         Ok(message.message.clone())
//     }
// }

impl<T: Config>
    bridge_types::traits::VerifierNew<T::Message>
    for Pallet<T>
{
    // type Result = T::Message;
    // type Proof = T::Proof;
    type Proof = Vec<[u8; 65]>;

    fn verify(
        network_id: GenericNetworkId,
        message: &T::Message,
        proof: &Self::Proof,
    ) -> DispatchResult {
        let hash = Keccak256::hash_of(&message);
        Ok(())
    }

    // fn verify(
    //     network_id: SubNetworkId,
    //     message: &Vec<[u8; 65],
    // ) -> DispatchResult {


    //     Ok(().into())
    // }


}
