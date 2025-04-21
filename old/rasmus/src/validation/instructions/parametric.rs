use crate::{
    entities::types::ValType,
    validation::{
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{ValidationStack, ValidationType},
    },
};

pub fn drop_val(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val()?;

    Ok(())
}

pub fn select(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;

    let t1 = val_stack.pop_val()?;
    let t2 = val_stack.pop_val()?;

    if !(t1.is_num() && t2.is_num() || (t1.is_vec() && t2.is_vec())) {
        return Err(ValidationError::InvalidSelectBranchTypes);
    }

    if t1 != t2 && !t1.is_unknown() && !t2.is_unknown() {
        return Err(ValidationError::InvalidSelectBranchTypes);
    }

    val_stack.push_val(if t1.is_unknown() { t2 } else { t1 });

    Ok(())
}

pub fn select_vec(
    val_stack: &mut ValidationStack,
    val_types: &Vec<ValType>,
) -> ValidationResult<()> {
    if val_types.len() != 1 {
        return Err(ValidationError::InvalidSelectVecOperandSequence);
    }

    let valiation_type = ValidationType::from(&val_types[0]);

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(valiation_type.clone())?;
    val_stack.pop_val_expect(valiation_type.clone())?;
    val_stack.push_val(valiation_type);

    Ok(())
}
