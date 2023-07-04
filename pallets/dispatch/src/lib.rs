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

use frame_support::dispatch::{DispatchResult, Dispatchable, Parameter};
use frame_support::traits::{Contains, EnsureOrigin};

use sp_core::RuntimeDebug;

use sp_std::prelude::*;

use bridge_types::traits;

use bridge_types::H256;
use codec::{Decode, Encode};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    RuntimeDebug,
    scale_info::TypeInfo,
    codec::MaxEncodedLen,
)]
pub struct RawOrigin<
    NetworkId,
    Additional,
    OriginOutput: traits::OriginOutput<NetworkId, Additional>,
> {
    pub origin: OriginOutput,
    network_id: sp_std::marker::PhantomData<NetworkId>,
    additional: sp_std::marker::PhantomData<Additional>,
}

impl<NetworkId, Additional, OriginOutput: traits::OriginOutput<NetworkId, Additional>>
    RawOrigin<NetworkId, Additional, OriginOutput>
{
    pub fn new(origin: OriginOutput) -> Self {
        Self {
            origin,
            network_id: Default::default(),
            additional: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct EnsureAccount<
    NetworkId,
    Additional,
    OriginOutput: traits::OriginOutput<NetworkId, Additional>,
>(sp_std::marker::PhantomData<(NetworkId, Additional, OriginOutput)>);

impl<
        NetworkId,
        Additional,
        OuterOrigin,
        OriginOutput: traits::OriginOutput<NetworkId, Additional>,
    > EnsureOrigin<OuterOrigin> for EnsureAccount<NetworkId, Additional, OriginOutput>
where
    OuterOrigin: Into<Result<RawOrigin<NetworkId, Additional, OriginOutput>, OuterOrigin>>
        + From<RawOrigin<NetworkId, Additional, OriginOutput>>,
{
    type Success = OriginOutput;

    fn try_origin(o: OuterOrigin) -> Result<Self::Success, OuterOrigin> {
        o.into().map(|o| o.origin)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<OuterOrigin, ()> {
        Err(())
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use bridge_types::GenericTimepoint;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::StorageVersion;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Hash;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(_);

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self, I>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The Id of the network (i.e. Ethereum network id).
        type NetworkId;

        /// The additional data for origin.
        type Additional;

        type OriginOutput: traits::OriginOutput<Self::NetworkId, Self::Additional>;

        /// The overarching origin type.
        type Origin: From<RawOrigin<Self::NetworkId, Self::Additional, Self::OriginOutput>>;

        /// Id of the message. Whenever message is passed to the dispatch module, it emits
        /// event with this id + dispatch result.
        type MessageId: Parameter;

        type Hashing: Hash<Output = H256>;

        /// The overarching dispatch call type.
        type Call: Parameter
            + Dispatchable<
                RuntimeOrigin = <Self as Config<I>>::Origin,
                PostInfo = frame_support::dispatch::PostDispatchInfo,
            >;

        /// The pallet will filter all incoming calls right before they're dispatched. If this filter
        /// rejects the call, special event (`Event::MessageRejected`) is emitted.
        type CallFilter: Contains<<Self as Config<I>>::Call>;
    }

    #[pallet::hooks]
    impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        /// Message has been dispatched with given result.
        MessageDispatched(T::MessageId, DispatchResult),
        /// Message has been rejected
        MessageRejected(T::MessageId),
        /// We have failed to decode a Call from the message.
        MessageDecodeFailed(T::MessageId),
    }

    #[pallet::origin]
    #[allow(type_alias_bounds)]
    pub type Origin<T: Config<I>, I: 'static = ()> = RawOrigin<
        <T as Config<I>>::NetworkId,
        <T as Config<I>>::Additional,
        <T as Config<I>>::OriginOutput,
    >;

    impl<T: Config<I>, I: 'static>
        traits::MessageDispatch<T, T::NetworkId, T::MessageId, T::Additional> for Pallet<T, I>
    {
        fn dispatch(
            network_id: T::NetworkId,
            message_id: T::MessageId,
            timepoint: GenericTimepoint,
            payload: &[u8],
            additional: T::Additional,
        ) {
            let call = match <T as Config<I>>::Call::decode(&mut &payload[..]) {
                Ok(call) => call,
                Err(_) => {
                    Self::deposit_event(Event::MessageDecodeFailed(message_id));
                    return;
                }
            };

            if !T::CallFilter::contains(&call) {
                Self::deposit_event(Event::MessageRejected(message_id));
                return;
            }

            let origin = RawOrigin::new(<T::OriginOutput as traits::OriginOutput<_, _>>::new(
                network_id,
                message_id.using_encoded(|v| <T as Config<I>>::Hashing::hash(v)),
                timepoint,
                additional,
            ))
            .into();
            let result = call.dispatch(origin);

            Self::deposit_event(Event::MessageDispatched(
                message_id,
                result.map(drop).map_err(|e| e.error),
            ));
        }

        #[cfg(feature = "runtime-benchmarks")]
        fn successful_dispatch_event(
            id: T::MessageId,
        ) -> Option<<T as frame_system::Config>::RuntimeEvent> {
            let event: <T as Config<I>>::RuntimeEvent =
                Event::<T, I>::MessageDispatched(id, Ok(())).into();
            Some(event.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridge_types::evm::AdditionalEVMInboundData;
    use bridge_types::traits::MessageDispatch as _;
    use bridge_types::{types, SubNetworkId};
    use bridge_types::{EVMChainId, H160};
    use frame_support::dispatch::DispatchError;
    use frame_support::parameter_types;
    use frame_support::traits::{ConstU32, Everything};
    use frame_system::{EventRecord, Phase};
    use sp_core::H256;
    use sp_runtime::testing::Header;
    use sp_runtime::traits::{BlakeTwo256, IdentityLookup, Keccak256};

    use crate as dispatch;

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Storage, Event<T>},
            Dispatch: dispatch::{Pallet, Storage, Origin<T>, Event<T>},
        }
    );

    type AccountId = u64;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl frame_system::Config for Test {
        type RuntimeOrigin = RuntimeOrigin;
        type Index = u64;
        type RuntimeCall = RuntimeCall;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = AccountId;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Header = Header;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type BaseCallFilter = Everything;
        type SystemWeightInfo = ();
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<65536>;
    }

    pub struct CallFilter;
    impl frame_support::traits::Contains<RuntimeCall> for CallFilter {
        fn contains(call: &RuntimeCall) -> bool {
            match call {
                RuntimeCall::System(frame_system::pallet::Call::<Test>::remark { .. }) => true,
                _ => false,
            }
        }
    }

    impl dispatch::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type NetworkId = EVMChainId;
        type Additional = AdditionalEVMInboundData;
        type OriginOutput = types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>;
        type Origin = RuntimeOrigin;
        type MessageId = types::MessageId;
        type Hashing = Keccak256;
        type Call = RuntimeCall;
        type CallFilter = CallFilter;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        sp_io::TestExternalities::new(t)
    }

    #[test]
    fn test_dispatch_bridge_message() {
        new_test_ext().execute_with(|| {
            let id = types::MessageId::batched(SubNetworkId::Mainnet.into(), SubNetworkId::Rococo.into(), 1, 37);
            let source = H160::repeat_byte(7);

            let message =
                RuntimeCall::System(frame_system::pallet::Call::<Test>::remark { remark: vec![] })
                    .encode();

            System::set_block_number(1);
            Dispatch::dispatch(
                2u32.into(),
                id,
                Default::default(),
                &message,
                AdditionalEVMInboundData { source },
            );

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::Initialization,
                    event: RuntimeEvent::Dispatch(crate::Event::<Test>::MessageDispatched(
                        id,
                        Err(DispatchError::BadOrigin)
                    )),
                    topics: vec![],
                }],
            );
        })
    }

    #[test]
    fn test_message_decode_failed() {
        new_test_ext().execute_with(|| {
            let id = types::MessageId::batched(SubNetworkId::Mainnet.into(), SubNetworkId::Rococo.into(), 1, 37);
            let source = H160::repeat_byte(7);

            let message: Vec<u8> = vec![1, 2, 3];

            System::set_block_number(1);
            Dispatch::dispatch(
                2u32.into(),
                id,
                Default::default(),
                &message,
                AdditionalEVMInboundData { source },
            );

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::Initialization,
                    event: RuntimeEvent::Dispatch(crate::Event::<Test>::MessageDecodeFailed(id)),
                    topics: vec![],
                }],
            );
        })
    }

    #[test]
    fn test_message_rejected() {
        new_test_ext().execute_with(|| {
            let id = types::MessageId::batched(SubNetworkId::Mainnet.into(), SubNetworkId::Rococo.into(), 1, 37);
            let source = H160::repeat_byte(7);

            let message =
                RuntimeCall::System(frame_system::pallet::Call::<Test>::set_code { code: vec![] })
                    .encode();

            System::set_block_number(1);
            Dispatch::dispatch(
                2u32.into(),
                id,
                Default::default(),
                &message,
                AdditionalEVMInboundData { source },
            );

            assert_eq!(
                System::events(),
                vec![EventRecord {
                    phase: Phase::Initialization,
                    event: RuntimeEvent::Dispatch(crate::Event::<Test>::MessageRejected(id)),
                    topics: vec![],
                }],
            );
        })
    }
}
