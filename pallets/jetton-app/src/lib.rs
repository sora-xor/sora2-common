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

//! # EVM Fungible App
//!
//! An application that implements bridged fungible (ERC-20 and native) assets.
//!
//! ## Overview
//!
//! ETH balances are stored in the tightly-coupled [`asset`] runtime module. When an account holder
//! burns some of their balance, a `Transfer` event is emitted. An external relayer will listen for
//! this event and relay it to the other chain.
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `burn`: Burn an ERC20 token balance.
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

use bridge_types::substrate::JettonAppCall;
use bridge_types::{MainnetAccountId, MainnetAssetId};
use bridge_types::{H160, U256};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use sp_core::Get;
use sp_std::prelude::*;

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
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use bridge_types::substrate::{TonAddress, TonAssetId};
    use bridge_types::traits::BridgeAssetLocker;
    use bridge_types::traits::{
        AppRegistry, BalancePrecisionConverter, BridgeApp, BridgeAssetRegistry,
        MessageStatusNotifier,
    };
    use bridge_types::types::{
        AssetKind, BridgeAppInfo, BridgeAssetInfo, CallOriginOutput, GenericAdditionalInboundData,
        MessageStatus, TonAppInfo, TonAssetInfo,
    };
    use bridge_types::MainnetAssetId;
    use bridge_types::{EVMChainId, GenericAccount, GenericNetworkId, H256};
    use frame_support::{fail, pallet_prelude::*};
    use frame_system::ensure_root;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Convert;
    use sp_runtime::traits::Zero;

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

        type CallOrigin: EnsureOrigin<
            Self::RuntimeOrigin,
            Success = CallOriginOutput<GenericNetworkId, H256, GenericAdditionalInboundData>,
        >;

        type MessageStatusNotifier: MessageStatusNotifier<
            AssetIdOf<Self>,
            Self::AccountId,
            BalanceOf<Self>,
        >;

        type AssetRegistry: BridgeAssetRegistry<Self::AccountId, AssetIdOf<Self>>;

        type AppRegistry: AppRegistry<EVMChainId, H160>;

        type AssetIdConverter: Convert<AssetIdOf<Self>, MainnetAssetId>;

        type BalancePrecisionConverter: BalancePrecisionConverter<
            AssetIdOf<Self>,
            BalanceOf<Self>,
            U256,
        >;

        type BridgeAssetLocker: BridgeAssetLocker<Self::AccountId>;

        #[pallet::constant]
        type BaseFeeLifetime: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type PriorityFee: Get<u128>;

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
    #[pallet::getter(fn app_address)]
    pub(super) type AppAddress<T: Config> = StorageValue<_, TonAddress>;

    #[pallet::storage]
    #[pallet::getter(fn asset_kind)]
    pub(super) type AssetKinds<T: Config> = StorageMap<_, Identity, AssetIdOf<T>, AssetKind>;

    #[pallet::storage]
    #[pallet::getter(fn token_address)]
    pub(super) type TokenAddresses<T: Config> = StorageMap<_, Identity, AssetIdOf<T>, TonAssetId>;

    #[pallet::storage]
    #[pallet::getter(fn asset_by_address)]
    pub(super) type AssetsByAddresses<T: Config> =
        StorageMap<_, Identity, TonAssetId, AssetIdOf<T>>;

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
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        /// [address]
        pub app: Option<TonAddress>,
        /// Vec<[asset_id, address, kind, precision]>
        pub assets: Vec<(AssetIdOf<T>, TonAddress, AssetKind, u8)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                app: Default::default(),
                assets: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            if let Some(app) = &self.app {
                AppAddress::<T>::set(Some(*app));
                for (asset_id, address, asset_kind, precision) in self.assets.iter() {
                    Pallet::<T>::register_asset_inner(
                        asset_id.clone(),
                        *address,
                        *asset_kind,
                        *precision,
                    )
                    .unwrap();
                }
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Internal calls to be used from Ethereum side.
        // DON'T CHANGE ORDER

        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            token: TonAssetId,
            sender: TonAddress,
            recipient: T::AccountId,
            amount: U256,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id: GenericNetworkId::TON,
                message_id,
                timepoint,
                additional: GenericAdditionalInboundData::TON(additional),
            } = T::CallOrigin::ensure_origin(origin.clone())? else {
                fail!(DispatchError::BadOrigin);
            };
            let asset_id = AssetsByAddresses::<T>::get(token)
                // should never return this error, because called from Ethereum
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let asset_kind =
                AssetKinds::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;
            let app_address = AppAddress::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;
            let sidechain_precision =
                SidechainPrecision::<T>::get(&asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;

            if additional.source != app_address {
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
                GenericNetworkId::TON,
                asset_kind,
                &recipient,
                &asset_id,
                &amount,
            )?;

            T::MessageStatusNotifier::inbound_request(
                GenericNetworkId::TON,
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

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::register_sidechain_asset())]
        pub fn register_sidechain_asset(
            origin: OriginFor<T>,
            address: TonAddress,
            symbol: AssetSymbolOf<T>,
            name: AssetNameOf<T>,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetsByAddresses::<T>::contains_key(address),
                Error::<T>::TokenAlreadyRegistered
            );
            let _target = AppAddress::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;

            let asset_id = T::AssetRegistry::register_asset(GenericNetworkId::TON, name, symbol)?;

            Self::register_asset_inner(asset_id, address, AssetKind::Sidechain, decimals)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::register_existing_sidechain_asset())]
        pub fn register_existing_sidechain_asset(
            origin: OriginFor<T>,
            address: TonAddress,
            asset_id: AssetIdOf<T>,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetsByAddresses::<T>::contains_key(address),
                Error::<T>::TokenAlreadyRegistered
            );
            let _target = AppAddress::<T>::get().ok_or(Error::<T>::AppIsNotRegistered)?;

            Self::register_asset_inner(asset_id, address, AssetKind::Sidechain, decimals)?;

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
            ensure!(AppAddress::<T>::exists(), Error::<T>::AppIsNotRegistered);
            ensure!(
                !TokenAddresses::<T>::contains_key(&asset_id),
                Error::<T>::TokenAlreadyRegistered
            );
            TokenAddresses::<T>::insert(&asset_id, contract);
            AssetsByAddresses::<T>::insert(contract, &asset_id);
            AssetKinds::<T>::insert(&asset_id, asset_kind);
            SidechainPrecision::<T>::insert(&asset_id, sidechain_precision);
            T::AssetRegistry::manage_asset(GenericNetworkId::TON, asset_id.clone())?;
            Self::deposit_event(Event::AssetRegistered { asset_id });
            Ok(())
        }
    }

    impl<T: Config> BridgeApp<T::AccountId, H160, AssetIdOf<T>, BalanceOf<T>> for Pallet<T> {
        fn is_asset_supported(_network_id: GenericNetworkId, _asset_id: AssetIdOf<T>) -> bool {
            false
        }

        fn transfer(
            _network_id: GenericNetworkId,
            _asset_id: AssetIdOf<T>,
            _sender: T::AccountId,
            _recipient: H160,
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
            if network_id != GenericNetworkId::TON {
                return vec![];
            }
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

        fn list_apps() -> Vec<BridgeAppInfo> {
            let address = AppAddress::<T>::get();
            if let Some(address) = address {
                vec![BridgeAppInfo::TON(
                    GenericNetworkId::TON,
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
            <T as Config>::WeightInfo::burn()
        }
    }
}
