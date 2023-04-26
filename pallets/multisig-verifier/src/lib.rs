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

// use bridge_common::simplified_proof::*;
// use bridge_common::beefy_types::*;
use bridge_types::GenericNetworkId;
// use bridge_types::types::AuxiliaryDigest;
// use bridge_types::types::AuxiliaryDigestItem;
// use bridge_types::SubNetworkId;
use codec::Decode;
use codec::Encode;
use frame_support::ensure;
// use frame_support::fail;
// use frame_support::log;
use frame_support::pallet_prelude::*;
// use frame_support::traits::Randomness;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::prelude::vec::Vec;
use sp_core::H256;
use sp_core::RuntimeDebug;
// use sp_io::hashing::keccak_256;
// use sp_runtime::traits::Hash;
// use sp_runtime::traits::Keccak256;
// use sp_runtime::DispatchError;
// use sp_std::collections::vec_deque::VecDeque;
use sp_std::collections::btree_set::BTreeSet;
use sp_core::ecdsa;

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
        StorageMap<_, Twox64Concat, GenericNetworkId, BTreeSet<ecdsa::Public>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_treshold)]
    pub type Treshold<T> =
        StorageMap<_, Twox64Concat, GenericNetworkId, u32, OptionQuery>;


    #[pallet::type_value]
    pub fn DefaultForThisNetworkId() -> GenericNetworkId {
        GenericNetworkId::Sub(SubNetworkId::Mainnet)
    }

    #[pallet::storage]
    #[pallet::getter(fn this_network_id)]
    pub type ThisNetworkId<T> = StorageValue<_, GenericNetworkId, ValueQuery, DefaultForThisNetworkId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NetworkInitialized(GenericNetworkId),
        VerificationSuccessful(GenericNetworkId),

        // Error events:

        NetworkNotInitialized(GenericNetworkId),
        /// NetworkId, Required Number, Current Number
        InvalidNumberOfSignatures(GenericNetworkId, u32, u32),
        InvalidSignature(GenericNetworkId, H256, ecdsa::Signature),
        NotTrustedPeerSignature(GenericNetworkId, H256, ecdsa::Signature, ecdsa::Public),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidInitParams,
        NetworkNotInitialized,
        InvalidNumberOfSignatures,
        InvalidSignature,
        NotTrustedPeerSignature,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        /// Network id for current network
        pub network_id: GenericNetworkId,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                network_id: GenericNetworkId::Sub(SubNetworkId::Mainnet),
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
            network_id: GenericNetworkId,
            keys_treshold: u32,
            keys: Vec<ecdsa::Public>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(keys_treshold > 0, Error::<T>::InvalidInitParams);
            ensure!(keys.len() > 0, Error::<T>::InvalidInitParams);
            ensure!(keys.len() >= keys_treshold as usize, Error::<T>::InvalidInitParams);

            let btree_keys = keys.into_iter().collect();
            PeerKeys::<T>::set(network_id, Some(btree_keys));
            Treshold::<T>::set(network_id, Some(keys_treshold));
            Self::deposit_event(Event::NetworkInitialized(network_id));
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn verify_signatures(
            network_id: GenericNetworkId,
            hash: &H256,
            // signatures: &Vec<[u8; 65]>,
            signatures: &Vec<ecdsa::Signature>,
        ) -> DispatchResult {
            let Some(treshold) = Treshold::<T>::get(network_id) else {
                Self::deposit_event(Event::NetworkNotInitialized(network_id));
                fail!(Error::<T>::NetworkNotInitialized)
            };
            let Some(peers) = PeerKeys::<T>::get(network_id) else {
                Self::deposit_event(Event::NetworkNotInitialized(network_id));
                fail!(Error::<T>::NetworkNotInitialized)
            };

            let len = signatures.len() as u32;
            ensure!(len >= treshold, {
                Self::deposit_event(Event::InvalidNumberOfSignatures(network_id, treshold, len));
                Error::<T>::InvalidNumberOfSignatures
            });

            // Insure that every sighnature exists in the storage
            for sign in signatures {
                // let Some(rec_sign) = recover_signature(&sign, &hash) else {
                let Some(rec_sign) = sign.recover_prehashed(&hash.0) else {
                    Self::deposit_event(Event::InvalidSignature(network_id, *hash, sign.clone()));
                    fail!(Error::<T>::InvalidSignature)
                };
                ensure!(peers.contains(&rec_sign), {
                    Self::deposit_event(Event::NotTrustedPeerSignature(network_id, *hash, sign.clone(), rec_sign));
                    Error::<T>::NotTrustedPeerSignature
                });
            }
            Self::deposit_event(Event::VerificationSuccessful(network_id));

            Ok(().into())
        }
    }
}

impl<T: Config>
    bridge_types::traits::Verifier
    for Pallet<T>
{
    type Proof = Vec<ecdsa::Signature>;
    
    #[inline]
    fn verify(
        network_id: GenericNetworkId,
        hash: H256,
        proof: &Vec<ecdsa::Signature>,
    ) -> DispatchResult {
        Self::verify_signatures(network_id, &hash, proof)?;
        Ok(())
    }
}