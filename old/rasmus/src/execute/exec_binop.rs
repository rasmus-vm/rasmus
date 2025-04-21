use crate::entities::traits::{Max, Min};

use crate::instances::{
    stack::{Stack, StackEntry},
    value::Val,
};
use crate::result::{RResult, Trap};

macro_rules! fcopysign {
    ($type: ty) => {
        |lhs: $type, rhs: $type| Ok(lhs.copysign(rhs))
    };
}

pub fn i32_add(stack: &mut Stack) -> RResult<()> {
    i32_binop(iadd_32, stack)
}

pub fn i64_add(stack: &mut Stack) -> RResult<()> {
    i64_binop(iadd_64, stack)
}

pub fn i32_sub(stack: &mut Stack) -> RResult<()> {
    i32_binop(isub_32, stack)
}

pub fn i64_sub(stack: &mut Stack) -> RResult<()> {
    i64_binop(isub_64, stack)
}

pub fn i32_mul(stack: &mut Stack) -> RResult<()> {
    i32_binop(imul_32, stack)
}

pub fn i64_mul(stack: &mut Stack) -> RResult<()> {
    i64_binop(imul_64, stack)
}

pub fn i32_div_u(stack: &mut Stack) -> RResult<()> {
    i32_binop(idiv_32_u, stack)
}

pub fn i32_div_s(stack: &mut Stack) -> RResult<()> {
    i32_binop(idiv_32_s, stack)
}

pub fn i64_div_u(stack: &mut Stack) -> RResult<()> {
    i64_binop(idiv_64_u, stack)
}

pub fn i64_div_s(stack: &mut Stack) -> RResult<()> {
    i64_binop(idiv_64_s, stack)
}

pub fn i32_rem_u(stack: &mut Stack) -> RResult<()> {
    i32_binop(irem_32_u, stack)
}

pub fn i32_rem_s(stack: &mut Stack) -> RResult<()> {
    i32_binop(irem_32_s, stack)
}

pub fn i64_rem_u(stack: &mut Stack) -> RResult<()> {
    i64_binop(irem_64_u, stack)
}

pub fn i64_rem_s(stack: &mut Stack) -> RResult<()> {
    i64_binop(irem_64_s, stack)
}

pub fn i32_and(stack: &mut Stack) -> RResult<()> {
    i32_binop(iand, stack)
}

pub fn i64_and(stack: &mut Stack) -> RResult<()> {
    i64_binop(iand, stack)
}

pub fn i32_or(stack: &mut Stack) -> RResult<()> {
    i32_binop(ior, stack)
}

pub fn i64_or(stack: &mut Stack) -> RResult<()> {
    i64_binop(ior, stack)
}

pub fn i32_xor(stack: &mut Stack) -> RResult<()> {
    i32_binop(ixor, stack)
}

pub fn i64_xor(stack: &mut Stack) -> RResult<()> {
    i64_binop(ixor, stack)
}

pub fn i32_shl(stack: &mut Stack) -> RResult<()> {
    i32_binop(ishl_32, stack)
}

pub fn i64_shl(stack: &mut Stack) -> RResult<()> {
    i64_binop(ishl_64, stack)
}

pub fn i32_shr_u(stack: &mut Stack) -> RResult<()> {
    i32_binop(ishr_u_32, stack)
}

pub fn i64_shr_u(stack: &mut Stack) -> RResult<()> {
    i64_binop(ishr_u_64, stack)
}

pub fn i32_shr_s(stack: &mut Stack) -> RResult<()> {
    i32_binop(ishr_s_32, stack)
}

pub fn i64_shr_s(stack: &mut Stack) -> RResult<()> {
    i64_binop(ishr_s_64, stack)
}

pub fn i32_rotl(stack: &mut Stack) -> RResult<()> {
    i32_binop(irotl_32, stack)
}

pub fn i64_rotl(stack: &mut Stack) -> RResult<()> {
    i64_binop(irotl_64, stack)
}

pub fn i32_rotr(stack: &mut Stack) -> RResult<()> {
    i32_binop(irotr_32, stack)
}

pub fn i64_rotr(stack: &mut Stack) -> RResult<()> {
    i64_binop(irotr_64, stack)
}

pub fn f32_add(stack: &mut Stack) -> RResult<()> {
    f32_binop(fadd, stack)
}

pub fn f64_add(stack: &mut Stack) -> RResult<()> {
    f64_binop(fadd, stack)
}

pub fn f32_sub(stack: &mut Stack) -> RResult<()> {
    f32_binop(fsub, stack)
}

pub fn f64_sub(stack: &mut Stack) -> RResult<()> {
    f64_binop(fsub, stack)
}

pub fn f32_mul(stack: &mut Stack) -> RResult<()> {
    f32_binop(fmul, stack)
}

pub fn f64_mul(stack: &mut Stack) -> RResult<()> {
    f64_binop(fmul, stack)
}

