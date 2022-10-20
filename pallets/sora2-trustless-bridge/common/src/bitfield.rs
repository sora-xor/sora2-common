use core::{
    ops::{Deref, DerefMut},
    usize,
};

use bitvec::{prelude::*, ptr::BitSpanError};
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::prelude::vec::Vec;

pub const SIZE: u128 = core::mem::size_of::<u128>() as u128;

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct BitFieldEncoded {
    pub data: Vec<u8>,
    pub remain_len: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub struct BitField(BitVec<u8, Msb0>);

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

    pub fn try_from_bit_field_encoded(
        encoded_bitfield: BitFieldEncoded,
    ) -> Result<Self, BitSpanError<u8>> {
        if encoded_bitfield.data.len() == 0 {
            return Self::try_from_slice(&[]);
        }

        if encoded_bitfield.remain_len == 0 {
            return Ok(Self::try_from_slice(&encoded_bitfield.data)?);
        }

        let mut result_bitfield =
            Self::try_from_slice(&encoded_bitfield.data[0..encoded_bitfield.data.len() - 1])?;

        let last = encoded_bitfield.data[encoded_bitfield.data.len() - 1];
        for i in 0..encoded_bitfield.remain_len {
            result_bitfield.push((last >> 7 - i & 1) == 1);
            if i >= 7 {
                break;
            }
        }
        Ok(result_bitfield)
    }

    #[inline]
    pub fn random_n_bits_with_prior_check<E>(
        prior: &BitField,
        n: u128,
        length: u128,
    ) -> Result<Self, E> {
        todo!()
    }
    // Util

    #[inline]
    pub fn to_bitfield_encoded(self) -> BitFieldEncoded {
        let remain_len = (self.len() % 8) as u8;
        BitFieldEncoded {
            data: self.to_bits(),
            remain_len,
        }
    }

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

pub fn create_bitfield(bits_to_set: Vec<u128>, length: u128) -> Vec<u128> {
    let array_length = (length + 255) / 256;
    let mut bitfield = Vec::with_capacity(array_length as usize);
    for i in 0..bits_to_set.len() {
        set(&mut bitfield, bits_to_set[i])
    }
    bitfield
}

pub fn set(self_val: &mut Vec<u128>, index: u128) {
    let element = index / SIZE;
    let within = (index % SIZE) as u8;
    // unsafe casting!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    self_val[element as usize] = self_val[element as usize] | 1 << within;
}

pub fn is_set(self_val: &Vec<u128>, index: u128) -> bool {
    let element = index / SIZE;
    let within = (index % SIZE) as u8;
    self_val[element as usize] >> within & 1 == 1
}

pub fn clear(self_val: &mut Vec<u128>, index: u128) {
    let element = index / SIZE;
    let within = (index % SIZE) as u8;
    self_val[element as usize] = self_val[element as usize] & !(1 << within);
}

// TODO
pub fn count_set_bits(_self_val: Vec<u128>) -> u128 {
    let count = 0;
    count
}

pub fn count_set_bits_bv(bitvec: BitField) -> u128 {
    bitvec.count_ones() as u128
}

pub fn create_bitfield_bv(bits_to_set: Vec<u128>, length: u128) -> BitField {
    // let mut bv = bitvec![u8, Msb0;];
    // let mut bv = BitVec::repeat(false, 10);
    // bv.push(false);
    // bv.push(true);
    // bv
}

#[cfg(test)]
mod test {
    use super::{BitField, BitFieldEncoded};

    #[test]
    fn bitfield_from_encoded() {
        let bf_encoded = BitFieldEncoded {
            data: vec![0, 1, 128],
            remain_len: 3,
        };
        let bf = BitField::try_from_bit_field_encoded(bf_encoded.clone()).unwrap();
        assert_eq!(bf.len(), 19);
        // assert_eq!(bf[bf.len() - 1], true);
        let bf_encoded_from = bf.clone().to_bitfield_encoded();
        let new_bf = BitField::try_from_bit_field_encoded(bf_encoded_from.clone()).unwrap();
        assert_eq!(new_bf.len(), bf.len());
        for i in 0..bf.len() {
            assert_eq!(bf[i], new_bf[i])
        }
    }
}
