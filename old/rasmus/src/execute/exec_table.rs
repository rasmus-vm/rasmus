use crate::entities::types::{ElemIdx, TableIdx, U32Type};

use crate::{
    address::{ElemAddr, TableAddr},
    execute::exec_const::i32_const,
    instances::{
        stack::{Stack, StackEntry},
        store::Store,
        value::Val,
    },
    result::{RResult, Trap},
};

use super::exec_const::ref_const;

pub fn table_get(
    stack: &mut Stack,
    store: &mut Store,
    &TableIdx(U32Type(idx)): &TableIdx,
) -> RResult<()> {
    let table_addr = get_table_addr(stack, idx)?;
    let table_instance = store.tables.get(table_addr).ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)? as usize;

    if i >= table_instance.elem.len() {
        return Err(Trap);
    }

    let val = table_instance.elem[i].clone();
    ref_const(val, stack)
}

pub fn table_set(
    stack: &mut Stack,
    store: &mut Store,
    &TableIdx(U32Type(idx)): &TableIdx,
) -> RResult<()> {
    let table_addr = get_table_addr(stack, idx)?;
    let table_instance = store.tables.get_mut(table_addr).ok_or(Trap)?;
    let new_ref = stack.pop_ref().ok_or(Trap)?;

    let i = stack.pop_i32().ok_or(Trap)? as usize;
    if i >= table_instance.elem.len() {
        return Err(Trap);
    }

    table_instance.elem[i] = new_ref;

    Ok(())
}

pub fn table_size(
    stack: &mut Stack,
    store: &mut Store,
    &TableIdx(U32Type(idx)): &TableIdx,
) -> RResult<()> {
    let table_addr = get_table_addr(stack, idx)?;
    let table_instance = store.tables.get(table_addr).ok_or(Trap)?;
    let size = table_instance.elem.len();

    i32_const(&(size as u32), stack)
}

pub fn table_grow(
    stack: &mut Stack,
    store: &mut Store,
    &TableIdx(U32Type(idx)): &TableIdx,
) -> RResult<()> {
    let table_addr = get_table_addr(stack, idx)?;
    let table_instance = store.tables.get_mut(table_addr).ok_or(Trap)?;
    let size = table_instance.elem.len();

    let n = stack.pop_i32().ok_or(Trap)?;
    let ref_val = stack.pop_ref().ok_or(Trap)?;
    let err = -1i32 as u32;

    let to_stack = match table_instance.grow_n(n, ref_val) {
        Ok(_) => size as u32,
        Err(_) => err,
    };

    i32_const(&to_stack, stack)
}

pub fn table_fill(stack: &mut Stack, store: &mut Store, idx: &TableIdx) -> RResult<()> {
    let table_addr = get_table_addr(stack, idx.0 .0)?;
    let table_instance = store.tables.get_mut(table_addr).ok_or(Trap)?;

    let n = stack.pop_i32().ok_or(Trap)?;
    let ref_val = stack.pop_ref().ok_or(Trap)?;
    let i = stack.pop_i32().ok_or(Trap)?;

    if (i + n) as usize > table_instance.elem.len() {
        return Err(Trap);
    }

    if n == 0 {
        return Ok(());
    }

    i32_const(&i, stack)?;
    ref_const(ref_val.clone(), stack)?;
    table_set(stack, store, idx)?;
    i32_const(&(i + 1), stack)?;
    ref_const(ref_val, stack)?;
    i32_const(&(n - 1), stack)?;

    table_fill(stack, store, idx)
}

