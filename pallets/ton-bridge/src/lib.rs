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

//! # TON Bridge manager
//!
//! An application that implements bridged fungible (Jetton and native) assets.
//!
//! ## Interface
//!
//! ### Dispatchable Calls
//!
//! - `burn`: Burn an ERC20 token balance.
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub const SUBMIT_BASE_FEE: u128 = 100_000_000;
pub const ADD_PEER_MAX_FEE: u128 = 100_000_000;
pub const REMOVE_PEER_MAX_FEE: u128 = 100_000_000;
pub const REGISTER_APP_MAX_FEE: u128 = 100_000_000;
pub const REMOVE_APP_MAX_FEE: u128 = 100_000_000;

extern crate alloc;

pub mod abi;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use bridge_types::substrate::TonBridgeCall;
use bridge_types::ton::{
    AdditionalTONOutboundData, Commitment, TonAddress, TonBalance, TonNetworkId,
};
use bridge_types::traits::BalancePrecisionConverter;
use bridge_types::traits::CommitmentHandler;
use bridge_types::traits::OutboundChannel;
use bridge_types::traits::PeerManager;
use bridge_types::traits::{AppRegistry, NetworkManager};
use bridge_types::{GenericNetworkId, MainnetAccountId};
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use frame_system::{ensure_signed, RawOrigin};
use sp_core::ed25519;
use sp_core::Get;
use sp_runtime::DispatchError;
use sp_runtime::Saturating;
use sp_std::prelude::*;

pub use pallet::*;
pub use weights::WeightInfo;

#[derive(
    Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Debug,
)]
pub struct FeeInfo<AssetId> {
    pub asset: AssetId,
    pub precision: u8,
}

#[derive(
    Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen, Debug,
)]
pub struct NetworkConfig<AssetId> {
    pub network_id: TonNetworkId,
    pub channel: TonAddress,
    pub fee_info: Option<FeeInfo<AssetId>>,
}

