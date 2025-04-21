use crate::{
    entities::{
        instructions::{BlockInstructionType, BlockType, ExpressionType, InstructionType},
        module::{
            Active0DataType, Active0ExprElementSegmentType, Active0FunctionsElementSegmentType,
            ActiveDataType, ActiveRefElementSegmentType, CodeType, DataModeActive, DataModeActive0,
            DataType, ElemKindActiveFunctionsElementSegmentType, ElemModeActive, ElemModeActive0,
            ElementSegmentType, ExportDescription, ExportType, Global, ImportDescription,
            ImportType, Module, StartType,
        },
        types::{FuncIdx, FuncType, MemType, MutType, NumType, ResultType, TableType, ValType},
    },
    instances::module::ExternalDependency,
    validation::{validation_error::ValidationError, validation_stack::ValidationStack},
};

use super::{
    context::ValidationContext,
    types_validation::{
        is_memory_type_valid, is_table_type_valid, validate_export_func_type, validate_func_type,
        validate_global_type,
    },
    validate_instruction::validate_instruction,
    validation_error::ValidationResult,
};

pub fn validate(module: &Module, externals: &Vec<ExternalDependency>) -> ValidationResult<()> {
    let ctx = create_context(module, externals);
    let sub_ctx = create_sub_context(module, externals);
    let mut val_stack = ValidationStack::new();

    for func_idx in &module.funcs {
        let idx = func_idx.0 .0 as usize;
        let func_code = module.code.get(idx).ok_or(ValidationError::CodeNotFound)?;
        let func_type = module
            .funcs
            .get(idx)
            .and_then(|func_idx| module.types.get(func_idx.0 .0 as usize))
            .ok_or(ValidationError::TypeNotFound)?;

        validate_func(&mut val_stack, &ctx, func_type, func_code)?;
    }

    if let Some(ref start_type) = module.start {
        validate_start_function(&ctx, start_type)?;
    }

    for import_type in &module.imports {
        validate_import(&ctx, import_type)?;
    }

    for export_type in &module.exports {
        validate_export(&ctx, export_type)?;
    }

    for table_type in &module.tables {
        validate_table(table_type)?;
    }

    for memory_type in &module.mems {
        validate_memory(memory_type)?;
    }

    for global in &module.globals {
        validate_global(global, &sub_ctx)?;
    }

    for elem in &module.elems {
        validate_elem(elem, &sub_ctx)?;
    }

    for data in &module.datas {
        validate_data(&data, &sub_ctx)?;
    }

    Ok(())
}

fn validate_func(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    func_type: &FuncType,
    code: &CodeType,
) -> ValidationResult<()> {
    let mut func_ctx = ctx.clone();
    for param in &func_type.parameters {
        func_ctx.locals.insert(0, param.clone());
    }
    func_ctx.labels = vec![ResultType(func_type.results.clone())];
    func_ctx.maybe_return = Some(ResultType(func_type.results.clone()));

    val_stack.push_ctrl(
        // should be administrative instruction frame
        InstructionType::Block(BlockInstructionType {
            blocktype: BlockType::Empty,
            instructions: code.code.expression.instructions.clone(),
        }),
        func_type.parameters.clone(),
        func_type.results.clone(),
        false,
    );

    for instruction in &code.code.expression.instructions {
        validate_instruction(instruction, &func_ctx, val_stack)?;
    }

    Ok(())
}

fn validate_table(table_type: &TableType) -> ValidationResult<()> {
    if !is_table_type_valid(table_type) {
        return Err(ValidationError::InvalidTableType {
            table_type: table_type.clone(),
        });
    }

    Ok(())
}

fn validate_memory(mem_type: &MemType) -> ValidationResult<()> {
    if !is_memory_type_valid(mem_type) {
        return Err(ValidationError::InvalidMemoryType {
            memory_type: mem_type.clone(),
        });
    }

    Ok(())
}

