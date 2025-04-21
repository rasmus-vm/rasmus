use crate::{
    execute::exec_binop::{iadd_32, imul_32},
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

use super::{to_lanes_16x8, vec_from_lanes};

pub fn i32x4_dot_i16x8s(stack: &mut Stack) -> RResult<()> {
    let i = stack.pop_v128().ok_or(Trap)?;
    let j = stack.pop_v128().ok_or(Trap)?;
    let i_lanes: Vec<u32> = to_lanes_16x8(i).iter().map(|l| *l as i16 as u32).collect();
    let j_lanes: Vec<u32> = to_lanes_16x8(j).iter().map(|l| *l as i16 as u32).collect();

    let k: Vec<u32> = i_lanes
        .iter()
        .zip(j_lanes.iter())
        .map(|(&li, &lj)| imul_32(li, lj).unwrap())
        .collect();

    let mut added = Vec::with_capacity(4);

    for i in 0..4 {
        added.push(iadd_32(k[2 * i], k[2 * i + 1]).unwrap());
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(added))));

    Ok(())
}
