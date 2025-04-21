use crate::{
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

use super::{to_lanes_16x8, to_lanes_32x4, vec_from_lanes};

pub fn shape_8x16_narrow_16x8_u(stack: &mut Stack) -> RResult<()> {
    let c2 = stack.pop_v128().ok_or(Trap)?;
    let l2 = to_lanes_16x8(c2);
    let c1 = stack.pop_v128().ok_or(Trap)?;
    let l1 = to_lanes_16x8(c1);

    let narrowed = l1
        .iter()
        .chain(l2.iter())
        .map(|l| {
            let signed = *l as i16;
            if signed < 0 {
                0u8
            } else if signed > u8::MAX as i16 {
                u8::MAX
            } else {
                *l as u8
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(narrowed))));

    Ok(())
}

pub fn shape_8x16_narrow_16x8_s(stack: &mut Stack) -> RResult<()> {
    let c2 = stack.pop_v128().ok_or(Trap)?;
    let l2 = to_lanes_16x8(c2);
    let c1 = stack.pop_v128().ok_or(Trap)?;
    let l1 = to_lanes_16x8(c1);

    let narrowed = l1
        .iter()
        .chain(l2.iter())
        .map(|l| {
            let signed = *l as i16;
            if signed < i8::MIN as i16 {
                i8::MIN as u8
            } else if signed > i8::MAX as i16 {
                i8::MAX as u8
            } else {
                *l as u8
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(narrowed))));

    Ok(())
}

pub fn shape_16x8_narrow_32x4_u(stack: &mut Stack) -> RResult<()> {
    let c2 = stack.pop_v128().ok_or(Trap)?;
    let l2 = to_lanes_32x4(c2);
    let c1 = stack.pop_v128().ok_or(Trap)?;
    let l1 = to_lanes_32x4(c1);

    let narrowed = l1
        .iter()
        .chain(l2.iter())
        .map(|l| {
            let signed = *l as i32;
            if signed < 0 {
                0u16
            } else if signed > u16::MAX as i32 {
                u16::MAX
            } else {
                *l as u16
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(narrowed))));

    Ok(())
}

pub fn shape_16x8_narrow_32x4_s(stack: &mut Stack) -> RResult<()> {
    let c2 = stack.pop_v128().ok_or(Trap)?;
    let l2 = to_lanes_32x4(c2);
    let c1 = stack.pop_v128().ok_or(Trap)?;
    let l1 = to_lanes_32x4(c1);

    let narrowed = l1
        .iter()
        .chain(l2.iter())
        .map(|l| {
            let signed = *l as i32;
            if signed < i16::MIN as i32 {
                i16::MIN as u16
            } else if signed > i16::MAX as i32 {
                i16::MAX as u16
            } else {
                *l as u16
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(narrowed))));

    Ok(())
}
