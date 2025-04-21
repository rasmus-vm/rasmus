use crate::entities::types::{GlobalIdx, LocalIdx, U32Type};

use crate::{
    instances::{
        stack::{Stack, StackEntry},
        store::Store,
    },
    result::{RResult, Trap},
};

pub fn local_get(stack: &mut Stack, &LocalIdx(U32Type(idx)): &LocalIdx) -> RResult<()> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    let val = current_frame
        .locals
        .borrow()
        .get(idx as usize)
        .ok_or(Trap)?
        .clone();

    stack.push_entry(StackEntry::Value(val));

    Ok(())
}

pub fn local_set(stack: &mut Stack, &LocalIdx(U32Type(idx)): &LocalIdx) -> RResult<()> {
    if stack
        .current_frame()
        .ok_or(Trap)?
        .locals
        .borrow()
        .get(idx as usize)
        .is_none()
    {
        return Err(Trap);
    }
    let val = stack.pop_value().ok_or(Trap)?;
    stack.current_frame().ok_or(Trap)?.locals.borrow_mut()[idx as usize] = val;

    Ok(())
}

pub fn local_tee(stack: &mut Stack, idx: &LocalIdx) -> RResult<()> {
    let val = stack.pop_value().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(val.clone()));
    stack.push_entry(StackEntry::Value(val));

    local_set(stack, idx)
}

pub fn global_get(
    stack: &mut Stack,
    store: &Store,
    &GlobalIdx(U32Type(idx)): &GlobalIdx,
) -> RResult<()> {
    let addr = stack
        .current_frame()
        .ok_or(Trap)?
        .module
        .borrow()
        .globaladdrs
        .get(idx as usize)
        .ok_or(Trap)?
        .clone();

    let global_val = store.globals.get(addr).ok_or(Trap)?.value.clone();
    stack.push_entry(StackEntry::Value(global_val));

    Ok(())
}

pub fn global_set(
    stack: &mut Stack,
    store: &mut Store,
    &GlobalIdx(U32Type(idx)): &GlobalIdx,
) -> RResult<()> {
    let addr = stack
        .current_frame()
        .ok_or(Trap)?
        .module
        .borrow()
        .globaladdrs
        .get(idx as usize)
        .ok_or(Trap)?
        .clone();

    if store.globals.get(addr).is_none() {
        return Err(Trap);
    }

    let val = stack.pop_value().ok_or(Trap)?;
    store.globals.get_mut(addr).ok_or(Trap)?.value = val;

    Ok(())
}