fn validate_global(global: &Global, ctx: &ValidationContext) -> ValidationResult<()> {
    let global_type = &global.global_type;
    let init = &global.init;

    validate_constant_expression(init, &ResultType(vec![global_type.val_type.clone()]), ctx)
}

fn validate_constant_expression(
    expression: &ExpressionType,
    result_type: &ResultType,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    let mut val_stack = ValidationStack::new();
    val_stack.push_ctrl(
        InstructionType::Block(BlockInstructionType {
            blocktype: BlockType::Empty,
            instructions: vec![],
        }),
        vec![],
        result_type.0.clone(),
        false,
    );

    for instr in &expression.instructions {
        validate_instruction_constant(&instr, ctx)?;
        validate_instruction(instr, ctx, &mut val_stack)?;
    }

    val_stack.pop_ctrl()?;

    Ok(())
}

fn validate_instruction_constant(
    instr: &InstructionType,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    match instr {
        InstructionType::I32Const(_)
        | InstructionType::I64Const(_)
        | InstructionType::F32Const(_)
        | InstructionType::F64Const(_)
        | InstructionType::V128Const(_)
        | InstructionType::RefNull(_)
        | InstructionType::RefFunc(_) => Ok(()),
        InstructionType::GlobalGet(global_idx) => {
            // TODO: or of the form global.get x, in which case C.globals[x] must be a global type of the form const t.
            // shall we recursively validate global init expression to be constant?
            ctx.globals
                .get(global_idx.0 .0 as usize)
                .ok_or_else(|| ValidationError::NonConstantInstruction {
                    instruction: instr.clone(),
                })
                .and_then(|global_type| {
                    if global_type.mut_type == MutType::Const {
                        Ok(())
                    } else {
                        Err(ValidationError::NonConstantInstruction {
                            instruction: instr.clone(),
                        })
                    }
                })
        }
        _ => Err(ValidationError::NonConstantInstruction {
            instruction: instr.clone(),
        }),
    }
}

fn validate_start_function(
    ctx: &ValidationContext,
    start_type: &StartType,
) -> ValidationResult<()> {
    let func_idx = start_type.func.0 .0 as usize;
    ctx.funcs
        .get(func_idx)
        .ok_or(ValidationError::FuncTypeNotFound { func_idx })
        .and_then(|func_type| {
            if !func_type.parameters.is_empty() || !func_type.results.is_empty() {
                Err(ValidationError::InvalidStartFunctionType)
            } else {
                Ok(())
            }
        })
}

fn validate_import(ctx: &ValidationContext, import_type: &ImportType) -> ValidationResult<()> {
    match import_type.desc {
        ImportDescription::Func(ref type_idx) => validate_func_type(ctx, type_idx),
        ImportDescription::Global(ref global_type) => validate_global_type(global_type),
        ImportDescription::Mem(ref mem_type) => validate_memory(mem_type),
        ImportDescription::Table(ref table_type) => validate_table(table_type),
    }
}

fn validate_export(ctx: &ValidationContext, export_type: &ExportType) -> ValidationResult<()> {
    match export_type.desc {
        ExportDescription::Func(ref func_idx) => {
            validate_export_func_type(ctx, &FuncIdx(func_idx.0.clone()))
        }
        ExportDescription::Global(ref global_idx) => ctx
            .globals
            .get(global_idx.0 .0 as usize)
            .ok_or_else(|| ValidationError::GlobalNotFound)
            .and_then(validate_global_type),
        ExportDescription::Mem(ref mem_idx) => ctx
            .mems
            .get(mem_idx.0 .0 as usize)
            .ok_or_else(|| ValidationError::MemNotFound)
            .and_then(validate_memory),
        ExportDescription::Table(ref table_idx) => ctx
            .tables
            .get(table_idx.0 .0 as usize)
            .ok_or_else(|| ValidationError::TableNotFound)
            .and_then(validate_table),
    }
}

