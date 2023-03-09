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

use crate::consts::*;
use crate::mock::*;
use crate::test_helpers::*;
use crate::Error;
use beefy_primitives::Payload;
use bridge_common::beefy_types::BeefyMMRLeaf;
use bridge_common::beefy_types::ValidatorSet;
use bridge_types::SubNetworkId;
use bridge_types::H160;
use bridge_types::H256;
use codec::Decode;
use frame_support::assert_noop;
use frame_support::assert_ok;
use hex_literal::hex;
use test_case::test_case;
// #[test_case(3, 5; "3 validators, 5 leaves")]
// #[test_case(3, 5000; "3 validators, 5000 leaves")]
// #[test_case(3, 5000000; "3 validators, 5000000 leaves")]
// #[test_case(37, 5; "37 validators, 5 leaves")]
// #[test_case(37, 5000; "37 validators, 5000 leaves")]
// #[test_case(69, 5000; "69 validators, 5000 leaves")]
// #[test_case(200, 5000; "200 validators, 5000 leaves")]
// #[test_case(10, 128; "10 validators, 128 leaves")]
// #[test_case(20, 256; "20 validators, 256 leaves")]
// #[test_case(40, 512; "40 validators, 512 leaves")]
// #[test_case(80, 1024; "80 validators, 1024 leaves")]
// #[test_case(160, 2048; "160 validators, 2048 leaves")]
// #[test_case(200, 4096; "200 validators, 4096 leaves")]
#[test_case(300, 8192; "300 validators, 8192 leaves")]
fn submit_fixture_success(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        println!("fixture: {:?}", fixture);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_ok!(BeefyLightClient::submit_signature_commitment(
            RuntimeOrigin::signed(alice::<Test>()),
            SubNetworkId::Mainnet,
            commitment,
            validator_proof,
            leaf,
            fixture.leaf_proof.into(),
        ));
    });
}

#[test_case(3, 5; "3 validators, 5 leaves")]
fn submit_fixture_failed_pallet_not_initialized(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::PalletNotInitialized
        );
    })
}

#[test_case(3, 5; "3 validators, 5 leaves")]
fn submit_fixture_failed_invalid_set_id(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let mut commitment = signed_commitment.commitment.clone();
        commitment.validator_set_id += 10;
        let validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::InvalidValidatorSetId
        );
    })
}

#[test_case(3, 5000; "3 validators, 5000 leaves")]
#[test_case(37, 5000; "37 validators, 5000 leaves")]
fn submit_fixture_failed_invalid_commitment_signatures_threshold(
    validators: usize,
    tree_size: usize,
) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let mut validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let count_set_bits = validator_proof.validator_claims_bitfield.count_set_bits();
        let treshold = validators - (validators - 1) / 3 - 1;
        let error_diff = count_set_bits - treshold;

        // "spoil" the bitfield
        let mut i = 0;
        let mut j = 0;
        while j < error_diff {
            if validator_proof.validator_claims_bitfield.is_set(i) {
                validator_proof.validator_claims_bitfield.clear(i);
                j += 1;
            }
            i += 1;
        }
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::NotEnoughValidatorSignatures
        );
    })
}

#[test_case(3, 5; "3 validators, 5 leaves")]
#[test_case(3, 5000; "3 validators, 5000 leaves")]
fn submit_fixture_failed_invalid_number_of_signatures(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let mut validator_proof_small = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let mut validator_proof_big = validator_proof_small.clone();
        validator_proof_small.signatures.pop();
        validator_proof_big.signatures.push(Vec::new());
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment.clone(),
                validator_proof_small,
                leaf.clone(),
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfSignatures
        );

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof_big,
                leaf,
                load_fixture(validators, tree_size).leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfSignatures
        );
    });
}

#[test_case(3, 5; "3 validators, 5 leaves")]
#[test_case(3, 5000; "3 validators, 5000 leaves")]
fn submit_fixture_failed_invalid_number_of_positions(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let mut validator_proof_small = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let mut validator_proof_big = validator_proof_small.clone();
        validator_proof_small.positions.pop();
        validator_proof_big.positions.push(0);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment.clone(),
                validator_proof_small,
                leaf.clone(),
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfPositions
        );

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof_big,
                leaf,
                load_fixture(validators, tree_size).leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfPositions
        );
    });
}

#[test_case(3, 5; "3 validators, 5 leaves")]
#[test_case(3, 5000; "3 validators, 5000 leaves")]
fn submit_fixture_failed_invalid_number_of_public_keys(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let mut validator_proof_small = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let mut validator_proof_big = validator_proof_small.clone();
        validator_proof_small.public_keys.pop();
        validator_proof_big.public_keys.push(H160([
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
        ]));
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment.clone(),
                validator_proof_small,
                leaf.clone(),
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfPublicKeys
        );

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof_big,
                leaf,
                load_fixture(validators, tree_size).leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfPublicKeys
        );
    });
}

#[test_case(3, 5; "3 validators, 5 leaves")]
#[test_case(3, 5000; "3 validators, 5000 leaves")]
fn submit_fixture_failed_invalid_number_of_public_keys_mp(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let mut validator_proof_small = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let mut validator_proof_big = validator_proof_small.clone();
        validator_proof_small.public_key_merkle_proofs.pop();
        validator_proof_big
            .public_key_merkle_proofs
            .push(Vec::new());
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment.clone(),
                validator_proof_small,
                leaf.clone(),
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfPublicKeys
        );

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof_big,
                leaf,
                load_fixture(validators, tree_size).leaf_proof.into(),
            ),
            Error::<Test>::InvalidNumberOfPublicKeys
        );
    });
}

