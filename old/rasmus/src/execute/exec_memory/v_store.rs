use crate::entities::types::{LaneIdx, U32Type};

use crate::{
    address::MemAddr,
    execute::exec_vector::{to_lanes_16x8, to_lanes_32x4, to_lanes_64x2, to_lanes_8x16},
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::memory_bytes::set_bytes;

pub fn v128_store(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 128;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = t.to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn v128_store8_lane(
    stack: &mut Stack,
    store: &mut Store,
    &((U32Type(offset), U32Type(_align)), LaneIdx(x)): &((U32Type, U32Type), LaneIdx),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 8;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let lanes = to_lanes_8x16(t);
    let b = lanes[x as usize].to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn v128_store16_lane(
    stack: &mut Stack,
    store: &mut Store,
    &((U32Type(offset), U32Type(_align)), LaneIdx(x)): &((U32Type, U32Type), LaneIdx),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 16;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let lanes = to_lanes_16x8(t);
    let b = lanes[x as usize].to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn v128_store32_lane(
    stack: &mut Stack,
    store: &mut Store,
    &((U32Type(offset), U32Type(_align)), LaneIdx(x)): &((U32Type, U32Type), LaneIdx),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let lanes = to_lanes_32x4(t);
    let b = lanes[x as usize].to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn v128_store64_lane(
    stack: &mut Stack,
    store: &mut Store,
    &((U32Type(offset), U32Type(_align)), LaneIdx(x)): &((U32Type, U32Type), LaneIdx),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 64;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let lanes = to_lanes_64x2(t);
    let b = lanes[x as usize].to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

fn get_mem_addr(stack: &mut Stack) -> RResult<MemAddr> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    current_frame
        .module
        .borrow()
        .memaddrs
        .get(0)
        .cloned()
        .ok_or(Trap)
}