fn validate_elem(
    el_segment_type: &ElementSegmentType,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    let init = el_segment_type.get_init();
    let result_type = ResultType(vec![ValType::RefType(el_segment_type.get_type())]);

    for expression in init {
        validate_constant_expression(&expression, &result_type, ctx)?;
    }

    match el_segment_type {
        ElementSegmentType::Active0Functions(Active0FunctionsElementSegmentType {
            mode: ElemModeActive0 { offset },
            ..
        })
        | ElementSegmentType::ElemKindActiveFunctions(
            ElemKindActiveFunctionsElementSegmentType {
                mode: ElemModeActive { offset, .. },
                ..
            },
        )
        | ElementSegmentType::Active0Expr(Active0ExprElementSegmentType {
            mode: ElemModeActive0 { offset },
            ..
        })
        | ElementSegmentType::ActiveRef(ActiveRefElementSegmentType {
            mode: ElemModeActive { offset, .. },
            ..
        }) => validate_constant_expression(
            &offset,
            &ResultType(vec![ValType::NumType(NumType::I32)]),
            ctx,
        ),
        // Valid with any reftype
        ElementSegmentType::ElemKindPassiveFunctions(_) => Ok(()),
        // Valid with any reftype
        ElementSegmentType::ElemKindDeclarativeFunctions(_) => Ok(()),
        // Valid with any reftype
        ElementSegmentType::PassiveRef(_) => Ok(()),
        // Valid with any reftype
        ElementSegmentType::DeclarativeRef(_) => Ok(()),
    }
}

fn validate_data(data_type: &DataType, ctx: &ValidationContext) -> ValidationResult<()> {
    match data_type {
        DataType::Passive(_) => Ok(()),
        DataType::Active(ActiveDataType {
            mode: DataModeActive { offset, .. },
            ..
        })
        | DataType::Active0(Active0DataType {
            mode: DataModeActive0 { offset },
            ..
        }) => validate_constant_expression(
            &offset,
            &ResultType(vec![ValType::NumType(NumType::I32)]),
            ctx,
        ),
    }
}

fn create_context(module_src: &Module, externals: &Vec<ExternalDependency>) -> ValidationContext {
    ValidationContext {
        types: module_src.types.clone(),
        funcs: get_func_types(module_src, externals),
        // TODO: add imported external types to tables
        tables: module_src.tables.clone(),
        // TODO: add imported external types to mems
        mems: module_src.mems.clone(),
        // TODO: add imported external types to globals
        globals: module_src
            .globals
            .iter()
            .map(|g| g.global_type.clone())
            .collect(),
        elems: module_src.elems.iter().map(|e| e.get_type()).collect(),
        datas: module_src.datas.clone(),
        locals: vec![],
        labels: vec![],
        maybe_return: None,
        // TODO: collect function indexes wherever they occur in the module, but skip its own functions and a start function
        refs: vec![],
    }
}

fn get_func_types(module_src: &Module, externals: &Vec<ExternalDependency>) -> Vec<FuncType> {
    let mut external_func_types: Vec<FuncType> = externals
        .iter()
        .filter_map(|external| {
            if let ExternalDependency::Func { func_type, .. } = external {
                Some(func_type.clone())
            } else {
                None
            }
        })
        .collect();
    let module_func_types: Vec<FuncType> = module_src
        .funcs
        .iter()
        .map(|idx| module_src.types[idx.0 .0 as usize].clone())
        .collect();

    external_func_types.extend_from_slice(&module_func_types);

    external_func_types
}

fn create_sub_context(
    module_src: &Module,
    externals: &Vec<ExternalDependency>,
) -> ValidationContext {
    ValidationContext {
        globals: module_src
            .globals
            .iter()
            .map(|g| g.global_type.clone())
            .collect(),
        funcs: get_func_types(module_src, externals),
        // TODO: collect function indexes wherever they occur in the module, but skip its own functions and a start function
        refs: vec![],
        ..Default::default()
    }
}