pub fn f32_div(stack: &mut Stack) -> RResult<()> {
    f32_binop(fdiv, stack)
}

pub fn f64_div(stack: &mut Stack) -> RResult<()> {
    f64_binop(fdiv, stack)
}

pub fn f32_min(stack: &mut Stack) -> RResult<()> {
    f32_binop(min, stack)
}

pub fn f64_min(stack: &mut Stack) -> RResult<()> {
    f64_binop(min, stack)
}

pub fn f32_max(stack: &mut Stack) -> RResult<()> {
    f32_binop(max, stack)
}

pub fn f64_max(stack: &mut Stack) -> RResult<()> {
    f64_binop(max, stack)
}

pub fn f32_copysign(stack: &mut Stack) -> RResult<()> {
    f32_binop(fcopysign!(f32), stack)
}

pub fn f64_copysign(stack: &mut Stack) -> RResult<()> {
    f64_binop(fcopysign!(f64), stack)
}

#[macro_export]
macro_rules! binop_impl {
    // ($fn_name:ident, $first_type: ty, $second_type: ty, $ret: ty) => {
    //     fn $fn_name(
    //         exec_fn: impl FnOnce($first_type, $second_type) -> $ret,
    //         stack: &mut Stack,
    //     ) -> RResult<()> {
    //         if let Some($first_type(second)) = stack.pop_value() {
    //             if let Some($second_type(first)) = stack.pop_value() {
    //                 let result = exec_fn(first, second)?;
    //                 stack.push_entry(StackEntry::Value(result));
    //             } else {
    //                 return Err(Trap);
    //             }
    //         } else {
    //             return Err(Trap);
    //         }
    //     }
    // };
    ($fn_name:ident, $pattern: path, $type: ty) => {
        #[inline]
        fn $fn_name(
            exec_fn: impl FnOnce($type, $type) -> RResult<$type>,
            stack: &mut Stack,
        ) -> RResult<()> {
            if let Some($pattern(second)) = stack.pop_value() {
                if let Some($pattern(first)) = stack.pop_value() {
                    let result = exec_fn(first, second)?;
                    stack.push_entry(StackEntry::Value($pattern(result)));
                    return Ok(());
                } else {
                    return Err(Trap);
                }
            } else {
                return Err(Trap);
            }
        }
    };
}

binop_impl!(i32_binop, Val::I32, u32);
binop_impl!(i64_binop, Val::I64, u64);
binop_impl!(f32_binop, Val::F32, f32);
binop_impl!(f64_binop, Val::F64, f64);

pub fn iadd_16(a: u16, b: u16) -> RResult<u16> {
    Ok(((a as u128) + (b as u128)).rem_euclid(2u128.pow(16)) as u16)
}
pub fn iadd_32(a: u32, b: u32) -> RResult<u32> {
    Ok(((a as u128) + (b as u128)).rem_euclid(2u128.pow(32)) as u32)
}
pub fn iadd_64(a: u64, b: u64) -> RResult<u64> {
    Ok(((a as u128) + (b as u128)).rem_euclid(2u128.pow(64)) as u64)
}

pub fn isub_32(a: u32, b: u32) -> RResult<u32> {
    let base = 2u128.pow(32);
    Ok(((a as u128) + base - (b as u128)).rem_euclid(base) as u32)
}
pub fn isub_64(a: u64, b: u64) -> RResult<u64> {
    let base = 2u128.pow(64);
    Ok(((a as u128) + base - (b as u128)).rem_euclid(base) as u64)
}

// pub fn imul_8(a: u8, b: u8) -> u8 {
//     let base = 2u128.pow(8);
//     ((a as u128) * (b as u128)).rem_euclid(base) as u8
// }

pub fn imul_16(a: u16, b: u16) -> u16 {
    let base = 2u128.pow(16);
    ((a as u128) * (b as u128)).rem_euclid(base) as u16
}

pub fn imul_32(a: u32, b: u32) -> RResult<u32> {
    let base = 2u128.pow(32);
    Ok(((a as u128) * (b as u128)).rem_euclid(base) as u32)
}
pub fn imul_64(a: u64, b: u64) -> RResult<u64> {
    let base = 2u128.pow(64);
    Ok(((a as u128) * (b as u128)).rem_euclid(base) as u64)
}

pub fn idiv_32_u(a: u32, b: u32) -> RResult<u32> {
    if b == 0 {
        return Err(Trap);
    }
    Ok(a.div_euclid(b))
}

pub fn idiv_32_s(a: u32, b: u32) -> RResult<u32> {
    let a_s = a as i32;
    let b_s = b as i32;
    if b_s == 0 {
        return Err(Trap);
    }
    let div = a_s.div_euclid(b_s);
    if div == 2u32.pow(31) as i32 {
        return Err(Trap);
    }
    Ok(div as u32)
}

