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

//! Channel for passing messages from ethereum to substrate.

use bridge_types::traits::{MessageDispatch, Verifier};
use bridge_types::types::MessageId;
use bridge_types::SubNetworkId;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Get;
use frame_system::ensure_signed;
use sp_core::U256;

use sp_runtime::traits::{Convert, Zero};
use sp_runtime::Perbill;
use traits::MultiCurrency;

mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod test;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_types::types::ParachainMessage;
    use frame_support::log::{debug, warn};
    use frame_support::traits::StorageVersion;
    use frame_support::{pallet_prelude::*, Parameter};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::CheckedSub;
    use sp_std::prelude::*;

    pub type AssetIdOf<T> = <<T as Config>::Currency as MultiCurrency<
        <T as frame_system::Config>::AccountId,
    >>::CurrencyId;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Verifier module for message verification.
        type Verifier: Verifier<
            SubNetworkId,
            Self::ProvedMessage,
            Result = Vec<ParachainMessage<BalanceOf<Self>>>,
        >;

        /// Message with proof
        type ProvedMessage: Parameter;

        /// Verifier module for message verification.
        type MessageDispatch: MessageDispatch<Self, SubNetworkId, MessageId, ()>;

        type FeeConverter: Convert<U256, BalanceOf<Self>>;

        /// The base asset as the core asset in all trading pairs
        type FeeAssetId: Get<AssetIdOf<Self>>;

        type Currency: MultiCurrency<Self::AccountId>;

        type FeeAccountId: Get<Self::AccountId>;

        type TreasuryAccountId: Get<Self::AccountId>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    pub type ChannelNonces<T: Config> = StorageMap<_, Identity, SubNetworkId, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn reward_fraction)]
    pub(super) type RewardFraction<T: Config> =
        StorageValue<_, Perbill, ValueQuery, DefaultRewardFraction>;

    #[pallet::type_value]
    pub(super) fn DefaultRewardFraction() -> Perbill {
        Perbill::from_percent(80)
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    // #[pallet::generate_deposit(pub(super) fn deposit_event)]
    // This pallet don't have events
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {
        /// Message came from an invalid network.
        InvalidNetwork,
        /// Message came from an invalid outbound channel on the Ethereum side.
        InvalidSourceChannel,
        /// Message has an invalid envelope.
        InvalidEnvelope,
        /// Message has an unexpected nonce.
        InvalidNonce,
        /// Incorrect reward fraction
        InvalidRewardFraction,
        /// This contract already exists
        ContractExists,
        /// Call encoding failed.
        CallEncodeFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::submit())]
        pub fn submit(
            origin: OriginFor<T>,
            network_id: SubNetworkId,
            message: T::ProvedMessage,
        ) -> DispatchResultWithPostInfo {
            let relayer = ensure_signed(origin)?;
            debug!("Received message from {:?}", relayer);
            // submit message to verifier for verification
            let messages = T::Verifier::verify(network_id, &message)?;

            for message in messages {
                // Verify message nonce
                <ChannelNonces<T>>::try_mutate(network_id, |nonce| -> DispatchResult {
                    if message.nonce != *nonce + 1 {
                        Err(Error::<T>::InvalidNonce.into())
                    } else {
                        *nonce += 1;
                        Ok(())
                    }
                })?;

                Self::handle_fee(message.fee, &relayer);

                let message_id = MessageId::inbound(message.nonce);
                T::MessageDispatch::dispatch(
                    network_id,
                    message_id,
                    message.timestamp,
                    &message.payload,
                    (),
                );
            }
            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::set_reward_fraction())]
        pub fn set_reward_fraction(
            origin: OriginFor<T>,
            fraction: Perbill,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            RewardFraction::<T>::set(fraction);
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /*
         * Pay the message submission fee into the relayer and treasury account.
         *
         * - If the fee is zero, do nothing
         * - Otherwise, withdraw the fee amount from the DotApp module account, returning a negative imbalance
         * - Figure out the fraction of the fee amount that should be paid to the relayer
         * - Pay the relayer if their account exists, returning a positive imbalance.
         * - Adjust the negative imbalance by offsetting the amount paid to the relayer
         * - Resolve the negative imbalance by depositing it into the treasury account
         */
        pub fn handle_fee(amount: BalanceOf<T>, relayer: &T::AccountId) {
            if amount.is_zero() {
                return;
            }
            let reward_fraction: Perbill = RewardFraction::<T>::get();
            let reward_amount = reward_fraction.mul_ceil(amount);

            if let Err(err) = <T as Config>::Currency::transfer(
                T::FeeAssetId::get(),
                &T::FeeAccountId::get(),
                relayer,
                reward_amount,
            ) {
                warn!("Unable to transfer reward to relayer: {:?}", err);
                return;
            }

            if let Some(treasure_amount) = amount.checked_sub(&reward_amount) {
                if let Err(err) = <T as Config>::Currency::transfer(
                    T::FeeAssetId::get(),
                    &T::FeeAccountId::get(),
                    &T::TreasuryAccountId::get(),
                    treasure_amount,
                ) {
                    warn!("Unable to transfer to treasury: {:?}", err);
                }
            }
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub reward_fraction: Perbill,
    }

    #[allow(clippy::derivable_impls)]
    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                reward_fraction: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            RewardFraction::<T>::set(self.reward_fraction);
        }
    }
}
