use nom::{bytes::complete::take, IResult as NomResult};

use crate::{
    binary::{
        parse_trait::ParseWithNom,
        syntax_error::{ParseResult, SyntaxError},
    },
    entities::{
        instructions::ExpressionType,
        module::{
            Active0DataType, Active0ExprElementSegmentType, Active0FunctionsElementSegmentType,
            ActiveDataType, ActiveRefElementSegmentType, DataModeActive, DataModeActive0,
            DataModePassive, DataType, DeclarativeRefElementSegmentType, ElemKind,
            ElemKindActiveFunctionsElementSegmentType,
            ElemKindDeclarativeFunctionsElementSegmentType,
            ElemKindPassiveFunctionsElementSegmentType, ElemModeActive, ElemModeActive0,
            ElemModeDeclarative, ElemModePassive, ElementSegmentType, ExportDescription,
            GenericDataType, Global, ImportDescription, LocalsType, PassiveDataType,
            PassiveRefElementSegmentType, SectionId, SectionIdValue, StartType,
        },
        types::{
            Byte, FuncIdx, GlobalIdx, GlobalType, MemIdx, MemType, RefType, TableIdx, TableType,
            TypeIdx, U32Type, ValType,
        },
    },
};

const CUSTOM_SECTION_ID_VALUE: u8 = 0;
const TYPE_SECTION_ID_VALUE: u8 = 1;
const IMPORT_SECTION_ID_VALUE: u8 = 2;
const FUNCTION_SECTION_ID_VALUE: u8 = 3;
const TABLE_SECTION_ID_VALUE: u8 = 4;
const MEMORY_SECTION_ID_VALUE: u8 = 5;
const GLOBAL_SECTION_ID_VALUE: u8 = 6;
const EXPORT_SECTION_ID_VALUE: u8 = 7;
const START_SECTION_ID_VALUE: u8 = 8;
const ELEMENT_SECTION_ID_VALUE: u8 = 9;
const CODE_SECTION_ID_VALUE: u8 = 10;
const DATA_SECTION_ID_VALUE: u8 = 11;
const DATA_COUNT_SECTION_ID_VALUE: u8 = 12;

const BITFIELD_DATA_TYPE_ACTIVE0: U32Type = U32Type(0);
const BITFIELD_DATA_TYPE_PASSIVE: U32Type = U32Type(1);
const BITFIELD_DATA_TYPE_ACTIVE: U32Type = U32Type(2);

const ENCODE_BYTE_ELEM_KIND_FUNC_REF: Byte = 0x00;

const BITFIELD_ELEMENT_SEGMENT_ACTIVE0: U32Type = U32Type(0);
const BITFIELD_ELEMENT_SEGMENT_ELEM_KIND_PASSIVE: U32Type = U32Type(1);
const BITFIELD_ELEMENT_SEGMENT_ELEM_KIND_ACTIVE: U32Type = U32Type(2);
const BITFIELD_ELEMENT_SEGMENT_ELEM_KIND_DECLARATIVE: U32Type = U32Type(3);
const BITFIELD_ELEMENT_SEGMENT_ACTIVE0_EXPR: U32Type = U32Type(4);
const BITFIELD_ELEMENT_SEGMENT_PASSIVE_REF: U32Type = U32Type(5);
const BITFIELD_ELEMENT_SEGMENT_ACTIVE_REF: U32Type = U32Type(6);
const BITFIELD_ELEMENT_SEGMENT_DECLARATIVE_REF: U32Type = U32Type(7);

pub const ENCODE_BYTE_IMPORT_BYTE_FUNC: Byte = 0x00;
pub const ENCODE_BYTE_IMPORT_TABLE: Byte = 0x01;
pub const ENCODE_BYTE_IMPORT_MEM: Byte = 0x02;
pub const ENCODE_BYTE_IMPORT_GLOBAL: Byte = 0x03;

pub const ENCODE_BYTE_EXPORT_FUNC: Byte = 0x00;
pub const ENCODE_BYTE_EXPORT_TABLE: Byte = 0x01;
pub const ENCODE_BYTE_EXPORT_MEM: Byte = 0x02;
pub const ENCODE_BYTE_EXPORT_GLOBAL: Byte = 0x03;

impl ParseWithNom for ExportDescription {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], ExportDescription> {
        let (bytes, encode_byte) =
            take(1usize)(bytes).map(|(b, encode_byte_slice)| (b, encode_byte_slice[0]))?;

