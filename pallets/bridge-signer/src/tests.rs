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

use crate::{mock::*, Error};
use bridge_types::{
    multisig::{MultiSignature, MultiSigner, MultiSigners},
    SubNetworkId, H256,
};
use frame_support::{assert_noop, assert_ok};
use sp_core::{bounded_vec, ecdsa, Pair};

fn test_peers() -> (MultiSigners<BridgeMaxPeers>, Vec<ecdsa::Pair>) {
    let pairs: Vec<ecdsa::Pair> = vec![
        ecdsa::Pair::generate_with_phrase(Some("password")),
        ecdsa::Pair::generate_with_phrase(Some("password1")),
        ecdsa::Pair::generate_with_phrase(Some("password2")),
        ecdsa::Pair::generate_with_phrase(Some("password3")),
        ecdsa::Pair::generate_with_phrase(Some("password4")),
        ecdsa::Pair::generate_with_phrase(Some("password5")),
    ]
    .into_iter()
    .map(|(x, _, _)| x)
    .collect();
    let mut peers = MultiSigners::Ecdsa(Default::default());
    for peer in pairs.iter() {
        peers.add_peer(MultiSigner::Ecdsa(peer.public()));
    }
    (peers, pairs)
}

fn test_signer() -> ecdsa::Pair {
    ecdsa::Pair::generate_with_phrase(Some("something")).0
}

#[test]
fn it_works_register_network() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let peers = test_peers().0;

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            Default::default()
        ));

        assert!(BridgeSigner::peers(network_id).is_some());
        assert!(BridgeSigner::peers(network_id).unwrap().len() == peers.len());
    });
}

#[test]
fn it_works_register_network_with_empty_peers() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let peers = MultiSigners::Ecdsa(Default::default());

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            Default::default()
        ));

        assert!(BridgeSigner::peers(network_id).is_some());
        assert!(BridgeSigner::peers(network_id).unwrap().is_empty());
    });
}

#[test]
fn it_fails_register_network_alredy_initialized() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            test_peers().0.try_into().unwrap(),
            Default::default()
        ));

        assert_noop!(
            BridgeSigner::register_network(
                RuntimeOrigin::root(),
                network_id,
                test_peers().0.try_into().unwrap(),
                Default::default()
            ),
            Error::<Test>::PalletInitialized
        );
    });
}

#[test]
fn it_works_approve() {
    new_test_ext().execute_with(|| {
        let sender = sp_keyring::AccountKeyring::Alice.to_account_id();
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, pairs) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            bounded_vec![sender.clone()]
        ));

        let data = [1u8; 32];
        let signature = MultiSignature::Ecdsa(pairs[0].public(), pairs[0].sign_prehashed(&data));
        assert!(BridgeSigner::peers(network_id)
            .unwrap()
            .contains(&pairs[0].public().into()));
        assert!(BridgeSigner::approvals(network_id, H256::from(data)).is_none());

        assert_ok!(BridgeSigner::approve(
            RuntimeOrigin::signed(sender),
            network_id,
            H256::from(data),
            signature,
        ));

        assert!(
            BridgeSigner::approvals(network_id, H256::from(data))
                .unwrap()
                .len()
                == 1
        );
    });
}

#[test]
fn it_fails_approve_nonexisted_peer() {
    new_test_ext().execute_with(|| {
        let sender = sp_keyring::AccountKeyring::Alice.to_account_id();
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            bounded_vec![sender.clone()]
        ));

        let data = [1u8; 32];
        let signature =
            MultiSignature::Ecdsa(test_signer().public(), test_signer().sign_prehashed(&data));
        assert!(BridgeSigner::approvals(network_id, H256::from(data)).is_none());

        assert_noop!(
            BridgeSigner::approve(
                RuntimeOrigin::signed(sender),
                network_id,
                H256::from(data),
                signature,
            ),
            Error::<Test>::FailedToVerifySignature
        );

        assert!(BridgeSigner::approvals(network_id, H256::from(data)).is_none());
    });
}

#[test]
fn it_fails_approve_sign_already_exist() {
    new_test_ext().execute_with(|| {
        let sender = sp_keyring::AccountKeyring::Alice.to_account_id();
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, pairs) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            bounded_vec![sender.clone()]
        ));

        let data = [1u8; 32];
        let signature = MultiSignature::Ecdsa(pairs[0].public(), pairs[0].sign_prehashed(&data));
        assert!(BridgeSigner::approvals(network_id, H256::from(data)).is_none());

        assert_ok!(BridgeSigner::approve(
            RuntimeOrigin::signed(sender.clone()),
            network_id,
            H256::from(data),
            signature.clone(),
        ));

        assert!(
            BridgeSigner::approvals(network_id, H256::from(data))
                .unwrap()
                .len()
                == 1
        );

        assert_noop!(
            BridgeSigner::approve(
                RuntimeOrigin::signed(sender),
                network_id,
                H256::from(data),
                signature,
            ),
            Error::<Test>::AlreadyApproved
        );

        assert!(
            BridgeSigner::approvals(network_id, H256::from(data))
                .unwrap()
                .len()
                == 1
        );
    });
}

