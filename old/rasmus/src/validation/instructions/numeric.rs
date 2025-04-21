use crate::validation::{
    validation_error::ValidationResult,
    validation_stack::{ValidationStack, ValidationType},
};

pub fn i32_const(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i64_const(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn f32_const(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.push_val(ValidationType::f32());
    Ok(())
}

pub fn f64_const(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.push_val(ValidationType::f64());
    Ok(())
}

pub fn i32_unop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i64_unop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn f32_unop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::f32());
    Ok(())
}

pub fn f64_unop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::f64());
    Ok(())
}

pub fn i32_testop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i64_testop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i32_binop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i64_binop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn f32_binop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::f32());
    Ok(())
}

pub fn f64_binop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::f64());
    Ok(())
}

pub fn i32_relop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i64_relop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn f32_relop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn f64_relop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

#[allow(dead_code)]
pub fn i64_to_i32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i32_to_i32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn i64_to_i64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn f32_to_i32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn f64_to_i32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::i32());
    Ok(())
}

pub fn f32_to_i64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn f64_to_i64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn i32_to_f32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::f32());
    Ok(())
}

pub fn i64_to_f32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::f32());
    Ok(())
}

pub fn i32_to_f64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::f64());
    Ok(())
}

pub fn i32_to_i64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i64());
    Ok(())
}

pub fn i64_to_f64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::f64());
    Ok(())
}

pub fn f64_to_f32_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::f32());
    Ok(())
}

pub fn f32_to_f64_cvtop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::f64());
    Ok(())
}
