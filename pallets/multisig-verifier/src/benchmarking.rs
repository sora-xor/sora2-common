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

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as MultisigVerifier;
use bridge_types::{GenericNetworkId, SubNetworkId};
use frame_benchmarking::benchmarks;
use frame_support::assert_ok;
use frame_system::{self, RawOrigin};
use sp_core::{ecdsa, Pair};

fn initial_keys<T: Config>(n: usize) -> BoundedVec<ecdsa::Public, <T as Config>::MaxPeers> {
    let mut keys = Vec::new();
    for i in 0..n {
        keys.push(
            ecdsa::Pair::generate_with_phrase(Some(format!("key{}", i).as_str()))
                .0
                .into(),
        );
    }

    keys.try_into().unwrap()
}

fn initialize_network<T: Config>(network_id: GenericNetworkId, n: usize) {
    let keys = initial_keys::<T>(n);
    assert_ok!(MultisigVerifier::<T>::initialize(
        RawOrigin::Root.into(),
        network_id,
        keys
    ));
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
    initialize {
        let n in [0, 20, 40, 60, 80, 100];
        let network_id = bridge_types::GenericNetworkId::Sub(bridge_types::SubNetworkId::Mainnet);
        let keys = initial_keys::<T>(n as usize);
    }: _(RawOrigin::Root, network_id, keys.into())
    verify {
        assert_last_event::<T>(Event::NetworkInitialized(network_id).into())
    }

    add_peer {
        let network_id = bridge_types::GenericNetworkId::Sub(bridge_types::SubNetworkId::Mainnet);

        initialize_network::<T>(network_id, 3);
        let key = ecdsa::Pair::generate_with_phrase(Some("Alice")).0.into();
    }: _(RawOrigin::Root, key)
    verify {
        assert!(MultisigVerifier::<T>::get_peer_keys(GenericNetworkId::Sub(SubNetworkId::Mainnet)).expect("add_peer: No key found").contains(&key));
    }

    remove_peer {
        let network_id = bridge_types::GenericNetworkId::Sub(bridge_types::SubNetworkId::Mainnet);

        initialize_network::<T>(network_id, 3);
        let key = ecdsa::Pair::generate_with_phrase(Some("Alice")).0.into();
        MultisigVerifier::<T>::add_peer(RawOrigin::Root.into(), key).expect("remove_peer: Error adding peer");
    }: _(RawOrigin::Root, key)
    verify {
        assert!(!MultisigVerifier::<T>::get_peer_keys(GenericNetworkId::Sub(SubNetworkId::Mainnet)).expect("add_peer: No key found").contains(&key));
    }

    impl_benchmark_test_suite!(MultisigVerifier, crate::mock::new_test_ext(), mock::Test)
}
