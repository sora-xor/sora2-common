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

use bridge_types::substrate::DataSignerCall;
pub use pallet::*;

impl<T: Config> From<DataSignerCall> for Call<T> {
    fn from(value: DataSignerCall) -> Self {
        match value {
            DataSignerCall::AddPeer { peer } => Call::finish_add_peer { peer },
            DataSignerCall::RemovePeer { peer } => Call::finish_remove_peer { peer },
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    #![allow(missing_docs)]

    use bridge_types::substrate::MultisigVerifierCall;
    use bridge_types::substrate::SubstrateBridgeMessageEncode;
    use bridge_types::traits::OutboundChannel;
    use bridge_types::types::CallOriginOutput;
    use bridge_types::{GenericNetworkId, SubNetworkId, H256};
    use frame_support::dispatch::Pays;
    use frame_support::{pallet_prelude::*, BoundedBTreeSet, BoundedVec};
    use frame_system::ensure_root;
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    use sp_core::ecdsa;
    use sp_core::Get;
    use sp_std::collections::btree_set::BTreeSet;

    /// BEEFY-MMR pallet.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// The module's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type OutboundChannel: OutboundChannel<SubNetworkId, Self::AccountId, ()>;

        type CallOrigin: EnsureOrigin<
            Self::RuntimeOrigin,
            Success = CallOriginOutput<SubNetworkId, H256, ()>,
        >;

        #[pallet::constant]
        type MaxPeers: Get<u32>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Initialized {
            network_id: GenericNetworkId,
            peers: BoundedVec<ecdsa::Public, T::MaxPeers>,
        },
        AddedPeer {
            network_id: GenericNetworkId,
            peer: ecdsa::Public,
        },
        RemovedPeer {
            network_id: GenericNetworkId,
            peer: ecdsa::Public,
        },
        ApprovalAccepted {
            network_id: GenericNetworkId,
            data: H256,
            signature: ecdsa::Signature,
        },
        Approved {
            network_id: GenericNetworkId,
            data: H256,
            signatures: BoundedVec<ecdsa::Signature, T::MaxPeers>,
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
    }

    /// Peers
    #[pallet::storage]
    #[pallet::getter(fn peers)]
    pub(super) type Peers<T: Config> = StorageMap<
        _,
        Identity,
        GenericNetworkId,
        BoundedBTreeSet<ecdsa::Public, T::MaxPeers>,
        OptionQuery,
    >;

    /// Pending peers
    #[pallet::storage]
    #[pallet::getter(fn pending_peer_update)]
    pub(super) type PendingPeerUpdate<T: Config> =
        StorageMap<_, Identity, GenericNetworkId, bool, ValueQuery>;

    /// Approvals
    #[pallet::storage]
    #[pallet::getter(fn approvals)]
    pub(super) type Approvals<T: Config> = StorageDoubleMap<
        _,
        Identity,
        GenericNetworkId,
        Identity,
        H256,
        BoundedVec<ecdsa::Signature, T::MaxPeers>,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn register_network(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            peers: BoundedVec<ecdsa::Public, T::MaxPeers>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Peers::<T>::try_mutate(network_id, |storage_peers| {
                if storage_peers.is_some() {
                    return Err(Error::<T>::PalletInitialized);
                } else {
                    *storage_peers = Some(
                        peers
                            .iter()
                            .cloned()
                            .collect::<BTreeSet<_>>()
                            .try_into()
                            .map_err(|_| Error::<T>::TooMuchPeers)?,
                    );
                }
                Ok(())
            })?;
            Self::deposit_event(Event::<T>::Initialized { network_id, peers });
            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn approve(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            data: H256,
            signature: ecdsa::Signature,
        ) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;
            let public = signature
                .recover_prehashed(&data.0)
                .ok_or(Error::<T>::FailedToVerifySignature)?;
            let peers = Peers::<T>::get(network_id).ok_or(Error::<T>::PalletNotInitialized)?;
            ensure!(peers.contains(&public), Error::<T>::PeerNotFound);
            Approvals::<T>::try_append(network_id, data, &signature)
                .map_err(|_| Error::<T>::TooMuchApprovals)?;
            let approvals_len =
                Approvals::<T>::decode_len(network_id, data).unwrap_or_default() as u32;
            let peers_len = peers.len() as u32;
            Self::deposit_event(Event::<T>::ApprovalAccepted {
                network_id,
                data,
                signature,
            });
            if approvals_len >= Self::threshold(peers_len) {
                let signatures = Approvals::<T>::get(network_id, data);
                Self::deposit_event(Event::<T>::Approved {
                    network_id,
                    data,
                    signatures,
                });
            }
            Ok(Pays::No.into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn add_peer(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            peer: ecdsa::Public,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                !PendingPeerUpdate::<T>::get(network_id),
                Error::<T>::HasPendingPeerUpdate
            );
            Peers::<T>::try_mutate(network_id, |peers| {
                if let Some(peers) = peers {
                    if peers.contains(&peer) {
                        return Err(Error::<T>::PeerExists);
                    } else {
                        peers
                            .try_insert(peer)
                            .map_err(|_| Error::<T>::TooMuchPeers)?;
                    }
                } else {
                    return Err(Error::<T>::PalletNotInitialized);
                }
                Ok(())
            })?;
            PendingPeerUpdate::<T>::insert(network_id, true);
            let network_id = network_id.sub().ok_or(Error::<T>::NetworkNotSupported)?;
            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &MultisigVerifierCall::AddPeer { peer }.prepare_message(),
                (),
            )?;
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn remove_peer(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            peer: ecdsa::Public,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ensure!(
                !PendingPeerUpdate::<T>::get(network_id),
                Error::<T>::HasPendingPeerUpdate
            );
            // Do nothing to ensure we have enough approvals for remove peer request
            // Will be actually removed after request from sidechain
            PendingPeerUpdate::<T>::insert(network_id, true);
            let network_id = network_id.sub().ok_or(Error::<T>::NetworkNotSupported)?;
            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &MultisigVerifierCall::RemovePeer { peer }.prepare_message(),
                (),
            )?;
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn finish_remove_peer(
            origin: OriginFor<T>,
            peer: ecdsa::Public,
        ) -> DispatchResultWithPostInfo {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin)?;
            let network_id: GenericNetworkId = network_id.into();
            ensure!(
                PendingPeerUpdate::<T>::get(network_id),
                Error::<T>::DontHavePendingPeerUpdates
            );
            Peers::<T>::try_mutate(network_id, |peers| {
                if let Some(peers) = peers {
                    if !peers.contains(&peer) {
                        return Err(Error::<T>::PeerNotExists);
                    } else {
                        peers.remove(&peer);
                    }
                } else {
                    return Err(Error::<T>::PalletNotInitialized);
                }
                Ok(())
            })?;
            PendingPeerUpdate::<T>::insert(network_id, false);
            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(0)]
        pub fn finish_add_peer(
            origin: OriginFor<T>,
            _peer: ecdsa::Public,
        ) -> DispatchResultWithPostInfo {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin)?;
            let network_id: GenericNetworkId = network_id.into();
            ensure!(
                PendingPeerUpdate::<T>::get(network_id),
                Error::<T>::DontHavePendingPeerUpdates
            );
            PendingPeerUpdate::<T>::insert(network_id, false);
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn threshold(peers: u32) -> u32 {
            let faulty = peers.saturating_sub(1) / 3;
            peers - faulty
        }
    }
}
