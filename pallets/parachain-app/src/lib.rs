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

//! # Parachain App
//!
//! An application that implements bridged parachain/relaychain assets transfer
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `burn`: Burn an backed parachain/relaychain or thischain token balance.
#![cfg_attr(not(feature = "std"), no_std)]

pub const TRANSFER_MAX_GAS: u64 = 100_000;

extern crate alloc;

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use bridge_types::substrate::SubAssetInfo;
use bridge_types::substrate::{ParachainAccountId, ParachainAppCall};
use bridge_types::traits::BridgeApp;
use bridge_types::traits::BridgeAssetLocker;
use bridge_types::types::{BridgeAppInfo, BridgeAssetInfo};
use bridge_types::GenericNetworkId;
use bridge_types::{MainnetAccountId, MainnetAssetId};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use frame_support::weights::Weight;
use frame_system::ensure_signed;
use sp_runtime::traits::{Convert, Get, Zero};
use sp_std::prelude::*;

pub use weights::WeightInfo;

pub use pallet::*;

impl<T: Config> From<ParachainAppCall> for Call<T>
where
    T::AccountId: From<MainnetAccountId>,
    AssetIdOf<T>: From<MainnetAssetId>,
{
    fn from(value: ParachainAppCall) -> Self {
        match value {
            ParachainAppCall::Transfer {
                sender,
                recipient,
                amount,
                asset_id,
            } => Call::mint {
                sender,
                recipient: recipient.into(),
                asset_id: asset_id.into(),
                amount,
            },
            ParachainAppCall::FinalizeAssetRegistration {
                asset_id,
                asset_kind,
            } => Call::finalize_asset_registration {
                asset_id: asset_id.into(),
                asset_kind,
            },
            ParachainAppCall::ReportXCMTransferResult {
                message_id,
                transfer_status,
            } => Call::update_transaction_status {
                message_id,
                transfer_status,
            },
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[frame_support::pallet]
pub mod pallet {

    use super::*;

    use bridge_types::substrate::XCMAppTransferStatus;
    use bridge_types::substrate::{
        ParachainAccountId, ParachainAssetId, SubstrateBridgeMessageEncode, XCMAppCall,
    };
    use bridge_types::traits::{
        BalancePrecisionConverter, BridgeAssetLocker, BridgeAssetRegistry, MessageStatusNotifier,
        OutboundChannel,
    };
    use bridge_types::types::{AssetKind, CallOriginOutput, MessageStatus};
    use bridge_types::{
        GenericAccount, GenericNetworkId, MainnetAccountId, MainnetAssetId, MainnetBalance,
        SubNetworkId, H256,
    };
    use frame_support::fail;
    use frame_support::pallet_prelude::{OptionQuery, ValueQuery, *};
    use frame_system::pallet_prelude::*;
    use frame_system::{ensure_root, RawOrigin};

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    pub type AssetIdOf<T> =
        <<T as Config>::BridgeAssetLocker as BridgeAssetLocker<AccountIdOf<T>>>::AssetId;

    pub type BalanceOf<T> =
        <<T as Config>::BridgeAssetLocker as BridgeAssetLocker<AccountIdOf<T>>>::Balance;

    pub type AssetNameOf<T> = <<T as Config>::AssetRegistry as BridgeAssetRegistry<
        AccountIdOf<T>,
        AssetIdOf<T>,
    >>::AssetName;
    pub type AssetSymbolOf<T> = <<T as Config>::AssetRegistry as BridgeAssetRegistry<
        AccountIdOf<T>,
        AssetIdOf<T>,
    >>::AssetSymbol;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type OutboundChannel: OutboundChannel<SubNetworkId, Self::AccountId, ()>;

        type CallOrigin: EnsureOrigin<
            Self::RuntimeOrigin,
            Success = CallOriginOutput<SubNetworkId, H256, ()>,
        >;

        type MessageStatusNotifier: MessageStatusNotifier<
            AssetIdOf<Self>,
            Self::AccountId,
            BalanceOf<Self>,
        >;

        type AssetRegistry: BridgeAssetRegistry<Self::AccountId, AssetIdOf<Self>>;

        type AccountIdConverter: Convert<Self::AccountId, MainnetAccountId>;

        type AssetIdConverter: Convert<AssetIdOf<Self>, MainnetAssetId>;

        type BalancePrecisionConverter: BalancePrecisionConverter<
            AssetIdOf<Self>,
            BalanceOf<Self>,
            MainnetBalance,
        >;

        type BridgeAssetLocker: BridgeAssetLocker<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// [network_id, asset_id, sender, recepient, amount]
        Burned(
            SubNetworkId,
            AssetIdOf<T>,
            T::AccountId,
            ParachainAccountId,
            BalanceOf<T>,
        ),
        /// [network_id, asset_id, sender, recepient, amount]
        Minted(
            SubNetworkId,
            AssetIdOf<T>,
            Option<ParachainAccountId>,
            T::AccountId,
            BalanceOf<T>,
        ),
    }

    #[pallet::storage]
    #[pallet::getter(fn asset_kind)]
    pub(super) type AssetKinds<T: Config> =
        StorageDoubleMap<_, Identity, SubNetworkId, Identity, AssetIdOf<T>, AssetKind, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn sidechain_precision)]
    pub(super) type SidechainPrecision<T: Config> =
        StorageDoubleMap<_, Identity, SubNetworkId, Identity, AssetIdOf<T>, u8, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn allowed_parachain_assets)]
    pub(super) type AllowedParachainAssets<T: Config> =
        StorageDoubleMap<_, Identity, SubNetworkId, Identity, u32, Vec<AssetIdOf<T>>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn relaychain_asset)]
    pub(super) type RelaychainAsset<T: Config> =
        StorageMap<_, Identity, SubNetworkId, AssetIdOf<T>, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {        
        TokenIsNotRegistered,
        AppIsNotRegistered,
        NotEnoughFunds,
        InvalidNetwork,
        TokenAlreadyRegistered,
        AppAlreadyRegistered,
        /// Call encoding failed.
        CallEncodeFailed,
        /// Amount must be > 0
        WrongAmount,
        TransferLimitReached,
        UnknownPrecision,
        MessageIdNotFound,
        InvalidDestinationParachain,
        InvalidDestinationParams,
        RelaychainAssetNotRegistered,
        NotRelayTransferableAsset,
        RelaychainAssetRegistered,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Mints tokens on this chain as part of a cross-chain transfer.
        ///
        /// This function is called internally by the bridge to complete a transfer from another chain
        /// to this chain. It mints (unlocks) tokens on this chain for the specified recipient.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be a valid bridge origin.
        /// * `asset_id` - The identifier of the asset to be minted.
        /// * `sender` - The sender's account on the source chain, if available.
        /// * `recipient` - The recipient's account on this chain.
        /// * `amount` - The amount of tokens to mint, in the source chain's precision.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The origin is not a valid bridge origin.
        /// * The asset is not registered for cross-chain transfers.
        /// * The asset's precision on this chain is unknown.
        /// * The amount conversion fails or results in zero tokens.
        /// * The BridgeAssetLocker fails to unlock the asset.
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Mint (unlock) the specified amount of tokens for the recipient.
        /// * Update the inbound transfer status to 'Done'.
        /// * Emit a `Minted` event with transfer details.
        ///
        /// # Note
        ///
        /// This function handles the necessary precision conversions between chains and
        /// ensures that the minted amount accurately reflects the transferred amount.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            sender: Option<ParachainAccountId>,
            recipient: T::AccountId,
            amount: MainnetBalance,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id,
                message_id,
                timepoint,
                ..
            } = T::CallOrigin::ensure_origin(origin.clone())?;

            let asset_kind = AssetKinds::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;

            let precision = SidechainPrecision::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::UnknownPrecision)?;
            let (amount, _) =
                T::BalancePrecisionConverter::from_sidechain(&asset_id, precision, amount)
                    .ok_or(Error::<T>::WrongAmount)?;
            ensure!(amount > Zero::zero(), Error::<T>::WrongAmount);

            T::BridgeAssetLocker::unlock_asset(
                network_id.into(),
                asset_kind,
                &recipient,
                &asset_id,
                &amount,
            )?;

            T::MessageStatusNotifier::inbound_request(
                GenericNetworkId::Sub(network_id),
                message_id,
                sender
                    .clone()
                    .map(GenericAccount::Parachain)
                    .unwrap_or(GenericAccount::Unknown),
                recipient.clone(),
                asset_id.clone(),
                amount.clone(),
                timepoint,
                MessageStatus::Done,
            );
            Self::deposit_event(Event::Minted(
                network_id, asset_id, sender, recipient, amount,
            ));
            Ok(())
        }

        /// Finalizes the registration of an asset for cross-chain transfers.
        ///
        /// This function is called internally by relayer, to complete the asset registration process.
        /// It sets the asset kind for a previously initialized asset, enabling it for cross-chain transfers.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be a valid bridge origin.
        /// * `asset_id` - The identifier of the asset being registered.
        /// * `asset_kind` - The kind of asset (e.g., Thischain, Sidechain) being registered.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The origin is not a valid bridge origin.
        /// * The asset has not been previously initialized (i.e., its precision is not set).
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Set the asset kind for the specified asset, completing its registration.
        /// * Enable the asset for cross-chain transfers.
        ///
        /// # Note
        ///
        /// This function is part of a two-step asset registration process. The first step initializes
        /// the asset (typically setting its precision), and this function completes the process by
        /// setting the asset kind. This two-step process helps ensure proper synchronization between
        /// chains in the network.
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::finalize_asset_registration())]
        pub fn finalize_asset_registration(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            asset_kind: AssetKind,
        ) -> DispatchResult {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin.clone())?;
            ensure!(
                SidechainPrecision::<T>::contains_key(network_id, &asset_id),
                Error::<T>::TokenIsNotRegistered
            );
            AssetKinds::<T>::insert(network_id, asset_id, asset_kind);
            Ok(())
        }

        /// Burns tokens on this chain to initiate a cross-chain transfer.
        ///
        /// This function locks (burns) tokens on the current chain and initiates a transfer
        /// to a specified recipient on another parachain or the relay chain.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be signed by the account burning tokens.
        /// * `network_id` - The identifier of the destination network.
        /// * `asset_id` - The identifier of the asset to be transferred.
        /// * `recipient` - The recipient's account on the destination chain, specified as a ParachainAccountId.
        /// * `amount` - The amount of tokens to burn and transfer.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The origin is not signed.
        /// * The amount is zero or negative.
        /// * The asset is not registered for cross-chain transfers.
        /// * The destination parachain or parameters are invalid.
        /// * The asset's precision on the sidechain is unknown.
        /// * The user doesn't have sufficient balance.
        /// * The asset cannot be locked by the BridgeAssetLocker.
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Lock (burn) the specified amount of tokens from the sender's account.
        /// * Create and submit an outbound message to initiate the transfer on the destination chain.
        /// * Emit a `Burned` event with transfer details.
        /// * Return the message ID of the outbound transfer request.
        ///
        /// # Note
        ///
        /// This function is part of a cross-chain transfer mechanism. The actual receipt of
        /// funds on the destination chain is subject to the processing of the outbound message
        /// by the bridge and the destination chain's systems.
        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: ParachainAccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::burn_inner(who, network_id, asset_id, recipient, amount)?;

            Ok(())
        }

        /// Registers an asset from this chain for cross-chain transfers.
        ///
        /// This function allows registering an existing asset on this chain for use in cross-chain transfers.
        /// It sets up the necessary mappings and permissions for the asset to be transferred to other parachains.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be root.
        /// * `network_id` - The identifier of the network where the asset will be registered.
        /// * `asset_id` - The identifier of the asset on this chain.
        /// * `sidechain_asset` - The identifier to be used for this asset on the sidechain.
        /// * `allowed_parachains` - A list of parachain IDs that are allowed to receive this asset.
        /// * `minimal_xcm_amount` - The minimum amount required for XCM transfers of this asset.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The caller is not root.
        /// * The asset is already registered for the given network.
        /// * The provided minimal XCM amount is invalid (zero or too large for conversion).
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Register the asset for cross-chain transfers.
        /// * Set up the necessary mappings for the asset.
        /// * Add the asset to the list of allowed assets for the specified parachains.
        /// * Send a message to the sidechain to register the asset there as well.
        ///
        /// # Note
        ///
        /// This function is typically used for assets that originate on this chain and need to be
        /// made available for transfer to other chains in the network.
        /// IMPORTANT! minimal_xcm_amount always has 18 precision!
        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::register_thischain_asset(allowed_parachains.len() as u32))]
        pub fn register_thischain_asset(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            sidechain_asset: ParachainAssetId,
            allowed_parachains: Vec<u32>,
            minimal_xcm_amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetKinds::<T>::contains_key(network_id, &asset_id),
                Error::<T>::TokenAlreadyRegistered
            );

            let sidechain_precision = T::AssetRegistry::get_raw_info(asset_id.clone()).precision;

            let (_, minimal_xcm_amount) = T::BalancePrecisionConverter::to_sidechain(
                &asset_id,
                sidechain_precision,
                minimal_xcm_amount,
            )
            .ok_or(Error::<T>::WrongAmount)?;

            ensure!(minimal_xcm_amount > 0, Error::<T>::WrongAmount);

            Self::register_asset_inner(
                network_id,
                asset_id,
                sidechain_asset,
                AssetKind::Thischain,
                sidechain_precision,
                allowed_parachains,
                minimal_xcm_amount,
            )?;

            Ok(())
        }

        /// Registers a sidechain asset for cross-chain transfers.
        ///
        /// This function allows registering a new asset that exists on a sidechain (e.g., another parachain)
        /// for use in cross-chain transfers. It creates a new asset on this chain that corresponds to the
        /// sidechain asset and sets up the necessary mappings and permissions for cross-chain operations.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be root.
        /// * `network_id` - The identifier of the network where the asset will be registered.
        /// * `sidechain_asset` - The identifier of the asset on the sidechain.
        /// * `symbol` - The symbol of the asset (e.g., "DOT" for Polkadot).
        /// * `name` - The full name of the asset (e.g., "Polkadot").
        /// * `decimals` - The number of decimal places for the asset's precision.
        /// * `allowed_parachains` - A list of parachain IDs that are allowed to transfer this asset.
        /// * `minimal_xcm_amount` - The minimum amount required for XCM transfers of this asset.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The caller is not root.
        /// * The asset is already registered for the given network.
        /// * Asset registration fails in the AssetRegistry.
        /// * The provided minimal XCM amount is invalid (zero or too large for conversion).
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Register a new asset in the AssetRegistry.
        /// * Set up the necessary mappings for cross-chain transfers.
        /// * Add the asset to the list of allowed assets for the specified parachains.
        /// * Send a message to the sidechain to register the asset there as well.
        ///
        /// # Note
        ///
        /// This function is typically used when introducing a new asset from another chain into this ecosystem.
        /// It should be used carefully as it affects the cross-chain asset landscape.
        /// IMPORTANT! minimal_xcm_amount always has 18 precision!
        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::register_sidechain_asset(allowed_parachains.len() as u32))]
        pub fn register_sidechain_asset(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            sidechain_asset: ParachainAssetId,
            symbol: AssetSymbolOf<T>,
            name: AssetNameOf<T>,
            decimals: u8,
            allowed_parachains: Vec<u32>,
            minimal_xcm_amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let asset_id = T::AssetRegistry::register_asset(network_id.into(), name, symbol)?;
            let (_, minimal_xcm_amount) =
                T::BalancePrecisionConverter::to_sidechain(&asset_id, decimals, minimal_xcm_amount)
                    .ok_or(Error::<T>::WrongAmount)?;

            ensure!(minimal_xcm_amount > 0, Error::<T>::WrongAmount);

            Self::register_asset_inner(
                network_id,
                asset_id,
                sidechain_asset,
                AssetKind::Sidechain,
                decimals,
                allowed_parachains,
                minimal_xcm_amount,
            )?;
            Ok(())
        }

        /// Adds an asset to the list of allowed assets for a specific parachain.
        ///
        /// This function allows adding a previously registered asset to the list of assets
        /// that are allowed to be transferred to a specific parachain. This can be used to
        /// expand or modify the set of assets that can be sent to particular parachains.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be root.
        /// * `network_id` - The identifier of the network for which this change applies.
        /// * `para_id` - The identifier of the parachain to which the asset will be allowed.
        /// * `asset_id` - The identifier of the asset to be added to the allowed list.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The caller is not root.
        /// * The specified asset is not registered for the given network.
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Add the specified asset to the list of allowed assets for the given parachain.
        /// * If the asset was already in the list for the parachain, no change will occur.
        ///
        /// # Note
        ///
        /// Adding an asset to the allowed list for a parachain enables future transfers
        /// of that asset to the specified parachain. This does not affect existing balances.
        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::WeightInfo::add_assetid_paraid())]
        pub fn add_assetid_paraid(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            para_id: u32,
            asset_id: AssetIdOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            AssetKinds::<T>::get(network_id, &asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

            AllowedParachainAssets::<T>::try_mutate(network_id, para_id, |x| -> DispatchResult {
                x.push(asset_id);
                Ok(())
            })?;

            Ok(())
        }

        /// Removes an asset from the list of allowed assets for a specific parachain.
        ///
        /// This function allows removing a previously registered asset from the list of assets
        /// that are allowed to be transferred to a specific parachain. This can be used to
        /// restrict or modify the set of assets that can be sent to particular parachains.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be root.
        /// * `network_id` - The identifier of the network for which this change applies.
        /// * `para_id` - The identifier of the parachain from which to remove the asset.
        /// * `asset_id` - The identifier of the asset to be removed from the allowed list.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The caller is not root.
        /// * The specified asset is not registered for the given network.
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Remove the specified asset from the list of allowed assets for the given parachain.
        /// * If the asset was not in the list for the parachain, no change will occur.
        ///
        /// # Note
        ///
        /// Removing an asset from the allowed list for a parachain will prevent future transfers
        /// of that asset to the specified parachain. Existing balances on the parachain are not affected.
        #[pallet::call_index(6)]
        #[pallet::weight(<T as Config>::WeightInfo::remove_assetid_paraid())]
        pub fn remove_assetid_paraid(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            para_id: u32,
            asset_id: AssetIdOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            AssetKinds::<T>::get(network_id, &asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

            AllowedParachainAssets::<T>::try_mutate(network_id, para_id, |x| -> DispatchResult {
                x.retain(|el| *el != asset_id);
                Ok(())
            })?;

            Ok(())
        }

        /// Updates the status of a cross-chain transaction.
        ///
        /// This function allows updating the status of a cross-chain transaction based on the result
        /// of an XCM (Cross-Chain Message) transfer. It is typically called by the bridge to report
        /// the outcome of a transfer initiated from this chain to another chain in the network.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be a valid bridge origin.
        /// * `message_id` - The unique identifier of the message/transaction.
        /// * `transfer_status` - The status of the XCM transfer, either Success or XCMTransferError.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The origin is not a valid bridge origin.
        /// * The message ID does not correspond to a known transaction.
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Update the status of the corresponding transaction in the MessageStatusNotifier.
        /// * Set the status to 'Done' if the transfer was successful, or 'Failed' if there was an error.
        ///
        /// # Note
        ///
        /// This function is crucial for maintaining accurate records of cross-chain transactions
        /// and ensuring that users and the system are informed about the final status of transfers.
        #[pallet::call_index(7)]
        #[pallet::weight(<T as Config>::WeightInfo::update_transaction_status())]
        pub fn update_transaction_status(
            origin: OriginFor<T>,
            message_id: H256,
            transfer_status: XCMAppTransferStatus,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id,
                timepoint,
                ..
            } = T::CallOrigin::ensure_origin(origin)?;

            let message_status = match transfer_status {
                XCMAppTransferStatus::Success => MessageStatus::Done,
                XCMAppTransferStatus::XCMTransferError => MessageStatus::Failed,
            };
            T::MessageStatusNotifier::update_status(
                network_id.into(),
                message_id,
                message_status,
                timepoint,
            );
            Ok(())
        }

        /// Sets the minimum amount of an asset required for incoming XCM transfers.
        ///
        /// This function allows setting or updating the minimum amount of a specific asset
        /// that is required for incoming XCM (Cross-Chain Message) transfers. This helps
        /// prevent dust attacks and ensures that only meaningful amounts are transferred.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be root.
        /// * `network_id` - The identifier of the network for which this setting applies.
        /// * `asset_id` - The identifier of the asset for which to set the minimum amount.
        /// * `minimal_xcm_amount` - The minimum amount required for XCM transfers of this asset.
        ///   IMPORTANT: The precision for this parameter is 18 decimal places.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The caller is not root.
        /// * The asset is not registered for the given network.
        /// * The provided minimal XCM amount is invalid (zero or too large for conversion).
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Update the minimum XCM transfer amount for the specified asset on this chain.
        /// * Send a message to the parachain to update the minimum XCM transfer amount there as well.
        ///
        /// # Note
        ///
        /// Changing this value can affect the ability of users to make small transfers,
        /// so it should be set carefully considering the asset's value and typical transfer amounts.
        #[pallet::call_index(8)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn set_minimum_xcm_incoming_asset_count(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            minimal_xcm_amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let Some(sidechain_precision) = Self::sidechain_precision(network_id, &asset_id) else {
                fail!(Error::<T>::UnknownPrecision);
            };

            let (_, minimal_xcm_amount) = T::BalancePrecisionConverter::to_sidechain(
                &asset_id,
                sidechain_precision,
                minimal_xcm_amount,
            )
            .ok_or(Error::<T>::WrongAmount)?;

            ensure!(minimal_xcm_amount > 0, Error::<T>::WrongAmount);

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &XCMAppCall::SetAssetMinAmount {
                    asset_id: T::AssetIdConverter::convert(asset_id.clone()),
                    minimal_xcm_amount,
                }
                .prepare_message(),
                (),
            )?;
            Ok(())
        }

        /// Registers a sidechain asset that already exists on this chain.
        ///
        /// This function allows binding an existing asset on this chain to a corresponding asset on the sidechain.
        /// It is used when the asset already exists on this chain but needs to be registered for cross-chain transfers.
        ///
        /// # Arguments
        ///
        /// * `origin` - The origin of the call. Must be root.
        /// * `network_id` - The identifier of the network where the asset will be registered.
        /// * `asset_id` - The identifier of the existing asset on this chain.
        /// * `sidechain_asset` - The identifier of the corresponding asset on the sidechain.
        /// * `sidechain_precision` - The precision (number of decimal places) of the asset on the sidechain.
        /// * `allowed_parachains` - A list of parachain IDs that are allowed to transfer this asset.
        /// * `minimal_xcm_amount` - The minimum amount required for XCM transfers of this asset.
        ///
        /// # Errors
        ///
        /// This function will return an error if:
        /// * The caller is not root.
        /// * The asset is already registered for the given network.
        /// * The provided minimal XCM amount is invalid
        ///
        /// # Effects
        ///
        /// If successful, this function will:
        /// * Register the asset for cross-chain transfers.
        /// * Set the sidechain precision for the asset.
        /// * Add the asset to the list of allowed assets for the specified parachains.
        /// * Send a message to the parachain to register the asset there as well.
        /// /// IMPORTANT! minimal_xcm_amount always has 18 precision!
        #[pallet::call_index(9)]
        #[pallet::weight(<T as Config>::WeightInfo::bind_sidechain_asset(allowed_parachains.len() as u32))]
        pub fn bind_sidechain_asset(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            sidechain_asset: ParachainAssetId,
            sidechain_precision: u8,
            allowed_parachains: Vec<u32>,
            minimal_xcm_amount: BalanceOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetKinds::<T>::contains_key(network_id, &asset_id),
                Error::<T>::TokenAlreadyRegistered
            );

            let (_, minimal_xcm_amount) = T::BalancePrecisionConverter::to_sidechain(
                &asset_id,
                sidechain_precision,
                minimal_xcm_amount,
            )
            .ok_or(Error::<T>::WrongAmount)?;

            ensure!(minimal_xcm_amount > 0, Error::<T>::WrongAmount);

            Self::register_asset_inner(
                network_id,
                asset_id,
                sidechain_asset,
                AssetKind::Sidechain,
                sidechain_precision,
                allowed_parachains,
                minimal_xcm_amount,
            )?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn register_asset_inner(
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            sidechain_asset: ParachainAssetId,
            asset_kind: AssetKind,
            sidechain_precision: u8,
            allowed_parachains: Vec<u32>,
            minimal_xcm_amount: u128,
        ) -> DispatchResult {
            T::AssetRegistry::manage_asset(network_id.into(), asset_id.clone())?;
            SidechainPrecision::<T>::insert(network_id, &asset_id, sidechain_precision);

            for paraid in allowed_parachains {
                AllowedParachainAssets::<T>::try_mutate(
                    network_id,
                    paraid,
                    |x| -> DispatchResult {
                        x.push(asset_id.clone());
                        Ok(())
                    },
                )?;
            }

            // if it is a native relaychain asset - register it on the pallet to identify if it is transferred
            if sidechain_asset == bridge_types::substrate::PARENT_PARACHAIN_ASSET {
                ensure!(
                    Self::relaychain_asset(network_id).is_none(),
                    Error::<T>::RelaychainAssetRegistered
                );
                RelaychainAsset::<T>::insert(network_id, asset_id.clone());
            }

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &XCMAppCall::RegisterAsset {
                    asset_id: T::AssetIdConverter::convert(asset_id),
                    sidechain_asset,
                    asset_kind,
                    minimal_xcm_amount,
                }
                .prepare_message(),
                (),
            )?;
            Ok(())
        }

        pub fn burn_inner(
            who: T::AccountId,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: ParachainAccountId,
            amount: BalanceOf<T>,
        ) -> Result<H256, DispatchError> {
            ensure!(amount > BalanceOf::<T>::zero(), Error::<T>::WrongAmount);

            let asset_kind = AssetKinds::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;

            Self::check_parachain_transfer_params(network_id, asset_id.clone(), recipient.clone())?;

            let precision = SidechainPrecision::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::UnknownPrecision)?;

            let (amount, sidechain_amount) =
                T::BalancePrecisionConverter::to_sidechain(&asset_id, precision, amount)
                    .ok_or(Error::<T>::WrongAmount)?;

            ensure!(sidechain_amount > 0, Error::<T>::WrongAmount);
            T::BridgeAssetLocker::lock_asset(
                network_id.into(),
                asset_kind,
                &who,
                &asset_id,
                &amount,
            )?;

            let message_id = T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Signed(who.clone()),
                &XCMAppCall::Transfer {
                    recipient: recipient.clone(),
                    amount: sidechain_amount,
                    asset_id: T::AssetIdConverter::convert(asset_id.clone()),
                    sender: T::AccountIdConverter::convert(who.clone()),
                }
                .prepare_message(),
                (),
            )?;

            T::MessageStatusNotifier::outbound_request(
                GenericNetworkId::Sub(network_id),
                message_id,
                who.clone(),
                GenericAccount::Parachain(recipient.clone()),
                asset_id.clone(),
                amount.clone(),
                MessageStatus::InQueue,
            );

            Self::deposit_event(Event::Burned(network_id, asset_id, who, recipient, amount));

            Ok(Default::default())
        }

        fn check_parachain_transfer_params(
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: ParachainAccountId,
        ) -> DispatchResult {
            use bridge_types::substrate::{Junction, VersionedMultiLocation::V3};

            let V3(ml) = recipient else {
                fail!(Error::<T>::InvalidDestinationParams)
            };

            // parents should be == 1
            if ml.parents != 1 {
                fail!(Error::<T>::InvalidDestinationParams)
            }

            if ml.interior.len() == 1 {
                // len == 1 is transfer to the relay chain

                let Some(relaychain_asset) = Self::relaychain_asset(network_id) else {
                    fail!(Error::<T>::RelaychainAssetNotRegistered)
                };

                // only native relaychain asset can be transferred to the relaychain
                ensure!(
                    asset_id == relaychain_asset,
                    Error::<T>::NotRelayTransferableAsset
                );
            } else if ml.interior.len() == 2 {
                // len == 2 is transfer to a parachain

                let mut parachains: Vec<u32> = Vec::with_capacity(1);
                for x in ml.interior {
                    if let Junction::Parachain(id) = x {
                        parachains.push(id)
                    }
                }

                // Only one parachain is allowed in query
                ensure!(parachains.len() == 1, Error::<T>::InvalidDestinationParams);

                // ensure that destination para id is allowed to transfer to
                ensure!(
                    Self::allowed_parachain_assets(network_id, parachains[0]).contains(&asset_id),
                    Error::<T>::InvalidDestinationParachain
                );
            } else {
                fail!(Error::<T>::InvalidDestinationParams)
            }
            Ok(())
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub assets: Vec<(SubNetworkId, AssetIdOf<T>, AssetKind)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                assets: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (network_id, asset_id, asset_kind) in &self.assets {
                AssetKinds::<T>::insert(network_id, asset_id, asset_kind);
            }
        }
    }
}

