use nom::{
    bytes::complete::{tag, take},
    IResult as NomResult,
};

use crate::entities::types::*;
use crate::{
    binary::{
        parse_trait::*,
        syntax_error::{ParseResult, SyntaxError},
    },
    entities::module::*,
};
pub struct ModuleParser;

macro_rules! get_section_content {
    ($bytes:expr) => {{
        let (bytes, (section_id_byte, section_content)) =
            Self::take_section($bytes).map_err(|_| SyntaxError::InvalidModuleSection)?;
        (
            bytes,
            SectionId::try_from(section_id_byte)?,
            section_content,
        )
    }};
}

impl ModuleParser {
    fn take_magic(bytes: &[Byte]) -> NomResult<&[Byte], ()> {
        tag(Module::MAGIC)(bytes).map(|(input, _)| (input, ()))
    }

    fn take_version(bytes: &[Byte]) -> NomResult<&[Byte], ()> {
        tag(Module::VERSION)(bytes).map(|(input, _)| (input, ()))
    }

    fn take_section(bytes: &[Byte]) -> NomResult<&[Byte], (u8, &[Byte])> {
        let (bytes, section_id_bytes) = take(1usize)(bytes)?;
        let (bytes, size) = U32Type::parse(bytes)?;
        let (bytes, section_content) = take(size.0)(bytes)?;

        Ok((bytes, (section_id_bytes[0], section_content)))
    }

