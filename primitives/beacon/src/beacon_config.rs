use codec::{Decode, Encode, MaxEncodedLen};
use ethereum_types::H256;
use hex_literal::hex;
use scale_info::TypeInfo;

use crate::{EthSpecId, ForkVersion};

#[derive(
    Copy,
    Clone,
    Encode,
    Decode,
    PartialEq,
    Eq,
    Debug,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ForkInfo {
    #[serde(with = "eth2_serde_utils::bytes_4_hex")]
    version: ForkVersion,
    epoch: u64,
}

#[derive(
    Copy,
    Clone,
    Encode,
    Decode,
    PartialEq,
    Eq,
    Debug,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ForkSchedule {
    pub phase0: ForkInfo,
    pub altair: ForkInfo,
    pub bellatrix: ForkInfo,
    pub capella: ForkInfo,
    // Disabled forks
    // sharding: BeaconFork,
}

impl ForkSchedule {
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
            phase0: ForkInfo {
                version: hex!("90000069"),
                epoch: 0,
            },
            altair: ForkInfo {
                version: hex!("90000070"),
                epoch: 50,
            },
            bellatrix: ForkInfo {
                version: hex!("90000071"),
                epoch: 100,
            },
            capella: ForkInfo {
                version: hex!("90000072"),
                epoch: 56832,
            },
        }
    }

    // https://github.com/ChainSafe/lodestar/blob/aa4349cee2b5bbefdf4e7c0bd58df36aaebff6de/packages/config/src/chainConfig/networks/goerli.ts
    pub fn goerli() -> Self {
        Self {
            phase0: ForkInfo {
                version: hex!("00001020"),
                epoch: 0,
            },
            altair: ForkInfo {
                version: hex!("01001020"),
                epoch: 36660,
            },
            bellatrix: ForkInfo {
                version: hex!("02001020"),
                epoch: 112260,
            },
            capella: ForkInfo {
                version: hex!("03001020"),
                epoch: 162304,
            },
        }
    }

    // https://github.com/eth-clients/merge-testnets/blob/302fe27afdc7a9d15b1766a0c0a9d64319140255/mainnet-shadow-fork-13/config.yaml
    pub fn mainnet() -> Self {
        Self {
            phase0: ForkInfo {
                version: hex!("00000000"),
                epoch: 0,
            },
            altair: ForkInfo {
                version: hex!("01000000"),
                epoch: 74240,
            },
            bellatrix: ForkInfo {
                version: hex!("02000000"),
                epoch: 144896,
            },
            capella: ForkInfo {
                version: hex!("04000000"),
                epoch: u64::MAX,
            },
        }
    }

    pub fn local() -> Self {
        Self {
            phase0: ForkInfo {
                version: hex!("00000001"),
                epoch: 0,
            },
            altair: ForkInfo {
                version: hex!("01000001"),
                epoch: 0,
            },
            bellatrix: ForkInfo {
                version: hex!("02000001"),
                epoch: 0,
            },
            capella: ForkInfo {
                version: hex!("03000001"),
                epoch: u64::MAX,
            },
        }
    }
}

#[derive(
    Clone,
    Copy,
    Encode,
    Decode,
    PartialEq,
    Eq,
    Debug,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ConsensusConfig {
    pub spec_id: EthSpecId,
    pub fork_schedule: ForkSchedule,
    pub genesis_validators_root: H256,
}

impl ConsensusConfig {
    pub fn mainnet() -> Self {
        Self {
            spec_id: EthSpecId::Mainnet,
            fork_schedule: ForkSchedule::mainnet(),
            genesis_validators_root: H256(hex!(
                "4b363db94e286120d76eb905340fdd4e54bfe9f06bf33ff6cf5ad27f511bfe95"
            )),
        }
    }

    pub fn goerli() -> Self {
        Self {
            spec_id: EthSpecId::Mainnet,
            fork_schedule: ForkSchedule::goerli(),
            genesis_validators_root: H256(hex!(
                "043db0d9a83813551ee2f33450d23797757d430911a9320530ad8a0eabc43efb"
            )),
        }
    }

    pub fn sepolia() -> Self {
        Self {
            spec_id: EthSpecId::Mainnet,
            fork_schedule: ForkSchedule::sepolia(),
            genesis_validators_root: H256(hex!(
                "d8ea171f3c94aea21ebc42a1ed61052acf3f9209c00e4efbaaddac09ed9b8078"
            )),
        }
    }

    pub fn fork_version_from_epoch(&self, epoch: u64) -> ForkVersion {
        self.fork_schedule.fork_version(epoch)
    }
}
