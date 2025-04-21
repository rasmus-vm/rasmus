use crate::entities::types::{DataIdx, U32Type};

use crate::{
    address::{DataAddr, MemAddr},
    execute::exec_const::i32_const,
    instances::{
        stack::{Stack, StackEntry},
        store::Store,
        value::Val,
    },
    result::{RResult, Trap},
    sign::Sign,
};

use super::{i32_load_8, i32_store8};

pub fn memory_size(stack: &mut Stack, store: &mut Store) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get(mem_addr).ok_or(Trap)?;

    let size = mem_inst.size();
    i32_const(&size, stack)
}

pub fn memory_grow(stack: &mut Stack, store: &mut Store) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;
    let size = mem_inst.size();
    let n = stack.pop_i32().ok_or(Trap)?;
    let err = -1i32 as u32;

    if mem_inst.grow_n(n).is_ok() {
        i32_const(&(size as u32), stack)
    } else {
        i32_const(&err, stack)
    }
}

pub fn memory_fill(stack: &mut Stack, store: &mut Store) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;

    let n = stack.pop_i32().ok_or(Trap)?;
    let val = Val::I32(stack.pop_i32().ok_or(Trap)?);
    let d = stack.pop_i32().ok_or(Trap)?;

    if (d + n) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    if n == 0 {
        return Ok(());
    }

    i32_const(&d, stack)?;
    stack.push_entry(StackEntry::Value(val.clone()));
    i32_store8(stack, store, &(U32Type(0), U32Type(0)))?;
    let next_d = d.checked_add(1).ok_or(Trap)?;
    i32_const(&next_d, stack)?;
    stack.push_entry(StackEntry::Value(val));
    i32_const(&(n - 1), stack)?;

    return memory_fill(stack, store);
}

pub fn memory_copy(stack: &mut Stack, store: &mut Store) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;

    let n = stack.pop_i32().ok_or(Trap)?;
    let s = stack.pop_i32().ok_or(Trap)?;
    let d = stack.pop_i32().ok_or(Trap)?;

    if (s + n) as usize > mem_inst.data.len() || (d + n) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    if n == 0 {
        return Ok(());
    }

    if d <= s {
        i32_const(&n, stack)?;
        i32_const(&s, stack)?;
        i32_load_8(stack, store, &(U32Type(0), U32Type(0)), Sign::Unsigned)?;
        i32_store8(stack, store, &(U32Type(0), U32Type(0)))?;
        let next_d = d.checked_add(1).ok_or(Trap)?;
        i32_const(&next_d, stack)?;
        let next_s = s.checked_add(1).ok_or(Trap)?;
        i32_const(&next_s, stack)?;
    } else {
        let next_d = d.checked_add(n).ok_or(Trap)? - 1;
        i32_const(&next_d, stack)?;
        let next_s = s.checked_add(n).ok_or(Trap)? - 1;
        i32_const(&next_s, stack)?;
        i32_load_8(stack, store, &(U32Type(0), U32Type(0)), Sign::Unsigned)?;
        i32_store8(stack, store, &(U32Type(0), U32Type(0)))?;
        i32_const(&d, stack)?;
        i32_const(&s, stack)?;
    }

    i32_const(&(n - 1), stack)?;

    return memory_copy(stack, store);
}

pub fn memory_init(
    stack: &mut Stack,
    store: &mut Store,
    &DataIdx(U32Type(x)): &DataIdx,
) -> RResult<()> {
    let mem_addr = get_mem_addr(stack)?;
    let mem_inst = store.mems.get_mut(mem_addr).ok_or(Trap)?;

    let data_addr = get_data_addr(stack, x as usize)?;
    let data_inst = store
        .datas
        .get_mut(data_addr)
        .ok_or(Trap)?
        .as_ref()
        .ok_or(Trap)?;

    let n = stack.pop_i32().ok_or(Trap)?;
    let s = stack.pop_i32().ok_or(Trap)?;
    let d = stack.pop_i32().ok_or(Trap)?;

    if (s + n) as usize > mem_inst.data.len() || (d + n) as usize > mem_inst.data.len() {
        return Err(Trap);
    }

    if n == 0 {
        return Ok(());
    }

    let b = data_inst.data.get(s as usize).ok_or(Trap)?;
    i32_const(&d, stack)?;
    i32_const(&(*b as u32), stack)?;
    i32_store8(stack, store, &(U32Type(0), U32Type(0)))?;

    let new_d = d.checked_add(1).ok_or(Trap)?;
    i32_const(&new_d, stack)?;
    let new_s = s.checked_add(1).ok_or(Trap)?;
    i32_const(&new_s, stack)?;
    i32_const(&(n - 1), stack)?;

    return memory_init(stack, store, &DataIdx(U32Type(x)));
}

pub fn data_drop(stack: &mut Stack, store: &mut Store, x: &DataIdx) -> RResult<()> {
    let &DataIdx(U32Type(data_idx)) = x;
    let data_addr = get_data_addr(stack, data_idx as usize)?;

    return store.drop_data(data_addr);
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

fn get_data_addr(stack: &mut Stack, x: usize) -> RResult<DataAddr> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    current_frame
        .module
        .borrow()
        .dataaddrs
        .get(x)
        .cloned()
        .ok_or(Trap)
}
