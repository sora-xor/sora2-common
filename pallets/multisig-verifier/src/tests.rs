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
use bridge_types::SubNetworkId;

use codec::Decode;
use frame_support::{assert_noop, assert_ok};
use sp_core::{ecdsa, Pair};
use sp_runtime::traits::{Hash, Keccak256};

fn alice<T: crate::Config>() -> T::AccountId {
    T::AccountId::decode(&mut [0u8; 32].as_slice()).unwrap()
}

fn slices() -> Vec<[u8; 32]>{
    vec![
        *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIZZt",
        *b"1RNFJVT1dwshqEPiS1FEd6I1qPywe9UM",
        *b"iBaviI7joIV2QxyqpIOuWOi2OfTek7kg",
        *b"9qLMBglJ5Wercu5xnzV6aAaz8Y44Bvpv",
        *b"76r750mUYogsQimzSjWVaFK5wkQvD6Oh",
    ]
}

fn test_pairs() -> Vec<ecdsa::Pair> {
    slices()
        .into_iter()
        .map(|x| ecdsa::Pair::from_seed(&x))
        .collect()
}

fn test_peers() -> Vec<ecdsa::Public> {
    test_pairs()
        .into_iter()
        .map(|x| x.public())
        .collect()
}

#[test]
fn it_works_initialize_pallet() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                test_peers().try_into().unwrap(),
            ),
            ().into()
        )
    });
}

#[test]
fn it_fails_initialize_pallet_empty_signatures() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                vec![].try_into().unwrap(),
            ),
            Error::<Test>::InvalidInitParams
        );
    });
}

#[test]
fn it_works_add_peer() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                test_peers().try_into().unwrap(),
            ),
            ().into()
        );

        let key = ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 6,
        ]);

        assert_ok!(
            TrustedVerifier::add_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            ().into()
        );

        assert!(
            TrustedVerifier::get_peer_keys(bridge_types::GenericNetworkId::Sub(
                SubNetworkId::Mainnet,
            ))
            .expect("it_works_add_peer: error reading pallet storage")
            .contains(&key)
        );
    });
}

#[test]
fn it_fails_add_peer_not_initialized() {
    new_test_ext().execute_with(|| {
        let key = ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 6,
        ]);

        assert_noop!(
            TrustedVerifier::add_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            Error::<Test>::NetworkNotInitialized
        );
    });
}

#[test]
fn it_works_delete_peer() {
    new_test_ext().execute_with(|| {
        let peers = test_peers();
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                peers.clone().try_into().unwrap(),
            ),
            ().into()
        );

        let key = peers.last().unwrap().clone();

        assert_ok!(
            TrustedVerifier::remove_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            ().into()
        );

        // check if already deleted
        assert_noop!(
            TrustedVerifier::remove_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            Error::<Test>::NoSuchPeer
        );

        assert!(
            !TrustedVerifier::get_peer_keys(bridge_types::GenericNetworkId::Sub(
                SubNetworkId::Mainnet,
            ))
            .expect("it_works_add_peer: error reading pallet storage")
            .contains(&key)
        );
    });
}

#[test]
fn it_fails_delete_peer_not_initialized() {
    new_test_ext().execute_with(|| {
        let key = test_peers().last().unwrap().clone();

        assert_noop!(
            TrustedVerifier::remove_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            Error::<Test>::NetworkNotInitialized
        );
    });
}

#[test]
fn it_works_verify_signatures() {
    new_test_ext().execute_with(|| {
        let pairs = test_pairs();
        let peers: Vec<ecdsa::Public> = pairs.clone().into_iter().map(|x| x.public()).collect();
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                peers.try_into().unwrap(),
            ),
            ().into()
        );

        let hash = Keccak256::hash_of(&"");
        let signatures: Vec<ecdsa::Signature> = pairs.into_iter().map(|x| x.sign_prehashed(&hash.0)).collect();

        assert_ok!(TrustedVerifier::verify_signatures(
            bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
            hash,
            &signatures,
        ));
    });
}

