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

//! # Core
//!
//! Common traits and types

use core::fmt::Debug;

use crate::types::AuxiliaryDigestItem;
use crate::H256;
use crate::U256;
use crate::{
    types::{BridgeAppInfo, BridgeAssetInfo, MessageStatus},
    GenericAccount, GenericNetworkId,
};
use codec::FullCodec;
use ethereum_types::Address;
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    Parameter,
};
use frame_system::{Config, RawOrigin};
use scale_info::TypeInfo;
use sp_std::prelude::*;

/// A trait for verifying messages.
///
/// This trait should be implemented by runtime modules that wish to provide message verification functionality.
pub trait Verifier {
    type Proof: FullCodec + TypeInfo + Clone + Debug + PartialEq;
    fn verify(network_id: GenericNetworkId, message: H256, proof: &Self::Proof) -> DispatchResult;
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
    fn successful_dispatch_event(id: MessageId) -> Option<<T as Config>::RuntimeEvent>;
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
        dest: AccountId,
        asset_id: AssetId,
        amount: Balance,
        start_timestamp: u64,
    );

    fn outbound_request(
        network_id: GenericNetworkId,
        message_id: H256,
        source: AccountId,
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
        _dest: AccountId,
        _asset_id: AssetId,
        _amount: Balance,
        _start_timestamp: u64,
    ) {
    }

    fn outbound_request(
        _network_id: GenericNetworkId,
        _message_id: H256,
        _source: AccountId,
        _dest: GenericAccount<AccountId>,
        _asset_id: AssetId,
        _amount: Balance,
    ) {
    }
}

/// Trait for tracking Ethereum-based network transaction fee paid by relayer for messages relayed
/// from Sora2 network to Ethereum-based network.
pub trait GasTracker<Balance> {
    /// Records fee paid.
    /// `network_id`: Ethereum-like network ID
    /// `message_id`: relayed message ID
    /// `ethereum_tx_hash`: tx hash on Ethereum-based side
    /// `ethereum_relayer_address`: address of relayer on Ethereum-based network (who paid fee)
    /// `gas_used`: gas used for relay tx
    /// `gas_price`: gas price of relay tx
    /// fee is `gas_used` * `gas_price`
    fn record_tx_fee(
        network_id: GenericNetworkId,
        message_id: H256,
        ethereum_tx_hash: H256,
        ethereum_relayer_address: Address,
        gas_used: U256,
        gas_price: U256,
    );
}

impl<Balance> GasTracker<Balance> for () {
    fn record_tx_fee(
        _network_id: GenericNetworkId,
        _message_id: H256,
        _ethereum_tx_hash: H256,
        _ethereum_relayer_address: Address,
        _gas_used: U256,
        _gas_price: U256,
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

pub trait AuxiliaryDigestHandler {
    fn add_item(item: AuxiliaryDigestItem);
}

impl AuxiliaryDigestHandler for () {
    fn add_item(_item: AuxiliaryDigestItem) {}
}
