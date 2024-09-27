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

use bridge_types::evm::AdditionalEVMOutboundData;
use bridge_types::substrate::BridgeMessage;
use bridge_types::ton::AdditionalTONOutboundData;
use bridge_types::ton::TonNetworkId;
use bridge_types::traits::OutboundChannel;
use bridge_types::traits::OutboundQueueVerifier;
use bridge_types::traits::TimepointProvider;
use bridge_types::EVMChainId;
use bridge_types::GenericNetworkId;
use bridge_types::SubNetworkId;
use frame_support::log::error;
use frame_support::pallet_prelude::*;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_system::pallet_prelude::*;
use frame_system::RawOrigin;
use sp_core::H256;
use sp_runtime::DispatchError;

use bridge_types::types::MessageNonce;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod test;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_types::traits::MessageStatusNotifier;
    use bridge_types::traits::OutboundQueueVerifier;
    use bridge_types::traits::TimepointProvider;
    use bridge_types::types::GenericCommitmentWithBlock;
    use bridge_types::types::MessageId;
    use bridge_types::types::MessageStatus;
    use bridge_types::GenericCommitment;
    use bridge_types::GenericMessageQueue;
    use bridge_types::GenericNetworkId;
    use bridge_types::GenericTimepoint;
    use frame_support::log::debug;
    use frame_support::traits::StorageVersion;
    use sp_runtime::traits::Zero;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Max bytes in a message payload
        type MaxMessagePayloadSize: Get<u32>;

        /// Max number of messages that can be queued and committed in one go for a given channel.
        type MaxMessagesPerCommit: Get<u32>;

        type AssetId;

        type Balance;

        type MessageStatusNotifier: MessageStatusNotifier<
            Self::AssetId,
            Self::AccountId,
            Self::Balance,
        >;

        type TimepointProvider: TimepointProvider;

        type EVMOutboundQueueVerifier: OutboundQueueVerifier<
            EVMChainId,
            bridge_types::evm::MessageQueue<
                Self::MaxMessagesPerCommit,
                Self::MaxMessagePayloadSize,
            >,
            bridge_types::evm::Message<Self::MaxMessagePayloadSize>,
        >;

        type TONOutboundQueueVerifier: OutboundQueueVerifier<
            TonNetworkId,
            bridge_types::ton::MessageQueue<
                Self::MaxMessagesPerCommit,
                Self::MaxMessagePayloadSize,
            >,
            bridge_types::ton::Message<Self::MaxMessagePayloadSize>,
        >;

        #[pallet::constant]
        type ThisNetworkId: Get<GenericNetworkId>;

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
        10u32.into()
    }

    /// Messages waiting to be committed. To update the queue, use `append_message_queue` and `take_message_queue` methods
    /// (to keep correct value in [QueuesTotalGas]).
    #[pallet::storage]
    pub(crate) type MessageQueues<T: Config> = StorageMap<
        _,
        Identity,
        GenericNetworkId,
        GenericMessageQueue<T::MaxMessagesPerCommit, T::MaxMessagePayloadSize>,
        OptionQuery,
    >;

    #[pallet::storage]
    pub type ChannelNonces<T: Config> = StorageMap<_, Identity, GenericNetworkId, u64, ValueQuery>;

    #[pallet::storage]
    pub type LatestCommitment<T: Config> = StorageMap<
        _,
        Identity,
        GenericNetworkId,
        GenericCommitmentWithBlock<
            BlockNumberFor<T>,
            T::MaxMessagesPerCommit,
            T::MaxMessagePayloadSize,
        >,
        OptionQuery,
    >;

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
        MessageAccepted {
            network_id: GenericNetworkId,
            batch_nonce: u64,
            message_nonce: MessageNonce,
        },
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
        /// Network does not support this kind of message (it's a developer's mistake)
        MessageTypeIsNotSupported,
        /// Message consume too much gas
        MessageGasLimitExceeded,
        /// Commitment consume too much gas
        CommitmentGasLimitExceeded,
    }

    impl<T: Config> Pallet<T> {
        pub(crate) fn commit(network_id: GenericNetworkId) -> Weight {
            debug!("Commit substrate messages");
            let Some(messages) = MessageQueues::<T>::take(network_id) else {
                return <T as Config>::WeightInfo::on_initialize_no_messages();
            };

            let batch_nonce = ChannelNonces::<T>::mutate(network_id, |nonce| {
                *nonce += 1;
                *nonce
            });

            for idx in 0..messages.len() as u64 {
                T::MessageStatusNotifier::update_status(
                    network_id,
                    MessageId::batched(T::ThisNetworkId::get(), network_id, batch_nonce, idx)
                        .hash(),
                    MessageStatus::Committed,
                    GenericTimepoint::Pending,
                );
            }

            let average_payload_size = messages.average_payload_size();
            let messages_count = messages.len();

            let commitment = match (network_id, messages) {
                (GenericNetworkId::EVM(_), GenericMessageQueue::EVM(messages)) => {
                    GenericCommitment::EVM(bridge_types::evm::Commitment::Outbound(
                        bridge_types::evm::OutboundCommitment {
                            messages: messages.queue,
                            total_max_gas: messages.total_gas,
                            nonce: batch_nonce,
                        },
                    ))
                }
                (GenericNetworkId::Sub(_), GenericMessageQueue::Sub(messages)) => {
                    bridge_types::GenericCommitment::Sub(bridge_types::substrate::Commitment {
                        messages: messages.queue,
                        nonce: batch_nonce,
                    })
                }
                (GenericNetworkId::TON(_), GenericMessageQueue::TON(messages)) => {
                    GenericCommitment::TON(bridge_types::ton::Commitment::Outbound(
                        bridge_types::ton::OutboundCommitment {
                            messages: messages.queue,
                            total_max_fee: messages.total_fee,
                            nonce: batch_nonce,
                        },
                    ))
                }
                (GenericNetworkId::EVMLegacy(_), _) => {
                    error!("EVMLegacy messages are not supported by this channel (if you noticed this message, please report it)");
                    return <T as Config>::WeightInfo::on_initialize_no_messages();
                }
                _ => {
                    error!("Network and message types mismatch (if you noticed this message, please report it)");
                    return <T as Config>::WeightInfo::on_initialize_no_messages();
                }
            };

            let commitment = bridge_types::types::GenericCommitmentWithBlock {
                commitment,
                block_number: <frame_system::Pallet<T>>::block_number(),
            };
            LatestCommitment::<T>::insert(network_id, commitment);

            <T as Config>::WeightInfo::on_initialize(
                messages_count as u32,
                average_payload_size as u32,
            )
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

    impl<T: Config> Pallet<T> {
        pub fn on_submit_message(
            network_id: GenericNetworkId,
            who: &RawOrigin<T::AccountId>,
            message_id: u64,
        ) -> Result<H256, DispatchError> {
            debug!("Send message from {:?} to network {:?}", who, network_id);
            let batch_nonce = ChannelNonces::<T>::get(network_id)
                .checked_add(1)
                .ok_or(Error::<T>::Overflow)?;

            Self::deposit_event(Event::MessageAccepted {
                network_id,
                batch_nonce,
                message_nonce: message_id,
            });
            Ok(
                MessageId::batched(T::ThisNetworkId::get(), network_id, batch_nonce, message_id)
                    .hash(),
            )
        }
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
        let message = BridgeMessage {
            payload: payload
                .to_vec()
                .try_into()
                .map_err(|_| Error::<T>::PayloadTooLarge)?,
            timepoint: T::TimepointProvider::get_timepoint(),
        };
        let message_id = MessageQueues::<T>::try_mutate(
            GenericNetworkId::Sub(network_id),
            |opt_queue| -> Result<usize, DispatchError> {
                let queue = opt_queue
                    .get_or_insert(bridge_types::GenericMessageQueue::Sub(Default::default()))
                    .as_sub_mut()
                    .ok_or(Error::<T>::MessageTypeIsNotSupported)?;
                let message_id = queue.len();
                queue
                    .try_push(message)
                    .map_err(|_| Error::<T>::QueueSizeLimitReached)?;
                Ok(message_id)
            },
        )?;
        Self::on_submit_message(network_id.into(), who, message_id as u64)
    }

    fn submit_weight() -> Weight {
        <T as Config>::WeightInfo::submit()
    }
}

impl<T: Config> OutboundChannel<EVMChainId, T::AccountId, AdditionalEVMOutboundData> for Pallet<T> {
    /// Submit message on the outbound channel
    fn submit(
        network_id: EVMChainId,
        who: &RawOrigin<T::AccountId>,
        payload: &[u8],
        additional_data: AdditionalEVMOutboundData,
    ) -> Result<H256, DispatchError> {
        let message = bridge_types::evm::Message {
            payload: payload
                .to_vec()
                .try_into()
                .map_err(|_| Error::<T>::PayloadTooLarge)?,
            target: additional_data.target,
            max_gas: additional_data.max_gas,
        };
        let message_id = MessageQueues::<T>::try_mutate(
            GenericNetworkId::EVM(network_id),
            |opt_queue| -> Result<usize, DispatchError> {
                let queue = opt_queue
                    .get_or_insert(bridge_types::GenericMessageQueue::EVM(Default::default()))
                    .as_evm_mut()
                    .ok_or(Error::<T>::MessageTypeIsNotSupported)?;
                T::EVMOutboundQueueVerifier::verify_queue(&network_id, queue, &message)?;
                let message_id = queue.len();
                queue
                    .try_push(message)
                    .map_err(|_| Error::<T>::QueueSizeLimitReached)?;
                Ok(message_id)
            },
        )?;
        Self::on_submit_message(network_id.into(), who, message_id as u64)
    }

    fn submit_weight() -> Weight {
        <T as Config>::WeightInfo::submit()
    }
}

impl<T: Config> OutboundChannel<TonNetworkId, T::AccountId, AdditionalTONOutboundData>
    for Pallet<T>
{
    /// Submit message on the outbound channel
    fn submit(
        network_id: TonNetworkId,
        who: &RawOrigin<T::AccountId>,
        payload: &[u8],
        additional_data: AdditionalTONOutboundData,
    ) -> Result<H256, DispatchError> {
        let message = bridge_types::ton::Message {
            payload: payload
                .to_vec()
                .try_into()
                .map_err(|_| Error::<T>::PayloadTooLarge)?,
            target: additional_data.target,
            max_fee: additional_data.max_fee,
        };
        let message_id = MessageQueues::<T>::try_mutate(
            GenericNetworkId::TON(network_id),
            |opt_queue| -> Result<usize, DispatchError> {
                let queue = opt_queue
                    .get_or_insert(bridge_types::GenericMessageQueue::TON(Default::default()))
                    .as_ton_mut()
                    .ok_or(Error::<T>::MessageTypeIsNotSupported)?;
                T::TONOutboundQueueVerifier::verify_queue(&network_id, queue, &message)?;
                let message_id = queue.len();
                queue
                    .try_push(message)
                    .map_err(|_| Error::<T>::QueueSizeLimitReached)?;
                Ok(message_id)
            },
        )?;
        Self::on_submit_message(network_id.into(), who, message_id as u64)
    }

    fn submit_weight() -> Weight {
        <T as Config>::WeightInfo::submit()
    }
}
