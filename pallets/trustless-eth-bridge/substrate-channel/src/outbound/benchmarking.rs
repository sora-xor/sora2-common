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