    fn parse_types_section(bytes: &[Byte]) -> ParseResult<Vec<FuncType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidTypesModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut func_types: Vec<FuncType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_type_parsed = Self::parse_func_type(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidTypesModuleSection)?;
            remaining_bytes = func_type_parsed.0;
            func_types.push(func_type_parsed.1);
        }

        Ok(func_types)
    }

    fn parse_func_type(bytes: &[Byte]) -> NomResult<&[Byte], FuncType> {
        let (bytes, _) = tag(&[FuncType::ENCODE_BYTE_FUNC])(bytes)?;
        let (bytes, parameters_vec_len) = U32Type::parse(bytes)?;

        let mut remaining_bytes = bytes;

        let mut parameters: Vec<ValType> = Vec::with_capacity(parameters_vec_len.0 as usize);
        for _ in 0..parameters_vec_len.0 {
            let parsed_val_type = ValType::parse(remaining_bytes)?;
            parameters.push(parsed_val_type.1);
            remaining_bytes = parsed_val_type.0;
        }

        let (bytes, results_vec_len) = U32Type::parse(remaining_bytes)?;
        remaining_bytes = bytes;
        let mut results: Vec<ValType> = Vec::with_capacity(results_vec_len.0 as usize);
        for _ in 0..results_vec_len.0 {
            let parsed_val_type = ValType::parse(remaining_bytes)?;
            results.push(parsed_val_type.1);
            remaining_bytes = parsed_val_type.0;
        }

        Ok((
            remaining_bytes,
            FuncType {
                parameters,
                results,
            },
        ))
    }

    fn parse_code_section(bytes: &[Byte]) -> ParseResult<Vec<CodeType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidCodeModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut code_types: Vec<CodeType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let code_type_parsed = Self::parse_code_type(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidCodeModuleSection)?;
            remaining_bytes = code_type_parsed.0;
            code_types.push(code_type_parsed.1);
        }

        Ok(code_types)
    }

    // TODO: make it as a method of CodeType
    fn parse_code_type(bytes: &[Byte]) -> NomResult<&[Byte], CodeType> {
        let (bytes, code_len) = U32Type::parse(bytes)?;
        let (bytes, code_bytes) = take(code_len.0 as usize)(bytes)?;

        let (code_bytes, locals_len) = U32Type::parse(code_bytes)?;
        let mut remaining_bytes = code_bytes;
        let mut locals: Vec<LocalsType> = Vec::with_capacity(locals_len.0 as usize);

        for _ in 0..locals_len.0 {
            let locals_type_parsed = LocalsType::parse(remaining_bytes)?;
            remaining_bytes = locals_type_parsed.0;
            locals.push(locals_type_parsed.1);
        }

        let (_, expression) = ExpressionType::parse(remaining_bytes)?;

        Ok((
            bytes,
            CodeType {
                size: code_len,
                code: FuncCodeType { locals, expression },
            },
        ))
    }

    fn parse_funcs_section(bytes: &[Byte]) -> ParseResult<Vec<TypeIdx>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidFuncsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut func_types: Vec<TypeIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let code_type_parsed = U32Type::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidFuncsModuleSection)?;
            remaining_bytes = code_type_parsed.0;
            func_types.push(TypeIdx(code_type_parsed.1));
        }

        Ok(func_types)
    }

    fn parse_import_section(bytes: &[Byte]) -> ParseResult<Vec<ImportType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut import_types: Vec<ImportType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let module_name_parsed = NameType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let module_name = module_name_parsed.1;
            remaining_bytes = module_name_parsed.0;

            let func_name_parsed = NameType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let func_name = func_name_parsed.1;
            remaining_bytes = func_name_parsed.0;

            let import_desc_parsed = ImportDescription::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let import_desc = import_desc_parsed.1;
            remaining_bytes = import_desc_parsed.0;

            import_types.push(ImportType {
                module: module_name,
                name: func_name,
                desc: import_desc,
            })
        }

        Ok(import_types)
    }

    fn parse_table_section(bytes: &[Byte]) -> ParseResult<Vec<TableType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidTablesModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut table_types: Vec<TableType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let table_type_parsed = TableType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidTablesModuleSection)?;

            remaining_bytes = table_type_parsed.0;
            table_types.push(table_type_parsed.1);
        }

        Ok(table_types)
    }

    fn parse_memory_section(bytes: &[Byte]) -> ParseResult<Vec<MemType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed =
            U32Type::parse(remaining_bytes).map_err(|_| SyntaxError::InvalidMemsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut mem_types: Vec<MemType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let mem_type_parsed = MemType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidMemsModuleSection)?;

            remaining_bytes = mem_type_parsed.0;
            mem_types.push(mem_type_parsed.1);
        }

        Ok(mem_types)
    }

    fn parse_globals_section(bytes: &[Byte]) -> ParseResult<Vec<Global>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidGlobalsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut global_types: Vec<Global> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let global_type_parsed = Global::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidGlobalsModuleSection)?;

            remaining_bytes = global_type_parsed.0;
            global_types.push(global_type_parsed.1);
        }

        Ok(global_types)
    }

    fn parse_exports_section(bytes: &[Byte]) -> ParseResult<Vec<ExportType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut export_types: Vec<ExportType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_name_parsed = NameType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let func_name = func_name_parsed.1;
            remaining_bytes = func_name_parsed.0;

            let export_desc_parsed = ExportDescription::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
            let export_desc = export_desc_parsed.1;
            remaining_bytes = export_desc_parsed.0;

            export_types.push(ExportType {
                name: func_name,
                desc: export_desc,
            })
        }

        Ok(export_types)
    }

    fn parse_start_section(bytes: &[Byte]) -> ParseResult<StartType> {
        StartType::parse(bytes)
            .map_err(|_| SyntaxError::InvalidStartModuleSection)
            .map(|v| v.1)
    }

    fn parse_elems_section(bytes: &[Byte]) -> ParseResult<Vec<ElementSegmentType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut elems_types: Vec<ElementSegmentType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let elem_segment_type_parsed = ElementSegmentType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidElementSegmentModuleSection)?;

            remaining_bytes = elem_segment_type_parsed.0;
            elems_types.push(elem_segment_type_parsed.1);
        }

        Ok(elems_types)
    }

    fn parse_data_section(bytes: &[Byte]) -> ParseResult<Vec<DataType>> {
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)
            .map_err(|_| SyntaxError::InvalidImportsModuleSection)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut data_types: Vec<DataType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let elem_segment_type_parsed = DataType::parse(remaining_bytes)
                .map_err(|_| SyntaxError::InvalidDatasModuleSection)?;

            remaining_bytes = elem_segment_type_parsed.0;
            data_types.push(elem_segment_type_parsed.1);
        }

        Ok(data_types)
    }
}

