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

use crate::header::Header;
use crate::U256;
use sp_runtime::RuntimeDebug;
use sp_std::convert::TryFrom;

use codec::{Decode, Encode};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Ethash Params. See https://ethereum.org/en/developers/docs/consensus-mechanisms/pow/mining-algorithms/ethash
/// Blocks per epoch
pub const EPOCH_LENGTH: u64 = 30000;
/// Etchash have increased epoch length
/// https://ecips.ethereumclassic.org/ECIPs/ecip-1099
pub const ETCHASH_EPOCH_LENGTH: u64 = 60000;
/// right-shifts equivalent to division by 2048
const DIFFICULTY_BOUND_DIVISOR: u32 = 11;
const EXP_DIFFICULTY_PERIOD: u64 = 100000;
const MINIMUM_DIFFICULTY: u32 = 131072;

#[derive(PartialEq, Eq, RuntimeDebug)]
pub enum BombDelay {
    // See https://eips.ethereum.org/EIPS/eip-649
    Byzantium = 3000000,
    // See https://eips.ethereum.org/EIPS/eip-1234
    Constantinople = 5000000,
    // See https://eips.ethereum.org/EIPS/eip-2384
    MuirGlacier = 9000000,
    // See https://eips.ethereum.org/EIPS/eip-3554
    London = 9700000,
    // See https://eips.ethereum.org/EIPS/eip-4345
    ArrowGlacier = 10700000,
    // See https://eips.ethereum.org/EIPS/eip-5133
    GrayGlacier = 11400000,
}

/// Describes when hard forks occurred in Ethereum Mainnet based networks
/// that affect difficulty calculations. These values are network-specific.
#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ForkConfig {
    // Block number on which Byzantium (EIP-649) rules activated
    pub byzantium_fork_block: u64,
    // Block number on which Constantinople (EIP-1234) rules activated
    pub constantinople_fork_block: u64,
    // Block number on which MuirGlacier (EIP-2384) activated
    pub muir_glacier_fork_block: u64,
    // Block number on which London (EIP-3554) activated
    pub london_fork_block: u64,
    // Block number on which ArrowGlacier (EIP-4345) activated
    pub arrow_glacier_fork_block: u64,
    // Block number on which GrayGlacier (EIP-5133) activated
    pub gray_glacier_fork_block: u64,
}

impl ForkConfig {
    pub fn bomb_delay(&self, block_number: u64) -> Option<BombDelay> {
        if block_number >= self.gray_glacier_fork_block {
            Some(BombDelay::GrayGlacier)
        } else if block_number >= self.arrow_glacier_fork_block {
            Some(BombDelay::ArrowGlacier)
        } else if block_number >= self.london_fork_block {
            Some(BombDelay::London)
        } else if block_number >= self.muir_glacier_fork_block {
            Some(BombDelay::MuirGlacier)
        } else if block_number >= self.constantinople_fork_block {
            Some(BombDelay::Constantinople)
        } else if block_number >= self.byzantium_fork_block {
            Some(BombDelay::Byzantium)
        } else {
            None
        }
    }

    pub fn mainnet() -> Self {
        ForkConfig {
            byzantium_fork_block: 4_370_000,
            constantinople_fork_block: 7_280_000,
            muir_glacier_fork_block: 9_200_000,
            london_fork_block: 12_965_000,
            arrow_glacier_fork_block: 13_773_000,
            gray_glacier_fork_block: 15_050_000,
        }
    }

    pub fn ropsten() -> Self {
        ForkConfig {
            byzantium_fork_block: 1_700_000,
            constantinople_fork_block: 4_230_000,
            muir_glacier_fork_block: 7_117_117,
            london_fork_block: 10_499_401,
            // Ropsten is PoS network and isn't affected by the difficulty bomb delay
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        }
    }

    pub fn sepolia() -> Self {
        ForkConfig {
            byzantium_fork_block: 0,
            constantinople_fork_block: 0,
            muir_glacier_fork_block: 0,
            london_fork_block: 0,
            // Sepolia is PoS network and isn't affected by the difficulty bomb delay
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        }
    }