pub fn idiv_64_u(a: u64, b: u64) -> RResult<u64> {
    if b == 0 {
        return Err(Trap);
    }
    Ok(a.div_euclid(b))
}

pub fn idiv_64_s(a: u64, b: u64) -> RResult<u64> {
    let a_s = a as i64;
    let b_s = b as i64;
    if b_s == 0 {
        return Err(Trap);
    }
    let div = a_s / b_s;
    if div == 2u64.pow(63) as i64 {
        return Err(Trap);
    }
    Ok(div as u64)
}

pub fn irem_32_u(a: u32, b: u32) -> RResult<u32> {
    if b == 0 {
        return Err(Trap);
    }

    Ok(a - b * (a / b))
}

pub fn irem_32_s(a: u32, b: u32) -> RResult<u32> {
    let a_s = a as i32;
    let b_s = b as i32;
    if b_s == 0 {
        return Err(Trap);
    }

    Ok((a_s - b_s * (a_s / b_s)) as u32)
}

pub fn irem_64_u(a: u64, b: u64) -> RResult<u64> {
    if b == 0 {
        return Err(Trap);
    }

    Ok(a - b * (a / b))
}

pub fn irem_64_s(a: u64, b: u64) -> RResult<u64> {
    let a_s = a as i64;
    let b_s = b as i64;
    if b_s == 0 {
        return Err(Trap);
    }

    Ok((a_s - b_s * (a_s / b_s)) as u64)
}

pub fn iand<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitAnd<Output = T>,
{
    Ok(lhs & rhs)
}

pub fn iandnot<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitAnd<Output = T> + std::ops::Not<Output = T>,
{
    Ok(lhs & !rhs)
}

pub fn ior<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitOr<Output = T>,
{
    Ok(lhs | rhs)
}

pub fn ixor<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitXor<Output = T>,
{
    Ok(lhs ^ rhs)
}

// pub fn bitselect<T>(first: T, second: T, third: T) -> RResult<T>
// where
//     T: ::std::ops::Not<Output = T>
//         + ::std::ops::BitAnd<Output = T>
//         + ::std::ops::BitOr<Output = T>
//         + ::std::marker::Copy,
// {
//     Ok((first & third) | (second & !third))
// }

pub fn ishl_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    let shifted = lhs << k;
    Ok((shifted as u128).rem_euclid((2u128).pow(32)) as u32)
}

pub fn ishl_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    let shifted = lhs << k;
    Ok((shifted as u128).rem_euclid((2u128).pow(64)) as u64)
}

pub fn ishr_u_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    let bit = 0b11111111111111111111111111111110;
    let mut res = lhs;
    for _ in 0..k {
        res = (res & bit).rotate_right(1);
    }

    Ok(res)
}

pub fn ishr_u_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    let bit = 0b1111111111111111111111111111111111111111111111111111111111111110;
    let mut res = lhs;
    for _ in 0..k {
        res = (res & bit).rotate_right(1);
    }

    Ok(res)
}

pub fn ishr_s_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    if lhs.leading_ones() > 0 {
        let most_significant_bit = 0b10000000000000000000000000000000u32;
        let mut res = lhs;
        for _ in 0..k {
            res = res.rotate_right(1) | most_significant_bit;
        }

        Ok(res)
    } else {
        ishr_u_32(lhs, rhs)
    }
}

pub fn ishr_s_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    if lhs.leading_ones() > 0 {
        let most_significant_bit =
            0b1000000000000000000000000000000000000000000000000000000000000000u64;
        let mut res = lhs;
        for _ in 0..k {
            res = res.rotate_right(1) | most_significant_bit;
        }

        Ok(res)
    } else {
        ishr_u_64(lhs, rhs)
    }
}

pub fn irotl_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    Ok(lhs.rotate_left(k))
}

pub fn irotl_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64) as u32;
    Ok(lhs.rotate_left(k))
}

pub fn irotr_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    Ok(lhs.rotate_right(k))
}

pub fn irotr_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64) as u32;
    Ok(lhs.rotate_right(k))
}

pub fn fadd<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Add<Output = T>,
{
    Ok(lhs + rhs)
}

pub fn fsub<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Sub<Output = T>,
{
    Ok(lhs - rhs)
}

pub fn fmul<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Mul<Output = T>,
{
    Ok(lhs * rhs)
}

pub fn fdiv<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Div<Output = T>,
{
    Ok(lhs / rhs)
}

pub fn min<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: Min,
{
    Ok(lhs.get_min(rhs))
}

pub fn max<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: Max,
{
    Ok(lhs.get_max(rhs))
}

// #[cfg(test)]
// mod test {
//     use syntax::{
//         module::InstructionType,
//         types::{F32Type, F64Type, I32Type, I64Type},
//     };

//     use crate::{
//         execute::execute_instruction,
//         instances::{stack::Stack, store::Store, value::Val},
//         test_instruction, test_instruction_assert,
//     };

