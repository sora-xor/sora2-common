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

use crate::bitfield::BitField;
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;
use sp_core::H160;

pub type EthAddress = H160;

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct Commitment {
    pub payload_prefix: Vec<u8>,
    pub payload: [u8; 32],
    pub payload_suffix: Vec<u8>,
    pub block_number: u32,
    pub validator_set_id: u64,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, scale_info::TypeInfo)]
pub struct ValidatorProof {
    pub validator_claims_bitfield: BitField,
    pub signatures: Vec<Vec<u8>>,
    pub positions: Vec<u128>,
    pub public_keys: Vec<EthAddress>,
    pub public_key_merkle_proofs: Vec<Vec<[u8; 32]>>,
}

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct BeefyMMRLeaf {
    pub version: u8,
    pub parent_number: u32,
    pub next_authority_set_id: u64,
    pub next_authority_set_len: u32,
    pub parent_hash: [u8; 32],
    pub next_authority_set_root: [u8; 32],
    pub random_seed: [u8; 32],
    pub digest_hash: [u8; 32],
}

#[derive(
    Encode,
    Decode,
    Clone,
    RuntimeDebug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    scale_info::TypeInfo,
    Default,
)]
pub struct ValidatorSet {
    pub id: u128,
    pub length: u128,
    pub root: [u8; 32],
}
