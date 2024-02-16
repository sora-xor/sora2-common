use super::*;
use bridge_types::evm::AdditionalEVMInboundData;
use bridge_types::types;
use bridge_types::EVMChainId;
use frame_support::parameter_types;
use frame_support::traits::{ConstU32, Everything};
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
impl Contains<RuntimeCall> for CallFilter {
    fn contains(call: &RuntimeCall) -> bool {
        matches!(
            call,
            RuntimeCall::System(frame_system::pallet::Call::<Test>::remark { .. })
        )
    }
}

impl dispatch::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OriginOutput = types::CallOriginOutput<EVMChainId, H256, AdditionalEVMInboundData>;
    type Origin = RuntimeOrigin;
    type MessageId = types::MessageId;
    type Hashing = Keccak256;
    type Call = RuntimeCall;
    type CallFilter = CallFilter;
    type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    sp_io::TestExternalities::new(t)
}
