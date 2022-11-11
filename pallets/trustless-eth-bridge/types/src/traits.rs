//! # Core
//!
//! Common traits and types

use crate::{
    types::{BridgeAppInfo, BridgeAssetInfo, MessageStatus},
    GenericAccount, GenericNetworkId,
};
use ethereum_types::H256;
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    Parameter,
};
use frame_system::{Config, RawOrigin};
use sp_std::prelude::*;

/// A trait for verifying messages.
///
/// This trait should be implemented by runtime modules that wish to provide message verification functionality.
pub trait Verifier<NetworkId, Message> {
    type Result;
    fn verify(network_id: NetworkId, message: &Message) -> Result<Self::Result, DispatchError>;
}

/// Outbound submission for applications
pub trait OutboundChannel<NetworkId, AccountId, Additional> {
    fn submit(
        network_id: NetworkId,
        who: &RawOrigin<AccountId>,
        payload: &[u8],
        additional: Additional,
    ) -> Result<H256, DispatchError>;
}

/// Dispatch a message
pub trait MessageDispatch<T: Config, NetworkId, MessageId, Additional> {
    fn dispatch(
        network_id: NetworkId,
        id: MessageId,
        timestamp: u64,
        payload: &[u8],
        additional: Additional,
    );
    #[cfg(feature = "runtime-benchmarks")]
    fn successful_dispatch_event(id: MessageId) -> Option<<T as Config>::Event>;
}

pub trait AppRegistry<NetworkId, Source> {
    fn register_app(network_id: NetworkId, app: Source) -> DispatchResult;
    fn deregister_app(network_id: NetworkId, app: Source) -> DispatchResult;
}

impl<NetworkId, Source> AppRegistry<NetworkId, Source> for () {
    fn register_app(_network_id: NetworkId, _app: Source) -> DispatchResult {
        Ok(())
    }

    fn deregister_app(_network_id: NetworkId, _app: Source) -> DispatchResult {
        Ok(())
    }
}

pub trait BridgeApp<NetworkId, AccountId, Recipient, AssetId, Balance> {
    fn is_asset_supported(network_id: NetworkId, asset_id: AssetId) -> bool;

    // Initiates transfer to Sidechain by burning the asset on substrate side
    fn transfer(
        network_id: NetworkId,
        asset_id: AssetId,
        sender: AccountId,
        recipient: Recipient,
        amount: Balance,
    ) -> Result<H256, DispatchError>;

    fn refund(
        network_id: NetworkId,
        message_id: H256,
        recipient: AccountId,
        asset_id: AssetId,
        amount: Balance,
    ) -> DispatchResult;

    fn list_supported_assets(network_id: NetworkId) -> Vec<BridgeAssetInfo<AssetId>>;

    fn list_apps(network_id: NetworkId) -> Vec<BridgeAppInfo>;
}

pub trait MessageStatusNotifier<AssetId, AccountId, Balance> {
    fn update_status(
        network_id: GenericNetworkId,
        message_id: H256,
        status: MessageStatus,
        end_timestamp: Option<u64>,
    );

    fn inbound_request(
        network_id: GenericNetworkId,
        message_id: H256,
        source: GenericAccount<AccountId>,
        dest: GenericAccount<AccountId>,
        asset_id: AssetId,
        amount: Balance,
        start_timestamp: u64,
    );

    fn outbound_request(
        network_id: GenericNetworkId,
        message_id: H256,
        source: GenericAccount<AccountId>,
        dest: GenericAccount<AccountId>,
        asset_id: AssetId,
        amount: Balance,
    );
}

impl<AssetId, AccountId, Balance> MessageStatusNotifier<AssetId, AccountId, Balance> for () {
    fn update_status(
        _network_id: GenericNetworkId,
        _message_id: H256,
        _status: MessageStatus,
        _end_timestamp: Option<u64>,
    ) {
    }

    fn inbound_request(
        _network_id: GenericNetworkId,
        _message_id: H256,
        _source: GenericAccount<AccountId>,
        _dest: GenericAccount<AccountId>,
        _asset_id: AssetId,
        _amount: Balance,
        _start_timestamp: u64,
    ) {
    }

    fn outbound_request(
        _network_id: GenericNetworkId,
        _message_id: H256,
        _source: GenericAccount<AccountId>,
        _dest: GenericAccount<AccountId>,
        _asset_id: AssetId,
        _amount: Balance,
    ) {
    }
}

/// Trait that every origin (like Ethereum origin or Parachain origin) should implement
pub trait OriginOutput<NetworkId, Additional> {
    /// Construct new origin
    fn new(network_id: NetworkId, message_id: H256, timestamp: u64, additional: Additional)
        -> Self;
}

pub trait BridgeAssetRegistry<AccountId, AssetId> {
    type AssetName: Parameter;
    type AssetSymbol: Parameter;
    type Decimals: Parameter;

    fn register_asset(
        owner: AccountId,
        name: Self::AssetName,
        symbol: Self::AssetSymbol,
        decimals: Self::Decimals,
    ) -> Result<AssetId, DispatchError>;
}