//     test_instruction!(
//         i32_extend8_s_positive,
//         vec![InstructionType::I32Const(I32Type(1))],
//         InstructionType::I32Extend8S,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i32_extend8_s_negative,
//         vec![InstructionType::I32Const(I32Type(4294967290))],
//         InstructionType::I32Extend8S,
//         Val::I32(-6i8 as u32)
//     );

//     test_instruction!(
//         i32_extend16_s_positive,
//         vec![InstructionType::I32Const(I32Type(1))],
//         InstructionType::I32Extend16S,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i32_extend16_s_negative,
//         vec![InstructionType::I32Const(I32Type(4294967285))],
//         InstructionType::I32Extend16S,
//         Val::I32(-11i16 as u32)
//     );

//     test_instruction!(
//         i64_extend8_s_positive,
//         vec![InstructionType::I64Const(I64Type(1))],
//         InstructionType::I64Extend8S,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i64_extend8_s_negative,
//         vec![InstructionType::I64Const(I64Type(4294967290))],
//         InstructionType::I64Extend8S,
//         Val::I64(-6i8 as u64)
//     );

//     test_instruction!(
//         i64_extend16_s_positive,
//         vec![InstructionType::I64Const(I64Type(1))],
//         InstructionType::I64Extend16S,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i64_extend16_s_negative,
//         vec![InstructionType::I64Const(I64Type(4294967285))],
//         InstructionType::I64Extend16S,
//         Val::I64(-11i16 as u64)
//     );

//     test_instruction!(
//         i64_extend32_s_positive,
//         vec![InstructionType::I64Const(I64Type(1))],
//         InstructionType::I64Extend16S,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i64_extend32_s_negative,
//         vec![InstructionType::I64Const(I64Type(4294967186))],
//         InstructionType::I64Extend16S,
//         Val::I64(-110i32 as u64)
//     );

//     test_instruction!(
//         i32_add_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(1)),
//             InstructionType::I32Const(I32Type(0))
//         ],
//         InstructionType::I32Add,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i32_add_with_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(2)),
//             InstructionType::I32Const(I32Type(u32::MAX)),
//         ],
//         InstructionType::I32Add,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i64_add_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(0)),
//             InstructionType::I64Const(I64Type(1)),
//         ],
//         InstructionType::I64Add,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i64_add_with_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(2)),
//             InstructionType::I64Const(I64Type(u64::MAX)),
//         ],
//         InstructionType::I64Add,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i32_sub_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(2)),
//             InstructionType::I32Const(I32Type(1)),
//         ],
//         InstructionType::I32Sub,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i32_sub_with_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(2)),
//             InstructionType::I32Const(I32Type(u32::MAX)),
//         ],
//         InstructionType::I32Sub,
//         Val::I32(3)
//     );

//     test_instruction!(
//         i64_sub_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(2)),
//             InstructionType::I64Const(I64Type(1)),
//         ],
//         InstructionType::I64Sub,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i64_sub_with_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(2)),
//             InstructionType::I64Const(I64Type(u64::MAX)),
//         ],
//         InstructionType::I64Sub,
//         Val::I64(3)
//     );

//     test_instruction!(
//         i32_mul_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(4)),
//             InstructionType::I32Const(I32Type(3)),
//         ],
//         InstructionType::I32Mul,
//         Val::I32(12)
//     );

//     test_instruction!(
//         i32_mul_with_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(3)),
//             InstructionType::I32Const(I32Type(u32::MAX / 2)),
//         ],
//         InstructionType::I32Mul,
//         Val::I32(2147483645)
//     );

//     test_instruction!(
//         i64_mul_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(4)),
//             InstructionType::I64Const(I64Type(3)),
//         ],
//         InstructionType::I64Mul,
//         Val::I64(12)
//     );

//     test_instruction!(
//         i64_mul_with_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(3)),
//             InstructionType::I64Const(I64Type(u64::MAX / 2)),
//         ],
//         InstructionType::I64Mul,
//         Val::I64(9223372036854775805)
//     );

//     test_instruction!(
//         i32_div_u_no_verflow,
//         vec![
//             InstructionType::I32Const(I32Type(6)),
//             InstructionType::I32Const(I32Type(2)),
//         ],
//         InstructionType::I32DivU,
//         Val::I32(3)
//     );

//     test_instruction!(
//         i32_div_u_with_verflow,
//         vec![
//             InstructionType::I32Const(I32Type(-6i32 as u32)),
//             InstructionType::I32Const(I32Type(3)),
//         ],
//         InstructionType::I32DivU,
//         Val::I32(1431655763)
//     );

//     test_instruction!(
//         i32_div_s_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(6)),
//             InstructionType::I32Const(I32Type(2)),
//         ],
//         InstructionType::I32DivS,
//         Val::I32(3)
//     );

//     test_instruction!(
//         i32_div_s_with_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(2)),
//             InstructionType::I32Const(I32Type(4294967295)),
//         ],
//         InstructionType::I32DivS,
//         Val::I32(-2i32 as u32)
//     );

