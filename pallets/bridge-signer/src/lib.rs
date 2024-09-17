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

use bridge_types::{
    multisig::{MultiSignature, MultiSignatures, MultiSigners},
    substrate::BridgeSignerCall,
    GenericNetworkId,
};
use codec::Encode;
use frame_support::{ensure, weights::Weight};
pub use pallet::*;
use sp_core::Get;
use sp_runtime::DispatchResult;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
use sp_core::H256;
pub use weights::WeightInfo;

impl<T: Config> From<BridgeSignerCall> for Call<T> {
    fn from(value: BridgeSignerCall) -> Self {
        match value {
            BridgeSignerCall::AddPeer { peer } => Call::add_peer_internal { peer },
            BridgeSignerCall::RemovePeer { peer } => Call::remove_peer_internal { peer },
            BridgeSignerCall::FinishAddPeer => Call::finish_add_peer {},
            BridgeSignerCall::FinishRemovePeer => Call::finish_remove_peer {},
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]

    use super::WeightInfo;
    use bridge_types::multisig::*;
    use bridge_types::types::CallOriginOutput;
    use bridge_types::types::GenericAdditionalInboundData;
    use bridge_types::{GenericNetworkId, H256};
    use frame_support::dispatch::Pays;
    use frame_support::pallet_prelude::*;
    use frame_support::weights::Weight;
    use frame_system::ensure_root;
    use frame_system::pallet_prelude::BlockNumberFor;
    use frame_system::pallet_prelude::*;
    use sp_core::Get;
    use sp_runtime::Saturating;

    /// BEEFY-MMR pallet.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// The module's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type CallOrigin: EnsureOrigin<
            Self::RuntimeOrigin,
            Success = CallOriginOutput<GenericNetworkId, H256, GenericAdditionalInboundData>,
        >;

        #[pallet::constant]
        type MaxPeers: Get<u32>;

        #[pallet::constant]
        type ThisNetworkId: Get<GenericNetworkId>;

        #[pallet::constant]
        type ApprovalCleanUpPeriod: Get<BlockNumberFor<Self>>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: BlockNumberFor<T>) -> Weight {
            let mut removed = 0;
            for ((network_id, hash), _) in CleanSchedule::<T>::drain_prefix(now) {
                Approvals::<T>::remove(network_id, hash);
                removed += 1;
            }
            T::DbWeight::get().reads_writes(removed * 2, removed * 2)
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Initialized {
            network_id: GenericNetworkId,
            peers: MultiSigners<T::MaxPeers>,
        },
        AddedPeer {
            network_id: GenericNetworkId,
            peer: MultiSigner,
        },
        RemovedPeer {
            network_id: GenericNetworkId,
            peer: MultiSigner,
        },
        ApprovalAccepted {
            network_id: GenericNetworkId,
            data: H256,
            signature: MultiSignature,
        },
        Approved {
            network_id: GenericNetworkId,
            data: H256,
            signatures: MultiSignatures<T::MaxPeers>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        PalletInitialized,
        PalletNotInitialized,
        PeerExists,
        PeerNotExists,
        TooMuchPeers,
        FailedToVerifySignature,
        PeerNotFound,
        TooMuchApprovals,
        ApprovalsNotFound,
        SignaturesNotFound,
        HasPendingPeerUpdate,
        DontHavePendingPeerUpdates,
        NetworkNotSupported,
        AlreadyApproved,
        CallerIsNotWhitelisted,
        CallerAlreadyWhitelisted,
        InvalidProof,
    }

    /// Peers
    #[pallet::storage]
    #[pallet::getter(fn peers)]
    pub(super) type Peers<T: Config> =
        StorageMap<_, Identity, GenericNetworkId, MultiSigners<T::MaxPeers>, OptionQuery>;

    /// Whitelist for approve call
    #[pallet::storage]
    #[pallet::getter(fn whitelist)]
    pub(super) type Whitelist<T: Config> =
        StorageDoubleMap<_, Identity, GenericNetworkId, Identity, T::AccountId, bool, ValueQuery>;

    /// Pending peers
    #[pallet::storage]
    #[pallet::getter(fn pending_peer_update)]
    pub(super) type PendingPeerUpdate<T: Config> =
        StorageMap<_, Identity, GenericNetworkId, MultiSigner, OptionQuery>;

    /// Approvals
    #[pallet::storage]
    #[pallet::getter(fn approvals)]
    pub(super) type Approvals<T: Config> = StorageDoubleMap<
        _,
        Identity,
        GenericNetworkId,
        Identity,
        H256,
        MultiSignatures<T::MaxPeers>,
        OptionQuery,
    >;

    /// Schedule for approvals cleaning
    #[pallet::storage]
    #[pallet::getter(fn schedule)]
    pub(super) type CleanSchedule<T: Config> = StorageDoubleMap<
        _,
        Identity,
        BlockNumberFor<T>,
        Identity,
        (GenericNetworkId, H256),
        bool,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::register_network())]
        pub fn register_network(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            peers: MultiSigners<T::MaxPeers>,
            callers: BoundedVec<T::AccountId, T::MaxPeers>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Peers::<T>::try_mutate(network_id, |storage_peers| {
                if storage_peers.is_some() {
                    return Err(Error::<T>::PalletInitialized);
                } else {
                    *storage_peers = Some(peers.clone());
                }
                Ok(())
            })?;
            for caller in callers {
                Whitelist::<T>::insert(network_id, caller, true);
            }
            Self::deposit_event(Event::<T>::Initialized { network_id, peers });
            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::approve())]
        pub fn approve(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            data: H256,
            signature: MultiSignature,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::whitelist(network_id, who),
                Error::<T>::CallerIsNotWhitelisted
            );
            let (approvals, is_approved) = Self::verify_approval(&network_id, &data, &signature)?;
            Approvals::<T>::insert(network_id, data, &approvals);
            if is_approved {
                Self::deposit_event(Event::<T>::Approved {
                    network_id,
                    data,
                    signatures: approvals,
                });
            } else {
                Self::deposit_event(Event::<T>::ApprovalAccepted {
                    network_id,
                    data,
                    signature,
                });
            }
            let clean_up_block = frame_system::Pallet::<T>::block_number()
                .saturating_add(T::ApprovalCleanUpPeriod::get());
            CleanSchedule::<T>::insert(clean_up_block, (network_id, data), true);
            Ok(Pays::No.into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::add_peer())]
        pub fn add_peer(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            peer: MultiSigner,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                PendingPeerUpdate::<T>::get(network_id).is_none(),
                Error::<T>::HasPendingPeerUpdate
            );
            Peers::<T>::try_mutate(network_id, |peers| {
                if let Some(peers) = peers {
                    if peers.contains(&peer) {
                        return Err(Error::<T>::PeerExists);
                    } else {
                        ensure!(peers.add_peer(peer.clone()), Error::<T>::TooMuchPeers);
                    }
                } else {
                    return Err(Error::<T>::NetworkNotSupported);
                }
                Ok(())
            })?;
            PendingPeerUpdate::<T>::insert(network_id, peer);
            // TODO: Send peer update
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::remove_peer())]
        pub fn remove_peer(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            peer: MultiSigner,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                PendingPeerUpdate::<T>::get(network_id).is_none(),
                Error::<T>::HasPendingPeerUpdate
            );
            // Do nothing to ensure we have enough approvals for remove peer request
            // Will be actually removed after request from sidechain
            PendingPeerUpdate::<T>::insert(network_id, peer);
            // TODO: Send peer update
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::finish_remove_peer())]
        pub fn finish_remove_peer(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin)?;
            let peer = PendingPeerUpdate::<T>::take(network_id)
                .ok_or(Error::<T>::DontHavePendingPeerUpdates)?;
            Peers::<T>::try_mutate(network_id, |peers| {
                if let Some(peers) = peers {
                    ensure!(peers.remove_peer(&peer), Error::<T>::PeerNotExists);
                } else {
                    return Err(Error::<T>::PalletNotInitialized);
                }
                Ok(())
            })?;
            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::WeightInfo::finish_add_peer())]
        pub fn finish_add_peer(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin)?;
            let peer = PendingPeerUpdate::<T>::take(network_id)
                .ok_or(Error::<T>::DontHavePendingPeerUpdates)?;
            let peers = Peers::<T>::get(network_id).ok_or(Error::<T>::PalletNotInitialized)?;
            ensure!(peers.contains(&peer), Error::<T>::PeerNotExists);
            Ok(().into())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(<T as Config>::WeightInfo::finish_remove_peer())]
        pub fn remove_peer_internal(
            origin: OriginFor<T>,
            peer: MultiSigner,
        ) -> DispatchResultWithPostInfo {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin)?;
            Peers::<T>::try_mutate(network_id, |peers| {
                if let Some(peers) = peers {
                    ensure!(peers.remove_peer(&peer), Error::<T>::PeerNotExists);
                } else {
                    return Err(Error::<T>::PalletNotInitialized);
                }
                Ok(())
            })?;
            // TODO: Send peer update
            Ok(().into())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(<T as Config>::WeightInfo::finish_add_peer())]
        pub fn add_peer_internal(
            origin: OriginFor<T>,
            peer: MultiSigner,
        ) -> DispatchResultWithPostInfo {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin)?;
            Peers::<T>::try_mutate(network_id, |peers| {
                if let Some(peers) = peers {
                    if peers.contains(&peer) {
                        return Err(Error::<T>::PeerExists);
                    } else {
                        ensure!(peers.add_peer(peer), Error::<T>::TooMuchPeers);
                    }
                } else {
                    return Err(Error::<T>::NetworkNotSupported);
                }
                Ok(())
            })?;
            // TODO: Send peer update
            Ok(().into())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(<T as Config>::WeightInfo::add_peer())]
        pub fn add_whitelisted_caller(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            caller: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Peers::<T>::get(network_id).ok_or(Error::<T>::NetworkNotSupported)?;
            ensure!(
                !Self::whitelist(network_id, caller.clone()),
                Error::<T>::CallerAlreadyWhitelisted
            );
            Whitelist::<T>::insert(network_id, caller, true);
            Ok(().into())
        }

        #[pallet::call_index(9)]
        #[pallet::weight(<T as Config>::WeightInfo::add_peer())]
        pub fn remove_whitelisted_caller(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            caller: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Peers::<T>::get(network_id).ok_or(Error::<T>::NetworkNotSupported)?;
            ensure!(
                Self::whitelist(network_id, caller.clone()),
                Error::<T>::CallerIsNotWhitelisted
            );
            Whitelist::<T>::remove(network_id, caller);
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn verify_approval(
        network_id: &GenericNetworkId,
        data: &H256,
        signature: &MultiSignature,
    ) -> Result<(MultiSignatures<T::MaxPeers>, bool), Error<T>> {
        ensure!(signature.verify(*data), Error::<T>::FailedToVerifySignature);
        let peers = Peers::<T>::get(network_id).ok_or(Error::<T>::PalletNotInitialized)?;
        ensure!(
            peers.contains(&signature.public()),
            Error::<T>::PeerNotFound
        );
        let mut approvals = if let Some(approvals) = Approvals::<T>::get(network_id, data) {
            approvals
        } else {
            peers.empty_signatures()
        };
        ensure!(
            !approvals.contains(&signature.public()),
            Error::<T>::AlreadyApproved
        );
        ensure!(
            approvals.add_signature(signature.clone()),
            Error::<T>::TooMuchApprovals
        );
        let is_approved =
            (approvals.len() as u32) >= bridge_types::utils::threshold(peers.len() as u32);
        Ok((approvals, is_approved))
    }
}

impl<T: Config> bridge_types::traits::Verifier for Pallet<T> {
    type Proof = MultiSignatures<T::MaxPeers>;

    fn verify(
        network_id: GenericNetworkId,
        commitment_hash: H256,
        proof: &Self::Proof,
    ) -> DispatchResult {
        let this_network_id = T::ThisNetworkId::get();
        let peers = Peers::<T>::get(network_id).ok_or(Error::<T>::NetworkNotSupported)?;
        let message_to_hash = (network_id, this_network_id, commitment_hash).encode();
        let message_hash = match peers {
            MultiSigners::Ecdsa(_) => sp_io::hashing::keccak_256(&message_to_hash),
            MultiSigners::Ed25519(_) => sp_io::hashing::sha2_256(&message_to_hash),
        };
        ensure!(
            proof.verify(&peers, message_hash.into()),
            Error::<T>::InvalidProof
        );
        Ok(())
    }

    fn verify_weight(_proof: &Self::Proof) -> Weight {
        <T as Config>::WeightInfo::register_network()
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn valid_proof() -> Option<Self::Proof> {
        None
    }
}
