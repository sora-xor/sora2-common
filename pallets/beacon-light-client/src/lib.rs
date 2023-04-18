//! # Ethereum Beacon Client
#![cfg_attr(not(feature = "std"), no_std)]

pub mod light_client;
pub mod weights;

use core::marker::PhantomData;

use beacon::{
    light_client_bootstrap::LightClientBootstrap, light_client_update::LightClientUpdate, EthSpec,
    EthSpecId, GnosisEthSpec, LightClientHeader, MainnetEthSpec, MinimalEthSpec, SyncCommittee,
};
use bridge_types::EVMChainId;
use codec::{Decode, Encode, MaxEncodedLen};
use light_client::LightClientStore;
use scale_info::TypeInfo;
pub use weights::WeightInfo;

use frame_support::{dispatch::DispatchResult, traits::UnixTime, RuntimeDebug};
use frame_system::ensure_signed;
use sp_core::H256;
use sp_std::prelude::*;

#[derive(Decode, Encode, MaxEncodedLen, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub enum GenericBootstrap {
    Minimal(LightClientBootstrap<MinimalEthSpec>),
    Mainnet(LightClientBootstrap<MainnetEthSpec>),
    Gnosis(LightClientBootstrap<GnosisEthSpec>),
}

impl GenericBootstrap {
    pub fn spec_id(&self) -> EthSpecId {
        match self {
            Self::Minimal(_) => EthSpecId::Minimal,
            Self::Mainnet(_) => EthSpecId::Mainnet,
            Self::Gnosis(_) => EthSpecId::Gnosis,
        }
    }
}

#[derive(Decode, Encode, MaxEncodedLen, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub enum GenericUpdate {
    Minimal(LightClientUpdate<MinimalEthSpec>),
    Mainnet(LightClientUpdate<MainnetEthSpec>),
    Gnosis(LightClientUpdate<GnosisEthSpec>),
}

impl GenericUpdate {
    pub fn spec_id(&self) -> EthSpecId {
        match self {
            Self::Minimal(_) => EthSpecId::Minimal,
            Self::Mainnet(_) => EthSpecId::Mainnet,
            Self::Gnosis(_) => EthSpecId::Gnosis,
        }
    }
}

#[derive(Decode, Encode, MaxEncodedLen, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub enum GenericHeader {
    Minimal(LightClientHeader<MinimalEthSpec>),
    Mainnet(LightClientHeader<MainnetEthSpec>),
    Gnosis(LightClientHeader<GnosisEthSpec>),
}

impl GenericHeader {
    pub fn spec_id(&self) -> EthSpecId {
        match self {
            Self::Minimal(_) => EthSpecId::Minimal,
            Self::Mainnet(_) => EthSpecId::Mainnet,
            Self::Gnosis(_) => EthSpecId::Gnosis,
        }
    }
}

