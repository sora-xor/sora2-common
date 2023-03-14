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
// #![recursion_limit = "512"]
use super::*;

use crate::test_helpers::*;
// use crate::benchmark_features::*;
#[allow(unused)]
use crate::Pallet as BeefyLightClient;
use crate::my_file::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use hex_literal::hex;



benchmarks! {
    // initialize {
    //     let root = hex!("36ee7c9903f810b22f7e6fca82c1c0cd6a151eca01f087683d92333094d94dc1");
    //     let curr_val_set = ValidatorSet {
    //         id: 0,
    //         len: 3,
    //         root: root.into(),
    //     };
    //     let next_val_set = ValidatorSet {
    //         id: 1,
    //         len: 3,
    //         root: root.into(),
    //     };
    // }: _(RawOrigin::Root, SubNetworkId::Mainnet, 1, curr_val_set, next_val_set)
    // verify {
    //     assert!(BeefyLightClient::<T>::current_validator_set(SubNetworkId::Mainnet).is_some());
    //     assert!(BeefyLightClient::<T>::next_validator_set(SubNetworkId::Mainnet).is_some());
    // }


    // submit_signature_commitment_10_128 {
    //     let validators = 10;
    //     let tree_size = 128;

    //     let fixture = load_slice_fixture(FIXTURE_10_128);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }

    // submit_signature_commitment_20_256 {
    //     let validators = 20;
    //     let tree_size = 256;

    //     let fixture = load_slice_fixture(FIXTURE_20_256);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }

    // submit_signature_commitment_40_512 {
    //     let validators = 40;
    //     let tree_size = 512;

    //     let fixture = load_slice_fixture(FIXTURE_40_512);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }

    // submit_signature_commitment_80_1024 {
    //     let validators = 80;
    //     let tree_size = 1024;

    //     let fixture = load_slice_fixture(FIXTURE_80_1024);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }

    // submit_signature_commitment_160_2048 {
    //     let validators = 160;
    //     let tree_size = 2048;

    //     let fixture = load_slice_fixture(FIXTURE_160_2048);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }

    // submit_signature_commitment_200_4096 {
    //     let validators = 200;
    //     let tree_size = 4096;

    //     let fixture = load_slice_fixture(FIXTURE_200_4096);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }

    // submit_signature_commitment_300_8192 {
    //     let validators = 300;
    //     let tree_size = 8192;

    //     let fixture = load_slice_fixture(FIXTURE_300_8192);
    //     let validator_set = fixture.validator_set.clone().into();
    //     let next_validator_set = fixture.next_validator_set.clone().into();

    //     BeefyLightClient::<T>::initialize(
    //         RawOrigin::Root.into(),
    //         SubNetworkId::Mainnet,
    //         0,
    //         validator_set,
    //         next_validator_set
    //     ).expect("Error while initializing pallet");

    //     let signed_commitment: beefy_primitives::SignedCommitment<
    //         u32,
    //         beefy_primitives::crypto::Signature,
    //     > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
    //     let commitment = signed_commitment.commitment.clone();
    //     let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
    //     let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    // }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    // verify {
    //     assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    // }


    submit_signature_commitment_10_128 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_10_256 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_10_512 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_10_1024 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_10_2048 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_10_4096 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_10_8192 {
        let validators = 10;

        let fixture = load_slice_fixture(FIXTURE_10_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 10);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_128 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_256 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_512 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_1024 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_2048 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_4096 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_20_8192 {
        let validators = 20;

        let fixture = load_slice_fixture(FIXTURE_20_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 20);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_128 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_256 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_512 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_1024 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_2048 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_4096 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_30_8192 {
        let validators = 30;

        let fixture = load_slice_fixture(FIXTURE_30_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 30);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_128 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_256 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_512 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_1024 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_2048 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_4096 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_40_8192 {
        let validators = 40;

        let fixture = load_slice_fixture(FIXTURE_40_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 40);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_128 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_256 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_512 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_1024 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_2048 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_4096 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_50_8192 {
        let validators = 50;

        let fixture = load_slice_fixture(FIXTURE_50_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 50);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_128 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_256 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_512 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_1024 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_2048 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_4096 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_60_8192 {
        let validators = 60;

        let fixture = load_slice_fixture(FIXTURE_60_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 60);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_128 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_256 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_512 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_1024 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_2048 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_4096 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_70_8192 {
        let validators = 70;

        let fixture = load_slice_fixture(FIXTURE_70_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 70);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_128 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_256 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_512 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_1024 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_2048 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_4096 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_80_8192 {
        let validators = 80;

        let fixture = load_slice_fixture(FIXTURE_80_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 80);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_128 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_256 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_512 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_1024 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_2048 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_4096 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_90_8192 {
        let validators = 90;

        let fixture = load_slice_fixture(FIXTURE_90_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 90);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_128 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_256 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_512 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_1024 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_2048 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_4096 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_100_8192 {
        let validators = 100;

        let fixture = load_slice_fixture(FIXTURE_100_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 100);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_128 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_256 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_512 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_1024 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_2048 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_4096 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_110_8192 {
        let validators = 110;

        let fixture = load_slice_fixture(FIXTURE_110_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 110);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_128 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_256 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_512 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_1024 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_2048 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_4096 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_120_8192 {
        let validators = 120;

        let fixture = load_slice_fixture(FIXTURE_120_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 120);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_128 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_256 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_512 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_1024 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_2048 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_4096 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_130_8192 {
        let validators = 130;

        let fixture = load_slice_fixture(FIXTURE_130_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 130);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_128 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_256 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_512 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_1024 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_2048 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_4096 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_140_8192 {
        let validators = 140;

        let fixture = load_slice_fixture(FIXTURE_140_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 140);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_128 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_256 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_512 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_1024 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_2048 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_4096 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_150_8192 {
        let validators = 150;

        let fixture = load_slice_fixture(FIXTURE_150_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 150);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_128 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_128);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_256 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_256);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_512 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_512);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_1024 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_1024);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_2048 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_2048);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_4096 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_4096);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }



    submit_signature_commitment_160_8192 {
        let validators = 160;

        let fixture = load_slice_fixture(FIXTURE_160_8192);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        ).expect("Error while initializing pallet");

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, 160);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();
    }: submit_signature_commitment(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
        assert!(BeefyLightClient::<T>::latest_mmr_roots(SubNetworkId::Mainnet).len() > 0);
    }
}

impl_benchmark_test_suite!(
    BeefyLightClient,
    crate::mock::new_test_ext(),
    crate::mock::Test,
);
