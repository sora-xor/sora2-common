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
use bridge_types::traits::EVMOutboundChannel;
use bridge_types::traits::OutboundChannel;
use bridge_types::traits::TimepointProvider;
use bridge_types::EVMChainId;
use bridge_types::GenericNetworkId;
use bridge_types::SubNetworkId;
use frame_support::ensure;
use frame_support::pallet_prelude::*;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_system::pallet_prelude::*;
use frame_system::RawOrigin;
use log::error;
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
    use bridge_types::traits::AuxiliaryDigestHandler;
    use bridge_types::traits::MessageStatusNotifier;
    use bridge_types::traits::TimepointProvider;
    use bridge_types::types::AuxiliaryDigestItem;
    use bridge_types::types::GenericCommitmentWithBlock;
    use bridge_types::types::MessageId;
    use bridge_types::types::MessageStatus;
    use bridge_types::GenericBridgeMessage;
    use bridge_types::GenericCommitment;
    use bridge_types::GenericNetworkId;
    use bridge_types::GenericTimepoint;
    use frame_support::traits::StorageVersion;
    use log::debug;
    use sp_core::U256;
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

        type MaxGasPerCommit: Get<U256>;

        type MaxGasPerMessage: Get<U256>;

        type MessageStatusNotifier: MessageStatusNotifier<
            Self::AssetId,
            Self::AccountId,
            Self::Balance,
        >;

        type AuxiliaryDigestHandler: AuxiliaryDigestHandler;

        type TimepointProvider: TimepointProvider;

        #[pallet::constant]
        type ThisNetworkId: Get<GenericNetworkId>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    /// Interval between committing messages.
    #[pallet::storage]
    #[pallet::getter(fn interval)]
    pub(crate) type Interval<T: Config> =
        StorageValue<_, BlockNumberFor<T>, ValueQuery, DefaultInterval<T>>;

    #[pallet::type_value]
    pub(crate) fn DefaultInterval<T: Config>() -> BlockNumberFor<T> {
        // TODO: Select interval
        10u32.into()
    }

    /// Messages waiting to be committed. To update the queue, use `append_message_queue` and `take_message_queue` methods
    /// (to keep correct value in [QueuesTotalGas]).
    #[pallet::storage]
    pub(crate) type MessageQueues<T: Config> = StorageMap<
        _,
        Identity,
        GenericNetworkId,
        BoundedVec<GenericBridgeMessage<T::MaxMessagePayloadSize>, T::MaxMessagesPerCommit>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub type QueueTotalGas<T: Config> = StorageMap<_, Identity, GenericNetworkId, U256, ValueQuery>;

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

    #[pallet::storage]
    pub type EVMSubmitGas<T: Config> =
        StorageMap<_, Identity, EVMChainId, U256, ValueQuery, DefaultEVMSubmitGas>;

    #[pallet::type_value]
    pub fn DefaultEVMSubmitGas() -> U256 {
        200_000u32.into()
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // Generate a message commitment every [`Interval`] blocks.
        //
        // The commitment hash is included in an [`AuxiliaryDigestItem`] in the block header,
        // with the corresponding commitment is persisted offchain.
        fn on_initialize(now: BlockNumberFor<T>) -> Weight {
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
            let messages = MessageQueues::<T>::take(network_id);
            if messages.is_empty() {
                return <T as Config>::WeightInfo::on_initialize_no_messages();
            }

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

            let average_payload_size = Self::average_payload_size(&messages);
            let messages_count = messages.len();

            let commitment = match network_id {
                GenericNetworkId::EVM(_) => {
                    let (messages, total_gas) = messages.iter().fold(
                        (BoundedVec::default(), U256::zero()),
                        |(mut messages, total_gas), message| match message {
                            GenericBridgeMessage::EVM(message) => {
                                let total_gas = total_gas.saturating_add(message.max_gas);
                                if messages.try_push(message.clone()).is_err() {
                                    error!("Messages limit exceeded, ignoring (if you noticed this message, please report it)");
                                }
                                (messages, total_gas)
                            }
                            _ => {
                                error!("Message is not an EVM message, ignoring (if you noticed this message, please report it)");
                                (messages, total_gas)
                            },
                        },
                    );
                    GenericCommitment::EVM(bridge_types::evm::Commitment::Outbound(
                        bridge_types::evm::OutboundCommitment {
                            messages,
                            total_max_gas: total_gas,
                            nonce: batch_nonce,
                        },
                    ))
                }
                GenericNetworkId::Sub(_) => {
                    let messages = messages.iter().fold(
                        BoundedVec::default(),
                        |mut messages, message| match message {
                            GenericBridgeMessage::Sub(message) => {
                                if messages.try_push(message.clone()).is_err() {
                                    error!("Messages limit exceeded, ignoring (if you noticed this message, please report it)");
                                }
                                messages
                            }
                            _ => {
                                error!("Message is not an Substrate bridge message, ignoring (if you noticed this message, please report it)");
                                messages
                            },
                        },
                    );
                    bridge_types::GenericCommitment::Sub(bridge_types::substrate::Commitment {
                        messages,
                        nonce: batch_nonce,
                    })
                }
                GenericNetworkId::EVMLegacy(_) => {
                    error!("EVMLegacy messages are not supported by this channel (if you noticed this message, please report it)");
                    return <T as Config>::WeightInfo::on_initialize_no_messages();
                }
                GenericNetworkId::TON(_) => {
                    error!("TON messages are not supported yet by this channel (if you noticed this message, please report it)");
                    return <T as Config>::WeightInfo::on_initialize_no_messages();
                }
            };

            let commitment_hash = commitment.hash();
            let digest_item = AuxiliaryDigestItem::Commitment(network_id, commitment_hash);
            T::AuxiliaryDigestHandler::add_item(digest_item);

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

        fn average_payload_size(
            messages: &[GenericBridgeMessage<T::MaxMessagePayloadSize>],
        ) -> usize {
            let sum: usize = messages.iter().fold(0, |acc, x| acc + x.payload().len());
            // We overestimate message payload size rather than underestimate.
            // So add 1 here to account for integer division truncation.
            (sum / messages.len()).saturating_add(1)
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub interval: BlockNumberFor<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                interval: 10u32.into(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            Interval::<T>::set(self.interval);
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn submit_message(
            network_id: GenericNetworkId,
            who: &RawOrigin<T::AccountId>,
            message: GenericBridgeMessage<T::MaxMessagePayloadSize>,
        ) -> Result<H256, DispatchError> {
            debug!("Send message from {:?} to network {:?}", who, network_id);
            let messages_count = MessageQueues::<T>::decode_len(network_id).unwrap_or(0) as u64;
            ensure!(
                messages_count < T::MaxMessagesPerCommit::get() as u64,
                Error::<T>::QueueSizeLimitReached,
            );
            let batch_nonce = ChannelNonces::<T>::get(network_id)
                .checked_add(1)
                .ok_or(Error::<T>::Overflow)?;

            MessageQueues::<T>::try_append(network_id, message)
                .map_err(|_| Error::<T>::QueueSizeLimitReached)?;
            Self::deposit_event(Event::MessageAccepted {
                network_id,
                batch_nonce,
                message_nonce: messages_count,
            });
            Ok(MessageId::batched(
                T::ThisNetworkId::get(),
                network_id,
                batch_nonce,
                messages_count,
            )
            .hash())
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
        Self::submit_message(
            network_id.into(),
            who,
            bridge_types::GenericBridgeMessage::Sub(message),
        )
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
        ensure!(
            additional_data.max_gas < T::MaxGasPerMessage::get(),
            Error::<T>::MessageGasLimitExceeded
        );
        QueueTotalGas::<T>::try_mutate(GenericNetworkId::EVM(network_id), |total_gas| {
            *total_gas = total_gas.saturating_add(additional_data.max_gas);
            ensure!(
                *total_gas < T::MaxGasPerCommit::get(),
                Error::<T>::CommitmentGasLimitExceeded
            );
            Ok::<(), DispatchError>(())
        })?;
        let message = bridge_types::evm::Message {
            payload: payload
                .to_vec()
                .try_into()
                .map_err(|_| Error::<T>::PayloadTooLarge)?,
            target: additional_data.target,
            max_gas: additional_data.max_gas,
        };
        Self::submit_message(
            network_id.into(),
            who,
            bridge_types::GenericBridgeMessage::EVM(message),
        )
    }

    fn submit_weight() -> Weight {
        <T as Config>::WeightInfo::submit()
    }
}

impl<T: Config> EVMOutboundChannel for Pallet<T> {
    fn submit_gas(network_id: EVMChainId) -> Result<sp_core::U256, DispatchError> {
        Ok(EVMSubmitGas::<T>::get(network_id))
    }
}
