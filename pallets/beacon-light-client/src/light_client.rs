use core::cell::RefCell;
use core::marker::PhantomData;

use beacon::{
    light_client_bootstrap::LightClientBootstrap,
    light_client_header::LightClientHeaderRef,
    light_client_update::{
        LightClientUpdate, CURRENT_SYNC_COMMITTEE_INDEX, CURRENT_SYNC_COMMITTEE_PROOF_LEN,
        EXECUTION_PAYLOAD_INDEX, EXECUTION_PAYLOAD_PROOF_LEN, FINALIZED_ROOT_INDEX,
        FINALIZED_ROOT_PROOF_LEN, NEXT_SYNC_COMMITTEE_INDEX, NEXT_SYNC_COMMITTEE_PROOF_LEN,
    },
    BeaconBlockHeader, Epoch, EthSpec, ForkVersion, LightClientHeader, Slot, SyncAggregate,
    SyncCommittee,
};
use beacon::{ForkSchedule, Unsigned};
use bridge_types::H256;
use frame_support::log;
use sp_io::hashing::sha2_256;
use tree_hash::TreeHash;

pub const DOMAIN_SYNC_COMMITTEE: u32 = 7;
pub const MIN_SYNC_COMMITTEE_PARTICIPANTS: usize = 1;

pub enum Error {
    InvalidMerkleBranch,
    ArithError,
    ZeroParticipants,
    NotEnoughParticipants,
    InvalidUpdate,
    InvalidPublicKeyBytes,
    SignatureVerificationFailed,
    DuplicateSyncCommitteeUpdate,
    InvalidSpecId,
    StoreNotInitialized,
}

impl From<beacon::safe_arith::ArithError> for Error {
    fn from(_value: beacon::safe_arith::ArithError) -> Self {
        Error::ArithError
    }
}

pub struct MemoryLightClientStore<E: EthSpec> {
    pub current_sync_committee: RefCell<Option<SyncCommittee<E>>>,
    pub next_sync_committee: RefCell<Option<SyncCommittee<E>>>,
    pub finalized_header: RefCell<Option<LightClientHeader<E>>>,
    pub optimistic_header: RefCell<Option<LightClientHeader<E>>>,
}

impl<E: EthSpec> LightClientStore<E> for MemoryLightClientStore<E> {
    fn get_current_sync_committee(&self) -> Result<SyncCommittee<E>, Error> {
        self.current_sync_committee
            .borrow()
            .as_ref()
            .map(|x| x.clone())
            .ok_or(Error::StoreNotInitialized)
    }

    fn set_current_sync_committee(&self, sync_committee: SyncCommittee<E>) -> Result<(), Error> {
        log::debug!("Current sync committee updated");
        let mut value = self.current_sync_committee.borrow_mut();
        *value = Some(sync_committee);
        Ok(())
    }

    fn get_next_sync_committee(&self) -> Result<Option<SyncCommittee<E>>, Error> {
        Ok(self
            .next_sync_committee
            .borrow()
            .as_ref()
            .map(|x| x.clone()))
    }

    fn set_next_sync_committee(
        &self,
        sync_committee: Option<SyncCommittee<E>>,
    ) -> Result<(), Error> {
        log::debug!("Next sync committee updated");
        let mut value = self.next_sync_committee.borrow_mut();
        *value = sync_committee;
        Ok(())
    }

    fn get_finalized_header(&self) -> Result<LightClientHeader<E>, Error> {
        self.finalized_header
            .borrow()
            .as_ref()
            .map(|x| x.clone())
            .ok_or(Error::StoreNotInitialized)
    }

    fn set_finalized_header(&self, header: LightClientHeader<E>) -> Result<(), Error> {
        log::debug!("Finalized header updated");
        let mut value = self.finalized_header.borrow_mut();
        *value = Some(header);
        Ok(())
    }

    fn get_optimistic_header(&self) -> Result<LightClientHeader<E>, Error> {
        self.optimistic_header
            .borrow()
            .as_ref()
            .map(|x| x.clone())
            .ok_or(Error::StoreNotInitialized)
    }

