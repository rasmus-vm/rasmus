use nom::{
    bytes::complete::take,
    error::ParseError as NomParseError,
    number::complete::{le_f32, le_f64},
    IResult as NomResult, Slice,
};

use crate::{
    binary::{
        parse_trait::ParseWithNom,
        parser_helpers::{parse as nom_parse, read_s33_leb128, read_u32_leb128, read_u64_leb128},
    },
    entities::types::{
        Byte, DataIdx, ElemIdx, F32Type, F64Type, FuncIdx, GlobalIdx, GlobalType, I32Type, I64Type,
        LabelIdx, LaneIdx, LimitsType, LocalIdx, MemIdx, MemType, MutType, NameType, NumType,
        RefType, ResultType, S33Type, TableIdx, TableType, TypeIdx, U32Type, ValType, VecType,
    },
};

const ENCODE_BYTE_I32: Byte = 0x7F;
const ENCODE_BYTE_I64: Byte = 0x7E;
const ENCODE_BYTE_F32: Byte = 0x7D;
const ENCODE_BYTE_F64: Byte = 0x7C;

const ENCODE_BYTE_FUNC_REF: Byte = 0x70;
const ENCODE_BYTE_EXTERN_REF: Byte = 0x6F;
const ENCODE_BYTE_V128: Byte = 0x7B;

const ENCODE_BYTE_LIMITS_MAX_NOT_PRESENT: Byte = 0x00;
const ENCODE_BYTE_LIMITS_MAX_PRESENT: Byte = 0x01;

const ENCODE_BYTE_CONST: Byte = 0x00;
const ENCODE_BYTE_VAR: Byte = 0x01;

pub fn recognize_type(byte: Byte) -> Option<ValType> {
    match byte {
        ENCODE_BYTE_I32 => Some(ValType::NumType(NumType::I32)),
        ENCODE_BYTE_I64 => Some(ValType::NumType(NumType::I64)),
        ENCODE_BYTE_F32 => Some(ValType::NumType(NumType::F32)),
        ENCODE_BYTE_F64 => Some(ValType::NumType(NumType::F64)),
        ENCODE_BYTE_V128 => Some(ValType::VecType(VecType::V128)),
        ENCODE_BYTE_FUNC_REF => Some(ValType::RefType(RefType::FuncRef)),
        ENCODE_BYTE_EXTERN_REF => Some(ValType::RefType(RefType::ExternRef)),
        _ => None,
    }
}

impl ParseWithNom for ValType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], ValType> {
        let encode_byte_parsed = take(1usize)(bytes)?;

        recognize_type(encode_byte_parsed.1[0])
            .ok_or_else(|| {
                nom::Err::Failure(nom::error::Error::new(bytes, nom::error::ErrorKind::Fail))
            })
            .map(|val| (encode_byte_parsed.0, val))
    }
}

impl ParseWithNom for RefType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, encode_byte_slice) = take(1usize)(bytes)?;

        match encode_byte_slice[0] {
            ENCODE_BYTE_EXTERN_REF => Ok((bytes, RefType::ExternRef)),
            ENCODE_BYTE_FUNC_REF => Ok((bytes, RefType::FuncRef)),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for NameType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], NameType> {
        match U32Type::parse(bytes).and_then(|(bytes, size)| take(size.0)(bytes)) {
            Ok((bytes, name_bytes)) => std::str::from_utf8(name_bytes)
                .map(|name| (bytes, NameType(name.into())))
                .map_err(|_| {
                    nom::Err::Failure(nom::error::Error::new(bytes, nom::error::ErrorKind::Char))
                }),
            Err(err) => Err(err),
        }
    }
}

impl ParseWithNom for TableType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], TableType> {
        let (bytes, element_ref_type) = RefType::parse(bytes)?;
        let (bytes, limits) = LimitsType::parse(bytes)?;

        Ok((
            bytes,
            TableType {
                limits,
                element_ref_type,
            },
        ))
    }
}

impl ParseWithNom for LimitsType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, has_max_limit_byte_slice) = take(1usize)(bytes)?;

        match has_max_limit_byte_slice[0] {
            ENCODE_BYTE_LIMITS_MAX_NOT_PRESENT => {
                let (bytes, min) = U32Type::parse(bytes)?;

                Ok((bytes, LimitsType { min, max: None }))
            }
            ENCODE_BYTE_LIMITS_MAX_PRESENT => {
                let (bytes, min) = U32Type::parse(bytes)?;
                let (bytes, max) = U32Type::parse(bytes)?;

                Ok((
                    bytes,
                    LimitsType {
                        min,
                        max: Some(max),
                    },
                ))
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for MemType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, limits) = LimitsType::parse(bytes)?;

        Ok((bytes, MemType { limits }))
    }
}

impl ParseWithNom for GlobalType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, val_type) = ValType::parse(bytes)?;
        let (bytes, mut_type) = MutType::parse(bytes)?;

        Ok((bytes, GlobalType { mut_type, val_type }))
    }
}

impl ParseWithNom for MutType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, mut_type_byte_slice) = take(1usize)(bytes)?;

        match mut_type_byte_slice[0] {
            ENCODE_BYTE_CONST => Ok((bytes, MutType::Const)),
            ENCODE_BYTE_VAR => Ok((bytes, MutType::Var)),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for TypeIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for FuncIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for TableIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for MemIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for GlobalIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for ElemIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for DataIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for LocalIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for LabelIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for LaneIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        take(1usize)(bytes).map(|(b, v)| (b, Self(v[0])))
    }
}

impl ParseWithNom for U32Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], U32Type> {
        let mut pos = 0usize;
        let val = read_u32_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), U32Type(val)))
    }
}

impl ParseWithNom for S33Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut pos = 0usize;
        let val = read_s33_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), S33Type(val)))
    }
}

impl ParseWithNom for I32Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut pos = 0usize;
        let val = read_u32_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), Self(val)))
    }
}

impl ParseWithNom for I64Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut pos = 0usize;
        let val = read_u64_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), Self(val)))
    }
}

impl ParseWithNom for F32Type {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        le_f32(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for F64Type {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        le_f64(bytes).map(|(b, v)| (b, Self(v)))
    }
}

impl ParseWithNom for ResultType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        nom_parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

pub struct NomError;

impl NomParseError<NomError> for NomError {
    fn from_error_kind(_input: NomError, _kind: nom::error::ErrorKind) -> Self {
        NomError
    }

    fn append(_input: NomError, _kind: nom::error::ErrorKind, _other: Self) -> Self {
        NomError
    }
}
