//! BridgeInboundChannel pallet benchmarking

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use bridge_types::types::ParachainMessage;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{self, EventRecord, RawOrigin};
use sp_std::prelude::*;

use bridge_types::types::MessageId;

const BASE_NETWORK_ID: SubNetworkId = SubNetworkId::Mainnet;

#[allow(unused_imports)]
use crate::inbound::Pallet as BridgeInboundChannel;

fn assert_last_event<T: Config>(system_event: <T as frame_system::Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

// This collection of benchmarks should include a benchmark for each
// call dispatched by the channel, i.e. each "app" pallet function
// that can be invoked by MessageDispatch. The most expensive call
// should be used in the `submit` benchmark.
//
// We rely on configuration via chain spec of the app pallets because
// we don't have access to their storage here.
benchmarks! {
    // Benchmark `submit` extrinsic under worst case conditions:
    // * `submit` dispatches the DotApp::unlock call
    // * `unlock` call successfully unlocks DOT
    submit {
        let caller: T::AccountId = whitelisted_caller();
        let fee = BalanceOf::<T>::zero();
        let message = ParachainMessage {
            nonce: 1,
            timestamp: 0,
            fee,
            payload: Default::default(),
        };
    }: _(RawOrigin::Signed(caller.clone()), BASE_NETWORK_ID, message)
    verify {
        assert_eq!(1, <ChannelNonces<T>>::get(BASE_NETWORK_ID));

        let message_id = MessageId::inbound(1);
        if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id.into()) {
            assert_last_event::<T>(event);
        }
    }

    // Benchmark `set_reward_fraction` under worst case conditions:
    // * The origin is authorized, i.e. equals UpdateOrigin
    set_reward_fraction {
        // Pick a value that is different from the initial RewardFraction
        let fraction = Perbill::from_percent(50);
        assert!(<RewardFraction<T>>::get() != fraction);

    }: _(RawOrigin::Root, fraction)
    verify {
        assert_eq!(<RewardFraction<T>>::get(), fraction);
    }
}

impl_benchmark_test_suite!(
    BridgeInboundChannel,
    crate::inbound::test::new_tester(),
    crate::inbound::test::Test,
);
