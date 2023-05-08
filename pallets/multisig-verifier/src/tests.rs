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

use crate::{mock::*, Error, Proof};
use bridge_types::{SubNetworkId, types::AuxiliaryDigest};

use codec::Decode;
use frame_support::{assert_noop, assert_ok};
// use hex_literal::hex;
use sp_core::ecdsa;
use sp_runtime::traits::{Keccak256, Hash};
// use test_case::test_case;

fn alice<T: crate::Config>() -> T::AccountId {
    T::AccountId::decode(&mut [0u8; 32].as_slice()).unwrap()
}

fn test_peers() -> Vec<ecdsa::Public> {
    vec![
        ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1,
        ]),
        ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 2,
        ]),
        ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 3,
        ]),
        ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 4,
        ]),
        ecdsa::Public::from_raw([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 5,
        ]),
    ]
}

#[test]
fn it_works_initialize_pallet() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                test_peers(),
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
                vec![],
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
                test_peers(),
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
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                test_peers(),
            ),
            ().into()
        );

        let key = test_peers().last().unwrap().clone();

        assert_ok!(
            TrustedVerifier::remove_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            ().into()
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
fn it_fails_delete_peer_not_existing() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                test_peers(),
            ),
            ().into()
        );

        let key = test_peers().last().unwrap().clone();

        assert_ok!(
            TrustedVerifier::remove_peer(RuntimeOrigin::signed(alice::<Test>()), key,),
            ().into()
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
fn it_works_verify() {
    new_test_ext().execute_with(|| {
        assert_ok!(
            TrustedVerifier::initialize(
                RuntimeOrigin::root(),
                bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet),
                test_peers(),
            ),
            ().into()
        );
        let proof = crate::Proof {
            digest: AuxiliaryDigest {
                logs: Vec::new()
            },
            proof: Vec::new(),
        };
        let mes = bridge_types::substrate::BridgeMessage {
            payload: Vec::new(),
            nonce: 0,
            timestamp: 0,
            fee: 0,
        };
        let messages = vec![mes];
        let hash = Keccak256::hash_of(&messages);
        assert_ok!(<TrustedVerifier as bridge_types::traits::Verifier>::verify(bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet), hash, &proof));
    });
}
