use crate::instances::stack::Stack;
use crate::instances::value::Val;
use crate::result::{RResult, Trap};
use crate::sign::Sign;

use super::exec_const::i64_const;

macro_rules! cvtop_impl {
    ($fn_name:ident, $input_val: path, $input_type: ty, $output_val: path, $output_type: ty) => {
        #[inline]
        pub fn $fn_name(
            exec_fn: impl FnOnce($input_type) -> RResult<$output_type>,
            stack: &mut Stack,
        ) -> $crate::result::RResult<()> {
            if let Some($input_val(arg)) = stack.pop_value() {
                let result = exec_fn(arg)?;
                stack.push_entry($crate::instances::stack::StackEntry::Value($output_val(
                    result,
                )));
            } else {
                return Err($crate::result::Trap);
            }
            Ok(())
        }
    };
}

macro_rules! trunc_s {
    ($arg_type: ty, $aux_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg.is_nan() || arg.is_infinite() {
                return Err($crate::result::Trap);
            }

            Ok(arg.trunc() as $aux_type as $ret_type)
        }
    };
}

macro_rules! trunc_u {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg.is_nan() || arg.is_infinite() {
                return Err($crate::result::Trap);
            }

            let trunked = arg.trunc() as u128;
            <$ret_type>::try_from(trunked).map_err(|_| $crate::result::Trap)
        }
    };
}

macro_rules! trunc_sat_u {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg.is_nan() {
                return Err($crate::result::Trap);
            }

            if arg.is_infinite() && arg.is_sign_negative() {
                return Ok(0);
            }

            if arg.is_infinite() && arg.is_sign_positive() {
                return Ok(<$ret_type>::MAX);
            }

            <$ret_type>::try_from(arg.trunc() as i128).or_else(|_| Ok(<$ret_type>::MAX))
        }
    };
}

macro_rules! trunc_sat_s {
    ($arg_type: ty, $aux_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg.is_nan() {
                return Err($crate::result::Trap);
            }

            if arg.is_infinite() && arg.is_sign_negative() {
                return Ok(<$aux_type>::MIN as $ret_type);
            }

            if arg.is_infinite() && arg.is_sign_positive() {
                return Ok(<$aux_type>::MAX as $ret_type);
            }

            let trunced = arg.trunc() as i128;

            if trunced > (<$aux_type>::MAX as i128) {
                return Ok(<$aux_type>::MAX as $ret_type);
            }

            if trunced < (<$aux_type>::MIN as i128) {
                return Ok(<$aux_type>::MIN as $ret_type);
            }

            Ok(trunced as $ret_type)
        }
    };
}

macro_rules! reinterpret {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| -> RResult<$ret_type> {
            let mut bytes = arg.to_le_bytes();
            Ok($crate::read_unsigned_leb128!($ret_type)(
                &mut bytes,
                &mut 0usize,
            ))
        }
    };
}

cvtop_impl!(i64_i32_cvtop, Val::I64, u64, Val::I32, u32);

cvtop_impl!(f32_i32_cvtop, Val::F32, f32, Val::I32, u32);
cvtop_impl!(f64_i32_cvtop, Val::F64, f64, Val::I32, u32);

cvtop_impl!(f32_i64_cvtop, Val::F32, f32, Val::I64, u64);
cvtop_impl!(f64_i64_cvtop, Val::F64, f64, Val::I64, u64);

cvtop_impl!(i32_f32_cvtop, Val::I32, u32, Val::F32, f32);
cvtop_impl!(i64_f32_cvtop, Val::I64, u64, Val::F32, f32);

cvtop_impl!(i32_f64_cvtop, Val::I32, u32, Val::F64, f64);
cvtop_impl!(i64_f64_cvtop, Val::I64, u64, Val::F64, f64);

cvtop_impl!(f32_f64_cvtop, Val::F64, f64, Val::F32, f32);
cvtop_impl!(f64_f32_cvtop, Val::F32, f32, Val::F64, f64);

pub fn i32_wrap_i64(stack: &mut Stack) -> RResult<()> {
    i64_i32_cvtop(
        |arg: u64| Ok((arg as u128).rem_euclid(2u128).pow(32) as u32),
        stack,
    )
}

// trunc instructions

pub fn i32_trunc_f32_u(stack: &mut Stack) -> RResult<()> {
    f32_i32_cvtop(trunc_u!(f32, u32), stack)
}

pub fn i32_trunc_f64_u(stack: &mut Stack) -> RResult<()> {
    f64_i32_cvtop(trunc_u!(f64, u32), stack)
}

