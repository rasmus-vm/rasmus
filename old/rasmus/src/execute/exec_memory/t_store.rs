use crate::entities::types::U32Type;

use crate::{
    address::MemAddr,
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::memory_bytes::set_bytes;

pub fn i32_store(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i32().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = t.to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn i64_store(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i64().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 64;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = t.to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn f32_store(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_f32().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = t.to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn f64_store(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_f64().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 64;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = t.to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn i32_store8(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i32().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 8;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = i32_wrap_8(t).to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn i32_store16(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i32().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 16;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = i32_wrap_16(t).to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn i64_store8(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i64().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 8;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = i64_wrap_8(t).to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn i64_store16(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i64().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 16;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = i64_wrap_16(t).to_le_bytes();
    set_bytes(&mut mem_inst.data, ea, b);

    Ok(())
}

pub fn i64_store32(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let t = stack.pop_i64().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let b = i64_wrap_32(t).to_le_bytes();
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

fn i32_wrap_8(t: u32) -> u8 {
    t.rem_euclid(2u32.pow(8)) as u8
}

fn i32_wrap_16(t: u32) -> u16 {
    t.rem_euclid(2u32.pow(16)) as u16
}

fn i64_wrap_8(t: u64) -> u8 {
    t.rem_euclid(2u64.pow(8)) as u8
}

fn i64_wrap_16(t: u64) -> u16 {
    t.rem_euclid(2u64.pow(16)) as u16
}
fn i64_wrap_32(t: u64) -> u32 {
    t.rem_euclid(2u64.pow(32)) as u32
}