    fn set_optimistic_header(&self, header: LightClientHeader<E>) -> Result<(), Error> {
        log::debug!("Optimistic header updated");
        let mut value = self.optimistic_header.borrow_mut();
        *value = Some(header);
        Ok(())
    }
}

pub trait LightClientStore<E: EthSpec> {
    fn get_current_sync_committee(&self) -> Result<SyncCommittee<E>, Error>;
    fn set_current_sync_committee(&self, sync_committee: SyncCommittee<E>) -> Result<(), Error>;
    fn get_next_sync_committee(&self) -> Result<Option<SyncCommittee<E>>, Error>;
    fn set_next_sync_committee(
        &self,
        sync_committee: Option<SyncCommittee<E>>,
    ) -> Result<(), Error>;
    fn get_finalized_header(&self) -> Result<LightClientHeader<E>, Error>;
    fn set_finalized_header(&self, header: LightClientHeader<E>) -> Result<(), Error>;
    fn get_optimistic_header(&self) -> Result<LightClientHeader<E>, Error>;
    fn set_optimistic_header(&self, header: LightClientHeader<E>) -> Result<(), Error>;
}

pub struct BeaconLightClient<E, S> {
    fork_schedule: ForkSchedule,
    validators_root: H256,
    store: S,
    _phantom: PhantomData<E>,
}

