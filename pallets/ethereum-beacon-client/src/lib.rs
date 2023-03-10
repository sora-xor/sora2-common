//! # Ethereum Beacon Client
#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

pub const DOMAIN_SYNC_COMMITTEE: u32 = 7;

pub use weights::WeightInfo;

use frame_support::{dispatch::DispatchResult, log, traits::UnixTime, transactional};
use frame_system::ensure_signed;
use sp_core::H256;
use sp_io::hashing::sha2_256;
use sp_std::prelude::*;

pub use pallet::*;

#[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen, scale_info::TypeInfo)]
pub struct ExecutionHeaderState {
    beacon_block_root: H256,
    beacon_slot: beacon::Slot,
    block_hash: H256,
    block_number: u64,
}

#[derive(codec::Decode, codec::Encode, codec::MaxEncodedLen, scale_info::TypeInfo)]
pub struct FinalizedHeaderState {
    import_time: u64,
    beacon_block_root: H256,
    beacon_slot: beacon::Slot,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use beacon::{
        light_client_bootstrap::LightClientBootstrap,
        light_client_update::{
            LightClientUpdate, CURRENT_SYNC_COMMITTEE_INDEX, CURRENT_SYNC_COMMITTEE_PROOF_LEN,
            FINALIZED_ROOT_INDEX, FINALIZED_ROOT_PROOF_LEN, NEXT_SYNC_COMMITTEE_INDEX,
            NEXT_SYNC_COMMITTEE_PROOF_LEN,
        },
        BeaconBlockHeader, BlindedBeaconBlock, ChainSpec, Epoch, EthSpec, ExecPayload,
        ExecutionPayloadHeader, Hash256, LightClientFinalityUpdate, LightClientOptimisticUpdate,
        Slot, SyncAggregate, SyncCommittee,
    };
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::DispatchError;
    use tree_hash::TreeHash;