    pub fn calc_difficulty(&self, time: u64, parent: &Header) -> Result<U256, &'static str> {
        let bomb_delay = self
            .bomb_delay(parent.number + 1)
            .ok_or("Cannot calculate difficulty for block number prior to Byzantium")?;

        calc_difficulty(Some(bomb_delay as u64), time, parent)
    }

    pub fn epoch_length(&self) -> u64 {
        EPOCH_LENGTH
    }
}

/// Describes when hard forks occurred in Ethereum Classic based networks
/// that affect difficulty calculations. These values are network-specific.
#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassicForkConfig {
    // https://ecips.ethereumclassic.org/ECIPs/ecip-1041
    ecip1041_block: u64,
    // https://ecips.ethereumclassic.org/ECIPs/ecip-1099
    ecip1099_block: u64,
}

impl ClassicForkConfig {
    pub fn classic() -> Self {
        ClassicForkConfig {
            ecip1041_block: 5_900_000,
            ecip1099_block: 11_700_000,
        }
    }

    pub fn mordor() -> Self {
        ClassicForkConfig {
            ecip1041_block: 0,
            ecip1099_block: 2_520_000,
        }
    }

    pub fn calc_difficulty(&self, time: u64, parent: &Header) -> Result<U256, &'static str> {
        if parent.number < self.ecip1041_block {
            return Err("Cannot calculate difficulty for block number prior to ECIP1041");
        }
        calc_difficulty(None, time, parent)
    }

    pub fn calc_epoch_length(&self, block_number: u64) -> u64 {
        if block_number < self.ecip1099_block {
            EPOCH_LENGTH
        } else {
            ETCHASH_EPOCH_LENGTH
        }
    }
}