        match encode_byte {
            ENCODE_BYTE_EXPORT_FUNC => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Func(FuncIdx(u32_val)))),
            ENCODE_BYTE_EXPORT_TABLE => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Table(TableIdx(u32_val)))),
            ENCODE_BYTE_EXPORT_MEM => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Mem(MemIdx(u32_val)))),
            ENCODE_BYTE_EXPORT_GLOBAL => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ExportDescription::Global(GlobalIdx(u32_val)))),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Char,
            ))),
        }
    }
}

impl ParseWithNom for ImportDescription {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], ImportDescription> {
        let (bytes, encode_byte) =
            take(1usize)(bytes).map(|(b, encode_byte_slice)| (b, encode_byte_slice[0]))?;

        match encode_byte {
            ENCODE_BYTE_IMPORT_BYTE_FUNC => U32Type::parse(bytes)
                .map(|(b, u32_val)| (b, ImportDescription::Func(TypeIdx(u32_val)))),
            ENCODE_BYTE_IMPORT_TABLE => {
                TableType::parse(bytes).map(|(b, val)| (b, ImportDescription::Table(val)))
            }
            ENCODE_BYTE_IMPORT_MEM => {
                MemType::parse(bytes).map(|(b, val)| (b, ImportDescription::Mem(val)))
            }
            ENCODE_BYTE_IMPORT_GLOBAL => {
                GlobalType::parse(bytes).map(|(b, val)| (b, ImportDescription::Global(val)))
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Char,
            ))),
        }
    }
}

impl TryFrom<SectionIdValue> for SectionId {
    type Error = SyntaxError;

    fn try_from(byte: SectionIdValue) -> ParseResult<Self> {
        match byte {
            CUSTOM_SECTION_ID_VALUE => Ok(SectionId::Custom),
            TYPE_SECTION_ID_VALUE => Ok(SectionId::Type),
            IMPORT_SECTION_ID_VALUE => Ok(SectionId::Import),
            FUNCTION_SECTION_ID_VALUE => Ok(SectionId::Function),
            TABLE_SECTION_ID_VALUE => Ok(SectionId::Table),
            MEMORY_SECTION_ID_VALUE => Ok(SectionId::Memory),
            GLOBAL_SECTION_ID_VALUE => Ok(SectionId::Global),
            EXPORT_SECTION_ID_VALUE => Ok(SectionId::Export),
            START_SECTION_ID_VALUE => Ok(SectionId::Start),
            ELEMENT_SECTION_ID_VALUE => Ok(SectionId::Element),
            CODE_SECTION_ID_VALUE => Ok(SectionId::Code),
            DATA_SECTION_ID_VALUE => Ok(SectionId::Data),
            DATA_COUNT_SECTION_ID_VALUE => Ok(SectionId::DataCount),
            _ => Err(SyntaxError::UnexpectedSectionIdValue),
        }
    }
}

impl ParseWithNom for StartType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        U32Type::parse(bytes).map(|(b, val)| (b, StartType { func: FuncIdx(val) }))
    }
}

impl ParseWithNom for Global {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, global_type) = GlobalType::parse(bytes)?;
        let (bytes, init) = ExpressionType::parse(bytes)?;

        Ok((bytes, Global { global_type, init }))
    }
}

impl ParseWithNom for ElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, bitfield) = U32Type::parse(bytes)?;

        match bitfield {
            BITFIELD_ELEMENT_SEGMENT_ACTIVE0 => Active0FunctionsElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::Active0Functions(v))),
            BITFIELD_ELEMENT_SEGMENT_ELEM_KIND_PASSIVE => {
                ElemKindPassiveFunctionsElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::ElemKindPassiveFunctions(v)))
            }
            BITFIELD_ELEMENT_SEGMENT_ELEM_KIND_ACTIVE => {
                ElemKindActiveFunctionsElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::ElemKindActiveFunctions(v)))
            }
            BITFIELD_ELEMENT_SEGMENT_ELEM_KIND_DECLARATIVE => {
                ElemKindDeclarativeFunctionsElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::ElemKindDeclarativeFunctions(v)))
            }
            BITFIELD_ELEMENT_SEGMENT_ACTIVE0_EXPR => Active0ExprElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::Active0Expr(v))),
            BITFIELD_ELEMENT_SEGMENT_PASSIVE_REF => PassiveRefElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::PassiveRef(v))),
            BITFIELD_ELEMENT_SEGMENT_ACTIVE_REF => ActiveRefElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::ActiveRef(v))),
            BITFIELD_ELEMENT_SEGMENT_DECLARATIVE_REF => {
                DeclarativeRefElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::DeclarativeRef(v)))
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for Active0FunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, expression) = ExpressionType::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            Active0FunctionsElementSegmentType {
                mode: ElemModeActive0 { offset: expression },
                init,
            },
        ))
    }
}