impl<T: Config> From<TonBridgeCall> for Call<T>
where
    T::AccountId: From<MainnetAccountId>,
{
    fn from(value: TonBridgeCall) -> Self {
        match value {
            TonBridgeCall::RegisterRelayer { relayer, account } => Call::register_relayer {
                relayer,
                owner: account.into(),
            },
            TonBridgeCall::CommitmentSubmitted { relayer, fee } => {
                Call::commitment_submitted { relayer, fee }
            }
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use bridge_types::ton::{AdditionalTONOutboundData, TonAddressWithPrefix, TonBalance};
    use bridge_types::traits::BridgeAssetLocker;
    use bridge_types::traits::{BalancePrecisionConverter, MessageStatusNotifier, OutboundChannel};
    use bridge_types::types::{CallOriginOutput, GenericAdditionalInboundData};
    use bridge_types::MainnetAssetId;
    use bridge_types::{GenericNetworkId, H256};
    use frame_support::{fail, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Convert;
    use sp_runtime::traits::Zero;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type AssetIdOf<T> =
        <<T as Config>::BridgeAssetLocker as BridgeAssetLocker<AccountIdOf<T>>>::AssetId;

    pub type BalanceOf<T> =
        <<T as Config>::BridgeAssetLocker as BridgeAssetLocker<AccountIdOf<T>>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
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

        type AssetIdConverter: Convert<AssetIdOf<Self>, MainnetAssetId>;

        type BalancePrecisionConverter: BalancePrecisionConverter<
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
        FeesClaimed {
            recipient: T::AccountId,
            asset_id: AssetIdOf<T>,
            amount: BalanceOf<T>,
        },
    }

    /// Collected fees
    #[pallet::storage]
    #[pallet::getter(fn collected_fees)]
    pub(super) type CollectedFees<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Fees spend by relayer
    #[pallet::storage]
    #[pallet::getter(fn spent_fees)]
    pub(super) type SpentFees<T: Config> =
        StorageMap<_, Identity, TonAddress, BalanceOf<T>, ValueQuery>;

    /// Relayer owner accounts to claim fees
    #[pallet::storage]
    #[pallet::getter(fn relayer_owner)]
    pub(super) type RelayerOwners<T: Config> =
        StorageMap<_, Identity, TonAddress, T::AccountId, OptionQuery>;

    /// Native asset precision
    #[pallet::storage]
    #[pallet::getter(fn network_config)]
    pub(super) type NetworkConfigs<T: Config> =
        StorageValue<_, NetworkConfig<AssetIdOf<T>>, OptionQuery>;

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
        BaseFeeLifetimeExceeded,
        InvalidSignature,
        NothingToClaim,
        NotEnoughFeesCollected,
        BaseFeeIsNotAvailable,
        InvalidBaseFeeUpdate,
        OwnerIsNotAdded,
        AccountIsNotOwner,
        NetworkNotSupported,
        InvalidSourceChannel,
        InvalidCommitment,
        StaleBaseFeeUpdate,
        WrongAccountPrefix,
        FeeAssetIsNotDefined,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::register_native_app())]
        pub fn claim_relayer_fees(
            origin: OriginFor<T>,
            relayer: TonAddress,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let config = NetworkConfigs::<T>::get().ok_or(Error::<T>::NetworkNotSupported)?;
            let fee_info = config.fee_info.ok_or(Error::<T>::NetworkNotSupported)?;
            let owner = RelayerOwners::<T>::get(relayer).ok_or(Error::<T>::OwnerIsNotAdded)?;
            ensure!(who == owner, Error::<T>::AccountIsNotOwner);
            let spent_fees = SpentFees::<T>::get(relayer);
            ensure!(spent_fees > Zero::zero(), Error::<T>::NothingToClaim);

            let collected_fees = CollectedFees::<T>::get();
            ensure!(
                collected_fees > Zero::zero(),
                Error::<T>::NotEnoughFeesCollected
            );
            let fees_to_claim = collected_fees.clone().min(spent_fees.clone());

            T::BridgeAssetLocker::refund_fee(
                config.network_id.into(),
                &who,
                &fee_info.asset,
                &fees_to_claim,
            )?;
            Self::deposit_event(Event::FeesClaimed {
                asset_id: fee_info.asset,
                recipient: who,
                amount: fees_to_claim.clone(),
            });

            SpentFees::<T>::insert(relayer, spent_fees.saturating_sub(fees_to_claim.clone()));
            CollectedFees::<T>::set(collected_fees.saturating_sub(fees_to_claim));

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn register_network(
            origin: OriginFor<T>,
            network_id: TonNetworkId,
            channel: TonAddress,
            fee_info: Option<FeeInfo<AssetIdOf<T>>>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            NetworkConfigs::<T>::put(NetworkConfig {
                network_id,
                channel,
                fee_info,
            });
            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn register_fee_info(
            origin: OriginFor<T>,
            fee_info: Option<FeeInfo<AssetIdOf<T>>>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            let config = Self::get_config()?;
            NetworkConfigs::<T>::put(NetworkConfig {
                network_id: config.network_id,
                channel: config.channel,
                fee_info,
            });
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn register_relayer(
            origin: OriginFor<T>,
            relayer: TonAddressWithPrefix,
            owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let CallOriginOutput {
                network_id: GenericNetworkId::TON(network_id),
                additional: GenericAdditionalInboundData::TON(additional),
                ..
            } = T::CallOrigin::ensure_origin(origin)? else {
                fail!(DispatchError::BadOrigin);
            };
            let relayer = relayer.address().ok_or(Error::<T>::WrongAccountPrefix)?;
            Self::ensure_channel(network_id, additional.source)?;
            RelayerOwners::<T>::insert(relayer, owner);
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        pub fn commitment_submitted(
            origin: OriginFor<T>,
            relayer: TonAddressWithPrefix,
            fee: TonBalance,
        ) -> DispatchResultWithPostInfo {
            let CallOriginOutput {
                network_id: GenericNetworkId::TON(network_id),
                additional: GenericAdditionalInboundData::TON(additional),
                ..
            } = T::CallOrigin::ensure_origin(origin)? else {
                fail!(DispatchError::BadOrigin);
            };
            Self::ensure_channel(network_id, additional.source)?;
            let relayer = relayer.address().ok_or(Error::<T>::WrongAccountPrefix)?;
            let config = Self::get_config()?;
            let fee_info = config.fee_info.ok_or(Error::<T>::FeeAssetIsNotDefined)?;
            let (amount, _) = T::BalancePrecisionConverter::from_sidechain(
                &fee_info.asset,
                fee_info.precision,
                fee,
            )
            .ok_or(Error::<T>::WrongAmount)?;
            ensure!(amount > Zero::zero(), Error::<T>::WrongAmount);
            SpentFees::<T>::mutate(relayer, |value| {
                *value = value.clone().saturating_add(amount);
            });
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn ensure_channel(network_id: TonNetworkId, channel: TonAddress) -> DispatchResult {
        let config = Self::get_config()?;
        ensure!(config.network_id == network_id, Error::<T>::InvalidNetwork);
        ensure!(config.channel == channel, Error::<T>::InvalidSourceChannel);
        Ok(())
    }

    fn get_config() -> Result<NetworkConfig<AssetIdOf<T>>, DispatchError> {
        NetworkConfigs::<T>::get().ok_or(Error::<T>::NetworkNotSupported.into())
    }
}

impl<T: Config> AppRegistry<TonNetworkId, TonAddress> for Pallet<T> {
    fn register_app(network_id: TonNetworkId, app: TonAddress) -> DispatchResult {
        let config = Self::get_config()?;
        ensure!(network_id == config.network_id, Error::<T>::InvalidNetwork);
        T::OutboundChannel::submit(
            network_id,
            &RawOrigin::Root,
            &abi::RegisterAppPayload { app }
                .encode()
                .ok_or(Error::<T>::CallEncodeFailed)?,
            AdditionalTONOutboundData {
                max_fee: REGISTER_APP_MAX_FEE.into(),
                target: config.channel,
            },
        )?;
        Ok(())
    }

    fn deregister_app(network_id: TonNetworkId, app: TonAddress) -> DispatchResult {
        let config = Self::get_config()?;
        ensure!(network_id == config.network_id, Error::<T>::InvalidNetwork);
        T::OutboundChannel::submit(
            network_id,
            &RawOrigin::Root,
            &abi::RemoveAppPayload { app }
                .encode()
                .ok_or(Error::<T>::CallEncodeFailed)?,
            AdditionalTONOutboundData {
                max_fee: REMOVE_APP_MAX_FEE.into(),
                target: config.channel,
            },
        )?;
        Ok(())
    }
}

impl<T: Config, MaxMessages: Get<u32>, MaxPayload: Get<u32>>
    CommitmentHandler<TonNetworkId, Commitment<MaxMessages, MaxPayload>> for Pallet<T>
{
    fn verify_commitment(
        network_id: &TonNetworkId,
        commitment: &Commitment<MaxMessages, MaxPayload>,
    ) -> DispatchResult {
        match commitment {
            Commitment::Inbound(inbound_commitment) => {
                Self::ensure_channel(*network_id, inbound_commitment.channel)?;
            }
            Commitment::Outbound(_) => {
                frame_support::fail!(Error::<T>::InvalidCommitment);
            }
        }
        Ok(())
    }

    fn handle_commitment(
        _network_id: &TonNetworkId,
        commitment: &Commitment<MaxMessages, MaxPayload>,
    ) -> DispatchResult {
        match commitment {
            Commitment::Inbound(_) => {}
            Commitment::Outbound(_) => {
                frame_support::fail!(Error::<T>::InvalidCommitment);
            }
        }
        Ok(())
    }
}

impl<T: Config> PeerManager<TonNetworkId, ed25519::Public> for Pallet<T> {
    fn add_peer(network_id: TonNetworkId, peer: ed25519::Public) -> DispatchResult {
        let config = Self::get_config()?;
        ensure!(network_id == config.network_id, Error::<T>::InvalidNetwork);
        T::OutboundChannel::submit(
            network_id,
            &RawOrigin::Root,
            &abi::AddPeerPayload { peer }
                .encode()
                .ok_or(Error::<T>::CallEncodeFailed)?,
            AdditionalTONOutboundData {
                max_fee: ADD_PEER_MAX_FEE.into(),
                target: config.channel,
            },
        )?;
        Ok(())
    }

    fn remove_peer(network_id: TonNetworkId, peer: ed25519::Public) -> DispatchResult {
        let config = Self::get_config()?;
        ensure!(network_id == config.network_id, Error::<T>::InvalidNetwork);
        T::OutboundChannel::submit(
            network_id,
            &RawOrigin::Root,
            &abi::RemovePeerPayload { peer }
                .encode()
                .ok_or(Error::<T>::CallEncodeFailed)?,
            AdditionalTONOutboundData {
                max_fee: REMOVE_PEER_MAX_FEE.into(),
                target: config.channel,
            },
        )?;
        Ok(())
    }

    fn submit_weight() -> frame_support::weights::Weight {
        <T::OutboundChannel as OutboundChannel<_, _, _>>::submit_weight()
    }
}

impl<T: Config> NetworkManager<T::AccountId, AssetIdOf<T>, BalanceOf<T>, TonBalance> for Pallet<T> {
    fn submit_fee(
        network_id: GenericNetworkId,
        message_fee: TonBalance,
    ) -> Result<(AssetIdOf<T>, BalanceOf<T>), DispatchError> {
        let config = Self::get_config()?;
        ensure!(
            GenericNetworkId::TON(config.network_id) == network_id,
            Error::<T>::InvalidNetwork
        );
        let fee_info = config.fee_info.ok_or(Error::<T>::FeeAssetIsNotDefined)?;
        let (amount, _) = T::BalancePrecisionConverter::from_sidechain(
            &fee_info.asset,
            fee_info.precision,
            SUBMIT_BASE_FEE.saturating_add(message_fee.balance()).into(),
        )
        .ok_or(Error::<T>::WrongAmount)?;
        Ok((fee_info.asset, amount))
    }

    fn on_withdraw_submit_fee(
        network_id: GenericNetworkId,
        message_fee: TonBalance,
    ) -> DispatchResult {
        let (_, fee) = Self::submit_fee(network_id, message_fee)?;
        CollectedFees::<T>::mutate(|value| {
            *value = value.clone().saturating_add(fee);
        });
        Ok(())
    }
}
