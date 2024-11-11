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
pub const ADD_PEER_MAX_GAS: u64 = 100_000;
pub const REMOVE_PEER_MAX_GAS: u64 = 100_000;

extern crate alloc;

mod abi;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use bridge_types::evm::AdditionalEVMOutboundData;
use bridge_types::evm::Commitment;
use bridge_types::traits::AppRegistry;
use bridge_types::traits::CommitmentHandler;
use bridge_types::traits::EVMOutboundChannel;
use bridge_types::traits::OutboundChannel;
use bridge_types::traits::PeerManager;
use bridge_types::traits::{BalancePrecisionConverter, BridgeAssetLocker};
use bridge_types::EVMChainId;
use bridge_types::{H160, U256};
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::ensure;
use frame_support::traits::EnsureOrigin;
use frame_system::ensure_signed;
use frame_system::RawOrigin;
use sp_core::ecdsa;
use sp_core::Get;
use sp_runtime::traits::Zero;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

pub use pallet::*;
pub use weights::WeightInfo;

#[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen)]
pub struct BaseFeeInfo<BlockNumber> {
    pub base_fee: U256,
    pub updated: BlockNumber,
    pub evm_block_number: u64,
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, codec::MaxEncodedLen)]
pub struct NetworkConfig<AssetId> {
    pub fee_asset: AssetId,
    pub fee_asset_precision: u8,
    pub channel_gas_overhead: U256,
    pub priority_fee: U256,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use bridge_types::traits::{
        AppRegistry, BalancePrecisionConverter, BridgeAssetRegistry, MessageStatusNotifier,
        OutboundChannel,
    };
    use bridge_types::traits::{BridgeAssetLocker, EVMOutboundChannel};
    use bridge_types::types::{CallOriginOutput, GenericAdditionalInboundData};
    use bridge_types::MainnetAssetId;
    use bridge_types::{EVMChainId, GenericNetworkId, H256};
    use frame_support::pallet_prelude::*;
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

        type OutboundChannel: OutboundChannel<EVMChainId, Self::AccountId, AdditionalEVMOutboundData>
            + EVMOutboundChannel;

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
        type MaxGasPerMessage: Get<u128>;

        #[pallet::constant]
        type MaxGasPerCommit: Get<u128>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Transfer to sidechain.
        Burned {
            network_id: EVMChainId,
            asset_id: AssetIdOf<T>,
            sender: T::AccountId,
            recipient: H160,
            amount: BalanceOf<T>,
        },
        /// Transfer from sidechain.
        Minted {
            network_id: EVMChainId,
            asset_id: AssetIdOf<T>,
            sender: H160,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Transfer failed, tokens refunded.
        Refunded {
            network_id: EVMChainId,
            recipient: T::AccountId,
            asset_id: AssetIdOf<T>,
            amount: BalanceOf<T>,
        },
        /// New asset registered.
        AssetRegistered {
            network_id: EVMChainId,
            asset_id: AssetIdOf<T>,
        },
        /// Fees paid by relayer in EVM was claimed.
        FeesClaimed {
            recipient: T::AccountId,
            asset_id: AssetIdOf<T>,
            amount: BalanceOf<T>,
        },
    }

    /// Collected fees
    #[pallet::storage]
    #[pallet::getter(fn collected_fees)]
    pub(super) type CollectedFees<T: Config> =
        StorageMap<_, Identity, EVMChainId, U256, ValueQuery>;

    /// Base fees
    #[pallet::storage]
    #[pallet::getter(fn base_fees)]
    pub(super) type BaseFees<T: Config> =
        StorageMap<_, Identity, EVMChainId, BaseFeeInfo<BlockNumberFor<T>>, OptionQuery>;

    /// Fees spend by relayer
    #[pallet::storage]
    #[pallet::getter(fn spent_fees)]
    pub(super) type SpentFees<T: Config> =
        StorageDoubleMap<_, Identity, EVMChainId, Identity, H160, U256, ValueQuery>;

    /// Relayer owner accounts to claim fees
    #[pallet::storage]
    #[pallet::getter(fn relayer_owner)]
    pub(super) type RelayerOwners<T: Config> =
        StorageDoubleMap<_, Identity, EVMChainId, Identity, H160, T::AccountId, OptionQuery>;