//     test_instruction!(
//         i64_div_u_no_verflow,
//         vec![
//             InstructionType::I64Const(I64Type(6)),
//             InstructionType::I64Const(I64Type(2)),
//         ],
//         InstructionType::I64DivU,
//         Val::I64(3)
//     );

//     test_instruction!(
//         i64_div_u_with_verflow,
//         vec![
//             InstructionType::I64Const(I64Type(-6i64 as u64)),
//             InstructionType::I64Const(I64Type(3)),
//         ],
//         InstructionType::I64DivU,
//         Val::I64(6148914691236517203)
//     );

//     test_instruction!(
//         i64_div_s_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(6)),
//             InstructionType::I64Const(I64Type(2)),
//         ],
//         InstructionType::I64DivS,
//         Val::I64(3)
//     );

//     test_instruction!(
//         i64_div_s_with_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(18446744073709551605)),
//             InstructionType::I64Const(I64Type(3)),
//         ],
//         InstructionType::I64DivS,
//         Val::I64(-3i64 as u64)
//     );

//     test_instruction!(
//         i32_rem_u,
//         vec![
//             InstructionType::I32Const(I32Type(7)),
//             InstructionType::I32Const(I32Type(2))
//         ],
//         InstructionType::I32RemU,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i32_rem_s_with_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(u32::MAX)),
//             InstructionType::I32Const(I32Type(2))
//         ],
//         InstructionType::I32RemS,
//         Val::I32(-1i32 as u32)
//     );

//     test_instruction!(
//         i32_rem_s_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(9)),
//             InstructionType::I32Const(I32Type(7))
//         ],
//         InstructionType::I32RemS,
//         Val::I32(2)
//     );

//     test_instruction!(
//         i64_rem_u,
//         vec![
//             InstructionType::I64Const(I64Type(7)),
//             InstructionType::I64Const(I64Type(2))
//         ],
//         InstructionType::I64RemU,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i64_rem_s_with_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(18446744073709551605)),
//             InstructionType::I64Const(I64Type(2))
//         ],
//         InstructionType::I64RemS,
//         Val::I64(-1i64 as u64)
//     );

//     test_instruction!(
//         i64_rem_s_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(9)),
//             InstructionType::I64Const(I64Type(7))
//         ],
//         InstructionType::I64RemS,
//         Val::I64(2)
//     );

//     test_instruction!(
//         i32_and_zero,
//         vec![
//             InstructionType::I32Const(I32Type(5)),
//             InstructionType::I32Const(I32Type(2))
//         ],
//         InstructionType::I32And,
//         Val::I32(0)
//     );

//     test_instruction!(
//         i32_and_not_zero,
//         vec![
//             InstructionType::I32Const(I32Type(5)),
//             InstructionType::I32Const(I32Type(3))
//         ],
//         InstructionType::I32And,
//         Val::I32(1)
//     );

//     test_instruction!(
//         i64_and_zero,
//         vec![
//             InstructionType::I64Const(I64Type(5)),
//             InstructionType::I64Const(I64Type(2))
//         ],
//         InstructionType::I64And,
//         Val::I64(0)
//     );

//     test_instruction!(
//         i64_and_not_zero,
//         vec![
//             InstructionType::I64Const(I64Type(5)),
//             InstructionType::I64Const(I64Type(3))
//         ],
//         InstructionType::I64And,
//         Val::I64(1)
//     );

//     test_instruction!(
//         i32_or_zero,
//         vec![
//             InstructionType::I32Const(I32Type(0)),
//             InstructionType::I32Const(I32Type(0))
//         ],
//         InstructionType::I32Or,
//         Val::I32(0)
//     );

//     test_instruction!(
//         i32_or_not_zero,
//         vec![
//             InstructionType::I32Const(I32Type(5)),
//             InstructionType::I32Const(I32Type(3))
//         ],
//         InstructionType::I32Or,
//         Val::I32(7)
//     );

//     test_instruction!(
//         i64_or_zero,
//         vec![
//             InstructionType::I64Const(I64Type(0)),
//             InstructionType::I64Const(I64Type(0))
//         ],
//         InstructionType::I64Or,
//         Val::I64(0)
//     );

//     test_instruction!(
//         i64_or_not_zero,
//         vec![
//             InstructionType::I64Const(I64Type(5)),
//             InstructionType::I64Const(I64Type(3))
//         ],
//         InstructionType::I64Or,
//         Val::I64(7)
//     );

//     test_instruction!(
//         i32_xor_zero,
//         vec![
//             InstructionType::I32Const(I32Type(2)),
//             InstructionType::I32Const(I32Type(2))
//         ],
//         InstructionType::I32Xor,
//         Val::I32(0)
//     );

