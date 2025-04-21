use crate::{
    entities::types::{GlobalIdx, LocalIdx, MutType},
    validation::{
        context::ValidationContext,
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{ValidationStack, ValidationType},
    },
};

pub fn local_get(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    local_idx: &LocalIdx,
) -> ValidationResult<()> {
    let local_type = ctx
        .locals
        .get(local_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::LocalNotFound)?;

    val_stack.push_val(ValidationType::from(local_type));

    Ok(())
}

pub fn local_set(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    local_idx: &LocalIdx,
) -> ValidationResult<()> {
    let local_type = ctx
        .locals
        .get(local_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::LocalNotFound)?;

    val_stack
        .pop_val_expect(ValidationType::from(local_type))
        .map(|_| ())
}

pub fn local_tee(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    local_idx: &LocalIdx,
) -> ValidationResult<()> {
    let local_type = ctx
        .locals
        .get(local_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::LocalNotFound)?;

    val_stack.push_val(ValidationType::from(local_type.clone()));
    val_stack.push_val(ValidationType::from(local_type));

    local_set(val_stack, ctx, local_idx)
}

pub fn global_get(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    global_idx: &GlobalIdx,
) -> ValidationResult<()> {
    let global_type = ctx
        .globals
        .get(global_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::GlobalNotFound)?
        .val_type
        .clone();

    val_stack.push_val(ValidationType::from(global_type));

    Ok(())
}

pub fn global_set(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    global_idx: &GlobalIdx,
) -> ValidationResult<()> {
    let global_type = ctx
        .globals
        .get(global_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::GlobalNotFound)?;

    if global_type.mut_type != MutType::Var {
        return Err(ValidationError::UnableToSetToConstGlobal);
    }

    val_stack
        .pop_val_expect(ValidationType::from(global_type.val_type.clone()))
        .map(|_| ())
}
