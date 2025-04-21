// use crate::types::*;

use crate::entities::types::Byte;

use super::parse_trait::ParseWithNom;

use nom::{IResult as NomResult, Slice};

#[macro_export]
macro_rules! read_unsigned_leb128 {
    ($int_ty:ty) => {
        |slice: &[u8], position: &mut usize| -> $int_ty {
            // The first iteration of this loop is unpeeled. This is a
            // performance win because this code is hot and integer values less
            // than 128 are very common, typically occurring 50-80% or more of
            // the time, even for u64 and u128.
            let byte = slice[*position];
            *position += 1;
            if (byte & 0x80) == 0 {
                return byte as $int_ty;
            }
            let mut result = (byte & 0x7F) as $int_ty;
            let mut shift = 7;
            loop {
                let byte = slice[*position];
                *position += 1;
                if (byte & 0x80) == 0 {
                    result |= (byte as $int_ty) << shift;
                    return result;
                } else {
                    result |= ((byte & 0x7F) as $int_ty) << shift;
                }
                shift += 7;
            }
        }
    };
}

// Copied from https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_serialize/leb128.rs.html
macro_rules! impl_read_unsigned_leb128 {
    ($fn_name:ident, $int_ty:ty) => {
        #[inline]
        pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
            $crate::read_unsigned_leb128!($int_ty)(slice, position)
        }
    };
}

impl_read_unsigned_leb128!(read_u32_leb128, u32);
impl_read_unsigned_leb128!(read_u64_leb128, u64);

// impl_read_signed_leb128 with 33 bytes and i64 as container type
pub fn read_s33_leb128(slice: &[u8], position: &mut usize) -> i64 {
    let mut result = 0;
    let mut shift = 0;
    let mut byte;

    loop {
        byte = slice[*position];
        *position += 1;
        result |= <i64>::from(byte & 0x7F) << shift;
        shift += 7;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    if (shift < 33) && ((byte & 0x40) != 0) {
        // sign extend
        result |= !0 << shift;
    }

    result
}

// Unlike to Vec::parse this function should be used for cases when a number
// of structures is unknown
pub fn parse_all_to_vec<T>(bytes: &[Byte], till: Byte) -> NomResult<&[Byte], Vec<T>>
where
    T: ParseWithNom + Sized,
{
    let mut remaining_bytes = bytes;
    let mut accumulator: Vec<T> = Vec::new();

    while !remaining_bytes.is_empty() {
        let parsed = T::parse(remaining_bytes)?;
        remaining_bytes = parsed.0;
        accumulator.push(parsed.1);

        if remaining_bytes.first().map(|b| *b == till).unwrap_or(true) {
            break;
        }
    }

    Ok((remaining_bytes.slice(1..), accumulator))
}

// TODO: create unit test for parse_all_to_vec

pub fn parse<T>(bytes: &[Byte]) -> NomResult<&[Byte], T>
where
    T: ParseWithNom,
{
    T::parse(bytes)
}

#[macro_export]
macro_rules! impl_write_unsigned_leb128 {
    ($fn_name:ident, $int_ty:ty) => {
        #[inline]
        pub fn $fn_name(
            out: &mut [::std::mem::MaybeUninit<u8>; max_leb128_len::<$int_ty>()],
            mut value: $int_ty,
        ) -> &[u8] {
            let mut i = 0;

            loop {
                if value < 0x80 {
                    unsafe {
                        *out.get_unchecked_mut(i).as_mut_ptr() = value as u8;
                    }

                    i += 1;
                    break;
                } else {
                    unsafe {
                        *out.get_unchecked_mut(i).as_mut_ptr() = ((value & 0x7f) | 0x80) as u8;
                    }

                    value >>= 7;
                    i += 1;
                }
            }

            unsafe { ::std::mem::MaybeUninit::slice_assume_init_ref(&out.get_unchecked(..i)) }
        }
    };
}