pub fn i32_trunc_f32_s(stack: &mut Stack) -> RResult<()> {
    f32_i32_cvtop(trunc_s!(f32, i32, u32), stack)
}

pub fn i32_trunc_f64_s(stack: &mut Stack) -> RResult<()> {
    f64_i32_cvtop(trunc_s!(f64, i32, u32), stack)
}

pub fn i64_trunc_f32_u(stack: &mut Stack) -> RResult<()> {
    f32_i64_cvtop(trunc_u!(f32, u64), stack)
}

pub fn i64_trunc_f64_u(stack: &mut Stack) -> RResult<()> {
    f64_i64_cvtop(trunc_u!(f64, u64), stack)
}

pub fn i64_trunc_f32_s(stack: &mut Stack) -> RResult<()> {
    f32_i64_cvtop(trunc_s!(f32, i64, u64), stack)
}

pub fn i64_trunc_f64_s(stack: &mut Stack) -> RResult<()> {
    f64_i64_cvtop(trunc_s!(f64, i64, u64), stack)
}

// trunc sat instruction

pub fn i32_trunc_sat_f32_u(stack: &mut Stack) -> RResult<()> {
    f32_i32_cvtop(trunc_sat_u!(f32, u32), stack)
}

pub fn i32_trunc_sat_f64_u(stack: &mut Stack) -> RResult<()> {
    f64_i32_cvtop(trunc_sat_u!(f64, u32), stack)
}

pub fn i32_trunc_sat_f32_s(stack: &mut Stack) -> RResult<()> {
    f32_i32_cvtop(trunc_sat_s!(f32, i32, u32), stack)
}

pub fn i32_trunc_sat_f64_s(stack: &mut Stack) -> RResult<()> {
    f64_i32_cvtop(trunc_sat_s!(f64, i32, u32), stack)
}

pub fn i64_trunc_sat_f32_u(stack: &mut Stack) -> RResult<()> {
    f32_i64_cvtop(trunc_sat_u!(f32, u64), stack)
}

pub fn i64_trunc_sat_f64_u(stack: &mut Stack) -> RResult<()> {
    f64_i64_cvtop(trunc_sat_u!(f64, u64), stack)
}

pub fn i64_trunc_sat_f32_s(stack: &mut Stack) -> RResult<()> {
    f32_i64_cvtop(trunc_sat_s!(f32, i64, u64), stack)
}

pub fn i64_trunc_sat_f64_s(stack: &mut Stack) -> RResult<()> {
    f64_i64_cvtop(trunc_sat_s!(f64, i64, u64), stack)
}

// convert

pub fn f32_convert_i32_s(stack: &mut Stack) -> RResult<()> {
    i32_f32_cvtop(|arg: u32| Ok(arg as i32 as f32), stack)
}

pub fn f32_convert_i32_u(stack: &mut Stack) -> RResult<()> {
    i32_f32_cvtop(|arg: u32| Ok(arg as f32), stack)
}

pub fn f32_convert_i64_s(stack: &mut Stack) -> RResult<()> {
    i64_f32_cvtop(|arg: u64| Ok(arg as i64 as f32), stack)
}

pub fn f32_convert_i64_u(stack: &mut Stack) -> RResult<()> {
    i64_f32_cvtop(|arg: u64| Ok(arg as f32), stack)
}

pub fn f64_convert_i32_s(stack: &mut Stack) -> RResult<()> {
    i32_f64_cvtop(|arg: u32| Ok(arg as i32 as f64), stack)
}

pub fn f64_convert_i32_u(stack: &mut Stack) -> RResult<()> {
    i32_f64_cvtop(|arg: u32| Ok(arg as f64), stack)
}

pub fn f64_convert_i64_s(stack: &mut Stack) -> RResult<()> {
    i64_f64_cvtop(|arg: u64| Ok(arg as i64 as f64), stack)
}

pub fn f64_convert_i64_u(stack: &mut Stack) -> RResult<()> {
    i64_f64_cvtop(|arg: u64| Ok(arg as f64), stack)
}

pub fn f32_demote_f64(stack: &mut Stack) -> RResult<()> {
    f32_f64_cvtop(|arg: f64| Ok(arg as f32), stack)
}

pub fn f64_promote_f32(stack: &mut Stack) -> RResult<()> {
    f64_f32_cvtop(|arg: f32| Ok(arg as f64), stack)
}

pub fn i32_reinterpret_f32(stack: &mut Stack) -> RResult<()> {
    f32_i32_cvtop(reinterpret!(f32, u32), stack)
}

