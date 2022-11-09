#![cfg_attr(not(feature = "std"), no_std)]

use bridge_common::{beefy_types::*, bitfield};
use codec::Encode;
use frame_support::log;
use frame_support::traits::Randomness;
use libsecp256k1::{Message, PublicKey, Signature};
pub use pallet::*;
use scale_info::prelude::vec::Vec;
use sp_io::hashing::keccak_256;

pub use bitfield::BitField;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub fn public_key_to_eth_address(pub_key: &PublicKey) -> EthAddress {
    let hash = keccak_256(&pub_key.serialize()[1..]);
    EthAddress::from_slice(&hash[12..])
}

pub fn prepare_message(msg: &[u8]) -> Message {
    let msg = keccak_256(msg);
    let mut prefix = b"\x19Ethereum Signed Message:\n32".to_vec();
    prefix.extend(&msg);
    let hash = keccak_256(&prefix);
    Message::parse_slice(&hash).expect("hash size == 256 bits; qed")
}

impl<T: Config, Output, BlockNumber> Randomness<Output, BlockNumber> for Pallet<T> {
    fn random(_: &[u8]) -> (Output, BlockNumber) {
        todo!()
    }

    fn random_seed() -> (Output, BlockNumber) {
        Self::random(&[][..])
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_common::{merkle_proof, simplified_mmr_proof::*};
    use frame_support::fail;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    pub const MMR_ROOT_HISTORY_SIZE: u32 = 30;

    pub const THRESHOLD_NUMERATOR: u128 = 22;
    pub const THRESHOLD_DENOMINATOR: u128 = 59;

    pub const NUMBER_OF_BLOCKS_PER_SESSION: u64 = 600;
    pub const ERROR_AND_SAFETY_BUFFER: u64 = 10;
    // pub const MAXIMUM_BLOCK_GAP: u64 = NUMBER_OF_BLOCKS_PER_SESSION - ERROR_AND_SAFETY_BUFFER;
    pub const MAXIMUM_BLOCK_GAP: u64 = 100;

    pub const MMR_ROOT_ID: [u8; 2] = [0x6d, 0x68];

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: frame_support::traits::Randomness<Self::Hash, Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    #[pallet::storage]
    #[pallet::getter(fn latest_mmr_roots)]
    pub type LatestMMRRoots<T> = StorageMap<_, Blake2_256, u128, [u8; 32], ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn latest_mmr_root_index)]
    pub type LatestMMRRootIndex<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn latest_beefy_block)]
    pub type LatestBeefyBlock<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn latest_random_seed)]
    pub type LatestRandomSeed<T> = StorageValue<_, [u8; 32], ValueQuery>;

    // Validator registry storage:
    #[pallet::storage]
    #[pallet::getter(fn validator_registry_root)]
    pub type ValidatorRegistryRoot<T> = StorageValue<_, [u8; 32], ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn validator_registry_num_of_validators)]
    // pub type NumOfValidators<T> = StorageValue<_, u128, ValueQuery>;

    // #[pallet::storage]
    // #[pallet::getter(fn validator_registry_id)]
    // pub type ValidatorRegistryId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn current_validator_set)]
    pub type CurrentValidatorSet<T> = StorageValue<_, ValidatorSet, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_validator_set)]
    pub type NextValidatorSet<T> = StorageValue<_, ValidatorSet, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        VerificationSuccessful(T::AccountId, u32),
        NewMMRRoot([u8; 32], u64),
        ValidatorRegistryUpdated([u8; 32], u128, u64),
    }

    // Errors inform users that something went wrong.
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
            latest_beefy_block: u32,
            validator_set: ValidatorSet,
            next_validator_set: ValidatorSet,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_root(origin)?;
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "==============================================================================================="
            );
            log::debug!(
                "beefy: ==============================================================================================="
            );
            LatestBeefyBlock::<T>::set(latest_beefy_block);
            CurrentValidatorSet::<T>::set(validator_set);
            NextValidatorSet::<T>::set(next_validator_set);
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn submit_signature_commitment(
            origin: OriginFor<T>,
            commitment: Commitment,
            validator_proof: ValidatorProof,
            latest_mmr_leaf: BeefyMMRLeaf,
            proof: SimplifiedMMRProof,
        ) -> DispatchResultWithPostInfo {
            let signer = ensure_signed(origin)?;
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "tokio-runtime-worker BeefyLightClient: submit_signature_commitment: {:?}",
                commitment
            );
            log::debug!(
                "tokio-runtime-worker BeefyLightClient: submit_signature_commitment validator proof: {:?}",
                validator_proof
            );
            log::debug!(
                "tokio-runtime-worker BeefyLightClient: submit_signature_commitment latest_mmr_leaf: {:?}",
                latest_mmr_leaf
            );
            log::debug!(
                "tokio-runtime-worker BeefyLightClient: submit_signature_commitment proof: {:?}",
                proof
            );
            let current_validator_set = Self::current_validator_set();
            let next_validator_set = Self::next_validator_set();
            let vset = match (commitment.validator_set_id as u128) == current_validator_set.id {
                true => current_validator_set,
                false => match (commitment.validator_set_id as u128) == next_validator_set.id {
                    true => next_validator_set,
                    false => fail!(Error::<T>::InvalidValidatorSetId),
                },
            };
            Self::verify_commitment(&commitment, &validator_proof, vset)?;
            Self::verity_newest_mmr_leaf(&latest_mmr_leaf, &commitment.payload, &proof)?;
            Self::process_payload(&commitment.payload, commitment.block_number.into())?;

            LatestRandomSeed::<T>::set(latest_mmr_leaf.random_seed);

            Self::deposit_event(Event::VerificationSuccessful(
                signer,
                commitment.block_number,
            ));
            Self::apply_validator_set_changes(
                latest_mmr_leaf.next_authority_set_id as u128,
                latest_mmr_leaf.next_authority_set_len as u128,
                latest_mmr_leaf.next_authority_set_root,
            )?;
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn add_known_mmr_root(root: [u8; 32]) -> u32 {
            let latest_mmr_root_index = LatestMMRRootIndex::<T>::get();
            let new_root_index = (latest_mmr_root_index + 1) % MMR_ROOT_HISTORY_SIZE;
            LatestMMRRoots::<T>::insert(new_root_index as u128, root);
            LatestMMRRootIndex::<T>::set(new_root_index);
            latest_mmr_root_index
        }

        pub fn is_known_root(root: [u8; 32]) -> bool {
            if root == <[u8; 32] as Default>::default() {
                return false;
            }
            let latest_mmr_root_index = LatestMMRRootIndex::<T>::get();
            let mut i = latest_mmr_root_index;
            loop {
                if root == LatestMMRRoots::<T>::get(i as u128) {
                    return true;
                }
                if i == 0 {
                    i = MMR_ROOT_HISTORY_SIZE
                }
                i = i - 1;
                if i != latest_mmr_root_index {
                    break;
                }
            }
            false
        }

        #[inline]
        pub fn get_latest_mmr_root() -> [u8; 32] {
            LatestMMRRoots::<T>::get(LatestMMRRootIndex::<T>::get() as u128)
        }

        #[inline]
        pub fn verity_beefy_merkle_leaf(
            beefy_mmr_leaf: [u8; 32],
            proof: SimplifiedMMRProof,
        ) -> bool {
            let proof_root = calculate_merkle_root(
                beefy_mmr_leaf,
                proof.merkle_proof_items,
                proof.merkle_proof_order_bit_field,
            );
            Self::is_known_root(proof_root)
        }

        #[inline]
        pub fn create_random_bit_field(
            validator_claims_bitfield: BitField,
            number_of_validators: u128,
        ) -> Result<BitField, Error<T>> {
            Self::random_n_bits_with_prior_check(
                &validator_claims_bitfield,
                Self::get_required_number_of_signatures(number_of_validators),
                number_of_validators,
            )
        }

        #[inline]
        pub fn create_initial_bitfield(bits_to_set: Vec<u128>, length: u128) -> BitField {
            BitField::create_bitfield(bits_to_set, length)
        }

        /* Private Functions */
        pub fn get_seed() -> [u8; 32] {
            let concated = bridge_common::concat_u8(&[
                &Self::latest_random_seed(),
                &Self::latest_beefy_block().to_be_bytes(),
            ]);
            keccak_256(&concated)
        }

        pub fn verity_newest_mmr_leaf(
            leaf: &BeefyMMRLeaf,
            root: &[u8; 32],
            proof: &SimplifiedMMRProof,
        ) -> DispatchResultWithPostInfo {
            let encoded_leaf = Self::encode_mmr_leaf(leaf.clone());
            let hash_leaf = Self::hash_mmr_leaf(encoded_leaf);
            ensure!(
                verify_inclusion_proof(*root, hash_leaf, proof.clone()),
                Error::<T>::InvalidMMRProof
            );
            Ok(().into())
        }

        // TODO
        // u64 casting to u32!!!!!!!
        pub fn process_payload(
            payload: &[u8; 32],
            block_number: u64,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                block_number > Self::latest_beefy_block() as u64,
                Error::<T>::PayloadBlocknumberTooOld
            );
            ensure!(
                block_number < Self::latest_beefy_block() as u64 + MAXIMUM_BLOCK_GAP,
                Error::<T>::PayloadBlocknumberTooNew
            );
            Self::add_known_mmr_root(*payload);
            // NOT SAFE!!!!!!!!!
            LatestBeefyBlock::<T>::set(block_number.try_into().unwrap());
            Self::deposit_event(Event::NewMMRRoot(*payload, block_number));
            Ok(().into())
        }

        pub fn apply_validator_set_changes(
            next_authority_set_id: u128,
            next_authority_set_len: u128,
            next_authority_set_root: [u8; 32],
        ) -> DispatchResultWithPostInfo {
            let next_validator_set = Self::next_validator_set();
            if next_authority_set_id > next_validator_set.id {    
                ensure!(
                    next_authority_set_id as u128 > next_validator_set.id,
                    Error::<T>::CannotSwitchOldValidatorSet
                );
                CurrentValidatorSet::<T>::set(next_validator_set);
                NextValidatorSet::<T>::set(ValidatorSet{
                    id: next_authority_set_id,
                    length: next_authority_set_len,
                    root: next_authority_set_root
                });
                // Self::validator_registry_update(
                //     next_authority_set_root,
                //     next_authority_set_len as u128,
                //     next_authority_set_id,
                // );
            }
            Ok(().into())
        }

        pub fn required_number_of_signatures() -> u128 {
            Self::get_required_number_of_signatures(Self::current_validator_set().length)
        }

        pub fn get_required_number_of_signatures(num_validators: u128) -> u128 {
            (num_validators * THRESHOLD_NUMERATOR + THRESHOLD_DENOMINATOR - 1)
                / THRESHOLD_DENOMINATOR
        }

        /**
        	* @dev https://github.com/sora-xor/substrate/blob/7d914ce3ed34a27d7bb213caed374d64cde8cfa8/client/beefy/src/round.rs#L62
        */
        // ON RELAYER???????
        pub fn check_commitment_signatures_threshold(
            num_of_validators: u128,
            validator_claims_bitfield: BitField,
        ) -> DispatchResultWithPostInfo {
            let threshold = num_of_validators - (num_of_validators - 1) / 3;
            let count = validator_claims_bitfield.count_set_bits();
            ensure!(count >= threshold, Error::<T>::NotEnoughValidatorSignatures);
            Ok(().into())
        }

        pub fn verify_commitment(
            commitment: &Commitment,
            proof: &ValidatorProof,
            vset: ValidatorSet,
        ) -> DispatchResultWithPostInfo {
            let number_of_validators = vset.length;
            let required_num_of_signatures =
                Self::get_required_number_of_signatures(number_of_validators);
            Self::check_commitment_signatures_threshold(
                number_of_validators,
                proof.validator_claims_bitfield.clone(),
            )?;
            let random_bitfield = Self::random_n_bits_with_prior_check(
                &proof.validator_claims_bitfield,
                required_num_of_signatures,
                number_of_validators,
            )?;
            log::debug!(
                "BeefyLightClient verify_commitment proof: {:?}",
                proof
            );
            log::debug!(
                "BeefyLightClient verify_commitment validator_claims_bitfield: {:?}",
                proof.validator_claims_bitfield.clone()
            );
            log::debug!(
                "BeefyLightClient verify_commitment random_bitfield: {:?}",
                random_bitfield.clone()
            );
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            log::debug!(
                "tokio-runtime-worker: ==============================================================================================="
            );
            Self::verify_validator_proof_lengths(required_num_of_signatures, proof.clone())?;
            let commitment_hash = Self::create_commitment_hash(commitment.clone());
            Self::verify_validator_proof_signatures(
                random_bitfield,
                proof.clone(),
                required_num_of_signatures,
                commitment_hash,
            )?;
            Ok(().into())
        }

        pub fn verify_validator_proof_lengths(
            required_num_of_signatures: u128,
            proof: ValidatorProof,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                proof.signatures.len() as u128 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfSignatures
            );
            ensure!(
                proof.positions.len() as u128 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfPositions
            );
            ensure!(
                proof.public_keys.len() as u128 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfPublicKeys
            );
            ensure!(
                proof.public_key_merkle_proofs.len() as u128 == required_num_of_signatures,
                Error::<T>::InvalidNumberOfPublicKeys
            );
            Ok(().into())
        }

        pub fn verify_validator_proof_signatures(
            random_bitfield: BitField,
            proof: ValidatorProof,
            required_num_of_signatures: u128,
            commitment_hash: [u8; 32],
        ) -> DispatchResultWithPostInfo {
            let required_num_of_signatures = required_num_of_signatures as usize;
            for i in 0..required_num_of_signatures {
                Self::verify_validator_signature(
                    random_bitfield.clone(),
                    proof.signatures[i].clone(),
                    proof.positions[i],
                    proof.public_keys[i].clone(),
                    proof.public_key_merkle_proofs[i].clone(),
                    commitment_hash,
                )?;
            }
            Ok(().into())
        }

        pub fn verify_validator_signature(
            mut random_bitfield: BitField,
            signature: Vec<u8>,
            position: u128,
            public_key: EthAddress,
            public_key_merkle_proof: Vec<[u8; 32]>,
            commitment_hash: [u8; 32],
        ) -> DispatchResultWithPostInfo {
            log::debug!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            );
            log::debug!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            );
            log::debug!(
                "POSITION: {:?}", position
            );
            log::debug!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            );
            log::debug!(
                "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            );
            ensure!(
                random_bitfield.is_set(position as usize),
                Error::<T>::ValidatorNotOnceInbitfield
            );
            random_bitfield.clear(position as usize);
            ensure!(
                Self::check_validator_in_set(public_key, position, public_key_merkle_proof),
                Error::<T>::ValidatorSetIncorrectPosition
            );
            let mes = prepare_message(&commitment_hash);
            let sig = match Signature::parse_standard_slice(&signature) {
                Err(_) => fail!(Error::<T>::InvalidSignature),
                Ok(p) => p,
            };
            let recovered_public =
                libsecp256k1::recover(&mes, &sig, &libsecp256k1::RecoveryId::parse(0).unwrap())
                    .unwrap();
            let addr = public_key_to_eth_address(&recovered_public);
            // let recovered_public = match libsecp256k1::recover(&commitment_hash) {
            // 	Err(_) => fail!(Error::<T>::InvalidSignature),
            // 	Ok(p) => p,
            // };
            // // TODO: Check if it is correct!
            ensure!(addr == public_key, Error::<T>::InvalidSignature);
            Ok(().into())
        }

        pub fn create_commitment_hash(commitment: Commitment) -> [u8; 32] {
            let concated = bridge_common::concat_u8(&[
                &commitment.payload_prefix,
                &MMR_ROOT_ID,
                &[0x80],
                &commitment.payload,
                &commitment.payload_suffix,
                &commitment.block_number.to_be_bytes(),
                &commitment.validator_set_id.to_be_bytes(),
            ]);
            keccak_256(&concated)
        }

        pub fn encode_mmr_leaf(leaf: BeefyMMRLeaf) -> Vec<u8> {
            leaf.encode()
        }

        pub fn hash_mmr_leaf(leaf: Vec<u8>) -> [u8; 32] {
            keccak_256(&leaf)
        }

        // pub fn validator_registry_update(
        //     new_root: [u8; 32],
        //     new_num_of_validators: u128,
        //     new_id: u64,
        // ) {
        //     ValidatorRegistryRoot::<T>::set(new_root);
        //     NumOfValidators::<T>::set(new_num_of_validators);
        //     ValidatorRegistryId::<T>::set(new_id);
        //     Self::deposit_event(Event::<T>::ValidatorRegistryUpdated(
        //         new_root,
        //         new_num_of_validators,
        //         new_id,
        //     ));
        // }

        pub fn check_validator_in_set(addr: EthAddress, pos: u128, proof: Vec<[u8; 32]>) -> bool {
            let hashed_leaf = keccak_256(&addr.encode());
            merkle_proof::verify_merkle_leaf_at_position(
                Self::validator_registry_root(),
                hashed_leaf,
                pos,
                Self::current_validator_set().length,
                proof,
            )
        }

        pub fn random_n_bits_with_prior_check(
            prior: &BitField,
            n: u128,
            length: u128,
        ) -> Result<BitField, Error<T>> {
            let raw_seed = T::Randomness::random_seed();
            let seed = codec::Encode::using_encoded(&raw_seed, sp_io::hashing::blake2_128);
            Ok(BitField::create_random_bitfield(
                prior,
                n,
                length,
                u128::from_be_bytes(seed),
            ))
        }
    }
}
