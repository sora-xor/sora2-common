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

use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use currencies::BasicCurrencyAdapter;

use frame_support::dispatch::DispatchError;
use frame_support::traits::{Everything, GenesisBuild};
use frame_support::{
    assert_noop, assert_ok, parameter_types, Deserialize, RuntimeDebug, Serialize,
};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_keyring::AccountKeyring as Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Verify};
use sp_runtime::{AccountId32, MultiSignature, Perbill};
use sp_std::convert::From;
use sp_std::marker::PhantomData;

use bridge_types::traits::MessageDispatch;
use bridge_types::types::ParachainMessage;
use bridge_types::U256;
use traits::parameter_type_with_key;

use crate::inbound::Error;

use crate::inbound as bridge_inbound_channel;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

const BASE_NETWORK_ID: SubNetworkId = SubNetworkId::Mainnet;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        BridgeInboundChannel: bridge_inbound_channel::{Pallet, Call, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[derive(
    Encode,
    Decode,
    PartialEq,
    Eq,
    RuntimeDebug,
    Clone,
    Copy,
    MaxEncodedLen,
    TypeInfo,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum AssetId {
    XOR,
    ETH,
    DAI,
}

pub type Balance = u128;
pub type Amount = i128;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<65536>;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
        0
    };
}

impl pallet_balances::Config for Test {
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = ();
}

impl tokens::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type CurrencyHooks = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type DustRemovalWhitelist = Everything;
}

impl currencies::Config for Test {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, u64>;
    type GetNativeCurrencyId = GetBaseAssetId;
    type WeightInfo = ();
}
parameter_types! {
    pub const GetBaseAssetId: AssetId = AssetId::XOR;
    pub GetTeamReservesAccountId: AccountId = AccountId32::from([0; 32]);
    pub GetFeesAccountId: AccountId = AccountId32::from([1; 32]);
    pub GetTreasuryAccountId: AccountId = AccountId32::from([2; 32]);
}

// Mock verifier
pub struct MockVerifier;

impl Verifier<SubNetworkId, ParachainMessage<Balance>> for MockVerifier {
    type Result = Vec<ParachainMessage<Balance>>;

    fn verify(
        network_id: SubNetworkId,
        message: &ParachainMessage<Balance>,
    ) -> Result<Self::Result, DispatchError> {
        if network_id == BASE_NETWORK_ID {
            Ok(vec![message.clone()])
        } else {
            Err(Error::<Test>::InvalidNetwork.into())
        }
    }
}

// Mock Dispatch
pub struct MockMessageDispatch;

impl MessageDispatch<Test, SubNetworkId, MessageId, ()> for MockMessageDispatch {
    fn dispatch(_: SubNetworkId, _: MessageId, _: u64, _: &[u8], _: ()) {}

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_dispatch_event(
        _: MessageId,
    ) -> Option<<Test as frame_system::Config>::RuntimeEvent> {
        None
    }
}

parameter_types! {
    pub SourceAccount: AccountId = Keyring::Eve.into();
}

pub struct FeeConverter<T: Config>(PhantomData<T>);

impl<T: Config> Convert<U256, BalanceOf<T>> for FeeConverter<T> {
    fn convert(_: U256) -> BalanceOf<T> {
        100u32.into()
    }
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

impl bridge_inbound_channel::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Verifier = MockVerifier;
    type ProvedMessage = ParachainMessage<Balance>;
    type MessageDispatch = MockMessageDispatch;
    type FeeConverter = FeeConverter<Self>;
    type FeeAssetId = GetBaseAssetId;
    type FeeAccountId = GetFeesAccountId;
    type TreasuryAccountId = GetTreasuryAccountId;
    type Currency = Currencies;
    type WeightInfo = ();
}

pub fn new_tester() -> sp_io::TestExternalities {
    new_tester_with_config(bridge_inbound_channel::GenesisConfig {
        reward_fraction: Perbill::from_percent(80),
    })
}

pub fn new_tester_with_config(
    config: bridge_inbound_channel::GenesisConfig,
) -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    GenesisBuild::<Test>::assimilate_storage(&config, &mut storage).unwrap();

    let bob: AccountId = Keyring::Bob.into();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bob, 1_000_000_000_000_000_000)],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}

#[test]
fn test_submit() {
    new_tester().execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = RuntimeOrigin::signed(relayer);

        // Submit message 1
        let message_1 = ParachainMessage {
            nonce: 1,
            timestamp: 0,
            fee: 0,
            payload: Default::default(),
        };
        assert_ok!(BridgeInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message_1
        ));
        let nonce: u64 = <ChannelNonces<Test>>::get(BASE_NETWORK_ID);
        assert_eq!(nonce, 1);

        // Submit message 2
        let message_2 = ParachainMessage {
            nonce: 2,
            timestamp: 0,
            fee: 0,
            payload: Default::default(),
        };
        assert_ok!(BridgeInboundChannel::submit(
            origin,
            BASE_NETWORK_ID,
            message_2
        ));
        let nonce: u64 = <ChannelNonces<Test>>::get(BASE_NETWORK_ID);
        assert_eq!(nonce, 2);
    });
}

#[test]
fn test_submit_with_invalid_nonce() {
    new_tester().execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = RuntimeOrigin::signed(relayer);

        // Submit message
        let message = ParachainMessage {
            nonce: 1,
            timestamp: 0,
            fee: 0,
            payload: Default::default(),
        };
        assert_ok!(BridgeInboundChannel::submit(
            origin.clone(),
            BASE_NETWORK_ID,
            message.clone()
        ));
        let nonce: u64 = <ChannelNonces<Test>>::get(BASE_NETWORK_ID);
        assert_eq!(nonce, 1);

        // Submit the same again
        assert_noop!(
            BridgeInboundChannel::submit(origin, BASE_NETWORK_ID, message),
            Error::<Test>::InvalidNonce
        );
    });
}

#[test]
#[ignore] // TODO: fix test_handle_fee test
fn test_handle_fee() {
    new_tester().execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let fee_asset_id = <Test as Config>::FeeAssetId::get();
        let treasury_acc = <Test as Config>::TreasuryAccountId::get();
        let fees_acc = <Test as Config>::FeeAccountId::get();

        Currencies::deposit(fee_asset_id, &fees_acc, 10_000).unwrap();

        let fee = 1000; // 1 DOT

        BridgeInboundChannel::handle_fee(fee, &relayer);
        assert_eq!(Currencies::total_balance(fee_asset_id, &treasury_acc), 200);
        assert_eq!(Currencies::total_balance(fee_asset_id, &relayer), 800);
    });
}

#[test]
fn test_set_reward_fraction_not_authorized() {
    new_tester().execute_with(|| {
        let bob: AccountId = Keyring::Bob.into();
        assert_noop!(
            BridgeInboundChannel::set_reward_fraction(
                RuntimeOrigin::signed(bob),
                Perbill::from_percent(60)
            ),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn test_submit_with_invalid_network_id() {
    new_tester().execute_with(|| {
        let relayer: AccountId = Keyring::Bob.into();
        let origin = RuntimeOrigin::signed(relayer);

        // Submit message
        let message = ParachainMessage {
            nonce: 1,
            timestamp: 0,
            fee: 0,
            payload: Default::default(),
        };
        assert_noop!(
            BridgeInboundChannel::submit(origin, SubNetworkId::Kusama, message),
            Error::<Test>::InvalidNetwork
        );
    });
}