#[test]
fn it_fails_verify_dublicated_signatures() {
    new_test_ext().execute_with(|| {
        let pairs = test_pairs();
        let peers: Vec<ecdsa::Public> = pairs.clone().into_iter().map(|x| x.public()).collect();
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                peers.try_into().unwrap(),
            ),
            ().into()
        );

        let hash = Keccak256::hash_of(&"");
        let signatures: Vec<ecdsa::Signature> = vec![
            *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIZZt",
            *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIZZt",
            *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIZZt",
            *b"9qLMBglJ5Wercu5xnzV6aAaz8Y44Bvpv",
            *b"76r750mUYogsQimzSjWVaFK5wkQvD6Oh",
        ].into_iter()
        .map(|x| ecdsa::Pair::from_seed(&x).sign_prehashed(&hash.0))
        .collect();
        
        assert_noop!(TrustedVerifier::verify_signatures(
            bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
            hash,
            &signatures,
        ), Error::<Test>::DuplicatedPeer);
    });
}

#[test]
fn it_fails_verify_dublicated_peer() {
    new_test_ext().execute_with(|| {
        let pairs = test_pairs();
        let peers: Vec<ecdsa::Public> = pairs.clone().into_iter().map(|x| x.public()).collect();
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                peers.try_into().unwrap(),
            ),
            ().into()
        );

        let hash = Keccak256::hash_of(&"");
        let pairs: Vec<ecdsa::Pair> = vec![
            *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIZZt",
            *b"1RNFJVT1dwshqEPiS1FEd6I1qPywe9UM",
            *b"iBaviI7joIV2QxyqpIOuWOi2OfTek7kg",
            *b"9qLMBglJ5Wercu5xnzV6aAaz8Y44Bvpv",
        ].into_iter()
        .map(|x| ecdsa::Pair::from_seed(&x))
        .collect();

        let dup_pair = pairs[0].clone();
        let mut signatures: Vec<ecdsa::Signature> = pairs.into_iter().map(|x| x.sign_prehashed(&hash.0)).collect();
        // insert dublicated peer
        let dup_sign = dup_pair.sign_prehashed(&hash.0);
        signatures.push(dup_sign);
        
        assert_noop!(TrustedVerifier::verify_signatures(
            bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
            hash,
            &signatures,
        ), Error::<Test>::DuplicatedPeer);
    });
}

#[test]
fn it_fails_verify_not_enough_signatures() {
    new_test_ext().execute_with(|| {
        let pairs = test_pairs();
        let peers: Vec<ecdsa::Public> = pairs.clone().into_iter().map(|x| x.public()).collect();
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                peers.try_into().unwrap(),
            ),
            ().into()
        );

        let hash = Keccak256::hash_of(&"");
        let signatures: Vec<ecdsa::Signature> = vec![
            *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIZZt",
            *b"9qLMBglJ5Wercu5xnzV6aAaz8Y44Bvpv",
            *b"76r750mUYogsQimzSjWVaFK5wkQvD6Oh",
        ].into_iter()
        .map(|x| ecdsa::Pair::from_seed(&x).sign_prehashed(&hash.0))
        .collect();
        
        assert_noop!(TrustedVerifier::verify_signatures(
            bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
            hash,
            &signatures,
        ), Error::<Test>::InvalidNumberOfSignatures);
    });
}

#[test]
fn it_fails_verify_invalid_signature() {
    new_test_ext().execute_with(|| {
        let pairs = test_pairs();
        let peers: Vec<ecdsa::Public> = pairs.clone().into_iter().map(|x| x.public()).collect();
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                peers.try_into().unwrap(),
            ),
            ().into()
        );

        let hash = Keccak256::hash_of(&"");
        let signatures: Vec<ecdsa::Signature> = vec![
            *b"axnzIDWTYX9aKTW0RhLlN8zEFrYdIPPP",
            *b"1RNFJVT1dwshqEPiS1FEd6I1qPywe9UM",
            *b"iBaviI7joIV2QxyqpIOuWOi2OfTek7kg",
            *b"9qLMBglJ5Wercu5xnzV6aAaz8Y44Bvpv",
            *b"76r750mUYogsQimzSjWVaFK5wkQvD6Oh",
        ].into_iter()
        .map(|x| ecdsa::Pair::from_seed(&x).sign_prehashed(&hash.0))
        .collect();
        
        assert_noop!(TrustedVerifier::verify_signatures(
            bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
            hash,
            &signatures,
        ), Error::<Test>::NotTrustedPeerSignature);
    });
}