#[test_case(69, 5000; "69 validators, 5000 leaves")]
fn submit_fixture_failed_not_once_in_bitfield(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let mut validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        // for example clear 4 pos that is used
        validator_proof.validator_claims_bitfield.clear(4);
        println!("{:?}", validator_proof.validator_claims_bitfield);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::ValidatorNotOnceInbitfield
        );
    });
}

#[test_case(69, 5000; "69 validators, 5000 leaves")]
#[test_case(200, 5000; "200 validators, 5000 leaves")]
fn submit_fixture_failed_validator_set_incorrect_position(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let mut validator_set: beefy_primitives::mmr::BeefyAuthoritySet<H256> =
            fixture.validator_set.clone().into();
        let mut next_validator_set: beefy_primitives::mmr::BeefyAuthoritySet<H256> =
            fixture.next_validator_set.clone().into();
        // just change authority set to some invalid to cause an error
        validator_set.root =
            hex!("36ee7c9903f810b22f7e6fca82c1c0cd6a151eca01f087683d92333094d94dc1").into();
        next_validator_set.root =
            hex!("36ee7c9903f810b22f7e6fca82c1c0cd6a151eca01f087683d92333094d94dc1").into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::ValidatorSetIncorrectPosition
        );
    });
}

#[test]
fn submit_fixture_failed_mmr_payload_not_found() {
    new_test_ext().execute_with(|| {
        let fixture: Fixture = serde_json::from_str(
            &std::fs::read_to_string(format!("src/fixtures/beefy-badpayload-88-1000.json"))
                .unwrap(),
        )
        .unwrap();
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();

        let validator_proof =
            validator_proof::<crate::mock::Test>(&fixture, signed_commitment.signatures, 88);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::MMRPayloadNotFound
        );
    });
}

#[test_case(37, 5000; "37 validators, 5000 leaves")]
#[test_case(69, 5000; "69 validators, 5000 leaves")]
#[test_case(200, 5000; "200 validators, 5000 leaves")]
fn submit_fixture_invalid_signature(validators: usize, tree_size: usize) {
    new_test_ext().execute_with(|| {
        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();
        assert_ok!(BeefyLightClient::initialize(
            RuntimeOrigin::root(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ));

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<crate::mock::Test>(
            &fixture,
            signed_commitment.signatures,
            validators,
        );
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

        let mut commitment1 = commitment.clone();
        let mut raw = commitment
            .payload
            .get_raw(&beefy_primitives::known_payloads::MMR_ROOT_ID)
            .unwrap()
            .clone();
        commitment1.payload = Payload::from_single_entry(*b"mm", raw.clone());

        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment1,
                validator_proof.clone(),
                leaf.clone(),
                fixture.leaf_proof.clone().into(),
            ),
            Error::<Test>::InvalidSignature
        );

        let mut commitment2 = commitment.clone();
        raw.pop();
        commitment2.payload =
            Payload::from_single_entry(beefy_primitives::known_payloads::MMR_ROOT_ID, raw);
        assert_noop!(
            BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment2,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ),
            Error::<Test>::InvalidSignature
        );
    });
}

// #[test_case(300, 8192; "300 validators, 8192 leaves")]
#[test]
fn submit_fixture_success_string_fixtures() {
    new_test_ext().execute_with(|| {
        let array = [
            (10, 128, FIXTURE_10_128),
            (20, 256, FIXTURE_20_256),
            (40, 512, FIXTURE_40_512),
            (80, 1028, FIXTURE_80_1024),
            (160, 2048, FIXTURE_160_2048),
            (200, 4096, FIXTURE_200_4096),
            (300, 8192, FIXTURE_300_8192),
        ];
        array.into_iter().for_each(|(validators, _, fixture_slice)| {
            let fixture = load_slice_fixture(fixture_slice);
            let validator_set = fixture.validator_set.clone().into();
            let next_validator_set = fixture.next_validator_set.clone().into();
            assert_ok!(BeefyLightClient::initialize(
                RuntimeOrigin::root(),
                SubNetworkId::Mainnet,
                0,
                validator_set,
                next_validator_set
            ));
    
            let signed_commitment: beefy_primitives::SignedCommitment<
                u32,
                beefy_primitives::crypto::Signature,
            > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
            let commitment = signed_commitment.commitment.clone();
            let validator_proof = validator_proof::<crate::mock::Test>(
                &fixture,
                signed_commitment.signatures,
                validators,
            );
            let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    
            assert_ok!(BeefyLightClient::submit_signature_commitment(
                RuntimeOrigin::signed(alice::<Test>()),
                SubNetworkId::Mainnet,
                commitment,
                validator_proof,
                leaf,
                fixture.leaf_proof.into(),
            ));
        });

    });
}

#[test]
fn it_works_initialize_pallet() {
    new_test_ext().execute_with(|| {
        let root = hex!("36ee7c9903f810b22f7e6fca82c1c0cd6a151eca01f087683d92333094d94dc1");
        assert_ok!(
            BeefyLightClient::initialize(
                RuntimeOrigin::root(),
                SubNetworkId::Mainnet,
                1,
                ValidatorSet {
                    id: 0,
                    len: 3,
                    root: root.into(),
                },
                ValidatorSet {
                    id: 1,
                    len: 3,
                    root: root.into(),
                }
            ),
            ().into()
        )
    });
}
