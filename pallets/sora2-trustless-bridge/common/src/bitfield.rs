use core::{ops::Deref, usize};

use scale_info::prelude::vec::Vec;
use bitvec::{prelude::*, ptr::BitSpanError};

pub const SIZE: u128 = core::mem::size_of::<u128>() as u128;
pub struct BitField(BitVec<u8, Msb0>);

impl BitField {
    // Constuctors:

    pub fn with_capacity(bit: bool, len: usize) -> Self {
        Self(BitVec::repeat(bit, len))
    }

    pub fn try_from_slice(slice: &[u8]) -> Result<Self, BitSpanError<u8>> {
        Ok(Self(BitVec::try_from_slice(slice)?))
    }

    // Util

    pub fn count_set_bits(&self) -> u128 {
        self.0.count_ones() as u128
    }

    pub fn to_bits(self) -> Vec<u8> {
        self.0.into_vec()
    }

    pub fn set(&mut self, index: usize) {
        self.0.set(index, true)
    }

    pub fn clear(&mut self, index: usize) {
        self.0.set(index, false)
    }

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

// pub fn create_bitfield_bv(bits_to_set: Vec<u128>, length: u128) -> BitField {
//     // let mut bv = bitvec![u8, Msb0;];
//     // let mut bv = BitVec::repeat(false, 10);
//     // bv.push(false);
//     // bv.push(true);
//     // bv
// }

#[cfg(test)]
mod test {
    #[test]
    fn is_set_returns_ok() {
        let a = 0b0010 as u8;
        assert_eq!(a >> 1 & 1, 1)
        // assert_eq!(a & 1 << 1, 1)
    }
}
