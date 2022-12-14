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

use bridge_types::H256;
use codec::{Decode, Encode};
use frame_support::log;
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;
use sp_io::hashing::keccak_256;

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct SimplifiedMMRProof {
    pub merkle_proof_items: Vec<H256>,
    pub merkle_proof_order_bit_field: u64,
}

pub fn verify_inclusion_proof(
    root: H256,
    leaf_node_hash: H256,
    proof: &SimplifiedMMRProof,
) -> bool {
    if proof.merkle_proof_items.len() >= 64 {
        return false;
    }
    log::debug!("verify_inclusion_proof: proof: {:?}", proof);
    root == calculate_merkle_root(
        leaf_node_hash,
        &proof.merkle_proof_items,
        proof.merkle_proof_order_bit_field,
    )
}

pub fn bit(self_val: u64, index: u64) -> bool {
    ((self_val >> index) & 1) as u8 == 1
}

pub fn calculate_merkle_root(
    leaf_node_hash: H256,
    merkle_proof_items: &[H256],
    merkle_proof_order_bit_field: u64,
) -> H256 {
    let mut current_hash = leaf_node_hash;
    for (i, sibling) in merkle_proof_items.iter().enumerate() {
        let is_sibling_left = bit(merkle_proof_order_bit_field, i as u64);
        current_hash = if is_sibling_left {
            keccak_256((sibling, current_hash).encode().as_slice())
        } else {
            keccak_256((current_hash, sibling).encode().as_slice())
        }
        .into();
    }
    current_hash
}
