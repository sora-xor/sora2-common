use super::*;
use currencies::BasicCurrencyAdapter;

use frame_support::dispatch::DispatchError;
use frame_support::traits::{Everything, GenesisBuild};
use frame_support::{assert_ok, parameter_types};
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

use common::mock::ExistentialDeposits;
use common::{balance, Amount, AssetId32, AssetName, AssetSymbol, DEXId, FromGenericPair, XOR};

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
        Assets: assets::{Pallet, Call, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        Technical: technical::{Pallet, Call, Config<T>, Event<T>},
        Permissions: permissions::{Pallet, Call, Config<T>, Storage, Event<T>},
        BridgeInboundChannel: bridge_inbound_channel::{Pallet, Call, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
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

impl pallet_balances::Config for Test {
    /// The ubiquitous event type.
    type Event = Event;
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

impl common::Config for Test {
    type DEXId = common::DEXId;
    type LstId = common::LiquiditySourceType;
}

impl permissions::Config for Test {
    type Event = Event;
}

impl tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = <Test as assets::Config>::AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
    type DustRemovalWhitelist = Everything;
}

impl currencies::Config for Test {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, u64>;
    type GetNativeCurrencyId = <Test as assets::Config>::GetBaseAssetId;
    type WeightInfo = ();
}
parameter_types! {
    pub const GetBaseAssetId: AssetId = XOR;
    pub GetTeamReservesAccountId: AccountId = AccountId32::from([0; 32]);
}

type AssetId = AssetId32<common::PredefinedAssetId>;

impl assets::Config for Test {
    type Event = Event;
    type ExtraAccountId = [u8; 32];
    type ExtraAssetRecordArg =
        common::AssetIdExtraAssetRecordArg<DEXId, common::LiquiditySourceType, [u8; 32]>;
    type AssetId = AssetId;
    type GetBaseAssetId = GetBaseAssetId;
    type Currency = currencies::Pallet<Test>;
    type GetTeamReservesAccountId = GetTeamReservesAccountId;
    type WeightInfo = ();
    type GetTotalBalance = ();
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
    fn successful_dispatch_event(_: MessageId) -> Option<<Test as frame_system::Config>::Event> {
        None
    }
}

parameter_types! {
    pub SourceAccount: AccountId = Keyring::Eve.into();
    pub TreasuryAccount: AccountId = Keyring::Dave.into();
    pub GetTrustlessBridgeFeesTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_FEES.to_vec(),
        );
        tech_account_id
    };
    pub GetTrustlessBridgeFeesAccountId: AccountId = {
        let tech_account_id = GetTrustlessBridgeFeesTechAccountId::get();
        let account_id =
            technical::Pallet::<Test>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
    pub GetTreasuryTechAccountId: TechAccountId = {
        let tech_account_id = TechAccountId::from_generic_pair(
            bridge_types::types::TECH_ACCOUNT_TREASURY_PREFIX.to_vec(),
            bridge_types::types::TECH_ACCOUNT_MAIN.to_vec(),
        );
        tech_account_id
    };
    pub GetTreasuryAccountId: AccountId = {
        let tech_account_id = GetTreasuryTechAccountId::get();
        let account_id =
            technical::Pallet::<Test>::tech_account_id_to_account_id(&tech_account_id)
                .expect("Failed to get ordinary account id for technical account id.");
        account_id
    };
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
    type Event = Event;
    type Verifier = MockVerifier;
    type MessageDispatch = MockMessageDispatch;
    type FeeConverter = FeeConverter<Self>;
    type FeeAssetId = ();
    type FeeTechAccountId = GetTrustlessBridgeFeesTechAccountId;
    type TreasuryTechAccountId = GetTreasuryTechAccountId;
    type WeightInfo = ();
}

pub type TechAccountId = common::TechAccountId<AccountId, TechAssetId, DEXId>;
pub type TechAssetId = common::TechAssetId<common::PredefinedAssetId>;

impl technical::Config for Test {
    type Event = Event;
    type TechAssetId = TechAssetId;
    type TechAccountId = TechAccountId;
    type Trigger = ();
    type Condition = ();
    type SwapAction = ();
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

    technical::GenesisConfig::<Test> {
        register_tech_accounts: vec![
            (
                GetTrustlessBridgeFeesAccountId::get(),
                GetTrustlessBridgeFeesTechAccountId::get(),
            ),
            (GetTreasuryAccountId::get(), GetTreasuryTechAccountId::get()),
        ],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(&config, &mut storage).unwrap();

    let bob: AccountId = Keyring::Bob.into();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bob.clone(), balance!(1))],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    assets::GenesisConfig::<Test> {
        endowed_assets: vec![(
            XOR.into(),
            bob,
            AssetSymbol(b"XOR".to_vec()),
            AssetName(b"SORA".to_vec()),
            18,
            0,
            true,
            None,
            None,
        )],
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
        let origin = Origin::signed(relayer);

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
            origin.clone(),
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
        let origin = Origin::signed(relayer);

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
        common::assert_noop_transactional!(
            BridgeInboundChannel::submit(origin.clone(), BASE_NETWORK_ID, message.clone()),
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
        let treasury_acc = <Test as Config>::TreasuryTechAccountId::get();
        let fees_acc = <Test as Config>::FeeTechAccountId::get();

        technical::Pallet::<Test>::mint(&fee_asset_id, &fees_acc, balance!(10)).unwrap();

        let fee = balance!(1); // 1 DOT

        BridgeInboundChannel::handle_fee(fee, &relayer);
        assert_eq!(
            technical::Pallet::<Test>::total_balance(&fee_asset_id, &treasury_acc,).unwrap(),
            balance!(0.2)
        );
        assert_eq!(
            assets::Pallet::<Test>::total_balance(&fee_asset_id, &relayer).unwrap(),
            balance!(0.8)
        );
    });
}

#[test]
fn test_set_reward_fraction_not_authorized() {
    new_tester().execute_with(|| {
        let bob: AccountId = Keyring::Bob.into();
        common::assert_noop_transactional!(
            BridgeInboundChannel::set_reward_fraction(
                Origin::signed(bob),
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
        let origin = Origin::signed(relayer);

        // Submit message
        let message = ParachainMessage {
            nonce: 1,
            timestamp: 0,
            fee: 0,
            payload: Default::default(),
        };
        common::assert_noop_transactional!(
            BridgeInboundChannel::submit(origin.clone(), SubNetworkId::Kusama, message.clone()),
            Error::<Test>::InvalidNetwork
        );
    });
}
