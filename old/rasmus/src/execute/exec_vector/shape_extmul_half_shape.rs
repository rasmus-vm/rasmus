use crate::{
    execute::exec_binop::{imul_16, imul_32, imul_64},
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

use super::{to_lanes_16x8, to_lanes_32x4, to_lanes_8x16, vec_from_lanes, Half};

pub fn i16x8_extmul_half_i8x16(stack: &mut Stack, half: Half, is_signed: bool) -> RResult<()> {
    let lanes_1 = to_lanes_8x16(stack.pop_v128().ok_or(Trap)?);
    let lanes_2 = to_lanes_8x16(stack.pop_v128().ok_or(Trap)?);

    let range = match half {
        Half::Low => 0..8,
        Half::High => 8..16,
    };

    let new_lanes = lanes_1[range.clone()]
        .iter()
        .zip(lanes_2[range.clone()].iter())
        .map(|(l1, l2)| {
            if is_signed {
                imul_16(*l1 as i8 as u16, *l2 as i8 as u16)
            } else {
                imul_16(*l1 as u16, *l2 as u16)
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub fn i32x4_extmul_half_i16x8(stack: &mut Stack, half: Half, is_signed: bool) -> RResult<()> {
    let lanes_1 = to_lanes_16x8(stack.pop_v128().ok_or(Trap)?);
    let lanes_2 = to_lanes_16x8(stack.pop_v128().ok_or(Trap)?);

    let range = match half {
        Half::Low => 0..4,
        Half::High => 4..8,
    };

    let new_lanes = lanes_1[range.clone()]
        .iter()
        .zip(lanes_2[range.clone()].iter())
        .map(|(l1, l2)| {
            if is_signed {
                imul_32(*l1 as i16 as u32, *l2 as i16 as u32).unwrap()
            } else {
                imul_32(*l1 as u32, *l2 as u32).unwrap()
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub fn i64x2_extmul_half_i32x4(stack: &mut Stack, half: Half, is_signed: bool) -> RResult<()> {
    let lanes_1 = to_lanes_32x4(stack.pop_v128().ok_or(Trap)?);
    let lanes_2 = to_lanes_32x4(stack.pop_v128().ok_or(Trap)?);

    let range = match half {
        Half::Low => 0..2,
        Half::High => 2..4,
    };

    let new_lanes = lanes_1[range.clone()]
        .iter()
        .zip(lanes_2[range.clone()].iter())
        .map(|(l1, l2)| {
            if is_signed {
                imul_64(*l1 as i32 as u64, *l2 as i32 as u64).unwrap()
            } else {
                imul_64(*l1 as u64, *l2 as u64).unwrap()
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}