impl<T: Config> BridgeApp<T::AccountId, ParachainAccountId, AssetIdOf<T>, BalanceOf<T>>
    for Pallet<T>
{
    fn is_asset_supported(network_id: GenericNetworkId, asset_id: AssetIdOf<T>) -> bool {
        let GenericNetworkId::Sub(network_id) = network_id else {
            return false;
        };
        AssetKinds::<T>::contains_key(network_id, asset_id)
    }

    fn transfer(
        network_id: GenericNetworkId,
        asset_id: AssetIdOf<T>,
        sender: T::AccountId,
        recipient: ParachainAccountId,
        amount: BalanceOf<T>,
    ) -> Result<bridge_types::H256, DispatchError> {
        let network_id = network_id.sub().ok_or(Error::<T>::InvalidNetwork)?;
        Self::burn_inner(sender, network_id, asset_id, recipient, amount)
    }

    fn refund(
        network_id: GenericNetworkId,
        _message_id: bridge_types::H256,
        recipient: T::AccountId,
        asset_id: AssetIdOf<T>,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let network_id = network_id.sub().ok_or(Error::<T>::InvalidNetwork)?;
        let asset_kind =
            AssetKinds::<T>::get(network_id, &asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

        T::BridgeAssetLocker::unlock_asset(
            network_id.into(),
            asset_kind,
            &recipient,
            &asset_id,
            &amount,
        )?;
        Ok(())
    }

    fn list_supported_assets(
        network_id: GenericNetworkId,
    ) -> Vec<bridge_types::types::BridgeAssetInfo> {
        let GenericNetworkId::Sub(network_id) = network_id else {
            return vec![];
        };
        AssetKinds::<T>::iter_prefix(network_id)
            .map(|(asset_id, asset_kind)| {
                let asset_id = T::AssetIdConverter::convert(asset_id);
                BridgeAssetInfo::Sub(SubAssetInfo {
                    asset_id,
                    asset_kind,
                    precision: 18,
                })
            })
            .collect()
    }

    fn list_apps() -> Vec<bridge_types::types::BridgeAppInfo> {
        AssetKinds::<T>::iter_keys()
            .map(|(network_id, _asset_id)| BridgeAppInfo::Sub(network_id.into()))
            .fold(vec![], |mut acc, value| {
                if !acc.iter().any(|x| value == *x) {
                    acc.push(value);
                }
                acc
            })
    }

    fn is_asset_supported_weight() -> Weight {
        T::DbWeight::get().reads(1)
    }

    fn refund_weight() -> Weight {
        <T as Config>::WeightInfo::refund()
    }

    fn transfer_weight() -> Weight {
        <T as Config>::WeightInfo::burn()
    }
}
