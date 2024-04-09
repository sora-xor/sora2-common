use bridge_types::traits::{AppRegistry, BalancePrecisionConverter, BridgeAssetRegistry};
use bridge_types::traits::{EVMOutboundChannel, OutboundChannel};
use currencies::BasicCurrencyAdapter;

// Mock runtime
use bridge_types::evm::{AdditionalEVMInboundData, AdditionalEVMOutboundData};
use bridge_types::types::AssetKind;
use bridge_types::H160;
use bridge_types::H256;
use bridge_types::{EVMChainId, GenericNetworkId, U256};
use frame_support::dispatch::DispatchResult;
use frame_support::parameter_types;
use frame_support::traits::{Everything, GenesisBuild};
use frame_system as system;
use sp_core::ConstU64;
use sp_keyring::sr25519::Keyring;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Keccak256, Verify};
use sp_runtime::{DispatchError, MultiSignature};
use traits::parameter_type_with_key;

use crate as erc20_app;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type AssetId = H256;
type Balance = u128;
type Amount = i128;

pub const XOR: AssetId = H256::repeat_byte(1);
pub const DAI: AssetId = H256::repeat_byte(2);
pub const ETH: AssetId = H256::repeat_byte(3);

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Event<T>},
        Tokens: tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
        Currencies: currencies::{Pallet, Call, Storage},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        Dispatch: dispatch::{Pallet, Call, Storage, Origin<T>, Event<T>},
        FungibleApp: erc20_app::{Pallet, Call, Config<T>, Storage, Event<T>},
    }
);

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub const BASE_NETWORK_ID: EVMChainId = EVMChainId::zero();

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

parameter_types! {
    pub const GetBaseAssetId: AssetId = H256::zero();
}

impl currencies::Config for Test {
    type MultiCurrency = Tokens;
    type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, u64>;
    type GetNativeCurrencyId = GetBaseAssetId;
    type WeightInfo = ();
}

impl dispatch::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OriginOutput =
        bridge_types::types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>;
    type Origin = RuntimeOrigin;
    type MessageId = u64;
    type Hashing = Keccak256;
    type Call = RuntimeCall;
    type CallFilter = Everything;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxMessagePayloadSize: u32 = 2048;
    pub const MaxMessagesPerCommit: u32 = 3;
    pub const MaxTotalGasLimit: u64 = 5_000_000;
    pub const Decimals: u32 = 12;
}

parameter_types! {
    pub const ThisNetworkId: bridge_types::GenericNetworkId = bridge_types::GenericNetworkId::Sub(bridge_types::SubNetworkId::Mainnet);
}

pub struct AppRegistryImpl;

impl AppRegistry<EVMChainId, H160> for AppRegistryImpl {
    fn register_app(_network_id: EVMChainId, _app: H160) -> DispatchResult {
        Ok(())
    }

    fn deregister_app(_network_id: EVMChainId, _app: H160) -> DispatchResult {
        Ok(())
    }
}

pub struct BalancePrecisionConverterImpl;

impl BalancePrecisionConverter<AssetId, Balance, U256> for BalancePrecisionConverterImpl {
    fn from_sidechain(
        _asset_id: &AssetId,
        _sidechain_precision: u8,
        amount: U256,
    ) -> Option<(Balance, U256)> {
        Some((amount.try_into().ok()?, amount))
    }

    fn to_sidechain(
        _asset_id: &AssetId,
        _sidechain_precision: u8,
        amount: Balance,
    ) -> Option<(Balance, U256)> {
        Some((amount, amount.into()))
    }
}

pub struct BridgeAssetRegistryImpl;

impl BridgeAssetRegistry<AccountId, AssetId> for BridgeAssetRegistryImpl {
    type AssetName = Vec<u8>;
    type AssetSymbol = Vec<u8>;

    fn register_asset(
        network_id: GenericNetworkId,
        _name: Self::AssetName,
        _symbol: Self::AssetSymbol,
    ) -> Result<AssetId, DispatchError> {
        let owner =
            bridge_types::test_utils::BridgeAssetLockerImpl::<()>::bridge_account(network_id);
        frame_system::Pallet::<Test>::inc_providers(&owner);
        Ok(H256::random())
    }

    fn manage_asset(
        network_id: GenericNetworkId,
        _asset_id: AssetId,
    ) -> frame_support::pallet_prelude::DispatchResult {
        let manager =
            bridge_types::test_utils::BridgeAssetLockerImpl::<()>::bridge_account(network_id);
        frame_system::Pallet::<Test>::inc_providers(&manager);
        Ok(())
    }

    fn get_raw_info(_asset_id: AssetId) -> bridge_types::types::RawAssetInfo {
        bridge_types::types::RawAssetInfo {
            name: Default::default(),
            symbol: Default::default(),
            precision: 18,
        }
    }

    fn ensure_asset_exists(_asset_id: AssetId) -> bool {
        true
    }
}

pub struct OutboundChannelImpl;

impl OutboundChannel<EVMChainId, AccountId, AdditionalEVMOutboundData> for OutboundChannelImpl {
    fn submit(
        _network_id: EVMChainId,
        _who: &system::RawOrigin<AccountId>,
        _payload: &[u8],
        _additional: AdditionalEVMOutboundData,
    ) -> Result<H256, DispatchError> {
        Ok(H256::random())
    }

    fn submit_weight() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_all(1)
    }
}

impl EVMOutboundChannel for OutboundChannelImpl {
    fn submit_gas(_chain_id: EVMChainId) -> Result<U256, DispatchError> {
        Ok(U256::one())
    }
}

impl erc20_app::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OutboundChannel = OutboundChannelImpl;
    type CallOrigin = dispatch::EnsureAccount<
        bridge_types::types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>,
    >;
    type WeightInfo = ();
    type MessageStatusNotifier = ();
    type BalancePrecisionConverter = BalancePrecisionConverterImpl;
    type AppRegistry = AppRegistryImpl;
    type AssetRegistry = BridgeAssetRegistryImpl;
    type AssetIdConverter = sp_runtime::traits::ConvertInto;
    type BridgeAssetLocker = bridge_types::test_utils::BridgeAssetLockerImpl<Currencies>;
    type BaseFeeLifetime = ConstU64<100>;
}

pub fn new_tester() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let bob: AccountId = Keyring::Bob.into();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(bob.clone(), 1_000_000_000_000_000_000u128)],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    GenesisBuild::<Test>::assimilate_storage(
        &erc20_app::GenesisConfig {
            apps: vec![
                (BASE_NETWORK_ID, H160::repeat_byte(1), AssetKind::Sidechain),
                (BASE_NETWORK_ID, H160::repeat_byte(2), AssetKind::Thischain),
            ],
            assets: vec![
                (
                    BASE_NETWORK_ID,
                    XOR,
                    H160::repeat_byte(3),
                    AssetKind::Thischain,
                    18,
                ),
                (
                    BASE_NETWORK_ID,
                    DAI,
                    H160::repeat_byte(4),
                    AssetKind::Sidechain,
                    18,
                ),
            ],
        },
        &mut storage,
    )
    .unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
