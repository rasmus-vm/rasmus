use crate::entities::types::{LaneIdx, U32Type};

use crate::{
    address::MemAddr,
    execute::{
        exec_const::v128_const,
        exec_vector::{to_lanes_16x8, to_lanes_32x4, to_lanes_64x2, to_lanes_8x16, vec_from_lanes},
    },
    instances::{
        stack::{Stack, StackEntry},
        store::Store,
        value::Val,
    },
    result::{RResult, Trap},
    sign::Sign,
};

use super::memory_bytes::{
    get_u16_bytes, get_u32_bytes, get_u64_bytes, get_u8_bytes, BytesGetter, MemoryBytesGetter,
};

pub fn v128_load(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 128;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    v128_const(&MemoryBytesGetter::get_bytes(&mem_inst.data, ea), stack)
}

pub fn v128_load_8x8(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + (8 * 8) / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = mem_inst.data[ea..(ea + (8usize * 8usize) / 8)].as_ref();
    let mk: Vec<u16> = (0..8)
        .map(|k| {
            let offset = k * 8 / 8;
            let n = u8::from_le_bytes(get_u8_bytes(b, offset));
            match sign {
                Sign::Signed => n as i8 as u16,
                Sign::Unsigned => n as u16,
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(mk))));

    Ok(())
}

pub fn v128_load_16x4(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + (16 * 4) / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = mem_inst.data[ea..(ea + (16usize * 4usize) / 8)].as_ref();
    let mk: Vec<u32> = (0..4)
        .map(|k| {
            let offset = k * 16 / 8;
            let n = u16::from_le_bytes(get_u16_bytes(b, offset));
            match sign {
                Sign::Signed => n as i16 as u32,
                Sign::Unsigned => n as u32,
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(mk))));

    Ok(())
}

pub fn v128_load_32x2(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + (32 * 2) / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = mem_inst.data[ea..(ea + (32usize * 2usize) / 8)].as_ref();
    let mk: Vec<u64> = (0..2)
        .map(|k| {
            let offset = k * 32 / 8;
            let n = u32::from_le_bytes(get_u32_bytes(b, offset));
            match sign {
                Sign::Signed => n as i32 as u64,
                Sign::Unsigned => n as u64,
            }
        })
        .collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(mk))));

    Ok(())
}

pub fn v128_load8_splat(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 8 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u8::from_le_bytes(get_u8_bytes(&mem_inst.data, ea));
    let lanes = (0..128 / 8).map(|_| n).collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes))));

    Ok(())
}

pub fn v128_load16_splat(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 16 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u16::from_le_bytes(get_u16_bytes(&mem_inst.data, ea));
    let lanes = (0..128 / 16).map(|_| n).collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes))));

    Ok(())
}

pub fn v128_load32_splat(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 32 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u32::from_le_bytes(get_u32_bytes(&mem_inst.data, ea));
    let lanes = (0..128 / 32).map(|_| n).collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes))));

    Ok(())
}

pub fn v128_load64_splat(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 64 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u64::from_le_bytes(get_u64_bytes(&mem_inst.data, ea));
    let lanes = (0..128 / 64).map(|_| n).collect();

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes))));

    Ok(())
}

pub fn v128_load32_zero(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 32 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u32::from_le_bytes(get_u32_bytes(&mem_inst.data, ea));

    stack.push_entry(StackEntry::Value(Val::Vec(n as u128)));

    Ok(())
}

pub fn v128_load64_zero(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 64 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u64::from_le_bytes(get_u64_bytes(&mem_inst.data, ea));

    stack.push_entry(StackEntry::Value(Val::Vec(n as u128)));

    Ok(())
}

pub fn v128_load8_lane(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    &LaneIdx(lane_idx): &LaneIdx,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let v = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 8 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u8::from_le_bytes(get_u8_bytes(&mem_inst.data, ea));
    let mut lanes = to_lanes_8x16(v);
    lanes[lane_idx as usize] = n;

    stack.push_entry(StackEntry::Value(Val::Vec(n as u128)));

    Ok(())
}

pub fn v128_load16_lane(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    &LaneIdx(lane_idx): &LaneIdx,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let v = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 16 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u16::from_le_bytes(get_u16_bytes(&mem_inst.data, ea));
    let mut lanes = to_lanes_16x8(v);
    lanes[lane_idx as usize] = n;

    stack.push_entry(StackEntry::Value(Val::Vec(n as u128)));

    Ok(())
}

pub fn v128_load32_lane(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    &LaneIdx(lane_idx): &LaneIdx,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let v = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 32 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u32::from_le_bytes(get_u32_bytes(&mem_inst.data, ea));
    let mut lanes = to_lanes_32x4(v);
    lanes[lane_idx as usize] = n;

    stack.push_entry(StackEntry::Value(Val::Vec(n as u128)));

    Ok(())
}

pub fn v128_load64_lane(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    &LaneIdx(lane_idx): &LaneIdx,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let v = stack.pop_v128().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;

    if (ea + 64 / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u64::from_le_bytes(get_u64_bytes(&mem_inst.data, ea));
    let mut lanes = to_lanes_64x2(v);
    lanes[lane_idx as usize] = n;

    stack.push_entry(StackEntry::Value(Val::Vec(n as u128)));

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
