use super::context::ValidationContext;
use super::validation_error::{ValidationError, ValidationResult};
use crate::entities::types::*;

pub fn is_limit_type_valid(limit: &LimitsType, range: U32Type) -> bool {
    limit.min <= range
        && limit
            .max
            .as_ref()
            .map(|max_value| max_value <= &range && limit.min <= *max_value)
            .unwrap_or(true)
}

pub fn is_table_type_valid(table_type: &TableType) -> bool {
    is_limit_type_valid(&table_type.limits, U32Type(u32::MAX))
}

pub fn is_memory_type_valid(memory_type: &MemType) -> bool {
    is_limit_type_valid(&memory_type.limits, U32Type(2u32.pow(16)))
}

pub fn validate_func_type(ctx: &ValidationContext, func_type: &TypeIdx) -> ValidationResult<()> {
    let func_idx = func_type.0 .0 as usize;
    ctx.types
        .get(func_idx)
        .ok_or(ValidationError::FuncTypeNotFound { func_idx })
        .map(|_| ())
}

pub fn validate_export_func_type(
    ctx: &ValidationContext,
    func_type: &FuncIdx,
) -> ValidationResult<()> {
    let func_idx = func_type.0 .0 as usize;
    ctx.funcs
        .get(func_idx)
        .ok_or(ValidationError::FuncTypeNotFound { func_idx })
        .map(|_| ())
}

pub fn validate_global_type(_global_type: &GlobalType) -> ValidationResult<()> {
    // global type is always valid
    Ok(())
}