//     test_instruction!(
//         i32_xor_not_zero,
//         vec![
//             InstructionType::I32Const(I32Type(5)),
//             InstructionType::I32Const(I32Type(3))
//         ],
//         InstructionType::I32Xor,
//         Val::I32(6)
//     );

//     test_instruction!(
//         i64_xor_zero,
//         vec![
//             InstructionType::I64Const(I64Type(2)),
//             InstructionType::I64Const(I64Type(2))
//         ],
//         InstructionType::I64Xor,
//         Val::I64(0)
//     );

//     test_instruction!(
//         i64_xor_not_zero,
//         vec![
//             InstructionType::I64Const(I64Type(5)),
//             InstructionType::I64Const(I64Type(3))
//         ],
//         InstructionType::I64Xor,
//         Val::I64(6)
//     );

//     test_instruction!(
//         ishl_32_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(10)),
//             InstructionType::I32Const(I32Type(3)),
//         ],
//         InstructionType::I32Shl,
//         Val::I32(80)
//     );

//     test_instruction!(
//         ishl_32_rot_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(10)),
//             InstructionType::I32Const(I32Type(32)),
//         ],
//         InstructionType::I32Shl,
//         Val::I32(10)
//     );

//     test_instruction!(
//         ishl_32_base_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
//             InstructionType::I32Const(I32Type(1))
//         ],
//         InstructionType::I32Shl,
//         Val::I32(0b10u32)
//     );

//     test_instruction!(
//         ishl_64_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(10)),
//             InstructionType::I64Const(I64Type(3)),
//         ],
//         InstructionType::I64Shl,
//         Val::I64(80)
//     );

//     test_instruction!(
//         ishl_64_rot_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(10)),
//             InstructionType::I64Const(I64Type(64)),
//         ],
//         InstructionType::I64Shl,
//         Val::I64(10)
//     );

//     test_instruction!(
//         ishl_64_base_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000000001u64
//             )),
//             InstructionType::I64Const(I64Type(1))
//         ],
//         InstructionType::I64Shl,
//         Val::I64(0b10u64)
//     );

//     test_instruction!(
//         ishr_u_32_not_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
//             InstructionType::I32Const(I32Type(1))
//         ],
//         InstructionType::I32ShrU,
//         Val::I32(0b01000000000000000000000000000000u32)
//     );

//     test_instruction!(
//         ishr_u_32_rot_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
//             InstructionType::I32Const(I32Type(32))
//         ],
//         InstructionType::I32ShrU,
//         Val::I32(0b10000000000000000000000000000001u32)
//     );

//     test_instruction!(
//         ishr_u_64_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000000001u64
//             )),
//             InstructionType::I64Const(I64Type(1))
//         ],
//         InstructionType::I64ShrU,
//         Val::I64(0b0100000000000000000000000000000000000000000000000000000000000000u64)
//     );

//     test_instruction!(
//         ishr_u_64_rot_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000000001u64
//             )),
//             InstructionType::I64Const(I64Type(64))
//         ],
//         InstructionType::I64ShrU,
//         Val::I64(0b1000000000000000000000000000000000000000000000000000000000000001u64)
//     );

//     test_instruction!(
//         ishr_s_32_no_overflow_zero,
//         vec![
//             InstructionType::I32Const(I32Type(0b00000000000000000000000000000010u32)),
//             InstructionType::I32Const(I32Type(1))
//         ],
//         InstructionType::I32ShrS,
//         Val::I32(0b0000000000000000000000000000001u32)
//     );

//     test_instruction!(
//         ishr_s_32_no_overflow_one,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000001000u32)),
//             InstructionType::I32Const(I32Type(3))
//         ],
//         InstructionType::I32ShrS,
//         Val::I32(0b11110000000000000000000000000001u32)
//     );

//     test_instruction!(
//         ishr_s_32_overflow_rot,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000001000u32)),
//             InstructionType::I32Const(I32Type(32))
//         ],
//         InstructionType::I32ShrS,
//         Val::I32(0b10000000000000000000000000001000u32)
//     );

//     test_instruction!(
//         ishr_s_64_no_overflow_zero,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b0000000000000000000000000000000000000000000000000000000000000010u64
//             )),
//             InstructionType::I64Const(I64Type(1))
//         ],
//         InstructionType::I64ShrS,
//         Val::I64(0b000000000000000000000000000000000000000000000000000000000000001u64)
//     );

//     test_instruction!(
//         ishr_s_64_no_overflow_one,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000001000u64
//             )),
//             InstructionType::I64Const(I64Type(3))
//         ],
//         InstructionType::I64ShrS,
//         Val::I64(0b1111000000000000000000000000000000000000000000000000000000000001u64)
//     );

//     test_instruction!(
//         ishr_s_64_overflow_rot,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000001000u64
//             )),
//             InstructionType::I64Const(I64Type(64))
//         ],
//         InstructionType::I64ShrS,
//         Val::I64(0b1000000000000000000000000000000000000000000000000000000000001000u64)
//     );

