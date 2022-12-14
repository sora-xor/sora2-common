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

use crate::{
    difficulty::{ClassicForkConfig, ForkConfig},
    EVMChainId,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Consensus {
    Ethash { fork_config: ForkConfig },
    Etchash { fork_config: ClassicForkConfig },
    Clique { period: u64, epoch: u64 },
}

impl Consensus {
    pub fn calc_epoch_length(&self, block_number: u64) -> u64 {
        match self {
            Consensus::Clique { epoch, .. } => *epoch,
            Consensus::Ethash { fork_config } => fork_config.epoch_length(),
            Consensus::Etchash { fork_config } => fork_config.calc_epoch_length(block_number),
        }
    }
}

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NetworkConfig {
    Mainnet,
    Ropsten,
    Sepolia,
    Rinkeby,
    Goerli,
    Classic,
    Mordor,
    Custom {
        chain_id: EVMChainId,
        consensus: Consensus,
    },
}

impl NetworkConfig {
    pub fn chain_id(&self) -> EVMChainId {
        match self {
            NetworkConfig::Mainnet => 1u32.into(),
            NetworkConfig::Ropsten => 3u32.into(),
            NetworkConfig::Sepolia => 11155111u32.into(),
            NetworkConfig::Rinkeby => 4u32.into(),
            NetworkConfig::Goerli => 5u32.into(),
            NetworkConfig::Classic => 61u32.into(),
            NetworkConfig::Mordor => 63u32.into(),
            NetworkConfig::Custom { chain_id, .. } => *chain_id,
        }
    }

    pub fn consensus(&self) -> Consensus {
        match self {
            NetworkConfig::Mainnet => Consensus::Ethash {
                fork_config: ForkConfig::mainnet(),
            },
            NetworkConfig::Ropsten => Consensus::Ethash {
                fork_config: ForkConfig::ropsten(),
            },
            NetworkConfig::Sepolia => Consensus::Ethash {
                fork_config: ForkConfig::sepolia(),
            },
            NetworkConfig::Classic => Consensus::Etchash {
                fork_config: ClassicForkConfig::classic(),
            },
            NetworkConfig::Mordor => Consensus::Etchash {
                fork_config: ClassicForkConfig::mordor(),
            },
            NetworkConfig::Rinkeby => Consensus::Clique {
                period: 15,
                epoch: 30000,
            },
            NetworkConfig::Goerli => Consensus::Clique {
                period: 15,
                epoch: 30000,
            },
            NetworkConfig::Custom {
                consensus: protocol,
                ..
            } => *protocol,
        }
    }
}
