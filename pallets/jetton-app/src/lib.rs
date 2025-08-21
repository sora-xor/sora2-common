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

//! # Jetton App
//!
//! An application that implements bridged fungible (Jettons and native) TON assets.
//!
//! ## Overview
//!
//! Provides logic for interaction with fungible TON assets
//! Sends and receives messages using bridge channels
//!
//! ## Interface
//!
//! ### Dispatchable Bridge Calls (incoming messages from TON)
//!
//! - `mint`: Mints given asset to user account
//!
//! ### Dispatchable Calls
//!
//! - `register_network`: Register TON network with new asset connected to native TON asset
//! - `register_network_with_existing_asset`: Register TON network with existing asset connected to native TON asset
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod abi;

use bridge_types::substrate::JettonAppCall;
use bridge_types::traits::OutboundChannel;
use bridge_types::{MainnetAccountId, MainnetAssetId};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use sp_core::Get;
use sp_std::prelude::*;

/// 0.2 TON
pub const TRANSFER_MAX_FEE: u128 = 200_000_000;
/// 0.2 TON
pub const REGISTER_TON_MAX_FEE: u128 = 200_000_000;
/// 0.2 TON
pub const REGISTER_JETTON_MAX_FEE: u128 = 200_000_000;
/// 0.3 TON
pub const REGISTER_SORA_JETTON_MAX_FEE: u128 = 300_000_000;

pub use pallet::*;
pub use weights::WeightInfo;

