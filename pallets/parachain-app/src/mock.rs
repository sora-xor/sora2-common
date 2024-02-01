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

use bridge_types::substrate::ParachainAssetId;
use bridge_types::substrate::PARENT_PARACHAIN_ASSET;
use bridge_types::traits::BalancePrecisionConverter;
use bridge_types::traits::BridgeAssetRegistry;
use bridge_types::traits::BridgeOriginOutput;
use bridge_types::traits::TimepointProvider;
use bridge_types::GenericNetworkId;
use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use currencies::BasicCurrencyAdapter;

// Mock runtime
use bridge_types::types::AssetKind;
use bridge_types::SubNetworkId;
use frame_support::parameter_types;
use frame_support::traits::{Everything, GenesisBuild};
use frame_support::Deserialize;
use frame_support::RuntimeDebug;
use frame_support::Serialize;
use frame_system as system;
use frame_system::Origin;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_keyring::sr25519::Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Keccak256, Verify};
use sp_runtime::{AccountId32, MultiSignature};
use traits::parameter_type_with_key;
use xcm::v3::Junction::GeneralKey;
use xcm::v3::Junction::Parachain;
use xcm::v3::Junctions::X2;
use xcm::v3::MultiLocation;

use crate as substrate_app;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

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
    Custom(u8),
}

pub type Balance = u128;
pub type Amount = i128;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        Dispatch: dispatch::{Pallet, Call, Storage, Origin<T>, Event<T>},
        BridgeOutboundChannel: substrate_bridge_channel::outbound::{Pallet, Config<T>, Storage, Event<T>},
        ParachainApp: substrate_app::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
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
    pub const ExistentialDeposit: u128 = 0;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
        0
    };
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
    pub GetBridgeAccountId: AccountId = AccountId32::from([3; 32]);
}

impl dispatch::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OriginOutput = bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>;
    type Origin = RuntimeOrigin;
    type MessageId = u64;
    type Hashing = Keccak256;
    type Call = RuntimeCall;
    type CallFilter = Everything;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxMessagePayloadSize: u32 = 2048;
    pub const MaxMessagesPerCommit: u32 = 5;
    pub const MaxTotalGasLimit: u64 = 5_000_000;
    pub const Decimals: u32 = 12;
}

parameter_types! {
    pub const FeeCurrency: AssetId = AssetId::XOR;
    pub const ThisNetworkId: GenericNetworkId = GenericNetworkId::Sub(SubNetworkId::Mainnet);
}

pub struct GenericTimepointProvider;

impl TimepointProvider for GenericTimepointProvider {
    fn get_timepoint() -> bridge_types::GenericTimepoint {
        bridge_types::GenericTimepoint::Sora(System::block_number() as u32)
    }
}

impl substrate_bridge_channel::outbound::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerCommit = MaxMessagesPerCommit;
    type MessageStatusNotifier = ();
    type AuxiliaryDigestHandler = ();
    type AssetId = ();
    type Balance = u128;
    type WeightInfo = ();
    type TimepointProvider = GenericTimepointProvider;
    type ThisNetworkId = ThisNetworkId;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

pub struct AssetRegistryImpl;

impl BridgeAssetRegistry<AccountId, AssetId> for AssetRegistryImpl {
    type AssetName = String;
    type AssetSymbol = String;

    fn register_asset(
        _network_id: GenericNetworkId,
        name: Self::AssetName,
        _symbol: Self::AssetSymbol,
    ) -> Result<AssetId, sp_runtime::DispatchError> {
        match name.as_str() {
            "XOR" => Ok(AssetId::XOR),
            "KSM" => Ok(AssetId::Custom(1)),
            _ => Ok(AssetId::Custom(0)),
        }
    }

    fn manage_asset(
        _network_id: GenericNetworkId,
        _asset_id: AssetId,
    ) -> frame_support::pallet_prelude::DispatchResult {
        Ok(())
    }

    fn get_raw_info(asset_id: AssetId) -> bridge_types::types::RawAssetInfo {
        match asset_id {
            AssetId::XOR => bridge_types::types::RawAssetInfo {
                name: "XOR".to_owned().into(),
                symbol: "XOR".to_owned().into(),
                precision: 18,
            },
            AssetId::Custom(1) => bridge_types::types::RawAssetInfo {
                name: "KSM".to_owned().into(),
                symbol: "KSM".to_owned().into(),
                precision: 12,
            },
            _ => bridge_types::types::RawAssetInfo {
                name: Default::default(),
                symbol: Default::default(),
                precision: 18,
            },
        }
    }
}

