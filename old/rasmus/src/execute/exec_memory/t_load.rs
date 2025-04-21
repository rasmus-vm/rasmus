use crate::entities::types::U32Type;

use crate::{
    address::MemAddr,
    execute::exec_const::{f32_const, f64_const, i32_const, i64_const},
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
    sign::Sign,
};

use super::memory_bytes::{BytesGetter, MemoryBytesGetter};

pub fn i32_load(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let c = u32::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));

    i32_const(&c, stack)
}

pub fn i64_load(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 64;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let c = u64::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));

    i64_const(&c, stack)
}

pub fn f32_load(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let c = f32::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));

    f32_const(&c, stack)
}

pub fn f64_load(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 64;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let c = f64::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));

    f64_const(&c, stack)
}

pub fn i32_load_8(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 8;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u8::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));
    let c = match sign {
        Sign::Signed => n as i8 as u32,
        Sign::Unsigned => n as u32,
    };

    i32_const(&c, stack)
}

pub fn i32_load_16(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 16;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u16::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));
    let c = match sign {
        Sign::Signed => n as i16 as u32,
        Sign::Unsigned => n as u32,
    };

    i32_const(&c, stack)
}

pub fn i64_load_8(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 8;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u8::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));
    let c = match sign {
        Sign::Signed => n as i8 as u64,
        Sign::Unsigned => n as u64,
    };

    i64_const(&c, stack)
}

pub fn i64_load_16(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 16;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u16::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));
    let c = match sign {
        Sign::Signed => n as i16 as u64,
        Sign::Unsigned => n as u64,
    };

    i64_const(&c, stack)
}

pub fn i64_load_32(
    stack: &mut Stack,
    store: &mut Store,
    &(U32Type(offset), U32Type(_align)): &(U32Type, U32Type),
    sign: Sign,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;
    let ea = (i + offset) as usize;
    let bits = 32;

    if (ea + bits / 8) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    let n = u32::from_le_bytes(MemoryBytesGetter::get_bytes(&mem_inst.data, ea));
    let c = match sign {
        Sign::Signed => n as i32 as u64,
        Sign::Unsigned => n as u64,
    };

    i64_const(&c, stack)
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