impl<T: Config> From<JettonAppCall> for Call<T>
where
    T::AccountId: From<MainnetAccountId>,
    AssetIdOf<T>: From<MainnetAssetId>,
{
    fn from(value: JettonAppCall) -> Self {
        match value {
            JettonAppCall::Transfer {
                sender,
                recipient,
                amount,
                token,
            } => Call::mint {
                sender,
                recipient: recipient.into(),
                token,
                amount,
            },
            JettonAppCall::SoraJettonRegistered { token, asset_id } => {
                Call::finish_asset_registration {
                    master: token,
                    asset_id,
                }
            }
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use abi::*;
    use bridge_types::ton::{
        AdditionalTONOutboundData, TonAddress, TonAddressWithPrefix, TonBalance, TonNetworkId,
    };
    use bridge_types::ton::{TonAppInfo, TonAssetInfo};
    use bridge_types::traits::BridgeAssetLocker;
    use bridge_types::traits::{AppRegistry, NetworkManager};
    use bridge_types::traits::{
        BalancePrecisionConverter, BridgeApp, BridgeAssetRegistry, MessageStatusNotifier,
    };
    use bridge_types::types::{
        AssetKind, BridgeAppInfo, BridgeAssetInfo, CallOriginOutput, GenericAdditionalInboundData,
        MessageStatus,
    };
    use bridge_types::MainnetAssetId;
    use bridge_types::{GenericAccount, GenericNetworkId, H256};
    use frame_support::{fail, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use frame_system::{ensure_root, RawOrigin};
    use sp_runtime::traits::Zero;
    use sp_runtime::traits::{Convert, ConvertBack};

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
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

        type OutboundChannel: OutboundChannel<
            TonNetworkId,
            Self::AccountId,
            AdditionalTONOutboundData,
        >;

        type CallOrigin: EnsureOrigin<
            Self::RuntimeOrigin,
            Success = CallOriginOutput<GenericNetworkId, H256, GenericAdditionalInboundData>,
        >;

        type MessageStatusNotifier: MessageStatusNotifier<
            AssetIdOf<Self>,
            Self::AccountId,
            BalanceOf<Self>,
        >;

        type AppRegistry: AppRegistry<TonNetworkId, TonAddress>;

        type AssetRegistry: BridgeAssetRegistry<Self::AccountId, AssetIdOf<Self>>;

        type AssetIdConverter: ConvertBack<AssetIdOf<Self>, MainnetAssetId>;

        type BalancePrecisionConverter: BalancePrecisionConverter<
            AssetIdOf<Self>,
            BalanceOf<Self>,
            TonBalance,
        >;

        type NetworkManager: NetworkManager<
            Self::AccountId,
            AssetIdOf<Self>,
            BalanceOf<Self>,
            TonBalance,
        >;

        type BridgeAssetLocker: BridgeAssetLocker<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Transfer to sidechain.
        Burned {
            asset_id: AssetIdOf<T>,
            sender: T::AccountId,
            recipient: TonAddress,
            amount: BalanceOf<T>,
        },
        /// Transfer from sidechain.
        Minted {
            asset_id: AssetIdOf<T>,
            sender: TonAddress,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Transfer failed, tokens refunded.
        Refunded {
            recipient: T::AccountId,
            asset_id: AssetIdOf<T>,
            amount: BalanceOf<T>,
        },
        /// New asset registered.
        AssetRegistered { asset_id: AssetIdOf<T> },
        /// Fees paid by relayer in EVM was claimed.
        FeesClaimed {
            asset_id: AssetIdOf<T>,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::storage]
    #[pallet::getter(fn app_info)]
    pub(super) type AppInfo<T: Config> = StorageValue<_, (TonNetworkId, TonAddress)>;

    #[pallet::storage]
    #[pallet::getter(fn asset_kind)]
    pub(super) type AssetKinds<T: Config> = StorageMap<_, Identity, AssetIdOf<T>, AssetKind>;

    #[pallet::storage]
    #[pallet::getter(fn token_address)]
    pub(super) type TokenAddresses<T: Config> = StorageMap<_, Identity, AssetIdOf<T>, TonAddress>;

    #[pallet::storage]
    #[pallet::getter(fn asset_by_address)]
    pub(super) type AssetsByAddresses<T: Config> =
        StorageMap<_, Identity, TonAddress, AssetIdOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn sidechain_precision)]
    pub(super) type SidechainPrecision<T: Config> = StorageMap<_, Identity, AssetIdOf<T>, u8>;

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
        /// Wrong bridge request for refund
        WrongRequest,
        /// Wrong bridge request status, must be Failed
        WrongRequestStatus,
        OperationNotSupported,
        WrongAccountPrefix,
        ShouldProvideWallet,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Internal calls to be used from TON side.
        // DON'T CHANGE ORDER

        /// Mint bridged tokens to user account
        ///
        /// Arguments:
        /// - `origin`: Bridge origin with information about network, source contract and etc.
        /// - `token`: Jetton master contract address, or 0:00..00 with prefix 0 for native TON asset
        /// - `sender`: Sender address on TON side
        /// - `recipient`: User account to mint tokens
        /// - `amount`: Amount of tokens to mint with TON network encoding and precision, so real amount could be different
        ///
        /// Fails if:
        /// - Origin network is not registered
        /// - Source contract (Jetton app) is not registered
        /// - Token is not registered
        /// - Sender or token address is wrong (for now we support only standart internal TON addresses)
        /// - Amount precision could not be adjusted to thischain
        /// - Failed to mint tokens for some reason
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            token: TonAddressWithPrefix,
            sender: TonAddressWithPrefix,
            recipient: T::AccountId,
            amount: TonBalance,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id: GenericNetworkId::TON(network_id),
                message_id,
                timepoint,
                additional: GenericAdditionalInboundData::TON(additional),
            } = T::CallOrigin::ensure_origin(origin.clone())? else {
                fail!(DispatchError::BadOrigin);
            };
            let sender = sender.address().ok_or(Error::<T>::WrongAccountPrefix)?;
            let token = token.address().ok_or(Error::<T>::WrongAccountPrefix)?;
            let asset_id = AssetsByAddresses::<T>::get(token)
                // should never return this error, because called from trusted contract on TON
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let asset_kind =
                AssetKinds::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;
            let (app_network_id, app_address) =
                AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;
            let sidechain_precision =
                SidechainPrecision::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

            if additional.source != app_address || network_id != app_network_id {
                return Err(DispatchError::BadOrigin);
            }

            let (amount, _) = T::BalancePrecisionConverter::from_sidechain(
                &asset_id,
                sidechain_precision,
                amount,
            )
            .ok_or(Error::<T>::WrongAmount)?;
            ensure!(amount > Zero::zero(), Error::<T>::WrongAmount);
            T::BridgeAssetLocker::unlock_asset(
                GenericNetworkId::TON(network_id),
                asset_kind,
                &recipient,
                &asset_id,
                &amount,
            )?;

            T::MessageStatusNotifier::inbound_request(
                GenericNetworkId::TON(network_id),
                message_id,
                GenericAccount::TON(sender),
                recipient.clone(),
                asset_id.clone(),
                amount.clone(),
                timepoint,
                MessageStatus::Done,
            );
            Self::deposit_event(Event::Minted {
                asset_id,
                sender,
                recipient,
                amount,
            });
            Ok(())
        }

        // Common exstrinsics

        /// Register network with the new asset for native TON
        ///
        /// Arguments:
        /// - `origin`: Only root can call this extrinsic
        /// - `network_id`: TON network id
        /// - `contract`: Jetton App contract address
        /// - `symbol`: Asset symbol
        /// - `name`: Asset name
        /// - `decimals`: Sidechain precision of native TON
        ///
        /// Fails if:
        /// - Origin is not root
        /// - Network already registered
        /// - Can't register asset
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::register_network())]
        pub fn register_network(
            origin: OriginFor<T>,
            network_id: TonNetworkId,
            contract: TonAddress,
            symbol: AssetSymbolOf<T>,
            name: AssetNameOf<T>,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(!AppInfo::<T>::exists(), Error::<T>::AppAlreadyRegistered);
            AppInfo::<T>::put((network_id, contract));
            let asset_id =
                T::AssetRegistry::register_asset(GenericNetworkId::TON(network_id), name, symbol)?;
            Self::register_asset_inner(
                asset_id,
                TonAddress::empty(),
                AssetKind::Sidechain,
                decimals,
            )?;

            T::AppRegistry::register_app(network_id, contract)?;

            let message = RegisterTonPayload;

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &message.encode().ok_or(Error::<T>::CallEncodeFailed)?,
                AdditionalTONOutboundData {
                    target: contract,
                    max_fee: REGISTER_TON_MAX_FEE.into(),
                },
            )?;
            Ok(())
        }

        /// Register network with the new asset for native TON
        ///
        /// Arguments:
        /// - `origin`: Only root can call this extrinsic
        /// - `network_id`: TON network id
        /// - `contract`: Jetton App contract address
        /// - `asset_id`: Existing TON asset id
        /// - `decimals`: Sidechain precision of native TON
        ///
        /// Fails if:
        /// - Origin is not root
        /// - Network already registered
        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::register_network_with_existing_asset())]
        pub fn register_network_with_existing_asset(
            origin: OriginFor<T>,
            network_id: TonNetworkId,
            contract: TonAddress,
            asset_id: AssetIdOf<T>,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(!AppInfo::<T>::exists(), Error::<T>::AppAlreadyRegistered);
            AppInfo::<T>::put((network_id, contract));
            Self::register_asset_inner(
                asset_id,
                TonAddress::empty(),
                AssetKind::Sidechain,
                decimals,
            )?;

            T::AppRegistry::register_app(network_id, contract)?;

            let message = RegisterTonPayload;

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &message.encode().ok_or(Error::<T>::CallEncodeFailed)?,
                AdditionalTONOutboundData {
                    target: contract,
                    max_fee: REGISTER_TON_MAX_FEE.into(),
                },
            )?;

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn burn(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            recipient: TonAddress,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::burn_inner(who, asset_id, recipient, amount)?;

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::register_network_with_existing_asset())]
        pub fn register_existing_sidechain_asset(
            origin: OriginFor<T>,
            master: TonAddress,
            wallet: TonAddress,
            asset_id: AssetIdOf<T>,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let (network_id, target) = AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;
            Self::register_asset_inner(asset_id, master, AssetKind::Sidechain, decimals)?;

            let message = RegisterJettonPayload { wallet, master };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &message.encode().ok_or(Error::<T>::CallEncodeFailed)?,
                AdditionalTONOutboundData {
                    target,
                    max_fee: REGISTER_TON_MAX_FEE.into(),
                },
            )?;

            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::WeightInfo::register_network_with_existing_asset())]
        pub fn register_sidechain_asset(
            origin: OriginFor<T>,
            master: TonAddress,
            wallet: TonAddress,
            symbol: AssetSymbolOf<T>,
            name: AssetNameOf<T>,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let (network_id, target) = AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;
            let asset_id = T::AssetRegistry::register_asset(
                GenericNetworkId::TON(network_id),
                name.clone(),
                symbol.clone(),
            )?;
            Self::register_asset_inner(asset_id, master, AssetKind::Sidechain, decimals)?;

            let message = RegisterJettonPayload { master, wallet };
            // let message = RegisterSoraJettonSimplePayload {
            //     name: name.as_ref().to_vec(),
            //     symbol: symbol.as_ref().to_vec(),
            // };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &message.encode().ok_or(Error::<T>::CallEncodeFailed)?,
                AdditionalTONOutboundData {
                    target,
                    max_fee: REGISTER_TON_MAX_FEE.into(),
                },
            )?;

            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(<T as Config>::WeightInfo::register_network_with_existing_asset())]
        pub fn register_thischain_asset(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            let (network_id, target) = AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;

            let info = T::AssetRegistry::get_raw_info(asset_id.clone());
            let asset_id = T::AssetIdConverter::convert(asset_id);

            let message = RegisterSoraJettonSimplePayload {
                name: AsRef::<[u8]>::as_ref(&info.name).to_vec(),
                symbol: AsRef::<[u8]>::as_ref(&info.symbol).to_vec(),
                asset_id,
            };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &message.encode().ok_or(Error::<T>::CallEncodeFailed)?,
                AdditionalTONOutboundData {
                    target,
                    max_fee: REGISTER_TON_MAX_FEE.into(),
                },
            )?;

            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn finish_asset_registration(
            origin: OriginFor<T>,
            master: TonAddressWithPrefix,
            asset_id: MainnetAssetId,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id: GenericNetworkId::TON(network_id),
                additional: GenericAdditionalInboundData::TON(additional),
                ..
            } = T::CallOrigin::ensure_origin(origin.clone())? else {
                fail!(DispatchError::BadOrigin);
            };
            let (app_network_id, app_address) =
                AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;

            if additional.source != app_address || network_id != app_network_id {
                return Err(DispatchError::BadOrigin);
            }

            let master = master.address().ok_or(Error::<T>::WrongAccountPrefix)?;
            let asset_id = T::AssetIdConverter::convert_back(asset_id);

            Self::register_asset_inner(asset_id, master, AssetKind::Thischain, 18)?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn register_asset_inner(
            asset_id: AssetIdOf<T>,
            contract: TonAddress,
            asset_kind: AssetKind,
            sidechain_precision: u8,
        ) -> DispatchResult {
            let (network_id, _) = AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;
            ensure!(
                !TokenAddresses::<T>::contains_key(&asset_id),
                Error::<T>::TokenAlreadyRegistered
            );
            TokenAddresses::<T>::insert(&asset_id, contract);
            AssetsByAddresses::<T>::insert(contract, &asset_id);
            AssetKinds::<T>::insert(&asset_id, asset_kind);
            SidechainPrecision::<T>::insert(&asset_id, sidechain_precision);
            T::AssetRegistry::manage_asset(GenericNetworkId::TON(network_id), asset_id.clone())?;
            Self::deposit_event(Event::AssetRegistered { asset_id });
            Ok(())
        }

        pub fn burn_inner(
            who: T::AccountId,
            asset_id: AssetIdOf<T>,
            recipient: TonAddress,
            amount: BalanceOf<T>,
        ) -> Result<H256, DispatchError> {
            let asset_kind =
                AssetKinds::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;
            let (network_id, target) = AppInfo::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;
            let sidechain_precision =
                SidechainPrecision::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

            let (_, sidechain_amount) = T::BalancePrecisionConverter::to_sidechain(
                &asset_id,
                sidechain_precision,
                amount.clone(),
            )
            .ok_or(Error::<T>::WrongAmount)?;

            ensure!(sidechain_amount.balance() > 0, Error::<T>::WrongAmount);

            T::BridgeAssetLocker::lock_asset(
                network_id.into(),
                asset_kind,
                &who,
                &asset_id,
                &amount,
            )?;

            let token_address =
                TokenAddresses::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

            let message = MintPayload {
                token: token_address,
                recipient,
                amount: sidechain_amount,
            };

            let message_id = T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Signed(who.clone()),
                &message.encode().ok_or(Error::<T>::CallEncodeFailed)?,
                AdditionalTONOutboundData {
                    target,
                    max_fee: TRANSFER_MAX_FEE.into(),
                },
            )?;
            T::MessageStatusNotifier::outbound_request(
                GenericNetworkId::TON(network_id),
                message_id,
                who.clone(),
                GenericAccount::TON(recipient),
                asset_id.clone(),
                amount.clone(),
                MessageStatus::InQueue,
            );
            Self::deposit_event(Event::Burned {
                asset_id,
                sender: who,
                recipient,
                amount,
            });

            Ok(message_id)
        }
    }

    impl<T: Config> BridgeApp<T::AccountId, TonAddress, AssetIdOf<T>, BalanceOf<T>> for Pallet<T> {
        fn is_asset_supported(_network_id: GenericNetworkId, _asset_id: AssetIdOf<T>) -> bool {
            false
        }

        fn transfer(
            _network_id: GenericNetworkId,
            _asset_id: AssetIdOf<T>,
            _sender: T::AccountId,
            _recipient: TonAddress,
            _amount: BalanceOf<T>,
        ) -> Result<H256, DispatchError> {
            frame_support::fail!(Error::<T>::InvalidNetwork);
        }

        fn refund(
            _network_id: GenericNetworkId,
            _message_id: H256,
            _recipient: T::AccountId,
            _asset_id: AssetIdOf<T>,
            _amount: BalanceOf<T>,
        ) -> DispatchResult {
            frame_support::fail!(Error::<T>::InvalidNetwork);
        }

        fn list_supported_assets(network_id: GenericNetworkId) -> Vec<BridgeAssetInfo> {
            let app_info = AppInfo::<T>::get();
            if let Some((app_network_id, _)) = app_info {
                if network_id != GenericNetworkId::TON(app_network_id) {
                    vec![]
                } else {
                    AssetKinds::<T>::iter()
                        .filter_map(|(asset_id, _asset_kind)| {
                            TokenAddresses::<T>::get(&asset_id)
                                .zip(SidechainPrecision::<T>::get(&asset_id))
                                .map(|(address, precision)| {
                                    Some(BridgeAssetInfo::Ton(TonAssetInfo {
                                        asset_id: T::AssetIdConverter::convert(asset_id),
                                        address,
                                        precision,
                                    }))
                                })
                                .unwrap_or_default()
                        })
                        .collect()
                }
            } else {
                vec![]
            }
        }

        fn list_apps() -> Vec<BridgeAppInfo> {
            let app_info = AppInfo::<T>::get();
            if let Some((network_id, address)) = app_info {
                vec![BridgeAppInfo::TON(
                    GenericNetworkId::TON(network_id),
                    TonAppInfo { address },
                )]
            } else {
                vec![]
            }
        }

        fn is_asset_supported_weight() -> Weight {
            T::DbWeight::get().reads(1)
        }

        fn refund_weight() -> Weight {
            Default::default()
        }

        fn transfer_weight() -> Weight {
            Default::default()
        }

        fn transfer_fee(
            network_id: GenericNetworkId,
        ) -> Result<(AssetIdOf<T>, BalanceOf<T>), DispatchError> {
            T::NetworkManager::submit_fee(network_id, TonBalance::new(TRANSFER_MAX_FEE))
        }
    }
}