pub fn table_copy(
    stack: &mut Stack,
    store: &mut Store,
    &(TableIdx(U32Type(x)), TableIdx(U32Type(y))): &(TableIdx, TableIdx),
) -> RResult<()> {
    let table_addr_x = get_table_addr(stack, x)?;
    let table_x_len = store.tables.get(table_addr_x).ok_or(Trap)?.elem.len();
    let table_addr_y = get_table_addr(stack, y)?;
    let table_y_len = store.tables.get(table_addr_y).ok_or(Trap)?.elem.len();

    let n = stack.pop_i32().ok_or(Trap)?;
    let s = stack.pop_i32().ok_or(Trap)?;
    let d = stack.pop_i32().ok_or(Trap)?;

    if (s + n) as usize > table_y_len || (d + n) as usize > table_x_len {
        return Err(Trap);
    }

    if n == 0 {
        return Ok(());
    }

    if d <= s {
        stack.push_entry(StackEntry::Value(Val::I32(d)));
        stack.push_entry(StackEntry::Value(Val::I32(s)));
        table_get(stack, store, &to_table_idx(y))?;
        table_set(stack, store, &to_table_idx(x))?;
        i32_const(&d.checked_add(1).ok_or(Trap)?, stack)?;
        i32_const(&s.checked_add(1).ok_or(Trap)?, stack)?;
    } else {
        i32_const(
            &d.checked_add(n)
                .and_then(|k| k.checked_add_signed(-1))
                .ok_or(Trap)?,
            stack,
        )?;
        i32_const(
            &s.checked_add(n)
                .and_then(|k| k.checked_add_signed(-1))
                .ok_or(Trap)?,
            stack,
        )?;
        table_get(stack, store, &to_table_idx(y))?;
        table_set(stack, store, &to_table_idx(x))?;
        i32_const(&d, stack)?;
        i32_const(&s, stack)?;
    }

    i32_const(&(n - 1), stack)?;

    table_copy(stack, store, &(to_table_idx(x), to_table_idx(y)))
}

pub fn table_init(
    stack: &mut Stack,
    store: &mut Store,
    &(TableIdx(U32Type(x)), ElemIdx(U32Type(y))): &(TableIdx, ElemIdx),
) -> RResult<()> {
    let table_addr_x = get_table_addr(stack, x)?;
    let table_x_len = store.tables.get(table_addr_x).ok_or(Trap)?.elem.len();
    let elem_addr_y = get_elem_addr(stack, y)?;
    let elem_y_len = store
        .elems
        .get(elem_addr_y)
        .cloned()
        .ok_or(Trap)?
        .ok_or(Trap)?
        .elem
        .len();

    let n = stack.pop_i32().ok_or(Trap)?;
    let s = stack.pop_i32().ok_or(Trap)?;
    let d = stack.pop_i32().ok_or(Trap)?;

    if s.checked_add(n).ok_or(Trap)? as usize > elem_y_len
        || d.checked_add(n).ok_or(Trap)? as usize > table_x_len
    {
        return Err(Trap);
    }

    if n == 0 {
        return Ok(());
    }

    // FIXME: should be a part of Store logic
    let val = store
        .elems
        .get(elem_addr_y)
        .cloned()
        .ok_or(Trap)?
        .ok_or(Trap)?
        .elem
        .get(s as usize)
        .cloned()
        .ok_or(Trap)?;

    i32_const(&d, stack)?;
    ref_const(val.clone(), stack)?;
    table_set(stack, store, &to_table_idx(x))?;
    i32_const(&d.checked_add(1).ok_or(Trap)?, stack)?;
    i32_const(&s.checked_add(1).ok_or(Trap)?, stack)?;
    i32_const(&(n - 1), stack)?;

    table_init(stack, store, &(to_table_idx(x), to_elem_idx(y)))
}

pub fn elem_drop(
    stack: &mut Stack,
    store: &mut Store,
    &ElemIdx(U32Type(x)): &ElemIdx,
) -> RResult<()> {
    let elem_addr = get_elem_addr(stack, x)?;
    store.drop_elem(elem_addr)
}

fn get_table_addr(stack: &mut Stack, idx: u32) -> RResult<TableAddr> {
    stack
        .current_frame()
        .ok_or(Trap)?
        .module
        .borrow()
        .tableaddrs
        .get(idx as usize)
        .ok_or(Trap)
        .cloned()
}

fn get_elem_addr(stack: &mut Stack, idx: u32) -> RResult<ElemAddr> {
    stack
        .current_frame()
        .ok_or(Trap)?
        .module
        .borrow()
        .elemaddrs
        .get(idx as usize)
        .ok_or(Trap)
        .cloned()
}

fn to_table_idx<T>(v: T) -> TableIdx
where
    T: Into<u32>,
{
    TableIdx(U32Type(v.into()))
}

fn to_elem_idx<T>(v: T) -> ElemIdx
where
    T: Into<u32>,
{
    ElemIdx(U32Type(v.into()))
}
