//! # ERC20
//!
//! An application that implements bridged ERC20 token assets.
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

use assets::AssetIdOf;
use bridge_types::substrate::SubstrateAppMessage;
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use frame_system::ensure_signed;
use sp_std::prelude::*;

pub use weights::WeightInfo;

pub use pallet::*;

impl<T: Config> Into<Call<T>> for SubstrateAppMessage<T::AccountId, AssetIdOf<T>, BalanceOf<T>> {
    fn into(self) -> Call<T> {
        match self {
            SubstrateAppMessage::Transfer {
                sender,
                recipient,
                amount,
                asset_id,
            } => Call::mint {
                sender,
                recipient,
                amount,
                asset_id,
            },
        }
    }
}

#[frame_support::pallet]
pub mod pallet {

    use super::*;

    use assets::AssetIdOf;
    use bridge_types::substrate::{
        ParachainAccountId, ParachainAssetId, SubstrateBridgeMessageEncode, XCMAppMessage,
    };
    use bridge_types::traits::{MessageStatusNotifier, OutboundChannel};
    use bridge_types::types::{AssetKind, CallOriginOutput};
    use bridge_types::{GenericAccount, GenericNetworkId, SubNetworkId, H256};
    use common::{AssetName, AssetSymbol, Balance};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_system::{ensure_root, RawOrigin};
    use traits::currency::MultiCurrency;

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type BalanceOf<T> =
        <<T as assets::Config>::Currency as MultiCurrency<AccountIdOf<T>>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config:
        frame_system::Config + assets::Config + permissions::Config + technical::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type OutboundChannel: OutboundChannel<SubNetworkId, Self::AccountId, ()>;

        type CallOrigin: EnsureOrigin<
            Self::Origin,
            Success = CallOriginOutput<SubNetworkId, H256, ()>,
        >;

        type MessageStatusNotifier: MessageStatusNotifier<Self::AssetId, Self::AccountId>;

        type BridgeTechAccountId: Get<Self::TechAccountId>;

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
            ParachainAccountId,
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /*
        Internal calls to be used from Ethereum side.
        DON'T CHANGE ORDER
         */

        #[pallet::weight(<T as Config>::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            asset_id: AssetIdOf<T>,
            sender: ParachainAccountId,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let CallOriginOutput {
                network_id,
                message_id,
                timestamp,
                ..
            } = T::CallOrigin::ensure_origin(origin.clone())?;

            let asset_kind = AssetKinds::<T>::get(network_id, asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;

            let bridge_account = Self::bridge_account()?;

            ensure!(amount > 0, Error::<T>::WrongAmount);
            match asset_kind {
                AssetKind::Thischain => {
                    assets::Pallet::<T>::transfer_from(
                        &asset_id,
                        &bridge_account,
                        &recipient,
                        amount,
                    )?;
                }
                AssetKind::Sidechain => {
                    assets::Pallet::<T>::mint_to(&asset_id, &bridge_account, &recipient, amount)?;
                }
            }
            T::MessageStatusNotifier::inbound_request(
                GenericNetworkId::Sub(network_id),
                message_id,
                GenericAccount::Parachain(sender.clone()),
                GenericAccount::Sora(recipient.clone()),
                asset_id,
                amount,
                timestamp,
            );
            Self::deposit_event(Event::Minted(
                network_id, asset_id, sender, recipient, amount,
            ));
            Ok(())
        }

        /*
        Common exstrinsics
         */

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

        #[pallet::weight(<T as Config>::WeightInfo::register_erc20_asset())]

        pub fn register_sidechain_asset(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            sidechain_asset: ParachainAssetId,
            symbol: AssetSymbol,
            name: AssetName,
            decimals: u8,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let bridge_account = Self::bridge_account()?;

            let asset_id = assets::Pallet::<T>::register_from(
                &bridge_account,
                symbol,
                name,
                decimals,
                Balance::from(0u32),
                true,
                None,
                None,
            )?;

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
            let bridge_account = Self::bridge_account()?;
            AssetKinds::<T>::insert(&network_id, &asset_id, &asset_kind);

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                &XCMAppMessage::<T::AccountId, AssetIdOf<T>, BalanceOf<T>>::RegisterAsset {
                    asset_id,
                    sidechain_asset,
                }
                .prepare_message(),
                (),
            )?;

            // Err when permission already exists
            for permission_id in [permissions::BURN, permissions::MINT] {
                let _ = permissions::Pallet::<T>::assign_permission(
                    bridge_account.clone(),
                    &bridge_account,
                    permission_id,
                    permissions::Scope::Limited(common::hash(&asset_id)),
                );
            }
            Ok(())
        }

        fn bridge_account() -> Result<T::AccountId, DispatchError> {
            Ok(technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::BridgeTechAccountId::get(),
            )?)
        }

        pub fn burn_inner(
            who: T::AccountId,
            network_id: SubNetworkId,
            asset_id: AssetIdOf<T>,
            recipient: ParachainAccountId,
            amount: BalanceOf<T>,
        ) -> Result<H256, DispatchError> {
            ensure!(amount > 0, Error::<T>::WrongAmount);
            let asset_kind = AssetKinds::<T>::get(network_id, &asset_id)
                .ok_or(Error::<T>::TokenIsNotRegistered)?;
            let bridge_account = Self::bridge_account()?;

            match asset_kind {
                AssetKind::Sidechain => {
                    assets::Pallet::<T>::burn_from(&asset_id, &bridge_account, &who, amount)?;
                }
                AssetKind::Thischain => {
                    assets::Pallet::<T>::transfer_from(&asset_id, &who, &bridge_account, amount)?;
                }
            }

            let message_id = T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Signed(who.clone()),
                &XCMAppMessage::Transfer {
                    recipient: recipient.clone(),
                    amount,
                    asset_id,
                    sender: who.clone(),
                }
                .prepare_message(),
                (),
            )?;
            T::MessageStatusNotifier::outbound_request(
                GenericNetworkId::Sub(network_id),
                message_id,
                GenericAccount::Sora(who.clone()),
                GenericAccount::Parachain(recipient.clone()),
                asset_id,
                amount,
            );
            Self::deposit_event(Event::Burned(network_id, asset_id, who, recipient, amount));

            Ok(Default::default())
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub assets: Vec<(SubNetworkId, T::AssetId, AssetKind)>,
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
