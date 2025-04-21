use crate::{
    entities::types::{DataIdx, LaneIdx, U32Type},
    validation::{
        context::ValidationContext,
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{ValidationStack, ValidationType},
    },
};

pub fn load_i32(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
) -> ValidationResult<()> {
    check_memarg(ctx, 32, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn load_i64(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
) -> ValidationResult<()> {
    check_memarg(ctx, 64, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i64());

    Ok(())
}

pub fn load_f32(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
) -> ValidationResult<()> {
    check_memarg(ctx, 32, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::f32());

    Ok(())
}

pub fn load_f64(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
) -> ValidationResult<()> {
    check_memarg(ctx, 64, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::f64());

    Ok(())
}

pub fn load_v128(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
) -> ValidationResult<()> {
    check_memarg(ctx, 128, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn load_i32_t(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    t: u8,
) -> ValidationResult<()> {
    check_memarg(ctx, t, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn load_i64_t(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    t: u8,
) -> ValidationResult<()> {
    check_memarg(ctx, t, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i64());

    Ok(())
}

pub fn store_value(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    value_type: ValidationType,
) -> ValidationResult<()> {
    check_memarg(ctx, 32, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(value_type)?;

    Ok(())
}

pub fn store_value_t(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    value_type: ValidationType,
    t: u8,
) -> ValidationResult<()> {
    check_memarg(ctx, t, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(value_type)?;

    Ok(())
}

pub fn load_vec_nm(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    (n, m): (u8, u8),
) -> ValidationResult<()> {
    if ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    if 2u8.pow(memarg.0 .0) > n / 8 * m {
        return Err(ValidationError::MemargAlignTooBig);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn load_vec_splat(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    t: u8,
) -> ValidationResult<()> {
    check_memarg(ctx, t, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn load_vec_lane(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    lane_idx: &LaneIdx,
    n: u8,
) -> ValidationResult<()> {
    if lane_idx.0 >= n / 8 {
        return Err(ValidationError::LaneIdxTooBix);
    }

    check_memarg(ctx, n, memarg)?;

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn store_vec_lane(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    memarg: &(U32Type, U32Type),
    lane_idx: &LaneIdx,
    n: u8,
) -> ValidationResult<()> {
    load_vec_lane(val_stack, ctx, memarg, lane_idx, n)
}

pub fn memory_size(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    if ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn memory_grow(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    if ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn memory_fill(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    if ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;

    Ok(())
}

pub fn memory_init(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    data_idx: &DataIdx,
) -> ValidationResult<()> {
    if ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    if ctx.datas.get(data_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::DataNotFound);
    }

    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_val_expect(ValidationType::i32())?;

    Ok(())
}

pub fn data_drop(
    _val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    data_idx: &DataIdx,
) -> ValidationResult<()> {
    if ctx.datas.get(data_idx.0 .0 as usize).is_none() {
        return Err(ValidationError::DataNotFound);
    }

    Ok(())
}

fn check_memarg(
    ctx: &ValidationContext,
    t: u8,
    memarg: &(U32Type, U32Type),
) -> ValidationResult<()> {
    if ctx.mems.get(0).is_none() {
        return Err(ValidationError::MemNotFound);
    }

    if 2u8.pow(memarg.0 .0) > t / 8 {
        return Err(ValidationError::MemargAlignTooBig);
    }

    Ok(())
}
