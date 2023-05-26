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

//! # Substrate App
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

use bridge_types::substrate::{
    MainnetAccountId, MainnetAssetId, MainnetBalance, ParachainAccountId, SubstrateAppCall,
};
use bridge_types::traits::BridgeApp;
use bridge_types::types::{AssetKind, BridgeAppInfo, BridgeAssetInfo, SubAssetInfo};
use bridge_types::GenericNetworkId;
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use frame_system::ensure_signed;
use sp_runtime::traits::{Convert, Zero};
use sp_std::prelude::*;
use traits::MultiCurrency;

pub use weights::WeightInfo;

pub use pallet::*;

impl<T: Config> From<SubstrateAppCall> for Call<T>
where
    T::AccountId: From<MainnetAccountId>,
    AssetIdOf<T>: From<MainnetAssetId>,
    BalanceOf<T>: From<MainnetBalance>,
{
    fn from(value: SubstrateAppCall) -> Self {
        match value {
            SubstrateAppCall::Transfer {
                sender,
                recipient,
                amount,
                asset_id,
            } => Call::mint {
                sender,
                recipient: recipient.into(),
                amount: amount.into(),
                asset_id: asset_id.into(),
            },
            SubstrateAppCall::FinalizeAssetRegistration {
                asset_id,
                asset_kind,
            } => Call::finalize_asset_registration {
                asset_id: asset_id.into(),
                asset_kind,
            },
        }
    }
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    use bridge_types::substrate::{
        MainnetAccountId, MainnetAssetId, MainnetBalance, ParachainAccountId, ParachainAssetId,
        SubstrateBridgeMessageEncode, XCMAppCall,
    };
    use bridge_types::traits::{BridgeAssetRegistry, MessageStatusNotifier, OutboundChannel};
    use bridge_types::types::{AssetKind, CallOriginOutput, MessageStatus};
    use bridge_types::{GenericAccount, GenericNetworkId, SubNetworkId, H256};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_system::{ensure_root, RawOrigin};
    use traits::currency::MultiCurrency;

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

    pub type AssetIdOf<T> = <<T as Config>::Currency as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::CurrencyId;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

    pub type AssetNameOf<T> = <<T as Config>::AssetRegistry as BridgeAssetRegistry<
        AccountIdOf<T>,
        AssetIdOf<T>,
    >>::AssetName;
    pub type AssetSymbolOf<T> = <<T as Config>::AssetRegistry as BridgeAssetRegistry<
        AccountIdOf<T>,
        AssetIdOf<T>,
    >>::AssetSymbol;
    pub type DecimalsOf<T> = <<T as Config>::AssetRegistry as BridgeAssetRegistry<
        AccountIdOf<T>,
        AssetIdOf<T>,
    >>::Decimals;

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

        type BridgeAccountId: Get<Self::AccountId>;

        type Currency: MultiCurrency<Self::AccountId>;

        type AccountIdConverter: Convert<Self::AccountId, MainnetAccountId>;

        type AssetIdConverter: Convert<AssetIdOf<Self>, MainnetAssetId>;

        type BalanceConverter: Convert<BalanceOf<Self>, MainnetBalance>;

        type WeightInfo: WeightInfo;

        type BridgeTransferLimit: Get<BalanceOf<Self>>;

        type BridgeTransferLimiter: BridgeTransferLimiter<MainnetAssetId, MainnetBalance>;
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Internal calls to be used from Parachain side.

        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            sender: Option<ParachainAccountId>,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id,
                message_id,
                timepoint,
                ..
            } = T::CallOrigin::ensure_origin(origin.clone())?;

            let asset_kind = AssetKinds::<T>::get(network_id, asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;

            let bridge_account = Self::bridge_account()?;

            ensure!(amount > BalanceOf::<T>::zero(), Error::<T>::WrongAmount);
            match asset_kind {
                AssetKind::Thischain => {
                    <T as Config>::Currency::transfer(
                        asset_id,
                        &bridge_account,
                        &recipient,
                        amount,
                    )?;
                }
                AssetKind::Sidechain => {
                    <T as Config>::Currency::deposit(asset_id, &recipient, amount)?;
                }
            }
            T::MessageStatusNotifier::inbound_request(
                GenericNetworkId::Sub(network_id),
                message_id,
                sender
                    .clone()
                    .map(GenericAccount::Parachain)
                    .unwrap_or(GenericAccount::Unknown),
                recipient.clone(),
                asset_id,
                amount,
                timepoint,
                MessageStatus::Done,
            );
            Self::deposit_event(Event::Minted(
                network_id, asset_id, sender, recipient, amount,
            ));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn finalize_asset_registration(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            asset_kind: AssetKind,
        ) -> DispatchResult {
            let CallOriginOutput { network_id, .. } = T::CallOrigin::ensure_origin(origin.clone())?;
            AssetKinds::<T>::insert(network_id, asset_id, asset_kind);
            Ok(())
        }

        // Common exstrinsics

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

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::register_erc20_asset())]
        pub fn register_thischain_asset(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            sidechain_asset: ParachainAssetId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                !AssetKinds::<T>::contains_key(network_id, asset_id),
                Error::<T>::TokenAlreadyRegistered
            );

            Self::register_asset_inner(
                network_id,
                asset_id,
                sidechain_asset,
                AssetKind::Thischain,
            )?;

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::register_erc20_asset())]
        pub fn register_sidechain_asset(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            sidechain_asset: ParachainAssetId,
            symbol: AssetSymbolOf<T>,
            name: AssetNameOf<T>,
            decimals: DecimalsOf<T>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let bridge_account = Self::bridge_account()?;

            let asset_id =
                T::AssetRegistry::register_asset(bridge_account, name, symbol, decimals)?;

            Self::register_asset_inner(
                network_id,
                asset_id,
                sidechain_asset,
                AssetKind::Sidechain,
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
        ) -> DispatchResult {
            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &XCMAppCall::RegisterAsset {
                    asset_id: T::AssetIdConverter::convert(asset_id),
                    sidechain_asset,
                    asset_kind,
                }
                .prepare_message(),
                (),
            )?;
            Ok(())
        }

        pub fn bridge_account() -> Result<T::AccountId, DispatchError> {
            Ok(T::BridgeAccountId::get())
        }

        pub fn burn_inner(
            who: T::AccountId,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: ParachainAccountId,
            amount: BalanceOf<T>,
        ) -> Result<H256, DispatchError> {
            ensure!(amount > BalanceOf::<T>::zero(), Error::<T>::WrongAmount);
            // Self::is_amount_under_limit(amount)?;\
            // let a = <T as Config>::BridgeTransferLimiter::is_transfer_under_limit();

            let asset_kind = AssetKinds::<T>::get(network_id, asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let bridge_account = Self::bridge_account()?;

            match asset_kind {
                AssetKind::Sidechain => {
                    T::Currency::withdraw(asset_id, &who, amount)?;
                }
                AssetKind::Thischain => {
                    T::Currency::transfer(asset_id, &who, &bridge_account, amount)?;
                }
            }

            let message_id = T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Signed(who.clone()),
                &XCMAppCall::Transfer {
                    recipient: recipient.clone(),
                    amount: T::BalanceConverter::convert(amount),
                    asset_id: T::AssetIdConverter::convert(asset_id),
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
                asset_id,
                amount,
                MessageStatus::InQueue,
            );
            Self::deposit_event(Event::Burned(network_id, asset_id, who, recipient, amount));

            Ok(Default::default())
        }

        fn is_amount_under_limit(amount: BalanceOf<T>) -> DispatchResult {
            let amount = <T as Config>::BalanceConverter::convert(amount);
            let limit_amount =
                <T as Config>::BalanceConverter::convert(<T as Config>::BridgeTransferLimit::get());
            ensure!(amount < limit_amount, Error::<T>::TransferLimitReached);
            Ok(())
        }

        // pub fn convert_precision(
        //     precision_from: u8,
        //     precision_to: u8,
        //     amount: MainnetBalance,
        // ) -> Result<(MainnetBalance, MainnetBalance), DispatchError> {
        //     if precision_from == precision_to {
        //         return Ok((amount, 0));
        //     }
        //     let pair = if precision_from < precision_to {
        //         let exp = (precision_to - precision_from) as u32;
        //         let coeff = 10_u128.pow(exp);
        //         let coerced_amount = amount.saturating_mul(coeff);
        //         // ensure!(
        //         //     coerced_amount / coeff == amount,
        //         //     Error::<T>::UnsupportedAssetPrecision
        //         // );
        //         (coerced_amount, 0)
        //     } else {
        //         let exp = (precision_from - precision_to) as u32;
        //         let coeff = 10_u128.pow(exp);
        //         let coerced_amount = amount / coeff;
        //         let diff = amount - coerced_amount * coeff;
        //         (coerced_amount, diff)
        //     };
        //     Ok(pair)
        // }
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
            AssetKinds::<T>::get(network_id, asset_id).ok_or(Error::<T>::TokenIsNotRegistered)?;
        let bridge_account = Self::bridge_account()?;

        match asset_kind {
            AssetKind::Sidechain => {
                T::Currency::deposit(asset_id, &recipient, amount)?;
            }
            AssetKind::Thischain => {
                T::Currency::transfer(asset_id, &bridge_account, &recipient, amount)?;
            }
        }
        Ok(())
    }

    fn list_supported_assets(
        network_id: GenericNetworkId,
    ) -> Vec<bridge_types::types::BridgeAssetInfo> {
        let GenericNetworkId::Sub(network_id) = network_id else {
            return vec![];
        };
        let assets = AssetKinds::<T>::iter_prefix(network_id)
            .map(|(asset_id, asset_kind)| {
                let asset_id = T::AssetIdConverter::convert(asset_id);
                BridgeAssetInfo::Sub(SubAssetInfo {
                    asset_id,
                    asset_kind,
                    precision: 18,
                })
            })
            .collect();
        assets
    }

    fn list_apps() -> Vec<bridge_types::types::BridgeAppInfo> {
        AssetKinds::<T>::iter_keys()
            .map(|(network_id, _asset_id)| BridgeAppInfo::Sub(network_id.into()))
            .fold(vec![], |mut acc, value| {
                if acc.iter().find(|x| value == **x).is_none() {
                    acc.push(value);
                }
                acc
            })
    }
}

pub trait BridgeTransferLimiter<AssetId, Balance> {
    fn is_transfer_under_limit(asset: AssetId, limit: Balance, amount: Balance) -> bool;
}