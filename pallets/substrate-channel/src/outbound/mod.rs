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

//! Channel for passing messages from substrate to ethereum.

use bridge_types::substrate::BridgeMessage;
use codec::{Decode, Encode};
use frame_support::ensure;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_core::{RuntimeDebug, H256};
use sp_io::offchain_index;
use sp_runtime::traits::Hash;
use sp_std::prelude::*;

use bridge_types::types::MessageNonce;
use bridge_types::SubNetworkId;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod test;

/// Wire-format for commitment
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Commitment {
    /// Messages passed through the channel in the current commit.
    pub messages: Vec<BridgeMessage>,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_types::traits::AuxiliaryDigestHandler;
    use bridge_types::traits::MessageStatusNotifier;
    use bridge_types::traits::OutboundChannel;
    use bridge_types::traits::TimepointProvider;
    use bridge_types::types::AuxiliaryDigestItem;
    use bridge_types::types::MessageId;
    use bridge_types::types::MessageStatus;
    use bridge_types::GenericNetworkId;
    use bridge_types::GenericTimepoint;
    use frame_support::log::debug;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::StorageVersion;
    use frame_support::Parameter;
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    use sp_runtime::traits::Zero;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Prefix for offchain storage keys.
        const INDEXING_PREFIX: &'static [u8];

        type Hashing: Hash<Output = H256>;

        /// Max bytes in a message payload
        type MaxMessagePayloadSize: Get<u64>;

        /// Max number of messages that can be queued and committed in one go for a given channel.
        type MaxMessagesPerCommit: Get<u64>;

        type AssetId: Parameter;

        type Balance: Parameter;

        type MessageStatusNotifier: MessageStatusNotifier<
            Self::AssetId,
            Self::AccountId,
            Self::Balance,
        >;

        type AuxiliaryDigestHandler: AuxiliaryDigestHandler;

        type TimepointProvider: TimepointProvider;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    /// Interval between committing messages.
    #[pallet::storage]
    #[pallet::getter(fn interval)]
    pub(crate) type Interval<T: Config> =
        StorageValue<_, T::BlockNumber, ValueQuery, DefaultInterval<T>>;

    #[pallet::type_value]
    pub(crate) fn DefaultInterval<T: Config>() -> T::BlockNumber {
        // TODO: Select interval
        10u32.into()
    }

    /// Messages waiting to be committed. To update the queue, use `append_message_queue` and `take_message_queue` methods
    /// (to keep correct value in [QueuesTotalGas]).
    #[pallet::storage]
    pub(crate) type MessageQueues<T: Config> =
        StorageMap<_, Identity, SubNetworkId, Vec<BridgeMessage>, ValueQuery>;

    #[pallet::storage]
    pub type ChannelNonces<T: Config> = StorageMap<_, Identity, SubNetworkId, u64, ValueQuery>;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // Generate a message commitment every [`Interval`] blocks.
        //
        // The commitment hash is included in an [`AuxiliaryDigestItem`] in the block header,
        // with the corresponding commitment is persisted offchain.
        fn on_initialize(now: T::BlockNumber) -> Weight {
            let interval = Self::interval();
            let mut weight = Default::default();
            if now % interval == Zero::zero() {
                for chain_id in MessageQueues::<T>::iter_keys() {
                    weight += Self::commit(chain_id);
                }
            }
            weight
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageAccepted(SubNetworkId, MessageNonce),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The message payload exceeds byte limit.
        PayloadTooLarge,
        /// No more messages can be queued for the channel during this commit cycle.
        QueueSizeLimitReached,
        /// Maximum gas for queued batch exceeds limit.
        MaxGasTooBig,
        /// Cannot pay the fee to submit a message.
        NoFunds,
        /// Cannot increment nonce
        Overflow,
        /// This channel already exists
        ChannelExists,
    }

    impl<T: Config> Pallet<T> {
        pub fn make_message_id(nonce: u64) -> H256 {
            MessageId::outbound(nonce).using_encoded(|v| <T as Config>::Hashing::hash(v))
        }

        fn commit(network_id: SubNetworkId) -> Weight {
            debug!("Commit substrate messages");
            let messages = Self::take_message_queue(network_id);
            if messages.is_empty() {
                return <T as Config>::WeightInfo::on_initialize_no_messages();
            }

            for message in messages.iter() {
                T::MessageStatusNotifier::update_status(
                    GenericNetworkId::Sub(network_id),
                    Self::make_message_id(message.nonce),
                    MessageStatus::Committed,
                    GenericTimepoint::Pending,
                );
            }

            let commitment = Commitment { messages };

            let average_payload_size = Self::average_payload_size(&commitment.messages);
            let messages_count = commitment.messages.len();
            let encoded_commitment = commitment.encode();
            let commitment_hash = <T as Config>::Hashing::hash(&encoded_commitment);
            let digest_item =
                AuxiliaryDigestItem::Commitment(GenericNetworkId::Sub(network_id), commitment_hash);
            T::AuxiliaryDigestHandler::add_item(digest_item);

            let key = Self::make_offchain_key(commitment_hash);
            offchain_index::set(&key, &encoded_commitment);

            <T as Config>::WeightInfo::on_initialize(
                messages_count as u32,
                average_payload_size as u32,
            )
        }

        fn average_payload_size(messages: &[BridgeMessage]) -> usize {
            let sum: usize = messages.iter().fold(0, |acc, x| acc + x.payload.len());
            // We overestimate message payload size rather than underestimate.
            // So add 1 here to account for integer division truncation.
            (sum / messages.len()).saturating_add(1)
        }

        pub fn make_offchain_key(hash: H256) -> Vec<u8> {
            (T::INDEXING_PREFIX, hash).encode()
        }

        /// Add message to queue and accumulate total maximum gas value    
        pub(crate) fn append_message_queue(network: SubNetworkId, msg: BridgeMessage) {
            MessageQueues::<T>::append(network, msg);
        }

        /// Take the queue together with accumulated total maximum gas value.
        pub(crate) fn take_message_queue(network: SubNetworkId) -> Vec<BridgeMessage> {
            MessageQueues::<T>::take(network)
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub interval: T::BlockNumber,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                interval: 10u32.into(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Interval::<T>::set(self.interval);
        }
    }

    impl<T: Config> OutboundChannel<SubNetworkId, T::AccountId, ()> for Pallet<T> {
        /// Submit message on the outbound channel
        fn submit(
            network_id: SubNetworkId,
            who: &RawOrigin<T::AccountId>,
            payload: &[u8],
            _: (),
        ) -> Result<H256, DispatchError> {
            debug!("Send message from {:?} to network {:?}", who, network_id);
            ensure!(
                MessageQueues::<T>::decode_len(network_id).unwrap_or(0)
                    < T::MaxMessagesPerCommit::get() as usize,
                Error::<T>::QueueSizeLimitReached,
            );
            ensure!(
                payload.len() <= T::MaxMessagePayloadSize::get() as usize,
                Error::<T>::PayloadTooLarge,
            );

            <ChannelNonces<T>>::try_mutate(network_id, |nonce| -> Result<H256, DispatchError> {
                if let Some(v) = nonce.checked_add(1) {
                    *nonce = v;
                } else {
                    return Err(Error::<T>::Overflow.into());
                }

                Self::append_message_queue(
                    network_id,
                    BridgeMessage {
                        nonce: *nonce,
                        payload: payload.to_vec(),
                        timepoint: T::TimepointProvider::get_timepoint(),
                    },
                );
                Self::deposit_event(Event::MessageAccepted(network_id, *nonce));
                Ok(Self::make_message_id(*nonce))
            })
        }
    }
}