impl ParseWithNom for ElemKindPassiveFunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, elem_kind) = ElemKind::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            ElemKindPassiveFunctionsElementSegmentType {
                elem_kind,
                init,
                mode: ElemModePassive {},
            },
        ))
    }
}

impl ParseWithNom for ElemKindActiveFunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, table_idx) = U32Type::parse(bytes).map(|(b, v)| (b, TableIdx(v)))?;
        let (bytes, expression) = ExpressionType::parse(bytes)?;
        let (bytes, elem_kind) = ElemKind::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            ElemKindActiveFunctionsElementSegmentType {
                elem_kind,
                init,
                mode: ElemModeActive {
                    table_idx,
                    offset: expression,
                },
            },
        ))
    }
}

impl ParseWithNom for ElemKindDeclarativeFunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, elem_kind) = ElemKind::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            ElemKindDeclarativeFunctionsElementSegmentType {
                elem_kind,
                init,
                mode: ElemModeDeclarative {},
            },
        ))
    }
}

impl ParseWithNom for Active0ExprElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, offset) = ExpressionType::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            Active0ExprElementSegmentType {
                init,
                mode: ElemModeActive0 { offset },
            },
        ))
    }
}

impl ParseWithNom for PassiveRefElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, ref_type) = RefType::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            PassiveRefElementSegmentType {
                ref_type,
                init,
                mode: ElemModePassive {},
            },
        ))
    }
}

impl ParseWithNom for ActiveRefElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, table_idx) = U32Type::parse(bytes)?;
        let (bytes, offset) = ExpressionType::parse(bytes)?;
        let (bytes, ref_type) = RefType::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            ActiveRefElementSegmentType {
                ref_type,
                init,
                mode: ElemModeActive {
                    table_idx: TableIdx(table_idx),
                    offset,
                },
            },
        ))
    }
}

impl ParseWithNom for DeclarativeRefElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, ref_type) = RefType::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            DeclarativeRefElementSegmentType {
                ref_type,
                init,
                mode: ElemModeDeclarative {},
            },
        ))
    }
}

impl ParseWithNom for ElemKind {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, elem_kind) = take(1usize)(bytes)?;

        match elem_kind[0] {
            ENCODE_BYTE_ELEM_KIND_FUNC_REF => Ok((bytes, ElemKind::FuncRef)),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for LocalsType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], LocalsType> {
        let (bytes, n) = U32Type::parse(bytes)?;
        let (bytes, val_type) = ValType::parse(bytes)?;

        Ok((bytes, LocalsType { n, val_type }))
    }
}

impl ParseWithNom for DataType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, bitfield) = U32Type::parse(bytes)?;

        match bitfield {
            BITFIELD_DATA_TYPE_ACTIVE0 => {
                Ok(Active0DataType::parse(bytes).map(|(b, v)| (b, DataType::Active0(v)))?)
            }
            BITFIELD_DATA_TYPE_ACTIVE => {
                Ok(ActiveDataType::parse(bytes).map(|(b, v)| (b, DataType::Active(v)))?)
            }
            BITFIELD_DATA_TYPE_PASSIVE => {
                Ok(PassiveDataType::parse(bytes).map(|(b, v)| (b, DataType::Passive(v)))?)
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for DataModeActive0 {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        ExpressionType::parse(bytes).map(|(b, offset)| (b, DataModeActive0 { offset }))
    }
}

impl ParseWithNom for DataModeActive {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, memory) = U32Type::parse(bytes).map(|(b, v)| (b, MemIdx(v)))?;
        ExpressionType::parse(bytes).map(|(b, offset)| (b, DataModeActive { memory, offset }))
    }
}

impl ParseWithNom for DataModePassive {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        Ok((bytes, DataModePassive {}))
    }
}

impl<Mode: ParseWithNom + std::fmt::Debug + Clone> ParseWithNom for GenericDataType<Mode> {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, mode) = Mode::parse(bytes)?;
        let (bytes, init_len) = U32Type::parse(bytes)?;
        take(init_len.0 as usize)(bytes).map(|(b, init)| {
            (
                b,
                GenericDataType {
                    mode,
                    init: init.to_vec(),
                },
            )
        })
    }
}
