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

//! BridgeOutboundChannel pallet benchmarking
use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::traits::OnInitialize;
use frame_system::RawOrigin;

const BASE_NETWORK_ID: SubNetworkId = SubNetworkId::Mainnet;

#[allow(unused_imports)]
use crate::outbound::Pallet as BridgeOutboundChannel;

benchmarks! {
    // Benchmark `on_initialize` under worst case conditions, i.e. messages
    // in queue are committed.
    on_initialize {
        let m in 1 .. T::MaxMessagesPerCommit::get() as u32;
        let p in 0 .. T::MaxMessagePayloadSize::get() as u32;

        for _ in 0 .. m {
            let payload: Vec<u8> = (0..).take(p as usize).collect();
            append_message_queue::<T>(BASE_NETWORK_ID, ParachainMessage {
                nonce: 0u64,
                fee: Default::default(),
                payload,
                timestamp: Default::default(),
            });
        }

        let block_number = 0u32.into();

    }: { BridgeOutboundChannel::<T>::on_initialize(block_number) }
    verify {
        assert_eq!(<MessageQueues<T>>::get(BASE_NETWORK_ID).len(), 0);
    }

    // Benchmark 'on_initialize` for the best case, i.e. nothing is done
    // because it's not a commitment interval.
    on_initialize_non_interval {
        take_message_queue::<T>(BASE_NETWORK_ID);
        let payload: Vec<u8> = (0..).take(10).collect();
        append_message_queue::<T>(BASE_NETWORK_ID, ParachainMessage {
            nonce: 0u64,
            fee: Default::default(),
            payload,
            timestamp: Default::default(),
        });

        let interval: T::BlockNumber = 10u32.into();
        Interval::<T>::put(interval);
        let block_number: T::BlockNumber = 12u32.into();

    }: { BridgeOutboundChannel::<T>::on_initialize(block_number) }
    verify {
        assert_eq!(<MessageQueues<T>>::get(BASE_NETWORK_ID).len(), 1);
    }

    // Benchmark 'on_initialize` for the case where it is a commitment interval
    // but there are no messages in the queue.
    on_initialize_no_messages {
        take_message_queue::<T>(BASE_NETWORK_ID);

        let block_number = Interval::<T>::get();

    }: { BridgeOutboundChannel::<T>::on_initialize(block_number.into()) }

    // Benchmark `set_fee` under worst case conditions:
    // * The origin is authorized, i.e. equals SetFeeOrigin
    set_fee {
        let new_fee: BalanceOf<T> = 32000000u32.into();
        assert!(<Fee<T>>::get() != new_fee);

    }: _(RawOrigin::Root, new_fee)
    verify {
        assert_eq!(<Fee<T>>::get(), new_fee);
    }
}

impl_benchmark_test_suite!(
    BridgeOutboundChannel,
    crate::outbound::test::new_tester(),
    crate::outbound::test::Test,
);