pub struct BalancePrecisionConverterImpl;

impl BalancePrecisionConverter<AssetId, Balance, Balance> for BalancePrecisionConverterImpl {
    fn to_sidechain(
        asset_id: &AssetId,
        _sidechain_precision: u8,
        amount: Balance,
    ) -> Option<(Balance, Balance)> {
        if matches!(asset_id, AssetId::Custom(_)) {
            Some((amount, amount))
        } else {
            Some((amount, amount * 10))
        }
    }

    fn from_sidechain(
        asset_id: &AssetId,
        _sidechain_precision: u8,
        amount: Balance,
    ) -> Option<(Balance, Balance)> {
        if matches!(asset_id, AssetId::Custom(_)) {
            Some((amount, amount))
        } else {
            Some((amount / 10, amount))
        }
    }
}

impl substrate_app::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type MessageStatusNotifier = ();
    type CallOrigin =
        dispatch::EnsureAccount<bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>>;
    type OutboundChannel = BridgeOutboundChannel;
    type AssetRegistry = AssetRegistryImpl;
    type WeightInfo = ();
    type AccountIdConverter = sp_runtime::traits::ConvertInto;
    type AssetIdConverter = ();
    type BalancePrecisionConverter = BalancePrecisionConverterImpl;
    type BridgeAssetLocker = bridge_types::test_utils::BridgeAssetLockerImpl<Currencies>;
}

pub const PARA_A: u32 = 2000;
pub const PARA_B: u32 = 2001;
pub const PARA_C: u32 = 2002;

pub fn new_tester() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (Keyring::Bob.into(), 1_000_000_000_000_000_000),
            (Keyring::Alice.into(), 1_000_000_000_000_000_000),
        ],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &substrate_bridge_channel::outbound::GenesisConfig { interval: 10 },
        &mut storage,
    )
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    let minimal_xcm_amount = 10;
    let sidechain_asset = ParachainAssetId::Concrete(MultiLocation::new(
        1,
        X2(
            Parachain(1),
            GeneralKey {
                length: 32,
                data: [0u8; 32],
            },
        ),
    ));
    let allowed_parachains = vec![PARA_A, PARA_B];
    ext.execute_with(|| {
        // register assets
        ParachainApp::register_thischain_asset(
            Origin::<Test>::Root.into(),
            SubNetworkId::Kusama,
            AssetId::XOR,
            sidechain_asset,
            allowed_parachains.clone(),
            minimal_xcm_amount,
        )
        .expect("XOR registration failed");
        ParachainApp::register_sidechain_asset(
            Origin::<Test>::Root.into(),
            SubNetworkId::Kusama,
            PARENT_PARACHAIN_ASSET,
            "KSM".to_owned(),
            "KSM".to_owned(),
            12,
            allowed_parachains.clone(),
            minimal_xcm_amount,
        )
        .expect("KSM registration failed");
        let origin_kusama: RuntimeOrigin = dispatch::RawOrigin::new(BridgeOriginOutput::new(
            SubNetworkId::Kusama,
            H256([0; 32]),
            bridge_types::GenericTimepoint::Unknown,
            (),
        ))
        .into();
        ParachainApp::finalize_asset_registration(
            origin_kusama.clone(),
            AssetId::XOR,
            AssetKind::Thischain,
        )
        .expect("XOR registration finalization failed");
        let kusama_asset = substrate_app::RelaychainAsset::<Test>::get(SubNetworkId::Kusama);
        ParachainApp::finalize_asset_registration(
            origin_kusama,
            kusama_asset.unwrap(),
            AssetKind::Sidechain,
        )
        .expect("KSM registration finalization failed");
    });
    ext
}

pub fn new_tester_no_registered_assets() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (Keyring::Bob.into(), 1_000_000_000_000_000_000),
            (Keyring::Alice.into(), 1_000_000_000_000_000_000),
        ],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &substrate_bridge_channel::outbound::GenesisConfig { interval: 10 },
        &mut storage,
    )
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
