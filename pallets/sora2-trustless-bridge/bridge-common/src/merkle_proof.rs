use ethabi::{encode_packed, Token};
use frame_support::log;
use scale_info::prelude::vec::Vec;
use sp_io::hashing::keccak_256;


pub fn verify_merkle_leaf_at_position(
    root: [u8; 32],
    leaf: [u8; 32],
    pos: u128,
    width: u128,
    proof: Vec<[u8; 32]>,
) -> Result<(), MerkleProofError> {
    let computed_hash = compute_root_from_proof_at_position(leaf, pos, width, proof)?;
    log::debug!("verify_merkle_leaf_at_position: leaf: {:?}", leaf);
    log::debug!("verify_merkle_leaf_at_position: root: {:?}", root);
    log::debug!(
        "verify_merkle_leaf_at_position: computed_hash: {:?}",
        computed_hash
    );
    log::debug!(
        "POS: {:?}, WIDTH: {:?}",
        pos,
        width
    );
    if root != computed_hash {
        return Err(MerkleProofError::RootComputedHashNotEqual);
    }
    Ok(())
}

pub fn compute_root_from_proof_at_position(
    leaf: [u8; 32],
    mut pos: u128,
    mut width: u128,
    proof: Vec<[u8; 32]>,
) -> Result<[u8; 32], MerkleProofError> {
    if pos >= width {
        return Err(MerkleProofError::MerklePositionTooHigh);
    }
    let mut computed_hash = leaf;

    let mut computed_hash_left: bool;
    let mut proof_element: [u8; 32];

    let mut i: u128 = 0;
    while width > 1 {
        computed_hash_left = pos % 2 == 0;

        // check if at rightmost branch and whether the computedHash is left
        if pos + 1 == width && computed_hash_left {
            // there is no sibling and also no element in proofs, so we just go up one layer in the tree
            pos /= 2;
            width = ((width - 1) / 2) + 1;
            continue;
        }

        if i >= proof.len() as u128 {
            return Err(MerkleProofError::MerkleProofTooShort);
        }

        proof_element = proof[i as usize];
        computed_hash = if computed_hash_left {
            keccak_256(&encode_packed(&[
                Token::Bytes(computed_hash.into()),
                Token::Bytes(proof_element.into()),
            ]))
        } else {
            keccak_256(&encode_packed(&[
                Token::Bytes(proof_element.into()),
                Token::Bytes(computed_hash.into()),
            ]))
        };

        pos /= 2;
        width = ((width - 1) / 2) + 1;

        // increments:
        i += 1;
    }

    if i < proof.len() as u128 {
        log::debug!("==================== {:?} >= {:?} =======================", i, proof.len());
        return Err(MerkleProofError::MerkleProofTooHigh);
    }

    Ok(computed_hash)
}

#[derive(Debug,PartialEq,Eq)]
pub enum MerkleProofError {
    MerklePositionTooHigh,
    MerkleProofTooShort,
    MerkleProofTooHigh,
    RootComputedHashNotEqual,
}

#[cfg(test)]
mod tests {
    use frame_support::{assert_ok};
    use super::*;


    #[test]
    pub fn it_works_verify_merkle_leaf_at_position() {
        let leafs = vec![b"a", b"b", b"c", b"d", b"e", b"f", b"g"];
        let hashs: Vec<[u8; 32]> = leafs.iter()
                                    .map(|x| keccak_256(&encode_packed(&[Token::Bytes(vec![x[0]])]))).collect();
        let hab = keccak_256(&encode_packed(&[
            Token::Bytes(hashs[0].into()),
            Token::Bytes(hashs[1].into()),
        ]));
        let hcd = keccak_256(&encode_packed(&[
            Token::Bytes(hashs[2].into()),
            Token::Bytes(hashs[3].into()),
        ]));
        let hef = keccak_256(&encode_packed(&[
            Token::Bytes(hashs[4].into()),
            Token::Bytes(hashs[5].into()),
        ]));
        let habcd =  keccak_256(&encode_packed(&[
            Token::Bytes(hab.into()),
            Token::Bytes(hcd.into()),
        ]));
        let hefg = keccak_256(&encode_packed(&[
            Token::Bytes(hef.into()),
            Token::Bytes(hashs[6].into()),
        ]));
        let root =  keccak_256(&encode_packed(&[
            Token::Bytes(habcd.into()),
            Token::Bytes(hefg.into()),
        ]));
        let proof_a = vec![hashs[1], hcd, hefg];
        let proof_f = vec![hashs[4], hashs[6], habcd];

        assert_ok!(verify_merkle_leaf_at_position(root, hashs[0], 0, 7, proof_a));
        assert_ok!(verify_merkle_leaf_at_position(root, hashs[5], 5, 7, proof_f.clone()));
        assert_eq!(verify_merkle_leaf_at_position(root, hashs[4], 5, 7, proof_f.clone()), Err(MerkleProofError::RootComputedHashNotEqual));
        assert_eq!(verify_merkle_leaf_at_position(root, hashs[5], 1, 7, proof_f), Err(MerkleProofError::RootComputedHashNotEqual));
    }
}