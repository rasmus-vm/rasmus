use std::ops::Neg;

use crate::{
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

#[macro_export]
macro_rules! nearest {
    ($ftype:ty) => {
        |v: $ftype| {
            if v.is_nan() || v.is_infinite() || v == 0.0 {
                v
            } else if v > 0.0 && v <= 0.5 {
                -0.0
            } else if v < 0.0 && v >= -0.5 {
                0.0
            } else if v < 0.0 {
                if v - v.trunc() >= -0.5 {
                    v.trunc()
                } else {
                    v.trunc() + 1.0
                }
            } else {
                if v - v.trunc() <= 0.5 {
                    v.trunc()
                } else {
                    v.trunc() + 1.0
                }
            }
        }
    };
}

pub fn i32_clz(stack: &mut Stack) -> RResult<()> {
    i32_unop(|v: u32| v.leading_zeros() as u32, stack)
}

pub fn i64_clz(stack: &mut Stack) -> RResult<()> {
    i64_unop(|v: u64| v.leading_zeros() as u64, stack)
}

pub fn i32_ctz(stack: &mut Stack) -> RResult<()> {
    i32_unop(|v: u32| v.trailing_zeros() as u32, stack)
}

pub fn i64_ctz(stack: &mut Stack) -> RResult<()> {
    i64_unop(|v: u64| v.trailing_zeros() as u64, stack)
}

pub fn i32_popcnt(stack: &mut Stack) -> RResult<()> {
    i32_unop(|v: u32| v.count_ones() as u32, stack)
}

pub fn i64_popcnt(stack: &mut Stack) -> RResult<()> {
    i64_unop(|v: u64| v.count_ones() as u64, stack)
}

pub fn f32_abs(stack: &mut Stack) -> RResult<()> {
    f32_unop(|v: f32| v.abs(), stack)
}

pub fn f64_abs(stack: &mut Stack) -> RResult<()> {
    f64_unop(|v: f64| v.abs(), stack)
}

pub fn f32_neg(stack: &mut Stack) -> RResult<()> {
    f32_unop(|v: f32| v.neg(), stack)
}

pub fn f64_neg(stack: &mut Stack) -> RResult<()> {
    f64_unop(|v: f64| v.neg(), stack)
}

pub fn f32_sqrt(stack: &mut Stack) -> RResult<()> {
    f32_unop(|v: f32| v.sqrt(), stack)
}

pub fn f64_sqrt(stack: &mut Stack) -> RResult<()> {
    f64_unop(|v: f64| v.sqrt(), stack)
}

pub fn f32_ceil(stack: &mut Stack) -> RResult<()> {
    f32_unop(|v: f32| v.ceil(), stack)
}

pub fn f64_ceil(stack: &mut Stack) -> RResult<()> {
    f64_unop(|v: f64| v.ceil(), stack)
}

pub fn f32_floor(stack: &mut Stack) -> RResult<()> {
    f32_unop(|v: f32| v.floor(), stack)
}

pub fn f64_floor(stack: &mut Stack) -> RResult<()> {
    f64_unop(|v: f64| v.floor(), stack)
}

pub fn f32_trunc(stack: &mut Stack) -> RResult<()> {
    f32_unop(|v: f32| v.trunc(), stack)
}

pub fn f64_trunc(stack: &mut Stack) -> RResult<()> {
    f64_unop(|v: f64| v.trunc(), stack)
}

pub fn f32_nearest(stack: &mut Stack) -> RResult<()> {
    f32_unop(nearest!(f32), stack)
}

pub fn f64_nearest(stack: &mut Stack) -> RResult<()> {
    f64_unop(nearest!(f64), stack)
}

pub fn i32_extend_8s(stack: &mut Stack) -> RResult<()> {
    i32_unop(|v: u32| (v as i8) as u32, stack)
}

pub fn i32_extend_16s(stack: &mut Stack) -> RResult<()> {
    i32_unop(|v: u32| (v as i16) as u32, stack)
}

pub fn i64_extend_8s(stack: &mut Stack) -> RResult<()> {
    i64_unop(|v: u64| (v as i8) as u64, stack)
}

pub fn i64_extend_16s(stack: &mut Stack) -> RResult<()> {
    i64_unop(|v: u64| (v as i16) as u64, stack)
}

pub fn i64_extend_32s(stack: &mut Stack) -> RResult<()> {
    i64_unop(|v: u64| (v as i32) as u64, stack)
}

fn i32_unop(exec_fn: impl FnOnce(u32) -> u32, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::I32(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::I32(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn i64_unop(exec_fn: impl FnOnce(u64) -> u64, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::I64(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::I64(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn f32_unop(exec_fn: impl FnOnce(f32) -> f32, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::F32(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::F32(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn f64_unop(exec_fn: impl FnOnce(f64) -> f64, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::F64(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::F64(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

#[cfg(test)]
mod test {
    use crate::entities::{
        module::InstructionType,
        types::{I32Type, I64Type},
    };

    use crate::{instances::value::Val, test_utils::test_instruction};

    #[test]
    fn i32_clz_no_zeros() {
        test_instruction(
            vec![InstructionType::I32Const(I32Type(u32::MAX))],
            InstructionType::I32Clz,
            Val::I32(0),
        );
    }

    #[test]
    fn i32_clz_except_first() {
        test_instruction(
            vec![InstructionType::I32Const(I32Type(1))],
            InstructionType::I32Clz,
            Val::I32(u32::BITS - 1),
        );
    }

    #[test]
    fn i64_clz_no_zeros() {
        test_instruction(
            vec![InstructionType::I64Const(I64Type(u64::MAX))],
            InstructionType::I64Clz,
            Val::I64(0),
        );
    }

    #[test]
    fn i64_clz_except_first() {
        test_instruction(
            vec![InstructionType::I64Const(I64Type(1))],
            InstructionType::I64Clz,
            Val::I64((u64::BITS - 1) as u64),
        );
    }
}