impl<E, S> BeaconLightClient<E, S>
where
    E: EthSpec,
    S: LightClientStore<E>,
{
    pub fn new(fork_schedule: ForkSchedule, validators_root: H256, store: S) -> Self {
        Self {
            fork_schedule,
            validators_root,
            store,
            _phantom: Default::default(),
        }
    }

    pub fn initialize(&self, bootstrap: LightClientBootstrap<E>) -> Result<(), Error> {
        self.is_valid_header(bootstrap.header())?;
        Self::is_valid_merkle_branch(
            bootstrap.current_sync_committee().tree_hash_root(),
            bootstrap.current_sync_committee_branch(),
            CURRENT_SYNC_COMMITTEE_PROOF_LEN,
            CURRENT_SYNC_COMMITTEE_INDEX,
            bootstrap.header().beacon().state_root,
        )?;
        self.store
            .set_current_sync_committee(bootstrap.current_sync_committee().clone())?;
        self.store
            .set_finalized_header(bootstrap.header().owned())?;
        self.store
            .set_optimistic_header(bootstrap.header().owned())?;
        Ok(())
    }

    pub fn import_update(&self, update: LightClientUpdate<E>) -> Result<(), Error> {
        self.validate_update(&update)?;
        let store_next_sync_committee = self.store.get_next_sync_committee()?;
        let sync_committee_bits = update.sync_aggregate().sync_committee_bits.num_set_bits();
        let sync_committee_size = E::SyncCommitteeSize::to_usize();

        let store_optimistic_slot = self.store.get_optimistic_header()?.beacon().slot;
        let update_attested_slot = update.attested_header().beacon().slot;
        let update_attested_period = update_attested_slot.sync_committee_period_with_spec::<E>();
        let update_finalized_slot = if let Some(header) = update.finalized_header() {
            header.beacon().slot
        } else {
            Default::default()
        };
        let update_finalized_period = update_finalized_slot.sync_committee_period_with_spec::<E>();
        let store_finalized_slot = self.store.get_finalized_header()?.beacon().slot;
        let store_finalized_period = store_finalized_slot.sync_committee_period_with_spec::<E>();

        // Update optimistic header
        // Requires at least 1/3 signatures
        if sync_committee_bits * 3 >= sync_committee_size
            && update_attested_slot > store_optimistic_slot
        {
            self.store
                .set_optimistic_header(update.attested_header().owned())?;
        }

        let update_has_finalized_next_sync_committee = store_next_sync_committee.is_none()
            && update.is_sync_committee_update()
            && update.is_finality_update()
            && update_attested_period == update_finalized_period;

        if sync_committee_bits * 3 >= sync_committee_size * 2
            && (update_finalized_slot > store_finalized_slot
                || update_has_finalized_next_sync_committee)
        {
            if let Some(next_sync_committee) = store_next_sync_committee {
                if update_finalized_period == store_finalized_period + 1 {
                    self.store.set_current_sync_committee(next_sync_committee)?;
                    self.store
                        .set_next_sync_committee(update.next_sync_committee().clone())?;
                }
            } else {
                if update_finalized_period != store_finalized_period {
                    return Err(Error::InvalidUpdate);
                }
                self.store
                    .set_next_sync_committee(update.next_sync_committee().clone())?;
            }
            if update_finalized_slot > store_finalized_slot {
                if let Some(finalized_header) = update.finalized_header() {
                    self.store.set_finalized_header(finalized_header.owned())?;
                }
            }
        }

        Ok(())
    }

    pub fn validate_update(&self, update: &LightClientUpdate<E>) -> Result<(), Error> {
        if update.sync_aggregate().sync_committee_bits.num_set_bits()
            < MIN_SYNC_COMMITTEE_PARTICIPANTS
        {
            return Err(Error::ZeroParticipants);
        }

        self.is_valid_header(update.attested_header())?;

        let update_finalized_slot = if let Some(header) = update.finalized_header() {
            header.beacon().slot
        } else {
            Default::default()
        };
        let update_attested_slot = update.attested_header().beacon().slot;

        // Sanity check
        if update.signature_slot() <= update_attested_slot
            || update_attested_slot < update_finalized_slot
        {
            return Err(Error::InvalidUpdate);
        }

        let store_finalized_slot = self.store.get_finalized_header()?.beacon().slot;
        let store_period = store_finalized_slot.sync_committee_period_with_spec::<E>();
        let update_signature_period = update
            .signature_slot()
            .sync_committee_period_with_spec::<E>();
        let next_sync_committee = self.store.get_next_sync_committee()?;
        if next_sync_committee.is_some() {
            if ![store_period, store_period + 1].contains(&update_signature_period) {
                return Err(Error::InvalidUpdate);
            }
        } else {
            if store_period != update_signature_period {
                return Err(Error::InvalidUpdate);
            }
        }

        let update_attested_period = update
            .attested_header()
            .beacon()
            .slot
            .sync_committee_period_with_spec::<E>();
        let update_has_next_sync_committee = next_sync_committee.is_none()
            && update.next_sync_committee().is_some()
            && update.next_sync_committee_branch().is_some()
            && update_attested_period == store_period;

        if !update_has_next_sync_committee && update_attested_slot <= store_finalized_slot {
            return Err(Error::InvalidUpdate);
        }

        match (update.finalized_header(), update.finality_branch()) {
            (Some(header), Some(branch)) => {
                self.is_valid_header(header)?;
                Self::is_valid_merkle_branch(
                    header.beacon().tree_hash_root(),
                    branch,
                    FINALIZED_ROOT_PROOF_LEN,
                    FINALIZED_ROOT_INDEX,
                    update.attested_header().beacon().state_root,
                )?;
            }
            (None, None) => {}
            _ => return Err(Error::InvalidUpdate),
        }

        match (
            update.next_sync_committee(),
            update.next_sync_committee_branch(),
        ) {
            (Some(sync_committee), Some(branch)) => {
                if let Some(next_sync_committee) = &next_sync_committee {
                    if update_attested_period == store_period
                        && next_sync_committee != sync_committee
                    {
                        return Err(Error::InvalidUpdate);
                    }
                }
                Self::is_valid_merkle_branch(
                    sync_committee.tree_hash_root(),
                    branch,
                    NEXT_SYNC_COMMITTEE_PROOF_LEN,
                    NEXT_SYNC_COMMITTEE_INDEX,
                    update.attested_header().beacon().state_root,
                )?;
            }
            (None, None) => {}
            _ => return Err(Error::InvalidUpdate),
        }

        // Verify sync committee aggregate signature
        let sync_committee = if update_signature_period == store_period {
            self.store.get_current_sync_committee()?
        } else {
            next_sync_committee.ok_or(Error::InvalidUpdate)?
        };
        self.validate_signed_header(
            update.sync_aggregate(),
            &sync_committee,
            update.attested_header().beacon(),
            update.signature_slot(),
        )?;

        Ok(())
    }

    pub fn validate_signed_header(
        &self,
        sync_aggregate: &SyncAggregate<E>,
        sync_committee: &SyncCommittee<E>,
        header: &BeaconBlockHeader,
        signature_slot: Slot,
    ) -> Result<(), Error> {
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
                        .map_err(|_| Error::InvalidPublicKeyBytes)?,
                );
            }
        }

        let fork =
            self.compute_fork_version((signature_slot.saturating_sub(1u64)).epoch_with_spec::<E>());

        // Domains are used for for seeds, for signatures, and for selecting aggregators.
        let domain =
            Self::compute_domain_with_constant(DOMAIN_SYNC_COMMITTEE, fork, self.validators_root);
        // Hash tree root of SigningData - object root + domain
        let signing_root = Self::compute_signing_root(header, domain);

        // Verify sync committee aggregate signature.
        if !sync_aggregate
            .sync_committee_signature
            .fast_aggregate_verify(
                signing_root,
                // TODO: Optimize
                &participant_pubkeys.iter().collect::<Vec<_>>(),
            )
        {
            return Err(Error::SignatureVerificationFailed.into());
        }
        Ok(())
    }

    pub fn is_valid_header(&self, header: LightClientHeaderRef<E>) -> Result<(), Error> {
        if let Ok(execution) = header.execution() {
            Self::is_valid_merkle_branch(
                execution.tree_hash_root(),
                header
                    .execution_branch()
                    .expect("execution branch exists if execution payload exists"),
                EXECUTION_PAYLOAD_PROOF_LEN,
                EXECUTION_PAYLOAD_INDEX,
                header.beacon().body_root,
            )?;
        }
        // Execution branch is not presented before Capella fork
        Ok(())
    }

    pub fn is_valid_merkle_branch(
        leaf: H256,
        branch: &[H256],
        depth: usize,
        index: usize,
        root: H256,
    ) -> Result<(), Error> {
        if branch.len() != depth {
            log::error!(target: "ethereum-beacon-client", "Merkle proof branch length doesn't match depth.");

            return Err(Error::InvalidMerkleBranch);
        }
        let mut value = leaf;
        for (i, item) in branch.iter().enumerate() {
            if (index / 2usize.pow(i as u32) % 2) == 0 {
                // left node
                let mut data = [0u8; 64];
                data[0..32].copy_from_slice(&(value.0));
                data[32..64].copy_from_slice(&(item.0));
                value = sha2_256(&data).into();
            } else {
                // right node
                let mut data = [0u8; 64];
                data[0..32].copy_from_slice(&(item.0));
                data[32..64].copy_from_slice(&(value.0));
                value = sha2_256(&data).into();
            }
        }

        if value != root {
            return Err(Error::InvalidMerkleBranch);
        }
        Ok(())
    }

    pub fn compute_signing_root(beacon_header: &BeaconBlockHeader, domain: H256) -> H256 {
        let beacon_header_root = beacon_header.tree_hash_root();
        let hash_root = beacon::SigningData {
            domain,
            object_root: beacon_header_root,
        }
        .tree_hash_root();
        hash_root
    }

    pub fn compute_fork_version(&self, epoch: Epoch) -> ForkVersion {
        self.fork_schedule.fork_version(epoch.as_u64())
    }

    /// Return the 32-byte fork data root for the `current_version` and `genesis_validators_root`.
    ///
    /// This is used primarily in signature domains to avoid collisions across forks/chains.
    ///
    /// Spec v0.12.1
    pub fn compute_fork_data_root(current_version: [u8; 4], genesis_validators_root: H256) -> H256 {
        beacon::ForkData {
            current_version,
            genesis_validators_root,
        }
        .tree_hash_root()
    }

    /// Compute a domain by applying the given `fork_version`.
    pub fn compute_domain_with_constant(
        domain_constant: u32,
        fork_version: [u8; 4],
        genesis_validators_root: H256,
    ) -> H256 {
        let mut domain = [0; 32];
        domain[0..4].copy_from_slice(&beacon::int_to_bytes::int_to_bytes4(domain_constant));
        domain[4..].copy_from_slice(
            Self::compute_fork_data_root(fork_version, genesis_validators_root)
                .as_bytes()
                .get(..28)
                .expect("fork has is 32 bytes so first 28 bytes should exist"),
        );

        H256::from(domain)
    }
}
