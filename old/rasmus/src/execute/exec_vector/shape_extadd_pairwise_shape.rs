use crate::{
    execute::exec_binop::{iadd_16, iadd_32},
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

use super::{to_lanes_16x8, to_lanes_8x16, vec_from_lanes};

pub fn i16x8_extadd_pairwise_i8x16(stack: &mut Stack, is_singed: bool) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(v);
    let l = lanes.len();

    let mut new_lanes: Vec<u16> = Vec::with_capacity(8);
    for i in 0..(l / 2) {
        if is_singed {
            new_lanes
                .push(iadd_16(lanes[2 * i] as i8 as u16, lanes[2 * i + 1] as i8 as u16).unwrap());
        } else {
            new_lanes.push(iadd_16(lanes[2 * i] as u16, lanes[2 * i + 1] as u16).unwrap());
        }
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub fn i32x4_extadd_pairwise_i16x8(stack: &mut Stack, is_singed: bool) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(v);
    let l = lanes.len();

    let mut new_lanes: Vec<u32> = Vec::with_capacity(8);
    for i in 0..(l / 2) {
        if is_singed {
            new_lanes
                .push(iadd_32(lanes[2 * i] as i16 as u32, lanes[2 * i + 1] as i16 as u32).unwrap());
        } else {
            new_lanes.push(iadd_32(lanes[2 * i] as u32, lanes[2 * i + 1] as u32).unwrap());
        }
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}