pub fn i64_reinterpret_f64(stack: &mut Stack) -> RResult<()> {
    f64_i64_cvtop(reinterpret!(f64, u64), stack)
}

pub fn f32_reinterpret_i32(stack: &mut Stack) -> RResult<()> {
    f32_i32_cvtop(reinterpret!(f32, u32), stack)
}

pub fn f64_reinterpret_i64(stack: &mut Stack) -> RResult<()> {
    f64_i64_cvtop(reinterpret!(f64, u64), stack)
}

pub fn i64_extend_i32(stack: &mut Stack, signed: Sign) -> RResult<()> {
    let i = stack.pop_i32().ok_or(Trap)?;
    let v = match signed {
        Sign::Signed => i as i32 as u64,
        Sign::Unsigned => i as u64,
    };
    i64_const(&v, stack)
}

#[cfg(test)]
mod test {
    use crate::entities::{
        module::InstructionType,
        types::{F32Type, F64Type, I64Type},
    };

    use crate::test_utils::test_instruction;

    use super::*;

    #[test]
    fn i32_wrap_i64_no_overflow() {
        test_instruction(
            vec![InstructionType::I64Const(I64Type(1))],
            InstructionType::I32WrapI64,
            Val::I32(1),
        );
    }

    #[test]
    fn i32_wrap_i64_with_overflow() {
        test_instruction(
            vec![InstructionType::I64Const(I64Type(2u64.pow(32) + 1))],
            InstructionType::I32WrapI64,
            Val::I32(1),
        );
    }

    #[test]
    fn i32_trunc_f32_u() {
        test_instruction(
            vec![InstructionType::F32Const(F32Type(2.2))],
            InstructionType::I32TruncF32U,
            Val::I32(2),
        );
    }

    #[test]
    fn i32_trunc_f64_u() {
        test_instruction(
            vec![InstructionType::F64Const(F64Type(2000000000.5))],
            InstructionType::I32TruncF64U,
            Val::I32(2000000000),
        );
    }

    #[test]
    fn i32_trunc_f32_s_negative() {
        test_instruction(
            vec![InstructionType::F32Const(F32Type(-2.2))],
            InstructionType::I32TruncF32S,
            Val::I32(-2i32 as u32),
        );
    }

    #[test]
    fn i32_trunc_f32_s_positive() {
        test_instruction(
            vec![InstructionType::F32Const(F32Type(2.2))],
            InstructionType::I32TruncF32S,
            Val::I32(2),
        );
    }

    #[test]
    fn i32_trunc_f64_s_negative() {
        test_instruction(
            vec![InstructionType::F64Const(F64Type(-2000000000.5))],
            InstructionType::I32TruncF64S,
            Val::I32(-2000000000i32 as u32),
        );
    }

    #[test]
    fn i32_trunc_f64_s_positive() {
        test_instruction(
            vec![InstructionType::F64Const(F64Type(2000000000.5))],
            InstructionType::I32TruncF64S,
            Val::I32(2000000000),
        );
    }

    #[test]
    fn i64_trunc_f32_u() {
        test_instruction(
            vec![InstructionType::F32Const(F32Type(2.2))],
            InstructionType::I64TruncF32U,
            Val::I64(2),
        );
    }

    #[test]
    fn i64_trunc_f64_u() {
        test_instruction(
            vec![InstructionType::F64Const(F64Type(2000000000.5))],
            InstructionType::I64TruncF64U,
            Val::I64(2000000000),
        );
    }

    #[test]
    fn i64_trunc_f32_s_negative() {
        test_instruction(
            vec![InstructionType::F32Const(F32Type(-2.2))],
            InstructionType::I64TruncF32S,
            Val::I64(-2i64 as u64),
        );
    }

    #[test]
    fn i64_trunc_f32_s_positive() {
        test_instruction(
            vec![InstructionType::F32Const(F32Type(2.2))],
            InstructionType::I64TruncF32S,
            Val::I64(2),
        );
    }

    #[test]
    fn i64_trunc_f64_s_negative() {
        test_instruction(
            vec![InstructionType::F64Const(F64Type(-2000000000.5))],
            InstructionType::I64TruncF64S,
            Val::I64(-2000000000i64 as u64),
        );
    }

    #[test]
    fn i64_trunc_f64_s_positive() {
        test_instruction(
            vec![InstructionType::F64Const(F64Type(2000000000.5))],
            InstructionType::I64TruncF64S,
            Val::I64(2000000000),
        );
    }

    // TODO: add tests for trunc sat instructions
}
