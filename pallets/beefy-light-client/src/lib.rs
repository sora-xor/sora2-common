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

#![cfg_attr(not(feature = "std"), no_std)]

use bridge_common::{beefy_types::*, bitfield, simplified_mmr_proof::SimplifiedMMRProof};
use bridge_types::types::AuxiliaryDigest;
use codec::Decode;
use codec::Encode;
use frame_support::log;
use frame_support::traits::Randomness;
pub use pallet::*;
use scale_info::prelude::vec::Vec;
use sp_core::RuntimeDebug;
use sp_core::H256;
use sp_io::hashing::keccak_256;
use sp_runtime::traits::Hash;
use sp_runtime::traits::Keccak256;
use sp_std::collections::vec_deque::VecDeque;

pub const RANDOMNESS_SUBJECT: &[u8] = b"beefy-light-client";

pub use bitfield::BitField;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
pub struct ProvedSubstrateBridgeMessage<Message> {
    pub message: Message,
    pub proof: SimplifiedMMRProof,
    pub leaf: BeefyMMRLeaf,
    pub digest: AuxiliaryDigest,
}

fn recover_signature(sig: &[u8; 65], msg_hash: &H256) -> Option<EthAddress> {
    use sp_io::crypto::secp256k1_ecdsa_recover;

    secp256k1_ecdsa_recover(sig, &msg_hash.0)
        .map(|pubkey| EthAddress::from(H256::from_slice(&keccak_256(&pubkey))))
        .ok()
}

impl<T: Config> Randomness<sp_core::H256, T::BlockNumber> for Pallet<T> {
    fn random(subject: &[u8]) -> (sp_core::H256, T::BlockNumber) {
        let (seed, block) = Self::latest_random_seed();
        (
            sp_runtime::traits::Keccak256::hash_of(&(subject, seed)),
            block,
        )
    }

