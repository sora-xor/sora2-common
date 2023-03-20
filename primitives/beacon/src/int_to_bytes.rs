#[cfg(not(feature = "std"))]
use crate::prelude::*;
use bytes::{BufMut, BytesMut};

/// Returns `int` as little-endian bytes with a length of 1.
pub fn int_to_bytes1(int: u8) -> Vec<u8> {
    vec![int]
}

/// Returns `int` as little-endian bytes with a length of 2.
pub fn int_to_bytes2(int: u16) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(2);
    bytes.put_u16_le(int);
    bytes.to_vec()
}

/// Returns `int` as little-endian bytes with a length of 3.
///
/// An `Option` is returned as Rust does not support a native
/// `u24` type.
///
/// The Eth 2.0 specification uses `int.to_bytes(2, 'little')`, which throws an error if `int`
/// doesn't fit within 3 bytes. The specification relies upon implicit asserts for some validity
/// conditions, so we ensure the calling function is aware of the error condition as opposed to
/// hiding it with a modulo.
pub fn int_to_bytes3(int: u32) -> Option<Vec<u8>> {
    if int < 2_u32.pow(3 * 8) {
        let mut bytes = BytesMut::with_capacity(4);
        bytes.put_u32_le(int);
        Some(bytes[0..3].to_vec())
    } else {
        None
    }
}

/// Returns `int` as little-endian bytes with a length of 4.
pub fn int_to_bytes4(int: u32) -> [u8; 4] {
    int.to_le_bytes()
}

/// Returns `int` as little-endian bytes with a length of 8.
pub fn int_to_bytes8(int: u64) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(8);
    bytes.put_u64_le(int);
    bytes.to_vec()
}

/// Returns `int` as little-endian bytes with a length of 32.
pub fn int_to_bytes32(int: u64) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(32);
    bytes.put_u64_le(int);
    bytes.resize(32, 0);
    bytes.to_vec()
}

/// Returns `int` as little-endian bytes with a length of 32.
pub fn int_to_fixed_bytes32(int: u64) -> [u8; 32] {
    let mut bytes = [0; 32];
    let int_bytes = int.to_le_bytes();
    bytes[0..int_bytes.len()].copy_from_slice(&int_bytes);
    bytes
}

/// Returns `int` as little-endian bytes with a length of 48.
pub fn int_to_bytes48(int: u64) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(48);
    bytes.put_u64_le(int);
    bytes.resize(48, 0);
    bytes.to_vec()
}

/// Returns `int` as little-endian bytes with a length of 96.
pub fn int_to_bytes96(int: u64) -> Vec<u8> {
    let mut bytes = BytesMut::with_capacity(96);
    bytes.put_u64_le(int);
    bytes.resize(96, 0);
    bytes.to_vec()
}
