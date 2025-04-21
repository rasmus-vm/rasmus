use crate::entities::types::*;
use nom::{bytes::complete::take, IResult as NomResult};

use super::syntax_error::ParseResult;

pub trait ParseBin<T: Sized> {
    fn parse(bytes: &[u8]) -> ParseResult<(Vec<Byte>, T)>
    where
        Self: Sized;
}

pub trait ParseWithNom {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized;
}

impl<A: ParseWithNom, B: ParseWithNom> ParseWithNom for (A, B) {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, a) = A::parse(bytes)?;
        let (bytes, b) = B::parse(bytes)?;

        Ok((bytes, (a, b)))
    }
}

impl<A: ParseWithNom, B: ParseWithNom, C: ParseWithNom> ParseWithNom for (A, B, C) {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, a) = A::parse(bytes)?;
        let (bytes, b) = B::parse(bytes)?;
        let (bytes, c) = C::parse(bytes)?;

        Ok((bytes, (a, b, c)))
    }
}

// TODO: utilize it everywhere where vectors are parsed
impl<T: ParseWithNom + Sized> ParseWithNom for Vec<T> {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;

        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut types: Vec<T> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_type_parsed = T::parse(remaining_bytes)?;
            remaining_bytes = func_type_parsed.0;
            types.push(func_type_parsed.1);
        }

        Ok((remaining_bytes, types))
    }
}

impl ParseWithNom for Vec<Byte> {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, vector_len) = U32Type::parse(bytes)?;

        take(vector_len.0 as usize)(bytes).map(|(b, v)| (b, v.to_vec()))
    }
}
