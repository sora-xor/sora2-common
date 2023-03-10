use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use hex_literal::hex;
use scale_info::TypeInfo;

pub type ForkVersion = [u8; 4];

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconFork {
    #[cfg_attr(
        feature = "std",
        serde(with = "crate::serde_utils::serde_fork_version")
    )]
    version: [u8; 4],
    epoch: u64,
}

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct BeaconForkSchedule {
    pub phase0: BeaconFork,
    pub altair: BeaconFork,
    pub bellatrix: BeaconFork,
    pub capella: BeaconFork,
    // Disabled forks
    // sharding: BeaconFork,
}

impl BeaconForkSchedule {
    pub fn fork_version(&self, epoch: u64) -> ForkVersion {
        if epoch >= self.capella.epoch {
            self.capella.version
        } else if epoch >= self.bellatrix.epoch {
            self.bellatrix.version
        } else if epoch >= self.altair.epoch {
            self.altair.version
        } else {
            self.phase0.version
        }
    }

    // https://github.com/ChainSafe/lodestar/blob/aa4349cee2b5bbefdf4e7c0bd58df36aaebff6de/packages/config/src/chainConfig/networks/sepolia.ts
    pub fn sepolia() -> Self {
        Self {
            phase0: BeaconFork {
                version: hex!("90000069"),
                epoch: 0,
            },
            altair: BeaconFork {
                version: hex!("90000070"),
                epoch: 50,
            },
            bellatrix: BeaconFork {
                version: hex!("90000071"),
                epoch: 100,
            },
            capella: BeaconFork {
                version: hex!("90000072"),
                epoch: 56832,
            },
        }
    }

    // https://github.com/ChainSafe/lodestar/blob/aa4349cee2b5bbefdf4e7c0bd58df36aaebff6de/packages/config/src/chainConfig/networks/goerli.ts
    pub fn goerli() -> Self {
        Self {
            phase0: BeaconFork {
                version: hex!("00001020"),
                epoch: 0,
            },
            altair: BeaconFork {
                version: hex!("01001020"),
                epoch: 36660,
            },
            bellatrix: BeaconFork {
                version: hex!("02001020"),
                epoch: 112260,
            },
            capella: BeaconFork {
                version: hex!("03001020"),
                epoch: 162304,
            },
        }
    }

    // https://github.com/eth-clients/merge-testnets/blob/302fe27afdc7a9d15b1766a0c0a9d64319140255/mainnet-shadow-fork-13/config.yaml
    pub fn mainnet() -> Self {
        Self {
            phase0: BeaconFork {
                version: hex!("00000000"),
                epoch: 0,
            },
            altair: BeaconFork {
                version: hex!("01000000"),
                epoch: 74240,
            },
            bellatrix: BeaconFork {
                version: hex!("02000000"),
                epoch: 144896,
            },
            capella: BeaconFork {
                version: hex!("04000000"),
                epoch: u64::MAX,
            },
        }
    }

    pub fn local() -> Self {
        Self {
            phase0: BeaconFork {
                version: hex!("00000001"),
                epoch: 0,
            },
            altair: BeaconFork {
                version: hex!("01000001"),
                epoch: 0,
            },
            bellatrix: BeaconFork {
                version: hex!("02000001"),
                epoch: 0,
            },
            capella: BeaconFork {
                version: hex!("03000001"),
                epoch: u64::MAX,
            },
        }
    }
}

#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum BeaconNetworkConfig {
    Mainnet,
    Minimal,
}

#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
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

    pub fn fork_version_from_epoch(&self, epoch: u64) -> ForkVersion {
        self.fork_schedule.fork_version(epoch)
    }
}
