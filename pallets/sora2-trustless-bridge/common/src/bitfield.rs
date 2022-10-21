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
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct BitField(pub BitVec<u8, Msb0>);

impl BitField {
    // Constuctors:

    #[inline]
    pub fn with_capacity(len: usize) -> Self {
        Self(BitVec::with_capacity(len))
    }

    #[inline]
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, BitSpanError<u8>> {
        Ok(Self(BitVec::try_from_slice(slice)?))
    }

    pub fn create_bitfield(bits_to_set: Vec<u128>, length: u128) -> Self {
        let mut bitfield = Self::with_capacity(length as usize);
        for i in 0..bits_to_set.len() {
            bitfield.set(bits_to_set[i] as usize)
        }
        bitfield
    }

    pub fn create_random_bitfield(prior: &BitField, n: u128, length: u128, seed: u128) -> Self {
        let mut bitfield = BitField::with_capacity(prior.len());
        for found in 0..n {
            let randomness = sp_io::hashing::blake2_128(&encode_packed(&[Token::Bytes(
                (seed + found).to_be_bytes().to_vec(),
            )]));

            let index = u128::from_be_bytes(randomness) % length;

            if !prior.is_set(index as usize) {
                continue;
            }

            if bitfield.is_set(index as usize) {
                continue;
            }

            bitfield.set(index as usize);
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
        self.0[index] == true
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
