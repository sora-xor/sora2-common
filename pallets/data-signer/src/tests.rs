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

use crate::{mock::*, Error, Event};
use bridge_types::{SubNetworkId};
use frame_support::{assert_noop, assert_ok};
use codec::Decode;
use sp_core::{ecdsa, Pair, bounded::{BoundedVec}};
use frame_system::Config;

fn alice<T: crate::Config>() -> T::AccountId {
    T::AccountId::decode(&mut [0u8; 32].as_slice()).unwrap()
}

fn test_peers() -> Vec<ecdsa::Public> {
    vec![
        ecdsa::Pair::generate_with_phrase(Some("password")),
        ecdsa::Pair::generate_with_phrase(Some("password1")),
        ecdsa::Pair::generate_with_phrase(Some("password2")),
        ecdsa::Pair::generate_with_phrase(Some("password3")),
        ecdsa::Pair::generate_with_phrase(Some("password4")),
        ecdsa::Pair::generate_with_phrase(Some("password5")),
    ]
    .into_iter()
    .map(|(x, _, _)| x.public())
    .collect()
}

#[test]
fn it_works_register_network() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let peers: BoundedVec<ecdsa::Public, BridgeMaxPeers> = test_peers().try_into().unwrap();

        assert_ok!(DataSigner::register_network(
            RuntimeOrigin::root(), 
            network_id,
            peers.clone(),
        ));

        assert!(DataSigner::peers(network_id).is_some());
        assert!(DataSigner::peers(network_id).unwrap().len() == peers.len());
    });
}

#[test]
fn it_works_register_network_with_empty_peers() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);
        let peers: BoundedVec<ecdsa::Public, BridgeMaxPeers> = vec![].try_into().unwrap();

        assert_ok!(DataSigner::register_network(
            RuntimeOrigin::root(), 
            network_id,
            peers,
        ));

        assert!(DataSigner::peers(network_id).is_some());
        assert!(DataSigner::peers(network_id).unwrap().is_empty());
    });
}

#[test]
fn it_fails_register_network_alredy_initialized() {
    new_test_ext().execute_with(|| {
        let network_id = bridge_types::GenericNetworkId::Sub(SubNetworkId::Mainnet);

        assert_ok!(DataSigner::register_network(
            RuntimeOrigin::root(), 
            network_id,
            test_peers().try_into().unwrap(),
        ));

        assert_noop!(
            DataSigner::register_network(
                RuntimeOrigin::root(), 
                network_id,
                test_peers().try_into().unwrap(),
            ),
            Error::<Test>::PalletInitialized
        );
    });
}