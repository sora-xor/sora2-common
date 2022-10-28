use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

pub type ParachainAccountId = xcm::VersionedMultiLocation;

pub type ParachainAssetId = xcm::VersionedMultiAsset;

pub trait SubstrateBridgeMessageEncode {
    fn prepare_message(self) -> Vec<u8>;
}

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum SubstrateAppMessage<AccountId, AssetId, Balance> {
    Transfer {
        asset_id: AssetId,
        sender: ParachainAccountId,
        recipient: AccountId,
        amount: Balance,
    },
}

impl<AccountId: Encode, Balance: Encode, AssetId: Encode> SubstrateBridgeMessageEncode
    for SubstrateAppMessage<AccountId, AssetId, Balance>
{
    fn prepare_message(self) -> Vec<u8> {
        SubstrateBridgeMessage::SubstrateApp(self).encode()
    }
}

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum XCMAppMessage<AccountId, AssetId, Balance> {
    Transfer {
        asset_id: AssetId,
        sender: AccountId,
        recipient: ParachainAccountId,
        amount: Balance,
    },
    RegisterAsset {
        asset_id: AssetId,
        sidechain_asset: ParachainAssetId,
    },
}

impl<AccountId: Encode, Balance: Encode, AssetId: Encode> SubstrateBridgeMessageEncode
    for XCMAppMessage<AccountId, AssetId, Balance>
{
    fn prepare_message(self) -> Vec<u8> {
        SubstrateBridgeMessage::XCMApp(self).encode()
    }
}

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub enum SubstrateBridgeMessage<AccountId, AssetId, Balance> {
    SubstrateApp(SubstrateAppMessage<AccountId, AssetId, Balance>),
    XCMApp(XCMAppMessage<AccountId, AssetId, Balance>),
}

impl<AccountId: Encode, Balance: Encode, AssetId: Encode> SubstrateBridgeMessageEncode
    for SubstrateBridgeMessage<AccountId, AssetId, Balance>
{
    fn prepare_message(self) -> Vec<u8> {
        self.encode()
    }
}
