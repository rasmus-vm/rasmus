use crate::{
    entities::types::{FuncIdx, U32Type},
    validation::{
        context::ValidationContext,
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{ValidationStack, ValidationType},
    },
};

pub fn ref_null(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.push_val(ValidationType::reference());

    Ok(())
}

pub fn ref_is_null(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::reference())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn ref_func(
    &FuncIdx(U32Type(func_idx)): &FuncIdx,
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    if ctx.funcs.get(func_idx as usize).is_none() || ctx.refs.get(func_idx as usize).is_none() {
        return Err(ValidationError::CannotFindRefFuncInValidationContext);
    }

    val_stack.push_val(ValidationType::reference());

    Ok(())
}