#[test]
fn it_works_add_peer() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            Default::default()
        ));

        let new_peer = test_signer().public().into();
        assert_ok!(BridgeSigner::add_peer(
            RuntimeOrigin::root(),
            network_id,
            new_peer,
        ));

        assert!(BridgeSigner::pending_peer_update(network_id).is_some());
    });
}

#[test]
fn it_fails_add_peer_pending_update() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            Default::default()
        ));

        let new_peer = test_signer().public().into();
        assert_ok!(BridgeSigner::add_peer(
            RuntimeOrigin::root(),
            network_id,
            new_peer,
        ));

        // cannot add another peer while pending peer update
        let new_peer = test_signer().public().into();
        assert_noop!(
            BridgeSigner::add_peer(RuntimeOrigin::root(), network_id, new_peer,),
            Error::<Test>::HasPendingPeerUpdate
        );

        assert!(BridgeSigner::pending_peer_update(network_id).is_some());
    });
}

#[test]
fn it_fails_add_peer_already_exists() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            Default::default()
        ));

        let peer = peers.signers()[0].clone();
        assert_noop!(
            BridgeSigner::add_peer(RuntimeOrigin::root(), network_id, peer,),
            Error::<Test>::PeerExists
        );

        assert!(BridgeSigner::pending_peer_update(network_id).is_none());
    });
}

#[test]
fn it_works_remove_peer() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            Default::default()
        ));

        let peer = peers.signers()[0].clone();
        assert_ok!(BridgeSigner::remove_peer(
            RuntimeOrigin::root(),
            network_id,
            peer,
        ));

        assert!(BridgeSigner::pending_peer_update(network_id).is_some());
    });
}

#[test]
fn it_fails_remove_peer_pending_update() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            Default::default()
        ));

        let peer = peers.signers()[0].clone();
        assert_ok!(BridgeSigner::remove_peer(
            RuntimeOrigin::root(),
            network_id,
            peer,
        ));

        // cannot remove another peer while pending peer update
        let peer = peers.signers()[1].clone();
        assert_noop!(
            BridgeSigner::remove_peer(RuntimeOrigin::root(), network_id, peer,),
            Error::<Test>::HasPendingPeerUpdate
        );

        assert!(BridgeSigner::pending_peer_update(network_id).is_some());
    });
}

#[test]
fn it_works_finish_remove_peer() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            Default::default()
        ));

        let peer = peers.signers()[0].clone();
        assert_ok!(BridgeSigner::remove_peer(
            RuntimeOrigin::root(),
            network_id,
            peer.clone(),
        ));

        assert!(BridgeSigner::pending_peer_update(network_id).is_some());

        assert_ok!(BridgeSigner::finish_remove_peer(RuntimeOrigin::root()));

        assert!(BridgeSigner::pending_peer_update(network_id).is_none());
        assert!(!BridgeSigner::peers(network_id).unwrap().contains(&peer));
    });
}

#[test]
fn it_fails_finish_remove_peer_no_updates() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers.clone(),
            Default::default()
        ));

        assert!(BridgeSigner::pending_peer_update(network_id).is_none());

        assert_noop!(
            BridgeSigner::finish_remove_peer(RuntimeOrigin::root()),
            Error::<Test>::DontHavePendingPeerUpdates
        );
    })
}

#[test]
fn it_fails_finish_remove_not_initialized() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let peer = test_signer().public().into();

        assert_ok!(BridgeSigner::remove_peer(
            RuntimeOrigin::root(),
            network_id,
            peer,
        ));

        assert_noop!(
            BridgeSigner::finish_remove_peer(RuntimeOrigin::root()),
            Error::<Test>::PalletNotInitialized
        );
    })
}

#[test]
fn it_works_finish_add_peer() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            Default::default()
        ));

        let new_peer: MultiSigner = test_signer().public().into();
        assert_ok!(BridgeSigner::add_peer(
            RuntimeOrigin::root(),
            network_id,
            new_peer.clone(),
        ));

        assert!(BridgeSigner::pending_peer_update(network_id).is_some());

        assert_ok!(BridgeSigner::finish_add_peer(RuntimeOrigin::root(),));

        assert!(BridgeSigner::pending_peer_update(network_id).is_none());
        assert!(BridgeSigner::peers(network_id).unwrap().contains(&new_peer));
    });
}

#[test]
fn it_fails_add_peer_no_pending_update() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let (peers, _) = test_peers();

        assert_ok!(BridgeSigner::register_network(
            RuntimeOrigin::root(),
            network_id,
            peers,
            Default::default()
        ));

        assert_noop!(
            BridgeSigner::finish_add_peer(RuntimeOrigin::root()),
            Error::<Test>::DontHavePendingPeerUpdates
        );
    });
}
