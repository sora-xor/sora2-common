use codec::{Decode, Encode};
use ethabi::{encode_packed, Token};
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;
use sp_io::hashing::keccak_256;
use frame_support::log;

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct SimplifiedMMRProof {
    pub merkle_proof_items: Vec<[u8; 32]>,
    pub merkle_proof_order_bit_field: u64,
}

pub fn verify_inclusion_proof(
    root: [u8; 32],
    leaf_node_hash: [u8; 32],
    proof: SimplifiedMMRProof,
) -> bool {
    if proof.merkle_proof_items.len() >= 64 {
        return false;
    }
    log::debug!("verify_inclusion_proof: proof: {:?}", proof);
    root == calculate_merkle_root(
        leaf_node_hash,
        proof.merkle_proof_items,
        proof.merkle_proof_order_bit_field,
    )
}

pub fn bit(self_val: u64, index: u64) -> bool {
    ((self_val >> index) & 1) as u8 == 1
}

pub fn calculate_merkle_root(
    leaf_node_hash: [u8; 32],
    merkle_proof_items: Vec<[u8; 32]>,
    merkle_proof_order_bit_field: u64,
) -> [u8; 32] {
    let mut current_hash = leaf_node_hash;
    for current_position in 0..merkle_proof_items.len() {
        let is_sibling_left = bit(merkle_proof_order_bit_field, current_position as u64);
        let sibling = merkle_proof_items[current_position];
        current_hash = if is_sibling_left {
            keccak_256(&encode_packed(&[
                Token::Bytes(sibling.into()),
                Token::Bytes(current_hash.into()),
            ]))
        } else {
            keccak_256(&encode_packed(&[
                Token::Bytes(current_hash.into()),
                Token::Bytes(sibling.into()),
            ]))
        };
    }
    current_hash
}

#[cfg(test)]
mod tests {
    use frame_support::{assert_ok};
    use super::*;

    #[test]
    pub fn it_works_calculate_merkle_root() {
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

        // assert_eq!(root, calculate_merkle_root(leafs[0], proof_a, 0, 7, proof_a));
        // assert_eq!(root, calculate_merkle_root(root, hashs[5], 5, 7, proof_f.clone()));
        // assert!(calculate_merkle_root(root, hashs[4], 5, 7, proof_f.clone()));
        // assert!(calculate_merkle_root(root, hashs[5], 1, 7, proof_f));
    }
}