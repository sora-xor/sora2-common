use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use hex_literal::hex;
use scale_info::TypeInfo;

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconFork {
    #[cfg_attr(
        feature = "std",
        serde(with = "crate::serde_utils::serde_fork_version")
    )]
    fork_version: ForkVersion,
    epoch: u64,
}

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconForkSchedule {
    pub phase0: BeaconFork,
    pub altair: BeaconFork,
    pub bellatrix: BeaconFork,
    // Disabled forks
    // capella: BeaconFork,
    // sharding: BeaconFork,
}

impl BeaconForkSchedule {
    pub fn fork_version(&self, epoch: u64) -> ForkVersion {
        if epoch >= self.bellatrix.epoch {
            self.bellatrix.fork_version
        } else if epoch >= self.altair.epoch {
            self.altair.fork_version
        } else {
            self.phase0.fork_version
        }
    }

    // https://github.com/ChainSafe/lodestar/blob/aa4349cee2b5bbefdf4e7c0bd58df36aaebff6de/packages/config/src/chainConfig/networks/sepolia.ts
    pub fn sepolia() -> Self {
        Self {
            phase0: BeaconFork {
                fork_version: hex!("90000069"),
                epoch: 0,
            },
            altair: BeaconFork {
                fork_version: hex!("90000070"),
                epoch: 50,
            },
            bellatrix: BeaconFork {
                fork_version: hex!("90000071"),
                epoch: 100,
            },
        }
    }

    // https://github.com/ChainSafe/lodestar/blob/aa4349cee2b5bbefdf4e7c0bd58df36aaebff6de/packages/config/src/chainConfig/networks/goerli.ts
    pub fn goerli() -> Self {
        Self {
            phase0: BeaconFork {
                fork_version: hex!("00001020"),
                epoch: 0,
            },
            altair: BeaconFork {
                fork_version: hex!("01001020"),
                epoch: 36660,
            },
            bellatrix: BeaconFork {
                fork_version: hex!("02001020"),
                epoch: 112260,
            },
        }
    }

    // https://github.com/eth-clients/merge-testnets/blob/302fe27afdc7a9d15b1766a0c0a9d64319140255/mainnet-shadow-fork-13/config.yaml
    pub fn mainnet() -> Self {
        Self {
            phase0: BeaconFork {
                fork_version: hex!("00000000"),
                epoch: 0,
            },
            altair: BeaconFork {
                fork_version: hex!("01000000"),
                epoch: 74240,
            },
            bellatrix: BeaconFork {
                fork_version: hex!("02000000"),
                epoch: 144896,
            },
        }
    }

    pub fn local() -> Self {
        Self {
            phase0: BeaconFork {
                fork_version: hex!("00000001"),
                epoch: 0,
            },
            altair: BeaconFork {
                fork_version: hex!("01000001"),
                epoch: 0,
            },
            bellatrix: BeaconFork {
                fork_version: hex!("02000001"),
                epoch: 0,
            },
        }
    }
}

#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum BeaconNetworkConfig {
    Mainnet,
    Minimal,
}

#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconConsensusConfig {
    pub config: BeaconNetworkConfig,
    pub fork_schedule: BeaconForkSchedule,
}

impl BeaconConsensusConfig {
    pub fn mainnet() -> Self {
        Self {
            config: BeaconNetworkConfig::Mainnet,
            fork_schedule: BeaconForkSchedule::mainnet(),
        }
    }

    pub fn goerli() -> Self {
        Self {
            config: BeaconNetworkConfig::Mainnet,
            fork_schedule: BeaconForkSchedule::goerli(),
        }
    }

    pub fn sepolia() -> Self {
        Self {
            config: BeaconNetworkConfig::Mainnet,
            fork_schedule: BeaconForkSchedule::sepolia(),
        }
    }

    pub fn local() -> Self {
        Self {
            config: BeaconNetworkConfig::Minimal,
            fork_schedule: BeaconForkSchedule::local(),
        }
    }

