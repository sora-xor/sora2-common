use bridge_types::traits::BridgeAssetRegistry;
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
use scale_info::TypeInfo;
use sp_core::H256;
use sp_keyring::sr25519::Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Keccak256, Verify};
use sp_runtime::{AccountId32, MultiSignature};
use traits::parameter_type_with_key;

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
    Custom,
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
        SubstrateApp: substrate_app::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub const BASE_NETWORK_ID: SubNetworkId = SubNetworkId::Mainnet;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
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
    pub const ExistentialDeposit: u128 = 0;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type Event = Event;
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
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = AssetId;
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
    type Event = Event;
    type NetworkId = SubNetworkId;
    type Additional = ();
    type OriginOutput = bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>;
    type Origin = Origin;
    type MessageId = u64;
    type Hashing = Keccak256;
    type Call = Call;
    type CallFilter = Everything;
}

const INDEXING_PREFIX: &'static [u8] = b"commitment";

parameter_types! {
    pub const MaxMessagePayloadSize: u64 = 2048;
    pub const MaxMessagesPerCommit: u64 = 3;
    pub const MaxTotalGasLimit: u64 = 5_000_000;
    pub const Decimals: u32 = 12;
}

parameter_types! {
    pub const FeeCurrency: AssetId = AssetId::XOR;
}

impl substrate_bridge_channel::outbound::Config for Test {
    const INDEXING_PREFIX: &'static [u8] = INDEXING_PREFIX;
    type Event = Event;
    type Hashing = Keccak256;
    type MaxMessagePayloadSize = MaxMessagePayloadSize;
    type MaxMessagesPerCommit = MaxMessagesPerCommit;
    type FeeAccountId = GetFeesAccountId;
    type FeeCurrency = FeeCurrency;
    type Currency = Currencies;
    type MessageStatusNotifier = ();
    type WeightInfo = ();
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
    type Decimals = u8;

    fn register_asset(
        _owner: AccountId,
        _name: Self::AssetName,
        _symbol: Self::AssetSymbol,
        _decimals: Self::Decimals,
    ) -> Result<AssetId, sp_runtime::DispatchError> {
        Ok(AssetId::Custom)
    }
}

impl substrate_app::Config for Test {
    type Event = Event;
    type BridgeAccountId = GetBridgeAccountId;
    type MessageStatusNotifier = ();
    type CallOrigin = dispatch::EnsureAccount<
        SubNetworkId,
        (),
        bridge_types::types::CallOriginOutput<SubNetworkId, H256, ()>,
    >;
    type OutboundChannel = BridgeOutboundChannel;
    type AssetRegistry = AssetRegistryImpl;
    type Currency = Currencies;
    type WeightInfo = ();
}

pub fn new_tester() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let bob: AccountId = Keyring::Bob.into();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bob.clone(), 1_000_000_000_000_000_000)],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &substrate_bridge_channel::outbound::GenesisConfig {
            fee: 10000,
            interval: 10,
        },
        &mut storage,
    )
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &substrate_app::GenesisConfig {
            assets: vec![
                (BASE_NETWORK_ID, AssetId::XOR, AssetKind::Thischain),
                (BASE_NETWORK_ID, AssetId::DAI, AssetKind::Sidechain),
            ],
        },
        &mut storage,
    )
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
