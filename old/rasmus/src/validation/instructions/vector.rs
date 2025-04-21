use crate::{
    entities::types::LaneIdx,
    validation::{
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{ValidationStack, ValidationType},
    },
};

pub fn v128_const(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn v128_unop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn v128_binop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn v128_ternop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn v128_testop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn i8x16_swizzle(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

const MAX_ALLOWED_LINE_IDX: u8 = 32;

pub fn i8x16_shuffle(
    lane_indexes: &Vec<LaneIdx>,
    val_stack: &mut ValidationStack,
) -> ValidationResult<()> {
    for lane_idx in lane_indexes {
        if lane_idx.0 >= MAX_ALLOWED_LINE_IDX {
            return Err(ValidationError::LaneIndexIsOutOfRange {
                value: lane_idx.0,
                max_allowed: MAX_ALLOWED_LINE_IDX,
            });
        }
    }

    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn i8x16_splat(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn i64x2_splat(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i64())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn f32x4_splat(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f32())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn f64x2_splat(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::f64())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn extract_line_i(
    dim: usize,
    &LaneIdx(lane_idx): &LaneIdx,
    val_stack: &mut ValidationStack,
) -> ValidationResult<()> {
    if lane_idx as usize >= dim {
        return Err(ValidationError::LaneIndexIsOutOfRange {
            value: lane_idx,
            max_allowed: dim as u8 - 1,
        });
    }

    val_stack.pop_val_expect(ValidationType::v128())?;

    let output = match dim {
        // i8x16 | i16x8 | i32x4
        16 | 8 | 4 => ValidationType::i32(),
        // i64x2
        2 => ValidationType::i64(),
        _ => unreachable!(),
    };

    val_stack.push_val(output);

    Ok(())
}

pub fn extract_line_f(
    dim: usize,
    &LaneIdx(lane_idx): &LaneIdx,
    val_stack: &mut ValidationStack,
) -> ValidationResult<()> {
    if lane_idx as usize >= dim {
        return Err(ValidationError::LaneIndexIsOutOfRange {
            value: lane_idx,
            max_allowed: dim as u8 - 1,
        });
    }

    val_stack.pop_val_expect(ValidationType::v128())?;

    let output = match dim {
        // f32x4
        4 => ValidationType::f32(),
        // f64x2
        2 => ValidationType::f64(),
        _ => unreachable!(),
    };

    val_stack.push_val(output);

    Ok(())
}

pub fn replace_line_i(
    dim: usize,
    &LaneIdx(lane_idx): &LaneIdx,
    val_stack: &mut ValidationStack,
) -> ValidationResult<()> {
    if lane_idx as usize >= dim {
        return Err(ValidationError::LaneIndexIsOutOfRange {
            value: lane_idx,
            max_allowed: dim as u8 - 1,
        });
    }

    val_stack.pop_val_expect(ValidationType::v128())?;

    let second_arg = match dim {
        // i8x16 | i16x8 | i32x4
        16 | 8 | 4 => ValidationType::i32(),
        // i64x2
        2 => ValidationType::i64(),
        _ => unreachable!(),
    };

    val_stack.pop_val_expect(second_arg)?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn replace_line_f(
    dim: usize,
    &LaneIdx(lane_idx): &LaneIdx,
    val_stack: &mut ValidationStack,
) -> ValidationResult<()> {
    if lane_idx as usize >= dim {
        return Err(ValidationError::LaneIndexIsOutOfRange {
            value: lane_idx,
            max_allowed: dim as u8 - 1,
        });
    }

    val_stack.pop_val_expect(ValidationType::v128())?;

    let second_arg = match dim {
        // i8x16 | i16x8 | i32x4
        16 | 8 | 4 => ValidationType::f32(),
        // i64x2
        2 => ValidationType::f64(),
        _ => unreachable!(),
    };

    val_stack.pop_val_expect(second_arg)?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn shape_bitop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_val(ValidationType::v128());

    Ok(())
}

pub fn v128_relop(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}

pub fn ishape_bitmask(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::v128())?;
    val_stack.push_val(ValidationType::i32());

    Ok(())
}
