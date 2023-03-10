macro_rules! impl_from_into_u64 {
    ($main: ident) => {
        impl From<u64> for $main {
            fn from(n: u64) -> $main {
                $main(n)
            }
        }

        impl Into<u64> for $main {
            fn into(self) -> u64 {
                self.0
            }
        }

        impl $main {
            pub fn as_u64(&self) -> u64 {
                self.0
            }
        }
    };
}

macro_rules! impl_from_into_usize {
    ($main: ident) => {
        impl From<usize> for $main {
            fn from(n: usize) -> $main {
                $main(n as u64)
            }
        }

        impl Into<usize> for $main {
            fn into(self) -> usize {
                self.0 as usize
            }
        }

        impl $main {
            pub fn as_usize(&self) -> usize {
                self.0 as usize
            }
        }
    };
}

macro_rules! impl_u64_eq_ord {
    ($type: ident) => {
        impl PartialEq<u64> for $type {
            fn eq(&self, other: &u64) -> bool {
                self.as_u64() == *other
            }
        }

        impl PartialOrd<u64> for $type {
            fn partial_cmp(&self, other: &u64) -> Option<core::cmp::Ordering> {
                self.as_u64().partial_cmp(other)
            }
        }
    };
}

macro_rules! impl_safe_arith {
    ($type: ident, $rhs_ty: ident) => {
        impl crate::safe_arith::SafeArith<$rhs_ty> for $type {
            const ZERO: Self = $type::new(0);
            const ONE: Self = $type::new(1);

            fn safe_add(&self, other: $rhs_ty) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_add(other.into())
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::Overflow)
            }

            fn safe_sub(&self, other: $rhs_ty) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_sub(other.into())
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::Overflow)
            }

            fn safe_mul(&self, other: $rhs_ty) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_mul(other.into())
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::Overflow)
            }

            fn safe_div(&self, other: $rhs_ty) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_div(other.into())
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::DivisionByZero)
            }

            fn safe_rem(&self, other: $rhs_ty) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_rem(other.into())
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::DivisionByZero)
            }

            fn safe_shl(&self, other: u32) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_shl(other)
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::Overflow)
            }

            fn safe_shr(&self, other: u32) -> crate::safe_arith::Result<Self> {
                self.0
                    .checked_shr(other)
                    .map(Self::new)
                    .ok_or(crate::safe_arith::ArithError::Overflow)
            }
        }
    };
}

macro_rules! impl_math_between {
    ($main: ident, $other: ident) => {
        impl Add<$other> for $main {
            type Output = $main;

            fn add(self, other: $other) -> $main {
                $main::from(self.0.saturating_add(other.into()))
            }
        }

        impl AddAssign<$other> for $main {
            fn add_assign(&mut self, other: $other) {
                self.0 = self.0.saturating_add(other.into());
            }
        }

        impl Sub<$other> for $main {
            type Output = $main;

            fn sub(self, other: $other) -> $main {
                $main::from(self.0.saturating_sub(other.into()))
            }
        }

        impl SubAssign<$other> for $main {
            fn sub_assign(&mut self, other: $other) {
                self.0 = self.0.saturating_sub(other.into());
            }
        }

        impl Mul<$other> for $main {
            type Output = $main;

            fn mul(self, rhs: $other) -> $main {
                let rhs: u64 = rhs.into();
                $main::from(self.0.saturating_mul(rhs))
            }
        }

        impl MulAssign<$other> for $main {
            fn mul_assign(&mut self, rhs: $other) {
                let rhs: u64 = rhs.into();
                self.0 = self.0.saturating_mul(rhs)
            }
        }

        impl Div<$other> for $main {
            type Output = $main;

            fn div(self, rhs: $other) -> $main {
                let rhs: u64 = rhs.into();
                $main::from(
                    self.0
                        .checked_div(rhs)
                        .expect("Cannot divide by zero-valued Slot/Epoch"),
                )
            }
        }

        impl DivAssign<$other> for $main {
            fn div_assign(&mut self, rhs: $other) {
                let rhs: u64 = rhs.into();
                self.0 = self
                    .0
                    .checked_div(rhs)
                    .expect("Cannot divide by zero-valued Slot/Epoch");
            }
        }

        impl Rem<$other> for $main {
            type Output = $main;

            fn rem(self, modulus: $other) -> $main {
                let modulus: u64 = modulus.into();
                $main::from(
                    self.0
                        .checked_rem(modulus)
                        .expect("Cannot divide by zero-valued Slot/Epoch"),
                )
            }
        }
    };
}

macro_rules! impl_math {
    ($type: ident) => {
        impl $type {
            pub fn saturating_sub<T: Into<$type>>(&self, other: T) -> $type {
                $type::new(self.as_u64().saturating_sub(other.into().as_u64()))
            }

            pub fn saturating_add<T: Into<$type>>(&self, other: T) -> $type {
                $type::new(self.as_u64().saturating_add(other.into().as_u64()))
            }

            pub fn is_power_of_two(&self) -> bool {
                self.0.is_power_of_two()
            }
        }
    };
}

macro_rules! impl_display {
    ($type: ident) => {
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! impl_debug {
    ($type: ident) => {
        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}({:?})", stringify!($type), self.0)
            }
        }
    };
}

macro_rules! impl_ssz {
    ($type: ident) => {
        impl Encode for $type {
            fn is_ssz_fixed_len() -> bool {
                <u64 as Encode>::is_ssz_fixed_len()
            }

            fn ssz_fixed_len() -> usize {
                <u64 as Encode>::ssz_fixed_len()
            }

            fn ssz_bytes_len(&self) -> usize {
                0_u64.ssz_bytes_len()
            }

            fn ssz_append(&self, buf: &mut Vec<u8>) {
                self.0.ssz_append(buf)
            }
        }

        impl Decode for $type {
            fn is_ssz_fixed_len() -> bool {
                <u64 as Decode>::is_ssz_fixed_len()
            }

            fn ssz_fixed_len() -> usize {
                <u64 as Decode>::ssz_fixed_len()
            }

            fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
                Ok($type(u64::from_ssz_bytes(bytes)?))
            }
        }

        impl tree_hash::TreeHash for $type {
            fn tree_hash_type() -> tree_hash::TreeHashType {
                tree_hash::TreeHashType::Basic
            }

            fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
                self.0.tree_hash_packed_encoding()
            }

            fn tree_hash_packing_factor() -> usize {
                32usize.wrapping_div(8)
            }

            fn tree_hash_root(&self) -> tree_hash::Hash256 {
                tree_hash::Hash256::from_slice(&crate::int_to_bytes::int_to_fixed_bytes32(self.0))
            }
        }

        impl SignedRoot for $type {}
    };
}

macro_rules! impl_from_str {
    ($type: ident) => {
        impl core::str::FromStr for $type {
            type Err = core::num::ParseIntError;

            fn from_str(s: &str) -> Result<$type, Self::Err> {
                u64::from_str(s).map($type)
            }
        }
    };
}

macro_rules! impl_common {
    ($type: ident) => {
        impl_from_into_u64!($type);
        impl_from_into_usize!($type);
        impl_u64_eq_ord!($type);
        impl_safe_arith!($type, $type);
        impl_safe_arith!($type, u64);
        impl_math_between!($type, $type);
        impl_math_between!($type, u64);
        impl_math!($type);
        impl_display!($type);
        impl_debug!($type);
        impl_ssz!($type);
        impl_from_str!($type);
    };
}