    use bridge_types::{beacon::ForkVersion, network_config::NetworkConfig, traits::Verifier};
    use bridge_types::{
        types::{Message, Proof},
        EVMChainId,
    };
    use ethereum_primitives::{Log, Receipt};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type EthSpec: EthSpec;
        type TimeProvider: UnixTime;
        type WeightInfo: WeightInfo;
        type WeakSubjectivityPeriodSeconds: Get<u64>;
        type MaxFinalizedHeaderSlotArray: Get<u32>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BeaconHeaderImported { block_hash: H256, slot: Slot },
        ExecutionHeaderImported { block_hash: H256, block_number: u64 },
        SyncCommitteeUpdated { period: u64 },
    }

    #[pallet::error]
    pub enum Error<T> {
        SyncCommitteeMissing,
        SyncCommitteeParticipantsNotSupermajority,
        InvalidHeaderMerkleProof,
        InvalidSyncCommitteeMerkleProof,
        SignatureVerificationFailed,
        HeaderNotFinalized,
        MissingHeader,
        InvalidProof,
        DecodeFailed,
        BridgeBlocked,
        InvalidSyncCommitteeHeaderUpdate,
        InvalidSyncCommitteePeriodUpdateWithGap,
        InvalidSyncCommitteePeriodUpdateWithDuplication,
        InvalidFinalizedHeaderUpdate,
        InvalidFinalizedPeriodUpdate,
        InvalidExecutionHeaderUpdate,
        FinalizedBeaconHeaderSlotsExceeded,
        WrongBlockBodyHashTreeRoot,
        ArithError,
        NetworkNotInitialized,
        InvalidPublicKeyBytes,
        WrongConsensus,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    pub(super) type FinalizedBeaconHeaders<T: Config> =
        StorageDoubleMap<_, Identity, EVMChainId, Identity, H256, BeaconBlockHeader, OptionQuery>;

    #[pallet::storage]
    pub(super) type FinalizedBeaconHeaderSlots<T: Config> = StorageMap<
        _,
        Identity,
        EVMChainId,
        BoundedVec<Slot, T::MaxFinalizedHeaderSlotArray>,
        OptionQuery,
    >;

    #[pallet::storage]
    pub(super) type FinalizedBeaconHeadersBlockRoot<T: Config> =
        StorageDoubleMap<_, Identity, EVMChainId, Identity, H256, H256, OptionQuery>;

    #[pallet::storage]
    pub(super) type ExecutionHeaders<T: Config> = StorageDoubleMap<
        _,
        Identity,
        EVMChainId,
        Identity,
        H256,
        ExecutionPayloadHeader<T::EthSpec>,
        OptionQuery,
    >;

    /// Current sync committee corresponding to the active header.
    /// TODO  prune older sync committees than xxx
    #[pallet::storage]
    pub(super) type SyncCommittees<T: Config> = StorageDoubleMap<
        _,
        Identity,
        EVMChainId,
        Identity,
        u64,
        SyncCommittee<T::EthSpec>,
        OptionQuery,
    >;

    #[pallet::storage]
    pub(super) type ValidatorsRoot<T: Config> =
        StorageMap<_, Identity, EVMChainId, H256, OptionQuery>;

    #[pallet::storage]
    pub(super) type LatestFinalizedHeaderState<T: Config> =
        StorageMap<_, Identity, EVMChainId, FinalizedHeaderState, OptionQuery>;

    #[pallet::storage]
    pub(super) type LatestExecutionHeaderState<T: Config> =
        StorageMap<_, Identity, EVMChainId, ExecutionHeaderState, OptionQuery>;

    #[pallet::storage]
    pub(super) type LatestSyncCommitteePeriod<T: Config> =
        StorageMap<_, Identity, EVMChainId, u64, OptionQuery>;

    #[pallet::storage]
    pub(super) type NetworkConfigs<T: Config> =
        StorageMap<_, Identity, EVMChainId, NetworkConfig, OptionQuery>;

    #[pallet::storage]
    pub(super) type Blocked<T: Config> = StorageMap<_, Identity, EVMChainId, bool, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::initialize())]
        #[transactional]
        pub fn initialize(
            origin: OriginFor<T>,
            chain_id: EVMChainId,
            bootstrap: LightClientBootstrap<T::EthSpec>,
            validators_root: H256,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if let Err(err) = Self::initial_sync(chain_id, bootstrap, validators_root) {
                log::error!(
                    target: "ethereum-beacon-client",
                    "💫 Sync committee period update failed with error {:?}",
                    err
                );
                return Err(err);
            }
            Self::check_network_config(chain_id)?;

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Network initialized",
            );

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::sync_committee_period_update())]
        #[transactional]
        pub fn sync_committee_period_update(
            origin: OriginFor<T>,
            chain_id: EVMChainId,
            update: LightClientUpdate<T::EthSpec>,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            Self::check_bridge_blocked_state(chain_id)?;
            Self::check_network_config(chain_id)?;

            let signature_epoch = update.signature_slot.epoch_with_spec::<T::EthSpec>();
            let sync_committee_period = signature_epoch
                .sync_committee_period_with_spec::<T::EthSpec>()
                .map_err(|_| Error::<T>::ArithError)?;
            log::info!(
                target: "ethereum-beacon-client",
                "💫 Received sync committee update for period {}. Applying update",
                sync_committee_period
            );

            if let Err(err) = Self::process_sync_committee_period_update(chain_id, update) {
                log::error!(
                    target: "ethereum-beacon-client",
                    "💫 Sync committee period update failed with error {:?}",
                    err
                );
                return Err(err);
            }

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Sync committee period update for period {} succeeded.",
                sync_committee_period
            );

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::import_finalized_header())]
        #[transactional]
        pub fn import_finalized_header(
            origin: OriginFor<T>,
            chain_id: EVMChainId,
            finalized_header_update: LightClientFinalityUpdate<T::EthSpec>,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            Self::check_bridge_blocked_state(chain_id)?;
            Self::check_network_config(chain_id)?;

            let slot = finalized_header_update.finalized_header.slot;

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Received finalized header for slot {}.",
                slot
            );

            if let Err(err) = Self::process_finalized_header(chain_id, finalized_header_update) {
                log::error!(
                    target: "ethereum-beacon-client",
                    "💫 Finalized header update failed with error {:?}",
                    err
                );
                return Err(err);
            }

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Stored finalized beacon header at slot {}.",
                slot
            );

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::import_execution_header())]
        #[transactional]
        pub fn import_execution_header(
            origin: OriginFor<T>,
            chain_id: EVMChainId,
            update: LightClientOptimisticUpdate<T::EthSpec>,
            block: BlindedBeaconBlock<T::EthSpec>,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            Self::check_bridge_blocked_state(chain_id)?;
            Self::check_network_config(chain_id)?;

            let slot = update.attested_header.slot;
            let block_hash = update.attested_header.body_root;

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Received header update for slot {}.",
                slot
            );

            if let Err(err) = Self::process_header(chain_id, update, block) {
                log::error!(
                    target: "ethereum-beacon-client",
                    "💫 Header update failed with error {:?}",
                    err
                );
                return Err(err);
            }

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Stored execution header {} at beacon slot {}.",
                block_hash,
                slot
            );

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::unblock_bridge())]
        #[transactional]
        pub fn unblock_bridge(origin: OriginFor<T>, chain_id: EVMChainId) -> DispatchResult {
            let _sender = ensure_root(origin)?;

            <Blocked<T>>::set(chain_id, false);

            log::info!(target: "ethereum-beacon-client","💫 syncing bridge from governance provided checkpoint.");

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_initial_sync(
            chain_id: EVMChainId,
            initial_sync: LightClientBootstrap<T::EthSpec>,
            validators_root: H256,
        ) -> DispatchResult {
            // initial_sync
            Self::verify_sync_committee(
                &initial_sync.current_sync_committee,
                initial_sync.current_sync_committee_branch.into(),
                initial_sync.header.state_root,
                CURRENT_SYNC_COMMITTEE_PROOF_LEN,
                CURRENT_SYNC_COMMITTEE_INDEX,
            )?;

            let period = Self::compute_current_sync_period(initial_sync.header.slot)?;

            let block_root = initial_sync.header.tree_hash_root();

            Self::store_sync_committee(chain_id, period, initial_sync.current_sync_committee);
            Self::store_validators_root(chain_id, validators_root);
            Self::store_finalized_header(chain_id, block_root, initial_sync.header)?;
            Ok(())
        }

        fn process_sync_committee_period_update(
            chain_id: EVMChainId,
            update: LightClientUpdate<T::EthSpec>,
        ) -> DispatchResult {
            ensure!(
                update.signature_slot > update.attested_header.slot
                    && update.attested_header.slot >= update.finalized_header.slot,
                Error::<T>::InvalidSyncCommitteeHeaderUpdate
            );
            Self::sync_committee_participation_is_supermajority(&update.sync_aggregate)?;
            Self::verify_sync_committee(
                &update.next_sync_committee,
                update.next_sync_committee_branch.into(),
                update.attested_header.state_root,
                NEXT_SYNC_COMMITTEE_PROOF_LEN,
                NEXT_SYNC_COMMITTEE_INDEX,
            )?;

            let block_root = update.finalized_header.tree_hash_root();
            Self::verify_header(
                block_root,
                update.finality_branch.into(),
                update.attested_header.state_root,
                FINALIZED_ROOT_PROOF_LEN,
                FINALIZED_ROOT_INDEX,
            )?;

            let current_period = Self::compute_current_sync_period(update.attested_header.slot)?;
            let latest_committee_period = <LatestSyncCommitteePeriod<T>>::get(chain_id)
                .ok_or(Error::<T>::NetworkNotInitialized)?;
            ensure!(
                <SyncCommittees<T>>::contains_key(chain_id, current_period),
                Error::<T>::SyncCommitteeMissing
            );
            let next_period = current_period + 1;
            ensure!(
                !<SyncCommittees<T>>::contains_key(chain_id, next_period),
                Error::<T>::InvalidSyncCommitteePeriodUpdateWithDuplication
            );
            ensure!(
                (next_period == latest_committee_period + 1),
                Error::<T>::InvalidSyncCommitteePeriodUpdateWithGap
            );

            let current_sync_committee =
                Self::get_sync_committee_for_period(chain_id, current_period)?;
            let validators_root =
                <ValidatorsRoot<T>>::get(chain_id).ok_or(Error::<T>::NetworkNotInitialized)?;

            Self::verify_signed_header(
                chain_id,
                update.sync_aggregate,
                current_sync_committee,
                update.attested_header,
                validators_root,
                update.signature_slot,
            )?;

            Self::store_sync_committee(chain_id, next_period, update.next_sync_committee);
            Self::store_finalized_header(chain_id, block_root, update.finalized_header)?;

            Ok(())
        }

        fn process_finalized_header(
            chain_id: EVMChainId,
            update: LightClientFinalityUpdate<T::EthSpec>,
        ) -> DispatchResult {
            let last_finalized_header = <LatestFinalizedHeaderState<T>>::get(chain_id)
                .ok_or(Error::<T>::NetworkNotInitialized)?;
            ensure!(
                update.signature_slot > update.attested_header.slot
                    && update.attested_header.slot >= update.finalized_header.slot
                    && update.finalized_header.slot > last_finalized_header.beacon_slot,
                Error::<T>::InvalidFinalizedHeaderUpdate
            );
            let import_time = last_finalized_header.import_time;
            let weak_subjectivity_period_check =
                import_time + T::WeakSubjectivityPeriodSeconds::get();
            let time: u64 = T::TimeProvider::now().as_secs();

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Checking weak subjectivity period. Current time is :{:?} Weak subjectivity period check: {:?}.",
                time,
                weak_subjectivity_period_check
            );

            if time > weak_subjectivity_period_check {
                log::info!(target: "ethereum-beacon-client","💫 Weak subjectivity period exceeded, blocking bridge.",);
                <Blocked<T>>::insert(chain_id, true);
                return Err(Error::<T>::BridgeBlocked.into());
            }

            Self::sync_committee_participation_is_supermajority(&update.sync_aggregate)?;

            let block_root = update.finalized_header.tree_hash_root();

            Self::verify_header(
                block_root,
                update.finality_branch.into(),
                update.attested_header.state_root,
                FINALIZED_ROOT_PROOF_LEN,
                FINALIZED_ROOT_INDEX,
            )?;

            let last_finalized_period =
                Self::compute_current_sync_period(last_finalized_header.beacon_slot)?;
            let current_period = Self::compute_current_sync_period(update.attested_header.slot)?;
            ensure!(
                (current_period == last_finalized_period
                    || current_period == last_finalized_period + 1),
                Error::<T>::InvalidFinalizedPeriodUpdate
            );
            let sync_committee = Self::get_sync_committee_for_period(chain_id, current_period)?;

            let validators_root =
                <ValidatorsRoot<T>>::get(chain_id).ok_or(Error::<T>::NetworkNotInitialized)?;
            Self::verify_signed_header(
                chain_id,
                update.sync_aggregate,
                sync_committee,
                update.attested_header,
                validators_root,
                update.signature_slot,
            )?;

            Self::store_finalized_header(chain_id, block_root, update.finalized_header)?;

            Ok(())
        }

        fn body_tree_root_hash(block: &BlindedBeaconBlock<T::EthSpec>) -> H256 {
            match block {
                BlindedBeaconBlock::Base(block) => block.body.tree_hash_root(),
                BlindedBeaconBlock::Altair(block) => block.body.tree_hash_root(),
                BlindedBeaconBlock::Merge(block) => block.body.tree_hash_root(),
                BlindedBeaconBlock::Capella(block) => block.body.tree_hash_root(),
            }
        }

        fn process_header(
            chain_id: EVMChainId,
            update: LightClientOptimisticUpdate<T::EthSpec>,
            block: BlindedBeaconBlock<T::EthSpec>,
        ) -> DispatchResult {
            let last_finalized_header = <LatestFinalizedHeaderState<T>>::get(chain_id)
                .ok_or(Error::<T>::NetworkNotInitialized)?;
            let latest_finalized_header_slot = last_finalized_header.beacon_slot;
            let block_slot = update.attested_header.slot;
            ensure!(
                block_slot <= latest_finalized_header_slot,
                Error::<T>::HeaderNotFinalized
            );

            let execution_header_state = <LatestExecutionHeaderState<T>>::get(chain_id)
                .ok_or(Error::<T>::NetworkNotInitialized)?;
            let execution_payload = block
                .body()
                .execution_payload()
                .map_err(|_| Error::<T>::ArithError)?
                .to_execution_payload_header();
            ensure!(
                execution_payload.block_number() > execution_header_state.block_number,
                Error::<T>::InvalidExecutionHeaderUpdate
            );
            let body_root_hash = Self::body_tree_root_hash(&block);

            ensure!(
                body_root_hash == update.attested_header.body_root,
                Error::<T>::WrongBlockBodyHashTreeRoot
            );

            let current_period = Self::compute_current_sync_period(update.attested_header.slot)?;
            let sync_committee = Self::get_sync_committee_for_period(chain_id, current_period)?;

            let validators_root =
                <ValidatorsRoot<T>>::get(chain_id).ok_or(Error::<T>::NetworkNotInitialized)?;
            Self::verify_signed_header(
                chain_id,
                update.sync_aggregate,
                sync_committee,
                update.attested_header,
                validators_root,
                update.signature_slot,
            )?;

            Self::store_execution_header(chain_id, execution_payload, block_slot, body_root_hash);

            Ok(())
        }

        fn check_bridge_blocked_state(chain_id: EVMChainId) -> DispatchResult {
            if <Blocked<T>>::get(chain_id) {
                return Err(Error::<T>::BridgeBlocked.into());
            }

            Ok(())
        }

        fn check_network_config(chain_id: EVMChainId) -> DispatchResult {
            let network_config =
                NetworkConfigs::<T>::get(chain_id).ok_or(Error::<T>::NetworkNotInitialized)?;
            match network_config.consensus() {
                ethereum_primitives::network_config::Consensus::Beacon(_config) => Ok(()),
                _ => Err(Error::<T>::WrongConsensus.into()),
            }
        }

        pub(super) fn verify_signed_header(
            chain_id: EVMChainId,
            sync_aggregate: SyncAggregate<T::EthSpec>,
            sync_committee: SyncCommittee<T::EthSpec>,
            header: BeaconBlockHeader,
            validators_root: H256,
            signature_slot: Slot,
        ) -> DispatchResult {
            let mut participant_pubkeys: Vec<bls::PublicKey> = Vec::new();
            // Gathers all the pubkeys of the sync committee members that participated in signing
            // the header.
            for (bit, pubkey) in sync_aggregate
                .sync_committee_bits
                .iter()
                .zip(sync_committee.pubkeys.iter())
            {
                if bit {
                    participant_pubkeys.push(
                        pubkey
                            .decompress()
                            .map_err(|_| Error::<T>::InvalidPublicKeyBytes)?,
                    );
                }
            }

            let fork = Self::compute_fork_version(
                chain_id,
                signature_slot.epoch_with_spec::<T::EthSpec>(),
            )?;
            // Domains are used for for seeds, for signatures, and for selecting aggregators.
            let domain = ChainSpec::compute_domain_with_constant(
                DOMAIN_SYNC_COMMITTEE,
                fork,
                validators_root,
            );
            // Hash tree root of SigningData - object root + domain
            let signing_root = Self::compute_signing_root(header, domain)?;

            // Verify sync committee aggregate signature.
            if !sync_aggregate
                .sync_committee_signature
                .fast_aggregate_verify(
                    signing_root,
                    &participant_pubkeys.iter().collect::<Vec<_>>(),
                )
            {
                return Err(Error::<T>::SignatureVerificationFailed.into());
            }
            Ok(())
        }

        pub(super) fn compute_signing_root(
            beacon_header: BeaconBlockHeader,
            domain: Hash256,
        ) -> Result<Hash256, DispatchError> {
            let beacon_header_root = beacon_header.tree_hash_root();
            let hash_root = beacon::SigningData {
                domain,
                object_root: beacon_header_root,
            }
            .tree_hash_root();
            Ok(hash_root)
        }

        fn verify_sync_committee(
            sync_committee: &SyncCommittee<T::EthSpec>,
            sync_committee_branch: Vec<H256>,
            header_state_root: H256,
            depth: usize,
            index: usize,
        ) -> DispatchResult {
            let sync_committee_root = sync_committee.tree_hash_root();

            ensure!(
                Self::is_valid_merkle_branch(
                    sync_committee_root,
                    sync_committee_branch,
                    depth,
                    index,
                    header_state_root
                ),
                Error::<T>::InvalidSyncCommitteeMerkleProof
            );

            Ok(())
        }

        fn verify_header(
            block_root: H256,
            proof_branch: Vec<H256>,
            attested_header_state_root: H256,
            depth: usize,
            index: usize,
        ) -> DispatchResult {
            ensure!(
                Self::is_valid_merkle_branch(
                    block_root,
                    proof_branch,
                    depth,
                    index,
                    attested_header_state_root
                ),
                Error::<T>::InvalidHeaderMerkleProof
            );

            Ok(())
        }

        fn store_sync_committee(
            chain_id: EVMChainId,
            period: u64,
            sync_committee: SyncCommittee<T::EthSpec>,
        ) {
            <SyncCommittees<T>>::insert(chain_id, period, sync_committee);

            log::trace!(
                target: "ethereum-beacon-client",
                "💫 Updated latest sync committee period stored to {}.",
                period
            );

            <LatestSyncCommitteePeriod<T>>::insert(chain_id, period);

            Self::deposit_event(Event::SyncCommitteeUpdated { period });
        }

        fn store_finalized_header(
            chain_id: EVMChainId,
            block_root: Hash256,
            header: BeaconBlockHeader,
        ) -> DispatchResult {
            let slot = header.slot;

            <FinalizedBeaconHeaders<T>>::insert(chain_id, block_root, header);
            Self::add_finalized_header_slot(chain_id, slot)?;

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Updated latest finalized block root {} at slot {}.",
                block_root,
                slot
            );

            LatestFinalizedHeaderState::<T>::insert(
                chain_id,
                FinalizedHeaderState {
                    import_time: T::TimeProvider::now().as_secs(),
                    beacon_block_root: block_root,
                    beacon_slot: slot,
                },
            );

            Self::deposit_event(Event::BeaconHeaderImported {
                block_hash: block_root,
                slot,
            });

            Ok(())
        }

        fn add_finalized_header_slot(chain_id: EVMChainId, slot: Slot) -> DispatchResult {
            <FinalizedBeaconHeaderSlots<T>>::try_mutate(chain_id, |b_vec| {
                let b_vec = b_vec.get_or_insert(Default::default());
                if b_vec.len() as u32 == T::MaxFinalizedHeaderSlotArray::get() {
                    b_vec.remove(0);
                }
                b_vec.try_push(slot)
            })
            .map_err(|_| <Error<T>>::FinalizedBeaconHeaderSlotsExceeded)?;

            Ok(())
        }

        fn store_execution_header(
            chain_id: EVMChainId,
            header: ExecutionPayloadHeader<T::EthSpec>,
            beacon_slot: Slot,
            beacon_block_root: H256,
        ) {
            let block_number = header.block_number();
            let block_hash = header.block_hash().into_root();

            <ExecutionHeaders<T>>::insert(chain_id, block_hash, header);

            log::trace!(
                target: "ethereum-beacon-client",
                "💫 Updated latest execution block at {} to number {}.",
                block_hash,
                block_number
            );

            LatestExecutionHeaderState::<T>::insert(
                chain_id,
                ExecutionHeaderState {
                    beacon_block_root,
                    beacon_slot,
                    block_hash,
                    block_number,
                },
            );

            Self::deposit_event(Event::ExecutionHeaderImported {
                block_hash,
                block_number,
            });
        }

        fn store_validators_root(chain_id: EVMChainId, validators_root: H256) {
            <ValidatorsRoot<T>>::insert(chain_id, validators_root);
        }

        pub(super) fn compute_current_sync_period(slot: Slot) -> Result<u64, Error<T>> {
            let period = slot
                .epoch_with_spec::<T::EthSpec>()
                .sync_committee_period_with_spec::<T::EthSpec>()
                .map_err(|_| Error::<T>::ArithError)?;
            Ok(period)
        }

        pub(super) fn is_valid_merkle_branch(
            leaf: H256,
            branch: Vec<H256>,
            depth: usize,
            index: usize,
            root: H256,
        ) -> bool {
            if branch.len() != depth {
                log::error!(target: "ethereum-beacon-client", "Merkle proof branch length doesn't match depth.");

                return false;
            }
            let mut value = leaf;
            if leaf.as_bytes().len() < 32 as usize {
                log::error!(target: "ethereum-beacon-client", "Merkle proof leaf not 32 bytes.");

                return false;
            }
            for i in 0..depth {
                if branch[i as usize].as_bytes().len() < 32 as usize {
                    log::error!(target: "ethereum-beacon-client", "Merkle proof branch not 32 bytes.");

                    return false;
                }
                if (index / 2usize.pow(i as u32) % 2) == 0 {
                    // left node
                    let mut data = [0u8; 64];
                    data[0..32].copy_from_slice(&(value.0));
                    data[32..64].copy_from_slice(&(branch[i as usize].0));
                    value = sha2_256(&data).into();
                } else {
                    let mut data = [0u8; 64]; // right node
                    data[0..32].copy_from_slice(&(branch[i as usize].0));
                    data[32..64].copy_from_slice(&(value.0));
                    value = sha2_256(&data).into();
                }
            }

            return value == root;
        }

        pub(super) fn sync_committee_participation_is_supermajority(
            sync_agg: &SyncAggregate<T::EthSpec>,
        ) -> DispatchResult {
            let sync_committee_sum = sync_agg.sync_committee_bits.num_set_bits();
            let sync_committee_len = sync_agg.sync_committee_bits.len();
            ensure!(
                (sync_committee_sum * 3 >= sync_committee_len * 2),
                Error::<T>::SyncCommitteeParticipantsNotSupermajority
            );

            Ok(())
        }

        pub(super) fn get_sync_committee_for_period(
            chain_id: EVMChainId,
            period: u64,
        ) -> Result<SyncCommittee<T::EthSpec>, DispatchError> {
            let sync_committee = <SyncCommittees<T>>::get(chain_id, period);

            if let Some(sync_committee) = sync_committee {
                Ok(sync_committee)
            } else {
                log::error!(target: "ethereum-beacon-client", "💫 Sync committee for period {} missing", period);
                return Err(Error::<T>::SyncCommitteeMissing.into());
            }
        }

        pub(super) fn compute_fork_version(
            chain_id: EVMChainId,
            epoch: Epoch,
        ) -> Result<ForkVersion, DispatchError> {
            let network_config =
                NetworkConfigs::<T>::get(chain_id).ok_or(Error::<T>::NetworkNotInitialized)?;
            match network_config.consensus() {
                ethereum_primitives::network_config::Consensus::Beacon(config) => {
                    Ok(config.fork_version_from_epoch(epoch.as_u64()))
                }
                _ => Err(Error::<T>::WrongConsensus.into()),
            }
        }

        pub(super) fn initial_sync(
            chain_id: EVMChainId,
            initial_sync: LightClientBootstrap<T::EthSpec>,
            validators_root: H256,
        ) -> DispatchResult {
            log::info!(
                target: "ethereum-beacon-client",
                "💫 Received initial sync, starting processing.",
            );

            if let Err(err) = Self::process_initial_sync(chain_id, initial_sync, validators_root) {
                log::error!(
                    target: "ethereum-beacon-client",
                    "Initial sync failed with error {:?}",
                    err
                );
                return Err(err);
            }

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Initial sync processing succeeded.",
            );

            Ok(())
        }

        // Verifies that the receipt encoded in proof.data is included
        // in the block given by proof.block_hash. Inclusion is only
        // recognized if the block has been finalized.
        fn verify_receipt_inclusion(
            stored_header: ExecutionPayloadHeader<T::EthSpec>,
            proof: &Proof,
        ) -> Result<Receipt, DispatchError> {
            let result = ethereum_primitives::Header::check_receipt_proof_with_root(
                stored_header.receipts_root(),
                &proof.data,
            )
            .ok_or(Error::<T>::InvalidProof)?;

            match result {
                Ok(receipt) => Ok(receipt),
                Err(err) => {
                    log::trace!(
                        target: "ethereum-beacon-client",
                        "💫 Failed to decode transaction receipt: {}",
                        err
                    );
                    Err(Error::<T>::InvalidProof.into())
                }
            }
        }
    }

    impl<T: Config> Verifier<EVMChainId, Message> for Pallet<T> {
        type Result = (Log, u64);
        /// Verify a message by verifying the existence of the corresponding
        /// Ethereum log in a block. Returns the log if successful.
        fn verify(chain_id: EVMChainId, message: &Message) -> Result<(Log, u64), DispatchError> {
            log::info!(
                target: "ethereum-beacon-client",
                "💫 Verifying message with block hash {}",
                message.proof.block_hash,
            );

            let stored_header = <ExecutionHeaders<T>>::get(chain_id, message.proof.block_hash)
                .ok_or(Error::<T>::MissingHeader)?;

            let block_number = stored_header.block_number();

            let receipt = match Self::verify_receipt_inclusion(stored_header, &message.proof) {
                Ok(receipt) => receipt,
                Err(err) => {
                    log::error!(
                        target: "ethereum-beacon-client",
                        "💫 Verify receipt inclusion failed for block {}: {:?}",
                        message.proof.block_hash,
                        err
                    );
                    return Err(err);
                }
            };

            log::trace!(
                target: "ethereum-beacon-client",
                "💫 Verified receipt inclusion for transaction at index {} in block {}",
                message.proof.tx_index, message.proof.block_hash,
            );

            let log = match rlp::decode(&message.data) {
                Ok(log) => log,
                Err(err) => {
                    log::error!(
                        target: "ethereum-beacon-client",
                        "💫 RLP log decoded failed {}: {:?}",
                        message.proof.block_hash,
                        err
                    );
                    return Err(Error::<T>::DecodeFailed.into());
                }
            };

            if !receipt.contains_log(&log) {
                log::error!(
                    target: "ethereum-beacon-client",
                    "💫 Event log not found in receipt for transaction at index {} in block {}",
                    message.proof.tx_index, message.proof.block_hash,
                );
                return Err(Error::<T>::InvalidProof.into());
            }

            log::info!(
                target: "ethereum-beacon-client",
                "💫 Receipt verification successful for {}",
                message.proof.block_hash,
            );

            Ok((log, block_number))
        }
    }
}
