use crate::{
    entities::types::{ElemIdx, TableIdx},
    validation::{
        context::ValidationContext,
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{ValidationStack, ValidationType},
    },
};

pub fn table_get(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    table_idx: &TableIdx,
) -> ValidationResult<()> {
    if ctx.tables.get(table_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::TableNotFound);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::reference());

    Ok(())
}

pub fn table_set(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    table_idx: &TableIdx,
) -> ValidationResult<()> {
    if ctx.tables.get(table_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::TableNotFound);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::reference())?;

    Ok(())
}

pub fn table_size(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    table_idx: &TableIdx,
) -> ValidationResult<()> {
    if ctx.tables.get(table_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::TableNotFound);
    }

    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn table_grow(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    table_idx: &TableIdx,
) -> ValidationResult<()> {
    if ctx.tables.get(table_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::TableNotFound);
    }

    val_stack.pop_val_expect(ValidationType::reference())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn table_fill(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    table_idx: &TableIdx,
) -> ValidationResult<()> {
    if ctx.tables.get(table_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::TableNotFound);
    }

    val_stack.pop_val_expect(ValidationType::reference())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::reference())?;

    Ok(())
}

pub fn table_copy(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    &(ref table_idx_lhs, ref table_idx_rhs): &(TableIdx, TableIdx),
) -> ValidationResult<()> {
    let table_lhs = ctx
        .tables
        .get(table_idx_lhs.0 .0 as usize)
        .ok_or_else(|| ValidationError::TableNotFound)?;

    let table_rhs = ctx
        .tables
        .get(table_idx_rhs.0 .0 as usize)
        .ok_or_else(|| ValidationError::TableNotFound)?;

    if table_lhs.element_ref_type != table_rhs.element_ref_type {
        return Err(ValidationError::UnableToCopyIncosistentTableTypes);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;

    Ok(())
}

pub fn table_init(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    &(ref table_idx, ref elem_idx): &(TableIdx, ElemIdx),
) -> ValidationResult<()> {
    let table = ctx
        .tables
        .get(table_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::TableNotFound)?;
    let table_type = table.element_ref_type.clone();

    let elem = ctx
        .elems
        .get(elem_idx.0 .0 as usize)
        .ok_or_else(|| ValidationError::ElemNotFound)?;

    if *elem != table_type {
        return Err(ValidationError::WrongElemType);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;

    Ok(())
}

pub fn elem_drop(ctx: &ValidationContext, elem_idx: &ElemIdx) -> ValidationResult<()> {
    if ctx.elems.get(elem_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::ElemNotFound);
    }

    Ok(())
}
