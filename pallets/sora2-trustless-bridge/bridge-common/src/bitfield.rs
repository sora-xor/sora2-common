use core::{
    ops::{Deref, DerefMut},
    usize,
};

use bitvec::{prelude::*, ptr::BitSpanError};
use codec::{Decode, Encode};
use ethabi::{encode_packed, Token};
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;

pub const SIZE: u128 = core::mem::size_of::<u128>() as u128;

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
    serde::Serialize,
    serde::Deserialize,
)]
pub struct BitField(pub BitVec<u8, Msb0>);

impl BitField {
    // Constuctors:

    #[inline]
    pub fn with_capacity(len: usize) -> Self {
        Self(BitVec::with_capacity(len))
    }

    #[inline]
    pub fn with_zeroes(len: usize) -> Self {
        Self(BitVec::repeat(false, len))
    }

    #[inline]
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, BitSpanError<u8>> {
        Ok(Self(BitVec::try_from_slice(slice)?))
    }

    pub fn create_bitfield(bits_to_set: Vec<u128>, length: u128) -> Self {
        let mut bitfield = Self::with_zeroes(length as usize);
        for i in 0..bits_to_set.len() {
            bitfield.set(bits_to_set[i] as usize)
        }
        bitfield
    }

    pub fn create_random_bitfield(prior: &BitField, n: u128, length: u128, seed: u128) -> Self {
        let mut bitfield = BitField::with_zeroes(prior.len());
        let mut found = 0;
        let mut i = 0;
        while found < n {
            // for found in 0..n {
            let randomness = sp_io::hashing::blake2_128(&encode_packed(&[Token::Bytes(
                (seed + i).to_be_bytes().to_vec(),
            )]));

            let index = u128::from_be_bytes(randomness) % length;

            if !prior.is_set(index as usize) {
                i += 1;
                continue;
            }

            if bitfield.is_set(index as usize) {
                i += 1;
                continue;
            }

            bitfield.set(index as usize);
            found += 1;
            i += 1;
        }
        bitfield
    }

    // Util:
    #[inline]
    pub fn count_set_bits(&self) -> u128 {
        self.0.count_ones() as u128
    }

    #[inline]
    pub fn to_bits(self) -> Vec<u8> {
        self.0.into_vec()
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        self.0.set(index, true)
    }

    #[inline]
    pub fn clear(&mut self, index: usize) {
        self.0.set(index, false)
    }

    #[inline]
    pub fn is_set(&self, index: usize) -> bool {
        self.0[index]
    }
}

impl Deref for BitField {
    type Target = BitVec<u8, Msb0>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn create_bitfield_success() {
        let bits_to_set = vec![0, 1, 2];
        let len = 3;
        let bf = super::BitField::create_bitfield(bits_to_set, len);
        assert!(bf[0]);
        assert!(bf[1]);
        assert!(bf[2]);
    }
}
