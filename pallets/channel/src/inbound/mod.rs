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

use bridge_types::evm::AdditionalEVMOutboundData;
use bridge_types::traits::{MessageDispatch, MessageStatusNotifier, OutboundChannel, Verifier};
use bridge_types::types::MessageId;
use bridge_types::SubNetworkId;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Get;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

pub const EVM_GAS_OVERHEAD: u64 = 20000;

#[cfg(test)]
mod test;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_types::evm::AdditionalEVMInboundData;
    use bridge_types::ton::{AdditionalTONInboundData, TonAddress, TonNetworkId};
    use bridge_types::traits::CommitmentHandler;
    use bridge_types::types::{GenericAdditionalInboundData, MessageStatus};
    use bridge_types::{EVMChainId, GenericNetworkId, GenericTimepoint};
    use frame_support::log::warn;
    use frame_support::pallet_prelude::{InvalidTransaction, *};
    use frame_support::traits::StorageVersion;
    use frame_support::weights::Weight;
    use frame_system::pallet_prelude::*;

    pub type CommitmentOf<T> = bridge_types::GenericCommitment<
        <T as Config>::MaxMessagesPerCommit,
        <T as Config>::MaxMessagePayloadSize,
    >;

    pub type EVMCommitmentOf<T> = bridge_types::evm::Commitment<
        <T as Config>::MaxMessagesPerCommit,
        <T as Config>::MaxMessagePayloadSize,
    >;

    pub type TONCommitmentOf<T> = bridge_types::ton::Commitment<
        <T as Config>::MaxMessagesPerCommit,
        <T as Config>::MaxMessagePayloadSize,
    >;

    pub type SubCommitmentOf<T> = bridge_types::substrate::Commitment<
        <T as Config>::MaxMessagesPerCommit,
        <T as Config>::MaxMessagePayloadSize,
    >;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Verifier module for message verification.
        type Verifier: Verifier;

        /// Verifier module for message verification.
        type MessageDispatch: MessageDispatch<
            Self,
            GenericNetworkId,
            MessageId,
            GenericAdditionalInboundData,
        >;

        type OutboundChannel: OutboundChannel<
            EVMChainId,
            Self::AccountId,
            AdditionalEVMOutboundData,
        >;

        type AssetId;

        type Balance;

        type MessageStatusNotifier: MessageStatusNotifier<
            Self::AssetId,
            Self::AccountId,
            Self::Balance,
        >;

        type EVMCommitmentVerifier: CommitmentHandler<EVMChainId, EVMCommitmentOf<Self>>;

        type TONCommitmentVerifier: CommitmentHandler<TonNetworkId, TONCommitmentOf<Self>>;

        /// A configuration for base priority of unsigned transactions.
        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        /// A configuration for longevity of unsigned transactions.
        #[pallet::constant]
        type UnsignedLongevity: Get<u64>;

        #[pallet::constant]
        type ThisNetworkId: Get<GenericNetworkId>;

        /// Max bytes in a message payload
        #[pallet::constant]
        type MaxMessagePayloadSize: Get<u32>;

        /// Max number of messages that can be queued and committed in one go for a given channel.
        #[pallet::constant]
        type MaxMessagesPerCommit: Get<u32>;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    pub type ChannelNonces<T: Config> = StorageMap<_, Identity, GenericNetworkId, u64, ValueQuery>;

    #[pallet::storage]
    pub type ReportedChannelNonces<T: Config> =
        StorageMap<_, Identity, GenericNetworkId, u64, ValueQuery>;

    #[pallet::storage]
    pub type TONChannelAddresses<T: Config> =
        StorageMap<_, Identity, TonNetworkId, TonAddress, OptionQuery>;

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
        /// Submitted invalid commitment type.
        InvalidCommitment,
        /// Message has an unexpected nonce.
        InvalidNonce,
        /// Incorrect reward fraction
        InvalidRewardFraction,
        /// This contract already exists
        ContractExists,
        /// Call encoding failed.
        CallEncodeFailed,
        /// Invalid base fee update.
        InvalidBaseFeeUpdate,
    }

    impl<T: Config> Pallet<T> {
        fn submit_weight(
            commitment: &CommitmentOf<T>,
            proof: &<T::Verifier as Verifier>::Proof,
        ) -> Weight {
            let commitment_weight = match commitment {
                bridge_types::GenericCommitment::EVM(commitment) => match commitment {
                    bridge_types::evm::Commitment::Outbound(_) => {
                        <T as frame_system::Config>::BlockWeights::get().max_block
                    }
                    bridge_types::evm::Commitment::Inbound(commitment) => {
                        T::MessageDispatch::dispatch_weight(&commitment.payload)
                    }
                    bridge_types::evm::Commitment::StatusReport(_) => Default::default(),
                    bridge_types::evm::Commitment::BaseFeeUpdate(_) => Default::default(),
                },
                bridge_types::GenericCommitment::TON(bridge_types::ton::Commitment::Inbound(
                    commitment,
                )) => T::MessageDispatch::dispatch_weight(&commitment.payload),
                bridge_types::GenericCommitment::TON(bridge_types::ton::Commitment::Outbound(
                    _,
                )) => <T as frame_system::Config>::BlockWeights::get().max_block,
                bridge_types::GenericCommitment::Sub(commitment) => commitment
                    .messages
                    .iter()
                    .map(|m| T::MessageDispatch::dispatch_weight(&m.payload))
                    .fold(Weight::zero(), |acc, w| acc.saturating_add(w)),
            };

            let proof_weight = T::Verifier::verify_weight(proof);

            <T as Config>::WeightInfo::submit()
                .saturating_add(commitment_weight)
                .saturating_add(proof_weight)
        }

        fn ensure_channel_nonce(network_id: GenericNetworkId, new_nonce: u64) -> DispatchResult {
            let nonce = ChannelNonces::<T>::get(network_id);
            ensure!(nonce + 1 == new_nonce, Error::<T>::InvalidNonce);
            Ok(())
        }

        fn ensure_reported_nonce(network_id: GenericNetworkId, new_nonce: u64) -> DispatchResult {
            let nonce = ReportedChannelNonces::<T>::get(network_id);
            ensure!(nonce + 1 == new_nonce, Error::<T>::InvalidNonce);
            Ok(())
        }

        fn update_channel_nonce(network_id: GenericNetworkId, new_nonce: u64) -> DispatchResult {
            <ChannelNonces<T>>::try_mutate(network_id, |nonce| -> DispatchResult {
                if new_nonce != *nonce + 1 {
                    Err(Error::<T>::InvalidNonce.into())
                } else {
                    *nonce += 1;
                    Ok(())
                }
            })?;
            Ok(())
        }

        fn update_reported_nonce(network_id: GenericNetworkId, new_nonce: u64) -> DispatchResult {
            <ReportedChannelNonces<T>>::try_mutate(network_id, |nonce| -> DispatchResult {
                if new_nonce != *nonce + 1 {
                    Err(Error::<T>::InvalidNonce.into())
                } else {
                    *nonce += 1;
                    Ok(())
                }
            })?;
            Ok(())
        }

        fn handle_ton_commitment(
            network_id: TonNetworkId,
            commitment: &TONCommitmentOf<T>,
        ) -> DispatchResult {
            let network_id = GenericNetworkId::TON(network_id);
            match commitment {
                bridge_types::ton::Commitment::Inbound(inbound_commitment) => {
                    Self::update_channel_nonce(network_id, inbound_commitment.nonce)?;
                    let message_id = MessageId::basic(
                        network_id,
                        T::ThisNetworkId::get(),
                        inbound_commitment.nonce,
                    );
                    T::MessageDispatch::dispatch(
                        network_id,
                        message_id,
                        GenericTimepoint::TON(inbound_commitment.transaction_id),
                        &inbound_commitment.payload,
                        AdditionalTONInboundData {
                            source: inbound_commitment.source,
                        }
                        .into(),
                    );
                }
                bridge_types::ton::Commitment::Outbound(_) => {
                    frame_support::fail!(Error::<T>::InvalidCommitment);
                }
            }
            Ok(())
        }

        fn verify_ton_commitment(
            ton_network_id: TonNetworkId,
            commitment: &TONCommitmentOf<T>,
        ) -> DispatchResult {
            T::TONCommitmentVerifier::verify_commitment(&ton_network_id, commitment)?;
            let network_id = GenericNetworkId::TON(ton_network_id);
            match commitment {
                bridge_types::ton::Commitment::Inbound(inbound_commitment) => {
                    Self::ensure_channel_nonce(network_id, inbound_commitment.nonce)?;
                }
                bridge_types::ton::Commitment::Outbound(_) => {
                    frame_support::fail!(Error::<T>::InvalidCommitment);
                }
            }
            Ok(())
        }

        fn handle_evm_commitment(
            chain_id: EVMChainId,
            commitment: &EVMCommitmentOf<T>,
        ) -> DispatchResult {
            let network_id = GenericNetworkId::EVM(chain_id);
            match commitment {
                bridge_types::evm::Commitment::Inbound(inbound_commitment) => {
                    Self::update_channel_nonce(network_id, inbound_commitment.nonce)?;
                    let message_id = MessageId::basic(
                        network_id,
                        T::ThisNetworkId::get(),
                        inbound_commitment.nonce,
                    );
                    T::MessageDispatch::dispatch(
                        chain_id.into(),
                        message_id,
                        GenericTimepoint::EVM(inbound_commitment.block_number),
                        &inbound_commitment.payload,
                        AdditionalEVMInboundData {
                            source: inbound_commitment.source,
                        }
                        .into(),
                    );
                }
                bridge_types::evm::Commitment::StatusReport(status_report) => {
                    Self::update_reported_nonce(network_id, status_report.nonce)?;
                    for (i, result) in status_report.results.iter().enumerate() {
                        let status = if *result {
                            MessageStatus::Done
                        } else {
                            MessageStatus::Failed
                        };
                        T::MessageStatusNotifier::update_status(
                            network_id,
                            MessageId::batched(
                                T::ThisNetworkId::get(),
                                network_id,
                                status_report.nonce,
                                i as u64,
                            )
                            .hash(),
                            status,
                            GenericTimepoint::EVM(status_report.block_number),
                        )
                    }
                }
                bridge_types::evm::Commitment::BaseFeeUpdate(_) => {}
                bridge_types::evm::Commitment::Outbound(_) => {
                    frame_support::fail!(Error::<T>::InvalidCommitment);
                }
            }
            Ok(())
        }

        fn verify_evm_commitment(
            chain_id: EVMChainId,
            commitment: &EVMCommitmentOf<T>,
        ) -> DispatchResult {
            T::EVMCommitmentVerifier::verify_commitment(&chain_id, commitment)?;
            let network_id = GenericNetworkId::EVM(chain_id);
            match commitment {
                bridge_types::evm::Commitment::Inbound(inbound_commitment) => {
                    Self::ensure_channel_nonce(network_id, inbound_commitment.nonce)?;
                }
                bridge_types::evm::Commitment::StatusReport(status_report) => {
                    Self::ensure_reported_nonce(network_id, status_report.nonce)?;
                }
                bridge_types::evm::Commitment::BaseFeeUpdate(_) => {}
                bridge_types::evm::Commitment::Outbound(_) => {
                    frame_support::fail!(Error::<T>::InvalidCommitment);
                }
            }
            Ok(())
        }

        fn handle_sub_commitment(
            sub_network_id: SubNetworkId,
            commitment: &SubCommitmentOf<T>,
        ) -> DispatchResult {
            let network_id = GenericNetworkId::Sub(sub_network_id);
            Self::update_channel_nonce(network_id, commitment.nonce)?;
            for (idx, message) in commitment.messages.iter().enumerate() {
                let message_id = MessageId::batched(
                    network_id,
                    T::ThisNetworkId::get(),
                    commitment.nonce,
                    idx as u64,
                );
                T::MessageDispatch::dispatch(
                    sub_network_id.into(),
                    message_id,
                    message.timepoint,
                    &message.payload,
                    GenericAdditionalInboundData::Sub,
                );
            }
            Ok(())
        }

        fn verify_sub_commitment(
            sub_network_id: SubNetworkId,
            commitment: &SubCommitmentOf<T>,
        ) -> DispatchResult {
            let network_id = GenericNetworkId::Sub(sub_network_id);
            Self::ensure_channel_nonce(network_id, commitment.nonce)?;
            Ok(())
        }

        fn verify_commitment(
            network_id: GenericNetworkId,
            commitment: &CommitmentOf<T>,
            proof: &<T::Verifier as Verifier>::Proof,
        ) -> DispatchResult {
            match (network_id, &commitment) {
                (
                    GenericNetworkId::EVM(evm_network_id),
                    bridge_types::GenericCommitment::EVM(evm_commitment),
                ) => Self::verify_evm_commitment(evm_network_id, evm_commitment)?,
                (
                    GenericNetworkId::Sub(sub_network_id),
                    bridge_types::GenericCommitment::Sub(sub_commitment),
                ) => Self::verify_sub_commitment(sub_network_id, sub_commitment)?,
                (
                    GenericNetworkId::TON(ton_network_id),
                    bridge_types::GenericCommitment::TON(ton_commitment),
                ) => Self::verify_ton_commitment(ton_network_id, ton_commitment)?,
                _ => {
                    return Err(Error::<T>::InvalidCommitment.into());
                }
            }
            let commitment_hash = commitment.hash();
            T::Verifier::verify(network_id, commitment_hash, proof).map_err(|e| {
                warn!("Bad submit proof received: {:?}", e);
                e
            })?;
            Ok(())
        }

        fn handle_commitment(
            network_id: GenericNetworkId,
            commitment: &CommitmentOf<T>,
            proof: &<T::Verifier as Verifier>::Proof,
        ) -> DispatchResult {
            Self::verify_commitment(network_id, commitment, proof)?;
            match (network_id, commitment) {
                (
                    GenericNetworkId::EVM(evm_network_id),
                    bridge_types::GenericCommitment::EVM(evm_commitment),
                ) => Self::handle_evm_commitment(evm_network_id, &evm_commitment)?,
                (
                    GenericNetworkId::Sub(sub_network_id),
                    bridge_types::GenericCommitment::Sub(sub_commitment),
                ) => Self::handle_sub_commitment(sub_network_id, &sub_commitment)?,
                (
                    GenericNetworkId::TON(ton_network_id),
                    bridge_types::GenericCommitment::TON(ton_commitment),
                ) => Self::handle_ton_commitment(ton_network_id, &ton_commitment)?,
                _ => {
                    frame_support::fail!(Error::<T>::InvalidCommitment);
                }
            }
            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Pallet::<T>::submit_weight(commitment, proof))]
        pub fn submit(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            commitment: CommitmentOf<T>,
            proof: <T::Verifier as Verifier>::Proof,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;
            Self::handle_commitment(network_id, &commitment, &proof)?;
            Ok(().into())
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;
        // mb add prefetch with validate_ancestors=true to not include unneccessary stuff
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::submit {
                network_id,
                commitment,
                proof,
            } = call
            {
                Self::verify_commitment(*network_id, commitment, proof)
                    .map_err(|_| InvalidTransaction::BadProof)?;
                ValidTransaction::with_tag_prefix("SubstrateBridgeChannelSubmit")
                    .priority(T::UnsignedPriority::get())
                    .longevity(T::UnsignedLongevity::get())
                    .and_provides((network_id, commitment.hash()))
                    .propagate(true)
                    .build()
            } else {
                warn!("Unknown unsigned call, can't validate");
                InvalidTransaction::Call.into()
            }
        }
    }
}