    fn random_seed() -> (sp_core::H256, T::BlockNumber) {
        Self::latest_random_seed()
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_common::simplified_mmr_proof::*;
    use bridge_types::types::AuxiliaryDigestItem;
    use bridge_types::{GenericNetworkId, SubNetworkId};
    use frame_support::fail;
    use frame_support::pallet_prelude::OptionQuery;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    pub const MMR_ROOT_HISTORY_SIZE: usize = 30;
    pub const THRESHOLD_NUMERATOR: u32 = 22;
    pub const THRESHOLD_DENOMINATOR: u32 = 59;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Randomness: frame_support::traits::Randomness<Self::Hash, Self::BlockNumber>;
        type Message: Parameter;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    #[pallet::storage]
    #[pallet::getter(fn latest_mmr_roots)]
    pub type LatestMMRRoots<T> = StorageValue<_, VecDeque<H256>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn latest_beefy_block)]
    pub type LatestBeefyBlock<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn latest_random_seed)]
    pub type LatestRandomSeed<T> =
        StorageValue<_, (H256, <T as frame_system::Config>::BlockNumber), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_validator_set)]
    pub type CurrentValidatorSet<T> = StorageValue<_, ValidatorSet, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_validator_set)]
    pub type NextValidatorSet<T> = StorageValue<_, ValidatorSet, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        VerificationSuccessful(T::AccountId, u32),
        NewMMRRoot(H256, u64),
        ValidatorRegistryUpdated(H256, u32, u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidValidatorSetId,
        InvalidMMRProof,
        PayloadBlocknumberTooOld,
        PayloadBlocknumberTooNew,
        CannotSwitchOldValidatorSet,
        NotEnoughValidatorSignatures,
        InvalidNumberOfSignatures,
        InvalidNumberOfPositions,
        InvalidNumberOfPublicKeys,
        ValidatorNotOnceInbitfield,
        ValidatorSetIncorrectPosition,
        InvalidSignature,
        MerklePositionTooHigh,
        MerkleProofTooShort,
        MerkleProofTooHigh,
        PalletNotInitialized,
        InvalidDigestHash,
        CommitmentNotFoundInDigest,
        MMRPayloadNotFound,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn initialize(
            origin: OriginFor<T>,
            latest_beefy_block: u64,
            validator_set: ValidatorSet,
            next_validator_set: ValidatorSet,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            LatestBeefyBlock::<T>::set(latest_beefy_block);
            CurrentValidatorSet::<T>::set(Some(validator_set));
            NextValidatorSet::<T>::set(Some(next_validator_set));
            Ok(().into())
        }

        #[pallet::weight(0)]
        #[frame_support::transactional]
        pub fn submit_signature_commitment(
            origin: OriginFor<T>,
            commitment: Commitment,
            validator_proof: ValidatorProof,
            latest_mmr_leaf: BeefyMMRLeaf,
            proof: SimplifiedMMRProof,
        ) -> DispatchResultWithPostInfo {
            let signer = ensure_signed(origin)?;
            log::debug!(
                "BeefyLightClient: submit_signature_commitment: {:?}",
                commitment
            );
            log::debug!(
                "BeefyLightClient: submit_signature_commitment validator proof: {:?}",
                validator_proof
            );
            log::debug!(
                "BeefyLightClient: submit_signature_commitment latest_mmr_leaf: {:?}",
                latest_mmr_leaf
            );
            log::debug!(
                "BeefyLightClient: submit_signature_commitment proof: {:?}",
                proof
            );
            let current_validator_set = match Self::current_validator_set() {
                None => fail!(Error::<T>::PalletNotInitialized),
                Some(x) => x,
            };
            let next_validator_set = match Self::next_validator_set() {
                None => fail!(Error::<T>::PalletNotInitialized),
                Some(x) => x,
            };
            let vset = match (commitment.validator_set_id) == current_validator_set.id {
                true => current_validator_set,
                false => match (commitment.validator_set_id) == next_validator_set.id {
                    true => next_validator_set,
                    false => fail!(Error::<T>::InvalidValidatorSetId),
                },
            };
            Self::verify_commitment(&commitment, &validator_proof, vset)?;
            let payload = commitment
                .payload
                .get_decoded::<H256>(&beefy_primitives::known_payloads::MMR_ROOT_ID)
                .ok_or(Error::<T>::MMRPayloadNotFound)?;
            Self::verity_newest_mmr_leaf(&latest_mmr_leaf, &payload, &proof)?;
            Self::process_payload(payload, commitment.block_number.into())?;

            let block_number = <frame_system::Pallet<T>>::block_number();
            LatestRandomSeed::<T>::set((latest_mmr_leaf.leaf_extra.random_seed, block_number));

            Self::deposit_event(Event::VerificationSuccessful(
                signer,
                commitment.block_number,
            ));
            Self::apply_validator_set_changes(latest_mmr_leaf.beefy_next_authority_set)?;
            Ok(().into())
        }
    }

    impl<T: Config>
        bridge_types::traits::Verifier<SubNetworkId, ProvedSubstrateBridgeMessage<T::Message>>
        for Pallet<T>
    {
        type Result = T::Message;
        fn verify(
            network_id: SubNetworkId,
            message: &ProvedSubstrateBridgeMessage<T::Message>,
        ) -> Result<Self::Result, DispatchError> {
            Self::verify_mmr_leaf(&message.leaf, &message.proof)?;
            let digest_hash = message.digest.using_encoded(keccak_256);
            ensure!(
                digest_hash == message.leaf.leaf_extra.digest_hash.0,
                Error::<T>::InvalidMMRProof
            );
            let commitment_hash = message.message.using_encoded(keccak_256);
            let count = message
                .digest
                .logs
                .iter()
                .filter(|x| {
                    let AuxiliaryDigestItem::Commitment(log_network_id, log_commitment_hash) = x;
                    if let GenericNetworkId::Sub(log_network_id) = log_network_id {
                        return *log_network_id == network_id
                            && commitment_hash == log_commitment_hash.0;
                    }
                    false
                })
                .count();
            ensure!(count == 1, Error::<T>::CommitmentNotFoundInDigest);

            Ok(message.message.clone())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn add_known_mmr_root(root: H256) {
            let mut mmr_roots = LatestMMRRoots::<T>::get();
            mmr_roots.push_back(root);
            if mmr_roots.len() > MMR_ROOT_HISTORY_SIZE {
                mmr_roots.pop_front();
            }
        }

        pub fn is_known_root(root: H256) -> bool {
            let mmr_roots = LatestMMRRoots::<T>::get();
            mmr_roots.contains(&root)
        }

        #[inline]
        pub fn get_latest_mmr_root() -> Option<H256> {
            LatestMMRRoots::<T>::get().back().cloned()
        }

        #[inline]
        pub fn verify_beefy_merkle_leaf(beefy_mmr_leaf: H256, proof: &SimplifiedMMRProof) -> bool {
            let proof_root = calculate_merkle_root(
                beefy_mmr_leaf,
                &proof.merkle_proof_items,
                proof.merkle_proof_order_bit_field,
            );
            Self::is_known_root(proof_root)
        }

        #[inline]
        pub fn create_random_bit_field(
            validator_claims_bitfield: BitField,
            number_of_validators: u32,
        ) -> Result<BitField, Error<T>> {
            Self::random_n_bits_with_prior_check(
                &validator_claims_bitfield,
                Self::get_required_number_of_signatures(number_of_validators),
                number_of_validators,
            )
        }

        #[inline]
        pub fn create_initial_bitfield(bits_to_set: &[u32], length: usize) -> BitField {
            BitField::create_bitfield(bits_to_set, length)
        }

        #[inline]
        pub fn required_number_of_signatures() -> u32 {
            let len = match Self::current_validator_set() {
                None => 0,
                Some(x) => x.len,
            };
            Self::get_required_number_of_signatures(len)
        }

        /* Private Functions */

        fn verity_newest_mmr_leaf(
            leaf: &BeefyMMRLeaf,
            root: &H256,
            proof: &SimplifiedMMRProof,
        ) -> DispatchResultWithPostInfo {
            let hash_leaf = Keccak256::hash_of(&leaf);
            ensure!(
                verify_inclusion_proof(*root, hash_leaf, proof),
                Error::<T>::InvalidMMRProof
            );
            Ok(().into())
        }

        fn verify_mmr_leaf(leaf: &BeefyMMRLeaf, proof: &SimplifiedMMRProof) -> DispatchResult {
            let hash_leaf = Keccak256::hash_of(&leaf);
            let root = calculate_merkle_root(
                hash_leaf,
                &proof.merkle_proof_items,
                proof.merkle_proof_order_bit_field,
            );
            ensure!(Self::is_known_root(root), Error::<T>::InvalidMMRProof);
            Ok(())
        }

        fn process_payload(payload: H256, block_number: u64) -> DispatchResultWithPostInfo {
            ensure!(
                block_number > Self::latest_beefy_block(),
                Error::<T>::PayloadBlocknumberTooOld
            );
            Self::add_known_mmr_root(payload);
            LatestBeefyBlock::<T>::set(block_number);
            Self::deposit_event(Event::NewMMRRoot(payload, block_number));
            Ok(().into())
        }

        fn apply_validator_set_changes(new_vset: ValidatorSet) -> DispatchResultWithPostInfo {
            let next_validator_set = match Self::next_validator_set() {
                None => fail!(Error::<T>::PalletNotInitialized),
                Some(x) => x,
            };
            if new_vset.id > next_validator_set.id {
                CurrentValidatorSet::<T>::set(Some(next_validator_set));
                NextValidatorSet::<T>::set(Some(new_vset));
            }
            Ok(().into())
        }

        fn get_required_number_of_signatures(num_validators: u32) -> u32 {
            (num_validators * THRESHOLD_NUMERATOR + THRESHOLD_DENOMINATOR - 1)
                / THRESHOLD_DENOMINATOR
        }

        /**
        	* @dev https://github.com/sora-xor/substrate/blob/7d914ce3ed34a27d7bb213caed374d64cde8cfa8/client/beefy/src/round.rs#L62
         */
        fn check_commitment_signatures_threshold(
            num_of_validators: u32,
            validator_claims_bitfield: &BitField,
        ) -> DispatchResultWithPostInfo {
            let threshold = num_of_validators - (num_of_validators - 1) / 3;
            let count = validator_claims_bitfield.count_set_bits() as u32;
            ensure!(count >= threshold, Error::<T>::NotEnoughValidatorSignatures);
            Ok(().into())
        }

        fn verify_commitment(
            commitment: &Commitment,
            proof: &ValidatorProof,
            vset: ValidatorSet,
        ) -> DispatchResultWithPostInfo {
            let number_of_validators = vset.len;
            let required_num_of_signatures =
                Self::get_required_number_of_signatures(number_of_validators);
            Self::check_commitment_signatures_threshold(
                number_of_validators,
                &proof.validator_claims_bitfield,
            )?;
            let random_bitfield = Self::random_n_bits_with_prior_check(
                &proof.validator_claims_bitfield,
                required_num_of_signatures,
                number_of_validators,
            )?;
            log::debug!("BeefyLightClient verify_commitment proof: {:?}", proof);
            log::debug!(
                "BeefyLightClient verify_commitment validator_claims_bitfield: {:?}",
                proof.validator_claims_bitfield
            );
            log::debug!(
                "BeefyLightClient verify_commitment random_bitfield: {:?}",
                random_bitfield
            );
            Self::verify_validator_proof_lengths(required_num_of_signatures, proof)?;
            let commitment_hash = Keccak256::hash_of(&commitment);
            Self::verify_validator_proof_signatures(
                random_bitfield,
                proof,
                required_num_of_signatures,
                commitment_hash,
            )?;
            Ok(().into())
        }

        fn verify_validator_proof_lengths(
            required_num_of_signatures: u32,
            proof: &ValidatorProof,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                proof.signatures.len() as u32 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfSignatures
            );
            ensure!(
                proof.positions.len() as u32 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfPositions
            );
            ensure!(
                proof.public_keys.len() as u32 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfPublicKeys
            );
            ensure!(
                proof.public_key_merkle_proofs.len() as u32 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfPublicKeys
            );
            Ok(().into())
        }

        fn verify_validator_proof_signatures(
            mut random_bitfield: BitField,
            proof: &ValidatorProof,
            required_num_of_signatures: u32,
            commitment_hash: H256,
        ) -> DispatchResultWithPostInfo {
            let required_num_of_signatures = required_num_of_signatures as usize;
            for i in 0..required_num_of_signatures {
                Self::verify_validator_signature(
                    &mut random_bitfield,
                    proof.signatures[i].clone(),
                    proof.positions[i],
                    proof.public_keys[i],
                    &proof.public_key_merkle_proofs[i],
                    commitment_hash,
                )?;
            }
            Ok(().into())
        }

        fn verify_validator_signature(
            random_bitfield: &mut BitField,
            signature: Vec<u8>,
            position: u128,
            public_key: EthAddress,
            public_key_merkle_proof: &[H256],
            commitment_hash: H256,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                random_bitfield.is_set(position as usize),
                Error::<T>::ValidatorNotOnceInbitfield
            );
            random_bitfield.clear(position as usize);
            Self::check_validator_in_set(public_key, position, public_key_merkle_proof)?;
            ensure!(signature.len() == 65, Error::<T>::InvalidSignature);
            let signature: [u8; 65] = match signature.try_into() {
                Ok(v) => v,
                Err(_) => fail!(Error::<T>::InvalidSignature),
            };
            let addr = match recover_signature(&signature, &commitment_hash) {
                Some(v) => v,
                None => fail!(Error::<T>::InvalidSignature),
            };
            ensure!(addr == public_key, Error::<T>::InvalidSignature);
            Ok(().into())
        }

        fn check_validator_in_set(
            addr: EthAddress,
            pos: u128,
            proof: &[H256],
        ) -> DispatchResultWithPostInfo {
            let vset = match Self::current_validator_set() {
                None => fail!(Error::<T>::PalletNotInitialized),
                Some(x) => x,
            };
            let current_validator_set_len = match Self::current_validator_set() {
                None => fail!(Error::<T>::PalletNotInitialized),
                Some(x) => x.len,
            };
            ensure!(
                beefy_merkle_tree::verify_proof::<sp_runtime::traits::Keccak256, _, _>(
                    &vset.root,
                    proof.iter().cloned(),
                    current_validator_set_len as usize,
                    pos as usize,
                    &addr
                ),
                Error::<T>::ValidatorSetIncorrectPosition
            );
            Ok(().into())
        }

        pub fn random_n_bits_with_prior_check(
            prior: &BitField,
            n: u32,
            length: u32,
        ) -> Result<BitField, Error<T>> {
            let (raw_seed, _block) = T::Randomness::random(RANDOMNESS_SUBJECT);
            let latest_beefy_block = Self::latest_beefy_block();
            let seed = codec::Encode::using_encoded(
                &(raw_seed, latest_beefy_block),
                sp_io::hashing::blake2_128,
            );
            Ok(BitField::create_random_bitfield(
                prior,
                n,
                length,
                u128::from_be_bytes(seed),
            ))
        }
    }
}