//     test_instruction!(
//         irotl_32_no_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
//             InstructionType::I32Const(I32Type(1))
//         ],
//         InstructionType::I32Rotl,
//         Val::I32(3)
//     );

//     test_instruction!(
//         irotl_32_rot_overflow,
//         vec![
//             InstructionType::I32Const(I32Type(0b10000000000000000000000000000001u32)),
//             InstructionType::I32Const(I32Type(32))
//         ],
//         InstructionType::I32Rotl,
//         Val::I32(0b10000000000000000000000000000001u32)
//     );

//     test_instruction!(
//         irotl_64_no_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000000001u64
//             )),
//             InstructionType::I64Const(I64Type(1))
//         ],
//         InstructionType::I64Rotl,
//         Val::I64(3)
//     );

//     test_instruction!(
//         irotl_64_rot_overflow,
//         vec![
//             InstructionType::I64Const(I64Type(
//                 0b1000000000000000000000000000000000000000000000000000000000000001u64
//             )),
//             InstructionType::I64Const(I64Type(64))
//         ],
//         InstructionType::I64Rotl,
//         Val::I64(0b1000000000000000000000000000000000000000000000000000000000000001u64)
//     );

//     test_instruction!(
//         f32_add_no_overflow,
//         vec![
//             InstructionType::F32Const(F32Type(0.9)),
//             InstructionType::F32Const(F32Type(0.1))
//         ],
//         InstructionType::F32Add,
//         Val::F32(1.0)
//     );

//     test_instruction!(
//         f64_add_no_overflow,
//         vec![
//             InstructionType::F64Const(F64Type(0.9)),
//             InstructionType::F64Const(F64Type(0.1))
//         ],
//         InstructionType::F64Add,
//         Val::F64(1.0)
//     );

//     test_instruction!(
//         f32_sub_no_overflow,
//         vec![
//             InstructionType::F32Const(F32Type(0.1)),
//             InstructionType::F32Const(F32Type(0.4))
//         ],
//         InstructionType::F32Sub,
//         Val::F32(-0.3)
//     );

//     test_instruction!(
//         f64_sub_no_overflow,
//         vec![
//             InstructionType::F64Const(F64Type(0.11)),
//             InstructionType::F64Const(F64Type(0.3))
//         ],
//         InstructionType::F64Sub,
//         Val::F64(-0.19)
//     );

//     test_instruction!(
//         f32_mul_inverse_sign,
//         vec![
//             InstructionType::F32Const(F32Type(-1.0)),
//             InstructionType::F32Const(F32Type(0.3))
//         ],
//         InstructionType::F32Mul,
//         Val::F32(-0.3)
//     );

//     test_instruction!(
//         f32_mul_no_overflow,
//         vec![
//             InstructionType::F32Const(F32Type(1.0)),
//             InstructionType::F32Const(F32Type(0.3))
//         ],
//         InstructionType::F32Mul,
//         Val::F32(0.3)
//     );

//     test_instruction!(
//         f64_mul_no_overflow,
//         vec![
//             InstructionType::F64Const(F64Type(1.0)),
//             InstructionType::F64Const(F64Type(0.3))
//         ],
//         InstructionType::F64Mul,
//         Val::F64(0.3)
//     );

//     test_instruction!(
//         f32_div_no_overflow,
//         vec![
//             InstructionType::F32Const(F32Type(1.0)),
//             InstructionType::F32Const(F32Type(0.2))
//         ],
//         InstructionType::F32Div,
//         Val::F32(5.0)
//     );

//     test_instruction!(
//         f32_div_neg_infinity,
//         vec![
//             InstructionType::F32Const(F32Type(1.0)),
//             InstructionType::F32Const(F32Type(-0.0))
//         ],
//         InstructionType::F32Div,
//         Val::F32(f32::NEG_INFINITY)
//     );

//     test_instruction!(
//         f32_div_pos_infinity,
//         vec![
//             InstructionType::F32Const(F32Type(1.0)),
//             InstructionType::F32Const(F32Type(0.0))
//         ],
//         InstructionType::F32Div,
//         Val::F32(f32::INFINITY)
//     );

//     test_instruction!(
//         f64_div_no_overflow,
//         vec![
//             InstructionType::F64Const(F64Type(1.0)),
//             InstructionType::F64Const(F64Type(0.2))
//         ],
//         InstructionType::F64Div,
//         Val::F64(5.0)
//     );

//     test_instruction!(
//         f64_div_neg_infinity,
//         vec![
//             InstructionType::F64Const(F64Type(1.0)),
//             InstructionType::F64Const(F64Type(-0.0))
//         ],
//         InstructionType::F64Div,
//         Val::F64(f64::NEG_INFINITY)
//     );

//     test_instruction!(
//         f64_div_pos_infinity,
//         vec![
//             InstructionType::F64Const(F64Type(1.0)),
//             InstructionType::F64Const(F64Type(0.0))
//         ],
//         InstructionType::F64Div,
//         Val::F64(f64::INFINITY)
//     );

