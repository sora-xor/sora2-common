use core::{
    ops::{Deref, DerefMut},
    usize,
};

use bitvec::{prelude::*, ptr::BitSpanError};
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::{prelude::vec::Vec, TypeInfo};

pub const SIZE: u128 = core::mem::size_of::<u128>() as u128;

#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct BitFieldEncoded {
    pub data: Vec<u8>,
    pub remain_len: u8,
}

// #[derive(Clone, PartialEq, Eq)]
// pub struct BitField{
//     pub bitvec: BitVec<u8, Msb0>, 
//     pub remain_len: u8,
// }

#[derive(Clone, PartialEq, Eq)]
pub struct BitField(
    pub BitVec<u8, Msb0>
);

// impl Encode for BitField {

// }

// impl Decode for BitField {
//     const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

//     fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
//         todo!()
//     }
// }

// impl scale_info::TypeInfo for BitField {
//     type Identity;

//     fn type_info() -> scale_info::Type {
//         todo!()
//     }
// }

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

    pub fn create_bitfield(bits_to_set: Vec<u128>, length: u128) -> Self  {
        let mut bitfield = Self::with_capacity(length as usize);
        for i in 0..bits_to_set.len() {
            bitfield.set(bits_to_set[i] as usize)
        }
        bitfield
    }

    // Util:

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