impl TryFrom<GenericHeader> for LightClientHeader<MinimalEthSpec> {
    type Error = ();
    fn try_from(value: GenericHeader) -> Result<Self, Self::Error> {
        match value {
            GenericHeader::Minimal(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl TryFrom<GenericHeader> for LightClientHeader<MainnetEthSpec> {
    type Error = ();
    fn try_from(value: GenericHeader) -> Result<Self, Self::Error> {
        match value {
            GenericHeader::Mainnet(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl TryFrom<GenericHeader> for LightClientHeader<GnosisEthSpec> {
    type Error = ();
    fn try_from(value: GenericHeader) -> Result<Self, Self::Error> {
        match value {
            GenericHeader::Gnosis(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl From<LightClientHeader<MinimalEthSpec>> for GenericHeader {
    fn from(value: LightClientHeader<MinimalEthSpec>) -> Self {
        Self::Minimal(value)
    }
}

impl From<LightClientHeader<MainnetEthSpec>> for GenericHeader {
    fn from(value: LightClientHeader<MainnetEthSpec>) -> Self {
        Self::Mainnet(value)
    }
}

impl From<LightClientHeader<GnosisEthSpec>> for GenericHeader {
    fn from(value: LightClientHeader<GnosisEthSpec>) -> Self {
        Self::Gnosis(value)
    }
}

#[derive(Decode, Encode, MaxEncodedLen, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub enum GenericSyncCommittee {
    Minimal(SyncCommittee<MinimalEthSpec>),
    Mainnet(SyncCommittee<MainnetEthSpec>),
    Gnosis(SyncCommittee<GnosisEthSpec>),
}

impl GenericSyncCommittee {
    pub fn spec_id(&self) -> EthSpecId {
        match self {
            Self::Minimal(_) => EthSpecId::Minimal,
            Self::Mainnet(_) => EthSpecId::Mainnet,
            Self::Gnosis(_) => EthSpecId::Gnosis,
        }
    }
}

impl TryFrom<GenericSyncCommittee> for SyncCommittee<MinimalEthSpec> {
    type Error = ();
    fn try_from(value: GenericSyncCommittee) -> Result<Self, Self::Error> {
        match value {
            GenericSyncCommittee::Minimal(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl TryFrom<GenericSyncCommittee> for SyncCommittee<MainnetEthSpec> {
    type Error = ();
    fn try_from(value: GenericSyncCommittee) -> Result<Self, Self::Error> {
        match value {
            GenericSyncCommittee::Mainnet(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl TryFrom<GenericSyncCommittee> for SyncCommittee<GnosisEthSpec> {
    type Error = ();
    fn try_from(value: GenericSyncCommittee) -> Result<Self, Self::Error> {
        match value {
            GenericSyncCommittee::Gnosis(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl From<SyncCommittee<MinimalEthSpec>> for GenericSyncCommittee {
    fn from(value: SyncCommittee<MinimalEthSpec>) -> Self {
        Self::Minimal(value)
    }
}

impl From<SyncCommittee<MainnetEthSpec>> for GenericSyncCommittee {
    fn from(value: SyncCommittee<MainnetEthSpec>) -> Self {
        Self::Mainnet(value)
    }
}

impl From<SyncCommittee<GnosisEthSpec>> for GenericSyncCommittee {
    fn from(value: SyncCommittee<GnosisEthSpec>) -> Self {
        Self::Gnosis(value)
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use beacon::{ConsensusConfig, Slot};
    use frame_support::{
        pallet_prelude::{ValueQuery, *},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::DispatchError;

    use crate::light_client::BeaconLightClient;
    use bridge_types::network_config::NetworkConfig;
    use bridge_types::{network_config::Consensus, types::Message};
    use bridge_types::{traits::Verifier, EVMChainId};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TimeProvider: UnixTime;
        type WeightInfo: WeightInfo;
        type WeakSubjectivityPeriodSeconds: Get<u64>;
        type MaxFinalizedStateRootArray: Get<u32>;
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
        InvalidMerkleBranch,
        SignatureVerificationFailed,
        DuplicateSyncCommitteeUpdate,
        InvalidUpdate,
        ArithError,
        NetworkNotInitialized,
        InvalidPublicKeyBytes,
        InvalidConsensus,
        InvalidSpecId,
        NetworkAlreadyRegistered,
        ZeroParticipants,
        NotEnoughParticipants,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::storage]
    pub(super) type FinalizedLightClientHeader<T: Config> =
        StorageMap<_, Identity, EVMChainId, GenericHeader, OptionQuery>;

    #[pallet::storage]
    pub(super) type OptimisticLightClientHeader<T: Config> =
        StorageMap<_, Identity, EVMChainId, GenericHeader, OptionQuery>;

    #[pallet::storage]
    pub(super) type CurrentSyncCommittee<T: Config> =
        StorageMap<_, Identity, EVMChainId, GenericSyncCommittee, OptionQuery>;

    #[pallet::storage]
    pub(super) type NextSyncCommittee<T: Config> =
        StorageMap<_, Identity, EVMChainId, GenericSyncCommittee, OptionQuery>;

    #[pallet::storage]
    pub(super) type LatestFinalizedExecutionStateRoots<T: Config> = StorageMap<
        _,
        Identity,
        EVMChainId,
        BoundedVec<H256, T::MaxFinalizedStateRootArray>,
        ValueQuery,
    >;

    #[pallet::storage]
    pub(super) type NetworkConfigs<T: Config> =
        StorageMap<_, Identity, EVMChainId, NetworkConfig, OptionQuery>;

    #[pallet::storage]
    pub(super) type LatestSyncCommitteeUpdate<T: Config> =
        StorageMap<_, Identity, EVMChainId, u64, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::initialize())]
        pub fn initialize(
            origin: OriginFor<T>,
            chain_id: EVMChainId,
            network_config: NetworkConfig,
            bootstrap: GenericBootstrap,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(
                !CurrentSyncCommittee::<T>::contains_key(chain_id),
                Error::<T>::NetworkAlreadyRegistered
            );

            let consensus = match network_config.consensus() {
                Consensus::Beacon(consensus) => consensus,
                _ => return Err(Error::<T>::InvalidConsensus.into()),
            };

            ensure!(
                consensus.spec_id == bootstrap.spec_id(),
                Error::<T>::InvalidSpecId
            );

            match bootstrap {
                GenericBootstrap::Minimal(bootstrap) => BeaconLightClient::new(
                    consensus.fork_schedule,
                    consensus.genesis_validators_root,
                    PalletLightClientStore::<T, MinimalEthSpec>::new(chain_id),
                )
                .initialize(bootstrap)
                .map_err(Error::<T>::from)?,
                GenericBootstrap::Mainnet(bootstrap) => BeaconLightClient::new(
                    consensus.fork_schedule,
                    consensus.genesis_validators_root,
                    PalletLightClientStore::<T, MainnetEthSpec>::new(chain_id),
                )
                .initialize(bootstrap)
                .map_err(Error::<T>::from)?,
                GenericBootstrap::Gnosis(bootstrap) => BeaconLightClient::new(
                    consensus.fork_schedule,
                    consensus.genesis_validators_root,
                    PalletLightClientStore::<T, GnosisEthSpec>::new(chain_id),
                )
                .initialize(bootstrap)
                .map_err(Error::<T>::from)?,
            }

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::import_update())]
        pub fn import_update(
            origin: OriginFor<T>,
            chain_id: EVMChainId,
            update: GenericUpdate,
        ) -> DispatchResult {
            // TODO: Weak subjectivity check
            // TODO: State root update
            let _sender = ensure_signed(origin)?;

            let consensus = Pallet::<T>::consensus_config(chain_id)?;

            ensure!(
                consensus.spec_id == update.spec_id(),
                Error::<T>::InvalidSpecId
            );

            match update {
                GenericUpdate::Minimal(update) => BeaconLightClient::new(
                    consensus.fork_schedule,
                    consensus.genesis_validators_root,
                    PalletLightClientStore::<T, MinimalEthSpec>::new(chain_id),
                )
                .import_update(update)
                .map_err(Error::<T>::from)?,
                GenericUpdate::Mainnet(update) => BeaconLightClient::new(
                    consensus.fork_schedule,
                    consensus.genesis_validators_root,
                    PalletLightClientStore::<T, MainnetEthSpec>::new(chain_id),
                )
                .import_update(update)
                .map_err(Error::<T>::from)?,
                GenericUpdate::Gnosis(update) => BeaconLightClient::new(
                    consensus.fork_schedule,
                    consensus.genesis_validators_root,
                    PalletLightClientStore::<T, GnosisEthSpec>::new(chain_id),
                )
                .import_update(update)
                .map_err(Error::<T>::from)?,
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn consensus_config(chain_id: EVMChainId) -> Result<ConsensusConfig, DispatchError> {
            let network_config =
                NetworkConfigs::<T>::get(chain_id).ok_or(Error::<T>::NetworkNotInitialized)?;
            match network_config.consensus() {
                Consensus::Beacon(consensus) => Ok(consensus),
                _ => Err(Error::<T>::InvalidConsensus.into()),
            }
        }
    }

    impl<T: Config> Verifier<EVMChainId, Message> for Pallet<T> {
        type Result = (H256, u64);
        /// Verify a message by verifying the existence of the corresponding
        /// Ethereum log in a block. Returns the log if successful.
        fn verify(_chain_id: EVMChainId, _message: &Message) -> Result<(H256, u64), DispatchError> {
            // TODO: Implement storage verification
            todo!();
        }
    }
}

impl<T: Config> From<light_client::Error> for Error<T> {
    fn from(value: light_client::Error) -> Self {
        use light_client::Error as E;
        match value {
            E::InvalidMerkleBranch => Error::InvalidMerkleBranch,
            E::ArithError => Error::ArithError,
            E::ZeroParticipants => Error::ZeroParticipants,
            E::NotEnoughParticipants => Error::NotEnoughParticipants,
            E::InvalidUpdate => Error::InvalidUpdate,
            E::InvalidPublicKeyBytes => Error::InvalidPublicKeyBytes,
            E::SignatureVerificationFailed => Error::SignatureVerificationFailed,
            E::DuplicateSyncCommitteeUpdate => Error::DuplicateSyncCommitteeUpdate,
            E::InvalidSpecId => Error::InvalidSpecId,
            E::StoreNotInitialized => Error::NetworkNotInitialized,
        }
    }
}

#[derive(Default)]
pub struct PalletLightClientStore<T, E> {
    chain_id: EVMChainId,
    _phantom: PhantomData<(T, E)>,
}

impl<T, E> PalletLightClientStore<T, E> {
    pub fn new(chain_id: EVMChainId) -> Self {
        Self {
            chain_id,
            _phantom: Default::default(),
        }
    }
}

impl<T, E> LightClientStore<E> for PalletLightClientStore<T, E>
where
    T: Config,
    E: EthSpec,
    SyncCommittee<E>: TryFrom<GenericSyncCommittee> + Into<GenericSyncCommittee>,
    LightClientHeader<E>: TryFrom<GenericHeader> + Into<GenericHeader>,
{
    fn get_current_sync_committee(&self) -> Result<SyncCommittee<E>, light_client::Error> {
        let sync_committee = CurrentSyncCommittee::<T>::get(self.chain_id)
            .ok_or(light_client::Error::StoreNotInitialized)?;
        let sync_committee = sync_committee
            .try_into()
            .map_err(|_| light_client::Error::InvalidSpecId)?;
        Ok(sync_committee)
    }

    fn set_current_sync_committee(
        &self,
        sync_committee: SyncCommittee<E>,
    ) -> Result<(), light_client::Error> {
        let sync_committee: GenericSyncCommittee = sync_committee.into();
        CurrentSyncCommittee::<T>::insert(self.chain_id, sync_committee);
        Ok(())
    }

    fn get_next_sync_committee(&self) -> Result<Option<SyncCommittee<E>>, light_client::Error> {
        let sync_committee = NextSyncCommittee::<T>::get(self.chain_id);
        if let Some(sync_committee) = sync_committee {
            let sync_committee = sync_committee
                .try_into()
                .map_err(|_| light_client::Error::InvalidSpecId)?;
            Ok(Some(sync_committee))
        } else {
            Ok(None)
        }
    }

    fn set_next_sync_committee(
        &self,
        sync_committee: Option<SyncCommittee<E>>,
    ) -> Result<(), light_client::Error> {
        if let Some(sync_committee) = sync_committee {
            let sync_committee: GenericSyncCommittee = sync_committee.into();
            NextSyncCommittee::<T>::insert(self.chain_id, sync_committee);
        } else {
            NextSyncCommittee::<T>::set(self.chain_id, None);
        }
        Ok(())
    }

    fn get_finalized_header(&self) -> Result<LightClientHeader<E>, light_client::Error> {
        let header = FinalizedLightClientHeader::<T>::get(self.chain_id)
            .ok_or(light_client::Error::StoreNotInitialized)?;
        let header = header
            .try_into()
            .map_err(|_| light_client::Error::InvalidSpecId)?;
        Ok(header)
    }

    fn set_finalized_header(
        &self,
        header: LightClientHeader<E>,
    ) -> Result<(), light_client::Error> {
        let header: GenericHeader = header.into();
        FinalizedLightClientHeader::<T>::insert(self.chain_id, header);
        Ok(())
    }

    fn get_optimistic_header(&self) -> Result<LightClientHeader<E>, light_client::Error> {
        let header = OptimisticLightClientHeader::<T>::get(self.chain_id)
            .ok_or(light_client::Error::StoreNotInitialized)?;
        let header = header
            .try_into()
            .map_err(|_| light_client::Error::InvalidSpecId)?;
        Ok(header)
    }

    fn set_optimistic_header(
        &self,
        header: LightClientHeader<E>,
    ) -> Result<(), light_client::Error> {
        let header: GenericHeader = header.into();
        OptimisticLightClientHeader::<T>::insert(self.chain_id, header);
        Ok(())
    }
}