//     test_instruction_assert!(
//         f32_min_nan_1,
//         vec![
//             InstructionType::F32Const(F32Type(f32::NAN)),
//             InstructionType::F32Const(F32Type(0.0))
//         ],
//         InstructionType::F32Min,
//         |val: Val| {
//             match val {
//                 Val::F32(val_32) => {
//                     assert!(val_32.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F32 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction_assert!(
//         f32_min_nan_2,
//         vec![
//             InstructionType::F32Const(F32Type(0.0)),
//             InstructionType::F32Const(F32Type(f32::NAN))
//         ],
//         InstructionType::F32Min,
//         |val: Val| {
//             match val {
//                 Val::F32(val_32) => {
//                     assert!(val_32.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F32 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction!(
//         f32_min,
//         vec![
//             InstructionType::F32Const(F32Type(0.0)),
//             InstructionType::F32Const(F32Type(-0.0))
//         ],
//         InstructionType::F32Min,
//         Val::F32(-0.0)
//     );

//     test_instruction_assert!(
//         f64_min_nan_1,
//         vec![
//             InstructionType::F64Const(F64Type(f64::NAN)),
//             InstructionType::F64Const(F64Type(0.0))
//         ],
//         InstructionType::F64Min,
//         |val: Val| {
//             match val {
//                 Val::F64(val_64) => {
//                     assert!(val_64.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F64 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction_assert!(
//         f64_min_nan_2,
//         vec![
//             InstructionType::F64Const(F64Type(0.0)),
//             InstructionType::F64Const(F64Type(f64::NAN))
//         ],
//         InstructionType::F64Min,
//         |val: Val| {
//             match val {
//                 Val::F64(val_64) => {
//                     assert!(val_64.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F64 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction!(
//         f64_min,
//         vec![
//             InstructionType::F64Const(F64Type(0.0)),
//             InstructionType::F64Const(F64Type(-0.0))
//         ],
//         InstructionType::F64Min,
//         Val::F64(-0.0)
//     );

//     test_instruction_assert!(
//         f32_max_nan_1,
//         vec![
//             InstructionType::F32Const(F32Type(f32::NAN)),
//             InstructionType::F32Const(F32Type(0.0))
//         ],
//         InstructionType::F32Max,
//         |val: Val| {
//             match val {
//                 Val::F32(val_32) => {
//                     assert!(val_32.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F32 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction_assert!(
//         f32_max_nan_2,
//         vec![
//             InstructionType::F32Const(F32Type(0.0)),
//             InstructionType::F32Const(F32Type(f32::NAN))
//         ],
//         InstructionType::F32Max,
//         |val: Val| {
//             match val {
//                 Val::F32(val_32) => {
//                     assert!(val_32.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F32 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction!(
//         f32_max,
//         vec![
//             InstructionType::F32Const(F32Type(0.0)),
//             InstructionType::F32Const(F32Type(-0.0))
//         ],
//         InstructionType::F32Max,
//         Val::F32(0.0)
//     );

//     test_instruction_assert!(
//         f64_max_nan_1,
//         vec![
//             InstructionType::F64Const(F64Type(f64::NAN)),
//             InstructionType::F64Const(F64Type(0.0))
//         ],
//         InstructionType::F64Max,
//         |val: Val| {
//             match val {
//                 Val::F64(val_64) => {
//                     assert!(val_64.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F64 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction_assert!(
//         f64_max_nan_2,
//         vec![
//             InstructionType::F64Const(F64Type(0.0)),
//             InstructionType::F64Const(F64Type(f64::NAN))
//         ],
//         InstructionType::F64Max,
//         |val: Val| {
//             match val {
//                 Val::F64(val_64) => {
//                     assert!(val_64.is_nan());
//                 }
//                 other => {
//                     assert!(false, "unexpected value type {other:?}, F64 is expected");
//                 }
//             }
//         }
//     );

//     test_instruction!(
//         f64_max,
//         vec![
//             InstructionType::F64Const(F64Type(0.0)),
//             InstructionType::F64Const(F64Type(-0.0))
//         ],
//         InstructionType::F64Max,
//         Val::F64(0.0)
//     );

//     test_instruction!(
//         f32_copy_sign_positive_to_negative,
//         vec![
//             InstructionType::F32Const(F32Type(-1.0)),
//             InstructionType::F32Const(F32Type(2.0))
//         ],
//         InstructionType::F32Copysign,
//         Val::F32(1.0)
//     );

//     test_instruction!(
//         f64_copy_sign_positive_to_negative,
//         vec![
//             InstructionType::F64Const(F64Type(-1.0)),
//             InstructionType::F64Const(F64Type(2.0))
//         ],
//         InstructionType::F64Copysign,
//         Val::F64(1.0)
//     );
// }