    pub fn fork_version_from_slot(&self, slot: u64) -> ForkVersion {
        let epoch = self.config.compute_epoch(slot);
        self.fork_schedule.fork_version(epoch)
    }
}

impl BeaconNetworkConfig {
    pub fn compute_current_sync_period(&self, slot: u64) -> u64 {
        match self {
            Self::Minimal => {
                slot / MINIMAL_SLOTS_PER_EPOCH / MINIMAL_EPOCHS_PER_SYNC_COMMITTEE_PERIOD
            }
            Self::Mainnet => {
                slot / MAINNET_SLOTS_PER_EPOCH / MAINNET_EPOCHS_PER_SYNC_COMMITTEE_PERIOD
            }
        }
    }

    pub fn compute_epoch(&self, slot: u64) -> u64 {
        match self {
            Self::Minimal => slot / MINIMAL_SLOTS_PER_EPOCH,
            Self::Mainnet => slot / MAINNET_SLOTS_PER_EPOCH,
        }
    }

    pub fn epoch_length(&self) -> u64 {
        match self {
            Self::Minimal => MINIMAL_SLOTS_PER_EPOCH,
            Self::Mainnet => MAINNET_SLOTS_PER_EPOCH,
        }
    }
}

pub const MAINNET_SLOTS_PER_EPOCH: u64 = 32;
pub const MAINNET_EPOCHS_PER_SYNC_COMMITTEE_PERIOD: u64 = 256;
pub const MAINNET_SYNC_COMMITTEE_SIZE: usize = 512;

pub const MINIMAL_SLOTS_PER_EPOCH: u64 = 8;
pub const MINIMAL_EPOCHS_PER_SYNC_COMMITTEE_PERIOD: u64 = 8;
pub const MINIMAL_SYNC_COMMITTEE_SIZE: usize = 32;

use super::ForkVersion;

pub const CURRENT_SYNC_COMMITTEE_INDEX: u64 = 22;
pub const CURRENT_SYNC_COMMITTEE_DEPTH: u64 = 5;

pub const NEXT_SYNC_COMMITTEE_DEPTH: u64 = 5;
pub const NEXT_SYNC_COMMITTEE_INDEX: u64 = 23;

pub const FINALIZED_ROOT_DEPTH: u64 = 6;
pub const FINALIZED_ROOT_INDEX: u64 = 41;

pub const MAX_PROPOSER_SLASHINGS: usize = 16;
pub const MAX_ATTESTER_SLASHINGS: usize = 2;
pub const MAX_ATTESTATIONS: usize = 128;
pub const MAX_DEPOSITS: usize = 16;
pub const MAX_VOLUNTARY_EXITS: usize = 16;
pub const MAX_VALIDATORS_PER_COMMITTEE: usize = 2048;
pub const MAX_EXTRA_DATA_BYTES: usize = 32;
pub const MAX_LOGS_BLOOM_SIZE: usize = 256;
pub const MAX_FEE_RECIPIENT_SIZE: usize = 20;
pub const MAX_TRANSACTIONS: usize = 1048576;
pub const MAX_BYTES_PER_TRANSACTION: usize = 1073741824;
pub const MAX_H256_PER_TRANSACTION: usize = (MAX_BYTES_PER_TRANSACTION + 31) / 32;

pub const DEPOSIT_CONTRACT_TREE_DEPTH: usize = 32;

/// GENESIS_FORK_VERSION('0x00000000')
pub const GENESIS_FORK_VERSION: ForkVersion = [30, 30, 30, 30];

/// DomainType('0x07000000')
/// https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/beacon-chain.md#domain-types
pub const DOMAIN_SYNC_COMMITTEE: [u8; 4] = [7, 0, 0, 0];

pub const PUBKEY_SIZE: usize = 48;
pub const SIGNATURE_SIZE: usize = 96;
