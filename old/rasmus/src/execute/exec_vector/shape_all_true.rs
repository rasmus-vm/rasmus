use crate::instances::stack::Stack;
use crate::instances::value::Val;
use crate::result::{RResult, Trap};

use super::{to_lanes_16x8, to_lanes_32x4, to_lanes_8x16, to_lanes_64x2};

pub fn all_true_8x16(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(v);
    let all_true = if lanes.iter().all(|v| *v != 0) {
        1u32
    } else {
        0u32
    };

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        all_true,
    )));

    Ok(())
}

pub fn all_true_16x8(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(v);
    let all_true = if lanes.iter().all(|v| *v != 0) {
        1u32
    } else {
        0u32
    };

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        all_true,
    )));

    Ok(())
}

pub fn all_true_32x4(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(v);
    let all_true = if lanes.iter().all(|v| *v != 0) {
        1u32
    } else {
        0u32
    };

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        all_true,
    )));

    Ok(())
}

pub fn all_true_64x2(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_64x2(v);
    let all_true = if lanes.iter().all(|v| *v != 0) {
        1u32
    } else {
        0u32
    };

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        all_true,
    )));

    Ok(())
}