impl ParseBin<Module> for ModuleParser {
    fn parse(bytes: &[Byte]) -> ParseResult<(Vec<Byte>, Module)> {
        let mut remainig_bytes = bytes;
        remainig_bytes = Self::take_magic(remainig_bytes)
            .map_err(|_| SyntaxError::ModuleMagicNotFound)?
            .0;
        remainig_bytes = Self::take_version(remainig_bytes)
            .map_err(|_| SyntaxError::ModuleVersionNotFound)?
            .0;

        let mut module = Module::default();

        while !remainig_bytes.is_empty() {
            let (b, section_id, section_content) = get_section_content!(remainig_bytes);
            match section_id {
                SectionId::Custom => {
                    // custom sections are not a part of the Module structure, so ignore so far
                }
                SectionId::Type => module.types = Self::parse_types_section(section_content)?,
                SectionId::Code => module.code = Self::parse_code_section(section_content)?,
                SectionId::Function => module.funcs = Self::parse_funcs_section(section_content)?,
                SectionId::Import => module.imports = Self::parse_import_section(section_content)?,
                SectionId::Table => module.tables = Self::parse_table_section(section_content)?,
                SectionId::Memory => module.mems = Self::parse_memory_section(section_content)?,
                SectionId::Global => module.globals = Self::parse_globals_section(section_content)?,
                SectionId::Export => module.exports = Self::parse_exports_section(section_content)?,
                SectionId::Start => {
                    module.start = Some(Self::parse_start_section(section_content)?)
                }
                SectionId::Element => module.elems = Self::parse_elems_section(section_content)?,
                SectionId::Data => module.datas = Self::parse_data_section(section_content)?,
                SectionId::DataCount => {
                    let (_, data_count) = U32Type::parse(section_content)
                        .map_err(|_| SyntaxError::InvalidDataCountModuleSection)?;
                    if module.datas.len() != data_count.0 as usize {
                        return Err(SyntaxError::DataCountDoesntMatchDataLen);
                    }
                }
            }
            remainig_bytes = b;
        }

        Ok((remainig_bytes.to_vec(), module))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::binary::parse_trait::ParseBin;

    #[test]
    fn test_complete_module() {
        let wasm = std::fs::read(format!(
            "{}/wasm_files/complete_module.wasm",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        ))
        .unwrap();
        let (remaining_bytes, module) = ModuleParser::parse(&wasm).unwrap();

        assert!(remaining_bytes.is_empty(), "should consume all bytes");

        assert_eq!(
            module.types,
            vec![
                FuncType {
                    parameters: vec![ValType::NumType(NumType::F32)],
                    results: vec![]
                },
                FuncType {
                    parameters: vec![],
                    results: vec![]
                },
            ],
            "module.types"
        );

        assert_eq!(
            module.imports,
            vec![
                ImportType {
                    module: NameType("foo".into()),
                    name: NameType("bar".into()),
                    desc: ImportDescription::Func(TypeIdx(U32Type(0)))
                },
                ImportType {
                    module: NameType("js".into()),
                    name: NameType("global".into()),
                    desc: ImportDescription::Global(GlobalType {
                        mut_type: MutType::Var,
                        val_type: ValType::NumType(NumType::I32)
                    })
                }
            ],
            "module.imports"
        );

        assert_eq!(
            module.funcs,
            vec![TypeIdx(U32Type(1)), TypeIdx(U32Type(1))],
            "module.funcs"
        );

        // TODO: double-check
        assert_eq!(
            module.tables,
            vec![TableType {
                element_ref_type: RefType::FuncRef,
                limits: LimitsType {
                    max: Some(U32Type(1)),
                    min: U32Type(0)
                }
            }],
            "module.tables"
        );

        // TODO: double-check
        assert_eq!(
            module.mems,
            vec![MemType {
                limits: LimitsType {
                    min: U32Type(1),
                    max: Some(U32Type(1))
                }
            }],
            "module.mems"
        );

        // FIXME:
        // assert_eq!(
        //     module.globals,
        //     vec![GlobalType {
        //         mut_type: MutType::Var,
        //         val_type: ValType::NumType(NumType::I32)
        //     }],
        //     "module.globals"
        // );

        assert_eq!(
            module.exports,
            vec![ExportType {
                name: NameType("e".into()),
                desc: ExportDescription::Func(FuncIdx(U32Type(1)))
            }],
            "module.exports"
        );

        assert_eq!(
            module.start,
            Some(StartType {
                func: FuncIdx(U32Type(1))
            }),
            "module.start"
        );

        assert_eq!(module.elems, vec![], "module.elems");

        assert_eq!(
            module.code,
            vec![
                CodeType {
                    size: U32Type(2),
                    code: FuncCodeType {
                        locals: vec![],
                        expression: ExpressionType {
                            instructions: vec![]
                        }
                    }
                },
                CodeType {
                    size: U32Type(5),
                    code: FuncCodeType {
                        locals: vec![],
                        expression: ExpressionType {
                            instructions: vec![
                                InstructionType::I32Const(I32Type(42)),
                                InstructionType::Drop
                            ]
                        }
                    }
                }
            ],
            "module.code"
        );
    }
}