/// This difficulty calculation follows Byzantium rules (https://eips.ethereum.org/EIPS/eip-649)
/// and shouldn't be used to calculate difficulty prior to the Byzantium fork.
pub fn calc_difficulty(
    bomb_delay: Option<u64>,
    time: u64,
    parent: &Header,
) -> Result<U256, &'static str> {
    let block_time_div_9: i64 = time
        .checked_sub(parent.timestamp)
        .ok_or("Invalid block time")
        .and_then(|x| i64::try_from(x / 9).or(Err("Invalid block time")))?;
    let sigma2: i64 = match parent.has_ommers() {
        true => 2 - block_time_div_9,
        false => 1 - block_time_div_9,
    }
    .max(-99);

    let mut difficulty_without_exp = parent.difficulty;
    if sigma2 < 0 {
        difficulty_without_exp -=
            (parent.difficulty >> DIFFICULTY_BOUND_DIVISOR) * sigma2.unsigned_abs();
    } else {
        difficulty_without_exp += (parent.difficulty >> DIFFICULTY_BOUND_DIVISOR) * sigma2 as u64;
    }

    difficulty_without_exp = difficulty_without_exp.max(MINIMUM_DIFFICULTY.into());

    if let Some(bomb_delay) = bomb_delay {
        // Subtract 1 less since we're using the parent block
        let fake_block_number = parent.number.saturating_sub(bomb_delay as u64 - 1);
        let period_count = fake_block_number / EXP_DIFFICULTY_PERIOD;

        // If period_count < 2, exp is fractional and we can skip adding it
        if period_count >= 2 {
            return Ok(difficulty_without_exp + U256::from(2).pow((period_count - 2).into()));
        }
    }

    Ok(difficulty_without_exp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::header::EMPTY_OMMERS_HASH;
    use ethereum_types::H256;
    use hex_literal::hex;
    use serde::{Deserialize, Deserializer};
    use serde_json::Value;
    use sp_std::convert::TryInto;
    use std::collections::BTreeMap;
    use std::fmt::Display;
    use std::fs::File;
    use std::path::PathBuf;

    // anything different from EMPTY_OMMERS_HASH
    const NON_EMPTY_OMMERS_HASH: [u8; 32] =
        hex!("1111111111111111111111111111111111111111111111111111111111111111");

    pub fn deserialize_uint_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: TryFrom<u128> + Deserialize<'de>,
        <T as TryFrom<u128>>::Error: Display,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt<T> {
            String(String),
            Number(T),
        }

        match StringOrInt::<T>::deserialize(deserializer)? {
            StringOrInt::String(s) => {
                let maybe_uint = {
                    if s.starts_with("0x") {
                        u128::from_str_radix(s.trim_start_matches("0x"), 16)
                    } else {
                        u128::from_str_radix(&s, 10)
                    }
                };
                match maybe_uint {
                    Err(e) => Err(serde::de::Error::custom(e)),
                    Ok(uint) => uint.try_into().map_err(serde::de::Error::custom),
                }
            }
            StringOrInt::Number(i) => Ok(i),
        }
    }

    /// Test case in `fixtures/tests/BasicTests/difficulty*.json` with explicit parent uncles hash.
    #[derive(Debug, PartialEq, Eq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BasicTestCase {
        /// Parent timestamp.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub parent_timestamp: u64,
        /// Parent difficulty.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub parent_difficulty: U256,
        /// Parent uncle hash flag:
        pub parent_uncles: H256,
        /// Current timestamp.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub current_timestamp: u64,
        /// Current difficulty.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub current_difficulty: U256,
        /// Current block number.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub current_block_number: u64,
    }

    /// Test suite in `fixtures/tests/BasicTests/difficulty.*.json`.
    #[derive(Debug, PartialEq, Deserialize)]
    pub struct BasicTestSuite(BTreeMap<String, BasicTestCase>);

    impl BasicTestSuite {
        /// Loads test from json.
        pub fn from_fixture(fixture: &str) -> Self {
            let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", fixture]
                .iter()
                .collect();
            serde_json::from_reader(File::open(&path).unwrap()).unwrap()
        }
    }

    /// Test case in `fixtures/tests/DifficultyTests/*.json` with parent uncles hash set as flag.
    #[derive(Debug, PartialEq, Eq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DifficultyTestCase {
        /// Parent timestamp.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub parent_timestamp: u64,
        /// Parent difficulty.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub parent_difficulty: U256,
        /// Parent uncle hash flag:
        /// - 0 - is absent - use empty ommers hash
        /// - 1 - present - use any other hash
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub parent_uncles: u64,
        /// Current timestamp.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub current_timestamp: u64,
        /// Current difficulty.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub current_difficulty: U256,
        /// Current block number.
        #[serde(deserialize_with = "deserialize_uint_from_string")]
        pub current_block_number: u64,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct DifficultyTestSet {
        #[serde(skip)]
        pub _info: Value,
        #[serde(
            alias = "Frontier",
            alias = "Homestead",
            alias = "Byzantium",
            alias = "Constantinople",
            alias = "Berlin",
            alias = "ArrowGlacier",
            alias = "GrayGlacier"
        )]
        pub test_cases: BTreeMap<String, DifficultyTestCase>,
    }

    /// Test suite in `fixtures/tests/DifficultyTests/*.json`.
    #[derive(Debug, PartialEq, Deserialize)]
    pub struct DifficultyTestSuite {
        #[serde(
            alias = "difficultyFrontier",
            alias = "difficultyHomestead",
            alias = "difficultyByzantium",
            alias = "difficultyConstantinople",
            alias = "difficultyEIP2384",
            alias = "difficultyEIP2384_random",
            alias = "difficultyEIP2384_random_to20M",
            alias = "difficultyArrowGlacier",
            alias = "difficultyArrowGlacierForkBlock",
            alias = "difficultyArrowGlacierMinus1",
            alias = "difficultyArrowGlacierTimeDiff1",
            alias = "difficultyGrayGlacier",
            alias = "difficultyGrayGlacierForkBlock",
            alias = "difficultyGrayGlacierMinus1",
            alias = "difficultyGrayGlacierTimeDiff1"
        )]
        pub difficulty_test: DifficultyTestSet,
    }

    impl DifficultyTestSuite {
        /// Loads test from json.
        pub fn from_fixture(fixture: &str) -> Self {
            let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", fixture]
                .iter()
                .collect();
            serde_json::from_reader(File::open(&path).unwrap()).unwrap()
        }
    }

    macro_rules! test_difficulty {
        ($config:ident, $test_case_name:ident, $test_case:ident, $parent:ident) => {
            let difficulty = $config.calc_difficulty($test_case.current_timestamp, &$parent);
            if $config.byzantium_fork_block > $test_case.current_block_number {
                assert_eq!(
                    difficulty,
                    Err("Cannot calculate difficulty for block number prior to Byzantium"),
                    "Test case {} failed: {:?}",
                    $test_case_name,
                    $test_case,
                );
            } else {
                assert_eq!(
                    difficulty,
                    Ok($test_case.current_difficulty),
                    "Test case {} failed: {:?}",
                    $test_case_name,
                    $test_case,
                );
            }
        };
    }

    /// Reads and executes test suite from `fixtures/tests/BasicTests/difficulty.*.json`.
    macro_rules! run_basic_test {
        ($fixture:literal, $config:ident) => {
            let test_cases = BasicTestSuite::from_fixture($fixture);

            for (test_case_name, test_case) in &test_cases.0 {
                let mut parent: Header = Default::default();
                parent.number = test_case.current_block_number - 1;
                parent.timestamp = test_case.parent_timestamp;
                parent.difficulty = test_case.parent_difficulty;
                parent.ommers_hash = test_case.parent_uncles;
                test_difficulty!($config, test_case_name, test_case, parent);
            }
        };
    }

    /// Reads and executes test suite from `fixtures/tests/DifficultyTests/*.json`.
    macro_rules! run_difficulty_test {
        ($fixture:literal, $config:ident) => {
            let test_cases = DifficultyTestSuite::from_fixture($fixture);

            for (test_case_name, test_case) in &test_cases.difficulty_test.test_cases {
                let mut parent: Header = Default::default();
                parent.number = test_case.current_block_number - 1;
                parent.timestamp = test_case.parent_timestamp;
                parent.difficulty = test_case.parent_difficulty;
                parent.ommers_hash = if test_case.parent_uncles == 0 {
                    EMPTY_OMMERS_HASH.into()
                } else {
                    NON_EMPTY_OMMERS_HASH.into()
                };
                test_difficulty!($config, test_case_name, test_case, parent);
            }
        };
    }

    #[test]
    fn frontier_difficulty_calc_is_correct() {
        let all_blocks_are_frontier = ForkConfig {
            byzantium_fork_block: u64::MAX,
            constantinople_fork_block: u64::MAX,
            muir_glacier_fork_block: u64::MAX,
            london_fork_block: u64::MAX,
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        };
        run_difficulty_test!("tests/difficultyFrontier.json", all_blocks_are_frontier);
    }

    #[test]
    fn homestead_difficulty_calc_is_correct() {
        let all_blocks_are_homestead = ForkConfig {
            byzantium_fork_block: u64::MAX,
            constantinople_fork_block: u64::MAX,
            muir_glacier_fork_block: u64::MAX,
            london_fork_block: u64::MAX,
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        };
        run_difficulty_test!("tests/difficultyHomestead.json", all_blocks_are_homestead);
    }

    #[test]
    fn byzantium_difficulty_calc_is_correct() {
        let all_blocks_are_byzantium = ForkConfig {
            byzantium_fork_block: 0,
            constantinople_fork_block: u64::MAX,
            muir_glacier_fork_block: u64::MAX,
            london_fork_block: u64::MAX,
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        };
        run_difficulty_test!("tests/difficultyByzantium.json", all_blocks_are_byzantium);
    }

    #[test]
    fn constantinople_difficulty_calc_is_correct() {
        let all_blocks_are_constantinople = ForkConfig {
            byzantium_fork_block: 0,
            constantinople_fork_block: 0,
            muir_glacier_fork_block: u64::MAX,
            london_fork_block: u64::MAX,
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        };
        run_difficulty_test!(
            "tests/difficultyConstantinople.json",
            all_blocks_are_constantinople
        );
    }

    #[test]
    fn muir_glacier_difficulty_calc_is_correct() {
        let all_blocks_are_muir_glacier = ForkConfig {
            byzantium_fork_block: 0,
            constantinople_fork_block: 0,
            muir_glacier_fork_block: 0,
            london_fork_block: u64::MAX,
            arrow_glacier_fork_block: u64::MAX,
            gray_glacier_fork_block: u64::MAX,
        };
        run_difficulty_test!("tests/difficultyEIP2384.json", all_blocks_are_muir_glacier);
        run_difficulty_test!(
            "tests/difficultyEIP2384_random.json",
            all_blocks_are_muir_glacier
        );
        run_difficulty_test!(
            "tests/difficultyEIP2384_random_to20M.json",
            all_blocks_are_muir_glacier
        );
    }

    #[test]
    fn arrow_glacier_difficulty_calc_is_correct() {
        let all_blocks_are_arrow_glacier = ForkConfig {
            byzantium_fork_block: 0,
            constantinople_fork_block: 0,
            muir_glacier_fork_block: 0,
            london_fork_block: 0,
            arrow_glacier_fork_block: 0,
            gray_glacier_fork_block: u64::MAX,
        };
        run_difficulty_test!(
            "tests/difficultyArrowGlacier.json",
            all_blocks_are_arrow_glacier
        );
        run_difficulty_test!(
            "tests/difficultyArrowGlacierForkBlock.json",
            all_blocks_are_arrow_glacier
        );
        run_difficulty_test!(
            "tests/difficultyArrowGlacierMinus1.json",
            all_blocks_are_arrow_glacier
        );
        run_difficulty_test!(
            "tests/difficultyArrowGlacierTimeDiff1.json",
            all_blocks_are_arrow_glacier
        );
        run_difficulty_test!(
            "tests/difficultyArrowGlacierTimeDiff2.json",
            all_blocks_are_arrow_glacier
        );
    }

    #[test]
    fn gray_glacier_difficulty_calc_is_correct() {
        let all_blocks_are_gray_glacier = ForkConfig {
            byzantium_fork_block: 0,
            constantinople_fork_block: 0,
            muir_glacier_fork_block: 0,
            london_fork_block: 0,
            arrow_glacier_fork_block: 0,
            gray_glacier_fork_block: 0,
        };
        run_difficulty_test!(
            "tests/difficultyGrayGlacier.json",
            all_blocks_are_gray_glacier
        );
        run_difficulty_test!(
            "tests/difficultyGrayGlacierForkBlock.json",
            all_blocks_are_gray_glacier
        );
        run_difficulty_test!(
            "tests/difficultyGrayGlacierMinus1.json",
            all_blocks_are_gray_glacier
        );
        run_difficulty_test!(
            "tests/difficultyGrayGlacierTimeDiff1.json",
            all_blocks_are_gray_glacier
        );
        run_difficulty_test!(
            "tests/difficultyGrayGlacierTimeDiff2.json",
            all_blocks_are_gray_glacier
        );
    }

    #[test]
    fn mainnet_difficulty_calc_is_correct() {
        let mainnet_config = ForkConfig::mainnet();
        run_basic_test!("tests/difficultyMainNetwork.json", mainnet_config);
    }

    #[test]
    fn ropsten_difficulty_calc_is_correct() {
        let ropsten_config = ForkConfig::ropsten();
        run_basic_test!("tests/difficultyRopsten.json", ropsten_config);
    }
}