    /// Native asset precision
    #[pallet::storage]
    #[pallet::getter(fn network_config)]
    pub(super) type NetworkConfigs<T: Config> =
        StorageMap<_, Identity, EVMChainId, NetworkConfig<AssetIdOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub type ChannelAddresses<T: Config> = StorageMap<_, Identity, EVMChainId, H160, OptionQuery>;

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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::register_native_app())]
        pub fn claim_relayer_fees(
            origin: OriginFor<T>,
            network_id: EVMChainId,
            relayer: H160,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let config =
                NetworkConfigs::<T>::get(network_id).ok_or(Error::<T>::NetworkNotSupported)?;
            let owner =
                RelayerOwners::<T>::get(network_id, relayer).ok_or(Error::<T>::OwnerIsNotAdded)?;
            ensure!(who == owner, Error::<T>::AccountIsNotOwner);
            let spent_fees = SpentFees::<T>::get(network_id, relayer);
            ensure!(spent_fees > U256::zero(), Error::<T>::NothingToClaim);

            let collected_fees = CollectedFees::<T>::get(network_id);
            ensure!(
                collected_fees > U256::zero(),
                Error::<T>::NotEnoughFeesCollected
            );
            let fees_to_claim = collected_fees.min(spent_fees);

            let (amount, _) = T::BalancePrecisionConverter::from_sidechain(
                &config.fee_asset,
                config.fee_asset_precision,
                fees_to_claim,
            )
            .ok_or(Error::<T>::WrongAmount)?;
            ensure!(amount > Zero::zero(), Error::<T>::WrongAmount);
            T::BridgeAssetLocker::refund_fee(network_id.into(), &who, &config.fee_asset, &amount)?;
            Self::deposit_event(Event::FeesClaimed {
                asset_id: config.fee_asset,
                recipient: who,
                amount,
            });

            SpentFees::<T>::insert(
                network_id,
                relayer,
                spent_fees.saturating_sub(fees_to_claim),
            );
            CollectedFees::<T>::set(network_id, collected_fees.saturating_sub(fees_to_claim));

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn register_channel(
            origin: OriginFor<T>,
            network_id: EVMChainId,
            channel_address: H160,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            ChannelAddresses::<T>::insert(network_id, channel_address);
            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn register_relayer(
            origin: OriginFor<T>,
            network_id: EVMChainId,
            relayer: H160,
            owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            RelayerOwners::<T>::insert(network_id, relayer, owner);
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    fn ensure_channel(chain_id: EVMChainId, channel: H160) -> DispatchResult {
        let channel_address =
            ChannelAddresses::<T>::get(chain_id).ok_or(Error::<T>::InvalidNetwork)?;
        ensure!(channel_address == channel, Error::<T>::InvalidSourceChannel);
        Ok(())
    }

    fn get_latest_base_fee(network_id: EVMChainId) -> Result<U256, DispatchError> {
        let base_fee = BaseFees::<T>::get(network_id).ok_or(Error::<T>::BaseFeeIsNotAvailable)?;
        ensure!(
            frame_system::Pallet::<T>::block_number()
                <= base_fee.updated + T::BaseFeeLifetime::get(),
            Error::<T>::BaseFeeLifetimeExceeded
        );
        Ok(base_fee.base_fee)
    }
}

impl<T: Config> bridge_types::traits::EVMBridgeWithdrawFee<T::AccountId, AssetIdOf<T>>
    for Pallet<T>
{
    fn withdraw_transfer_fee(
        who: &T::AccountId,
        chain_id: bridge_types::EVMChainId,
        _asset_id: AssetIdOf<T>,
    ) -> DispatchResult {
        let config = NetworkConfigs::<T>::get(chain_id).ok_or(Error::<T>::NetworkNotSupported)?;
        let gas = T::OutboundChannel::submit_gas(chain_id)?.saturating_add(TRANSFER_MAX_GAS.into());
        let base_fee = Self::get_latest_base_fee(chain_id)?.saturating_add(config.priority_fee);
        let fee = gas.saturating_mul(base_fee);

        let (amount, _) = T::BalancePrecisionConverter::from_sidechain(
            &config.fee_asset,
            config.fee_asset_precision,
            fee,
        )
        .ok_or(Error::<T>::WrongAmount)?;
        ensure!(amount > Zero::zero(), Error::<T>::WrongAmount);
        T::BridgeAssetLocker::withdraw_fee(chain_id.into(), who, &config.fee_asset, &amount)?;
        CollectedFees::<T>::mutate(chain_id, |fees| {
            *fees = fees.saturating_add(fee);
        });
        Ok(())
    }
}

impl<T: Config> AppRegistry<EVMChainId, H160> for Pallet<T> {
    fn register_app(network_id: EVMChainId, app: H160) -> DispatchResult {
        let target = ChannelAddresses::<T>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?;

        // let message = abi::RegisterAppPayload { app };

        // T::OutboundChannel::submit(
        //     network_id,
        //     &RawOrigin::Root,
        //     message
        //         .encode()
        //         .map_err(|_| Error::<T>::CallEncodeFailed)?
        //         .as_ref(),
        //     AdditionalEVMOutboundData {
        //         target,
        //         max_gas: 100000u64.into(),
        //     },
        // )?;
        Ok(())
    }

    fn deregister_app(network_id: EVMChainId, app: H160) -> DispatchResult {
        let target = ChannelAddresses::<T>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?;

        // let message = abi::RemoveAppPayload { app };

        // T::OutboundChannel::submit(
        //     network_id,
        //     &RawOrigin::Root,
        //     message
        //         .encode()
        //         .map_err(|_| Error::<T>::CallEncodeFailed)?
        //         .as_ref(),
        //     AdditionalEVMOutboundData {
        //         target,
        //         max_gas: 100000u64.into(),
        //     },
        // )?;
        Ok(())
    }
}

impl<T: Config, MaxMessages: Get<u32>, MaxPayload: Get<u32>>
    CommitmentHandler<EVMChainId, Commitment<MaxMessages, MaxPayload>> for Pallet<T>
{
    fn verify_commitment(
        chain_id: &EVMChainId,
        commitment: &Commitment<MaxMessages, MaxPayload>,
    ) -> DispatchResult {
        match commitment {
            bridge_types::evm::Commitment::Inbound(inbound_commitment) => {
                Self::ensure_channel(*chain_id, inbound_commitment.channel)?;
            }
            bridge_types::evm::Commitment::StatusReport(status_report) => {
                Self::ensure_channel(*chain_id, status_report.channel)?;
            }
            bridge_types::evm::Commitment::BaseFeeUpdate(update) => {
                if let Some(base_fee) = BaseFees::<T>::get(chain_id) {
                    // Probably it's first base fee update
                    ensure!(
                        base_fee.evm_block_number < update.evm_block_number,
                        Error::<T>::StaleBaseFeeUpdate
                    );
                }
            }
            bridge_types::evm::Commitment::Outbound(_) => {
                frame_support::fail!(Error::<T>::InvalidCommitment);
            }
        }
        Ok(())
    }

    fn handle_commitment(
        chain_id: &EVMChainId,
        commitment: &Commitment<MaxMessages, MaxPayload>,
    ) -> DispatchResult {
        match commitment {
            Commitment::BaseFeeUpdate(update) => {
                let block_number = frame_system::Pallet::<T>::block_number();
                BaseFees::<T>::insert(
                    chain_id,
                    BaseFeeInfo {
                        base_fee: update.new_base_fee,
                        updated: block_number,
                        evm_block_number: update.evm_block_number,
                    },
                );
            }
            Commitment::StatusReport(status_report) => {
                let config =
                    NetworkConfigs::<T>::get(chain_id).ok_or(Error::<T>::NetworkNotSupported)?;
                let gas_used = status_report
                    .gas_spent
                    .saturating_add(config.channel_gas_overhead);
                // Priority fee and some additional reward
                let gas_price = status_report.base_fee.saturating_add(config.priority_fee);
                let fee_paid = gas_used.saturating_mul(gas_price);
                SpentFees::<T>::mutate(chain_id, status_report.relayer, |fees| {
                    *fees = fees.saturating_add(fee_paid);
                })
            }
            Commitment::Inbound(_) => {}
            Commitment::Outbound(_) => {
                frame_support::fail!(Error::<T>::InvalidCommitment);
            }
        }
        Ok(())
    }
}

impl<T: Config> PeerManager<EVMChainId, ecdsa::Public> for Pallet<T> {
    fn add_peer(network_id: EVMChainId, peer: ecdsa::Public) -> DispatchResult {
        // T::OutboundChannel::submit(
        //     network_id,
        //     &RawOrigin::Root,
        //     &abi::AddPeerPayload {
        //         peer: bridge_types::utils::public_key_to_address(peer)
        //             .ok_or(Error::<T>::CallEncodeFailed)?,
        //     }
        //     .encode()
        //     .map_err(|_| Error::<T>::CallEncodeFailed)?,
        //     AdditionalEVMOutboundData {
        //         max_gas: ADD_PEER_MAX_GAS.into(),
        //         target: ChannelAddresses::<T>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?,
        //     },
        // )?;
        Ok(())
    }

    fn remove_peer(network_id: EVMChainId, peer: ecdsa::Public) -> DispatchResult {
        // T::OutboundChannel::submit(
        //     network_id,
        //     &RawOrigin::Root,
        //     &abi::RemovePeerPayload {
        //         peer: bridge_types::utils::public_key_to_address(peer)
        //             .ok_or(Error::<T>::CallEncodeFailed)?,
        //     }
        //     .encode()
        //     .map_err(|_| Error::<T>::CallEncodeFailed)?,
        //     AdditionalEVMOutboundData {
        //         max_gas: REMOVE_PEER_MAX_GAS.into(),
        //         target: ChannelAddresses::<T>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?,
        //     },
        // )?;
        Ok(())
    }

    fn submit_weight() -> frame_support::weights::Weight {
        <T::OutboundChannel as OutboundChannel<_, _, _>>::submit_weight()
    }
}
