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

use crate::Vec;
use bridge_common::beefy_types::ValidatorSet;
use bridge_common::simplified_mmr_proof::SimplifiedMMRProof;
use bridge_types::H160;
use bridge_types::H256;
use codec::Decode;
use serde::Deserialize;

pub fn alice<T: crate::Config>() -> T::AccountId {
    T::AccountId::decode(&mut [0u8; 32].as_slice()).unwrap()
}

#[derive(Debug, Clone, Deserialize)]
pub struct MMRProof {
    pub order: u64,
    pub items: Vec<H256>,
}

impl From<MMRProof> for SimplifiedMMRProof {
    fn from(proof: MMRProof) -> Self {
        SimplifiedMMRProof {
            merkle_proof_items: proof.items,
            merkle_proof_order_bit_field: proof.order,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FixtureValidatorSet {
    pub id: u64,
    pub root: H256,
    pub len: u32,
}

impl From<FixtureValidatorSet> for ValidatorSet {
    fn from(f: FixtureValidatorSet) -> Self {
        ValidatorSet {
            id: f.id,
            len: f.len,
            root: f.root,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub addresses: Vec<H160>,
    pub validator_set: FixtureValidatorSet,
    pub next_validator_set: FixtureValidatorSet,
    pub validator_set_proofs: Vec<Vec<H256>>,
    pub commitment: Vec<u8>,
    pub leaf_proof: MMRProof,
    pub leaf: Vec<u8>,
}

pub fn load_fixture(validators: usize, tree_size: usize) -> Fixture {
    let fixture: Fixture = serde_json::from_str(
        &std::fs::read_to_string(format!(
            "src/fixtures/beefy-{}-{}.json",
            validators, tree_size
        ))
        .unwrap(),
    )
    .unwrap();
    fixture
}

pub fn load_slice_fixture(slice: &[u8]) -> Fixture {
    // let fixture: Fixture = serde_json::from_str(
    //     &std::fs::read_to_string(format!(
    //         "src/fixtures/beefy-{}-{}.json",
    //         validators, tree_size
    //     ))
    //     .unwrap(),
    // )
    // .unwrap();
    let fixture: Fixture = serde_json::from_slice(slice).expect("Error loading fixture");
    fixture
}

pub fn validator_proof<T: crate::Config>(
    fixture: &Fixture,
    signatures: Vec<Option<beefy_primitives::crypto::Signature>>,
    count: usize,
) -> bridge_common::beefy_types::ValidatorProof {
    let bits_to_set = signatures
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.clone().map(|_| i as u32))
        .take(count)
        .collect::<Vec<_>>();
    let initial_bitfield =
        bridge_common::bitfield::BitField::create_bitfield(&bits_to_set, signatures.len());
    let random_bitfield = crate::Pallet::<T>::create_random_bit_field(
        bridge_types::SubNetworkId::Mainnet,
        initial_bitfield.clone(),
        signatures.len() as u32,
    )
    .unwrap();
    let mut positions = vec![];
    let mut proof_signatures = vec![];
    let mut public_keys = vec![];
    let mut public_key_merkle_proofs = vec![];
    for i in 0..random_bitfield.len() {
        let bit = random_bitfield.is_set(i);
        if bit {
            positions.push(i as u128);
            let mut signature = signatures.get(i).unwrap().clone().unwrap().to_vec();
            signature[64] += 27;
            proof_signatures.push(signature);
            public_keys.push(fixture.addresses[i]);
            public_key_merkle_proofs.push(fixture.validator_set_proofs[i].clone());
        }
    }
    let validator_proof = bridge_common::beefy_types::ValidatorProof {
        signatures: proof_signatures,
        positions,
        public_keys,
        public_key_merkle_proofs: public_key_merkle_proofs,
        validator_claims_bitfield: initial_bitfield,
    };
    validator_proof
}
