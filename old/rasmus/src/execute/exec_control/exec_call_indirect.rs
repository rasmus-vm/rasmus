use crate::{
    entities::{
        module::InstructionType,
        types::{TableIdx, TypeIdx, U32Type},
    },
    execute::executor::ExitType,
};

use crate::{
    address::TableAddr,
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::invoke;

pub fn exec_call_indirect(
    stack: &mut Stack,
    store: &mut Store,
    &(TableIdx(U32Type(table_idx)), TypeIdx(U32Type(type_idx))): &(TableIdx, TypeIdx),
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<ExitType>
        + Copy,
) -> RResult<ExitType> {
    let table_addr = get_table_addr(stack, table_idx)?;
    let table_inst = store.tables.get(table_addr).ok_or(Trap)?;
    let expected_type = stack
        .current_frame()
        .ok_or(Trap)?
        .module
        .borrow()
        .types
        .get(type_idx as usize)
        .cloned()
        .ok_or(Trap)?;

    let i = stack.pop_i32().ok_or(Trap)?;

    if i as usize >= table_inst.elem.len() {
        return Err(Trap);
    }

    let reference = table_inst.elem.get(i as usize).ok_or(Trap)?;

    if reference.is_null() {
        return Err(Trap);
    }

    let a = reference.as_func()?;
    let type_is_as_expected = store
        .funcs
        .get(a)
        .ok_or(Trap)
        .map(|f| f.get_type() == &expected_type)?;

    if !type_is_as_expected {
        return Err(Trap);
    }

    invoke(stack, store, a, execute_instruction_fn)
}

fn get_table_addr(stack: &mut Stack, idx: u32) -> RResult<TableAddr> {
    stack
        .current_frame()
        .ok_or(Trap)?
        .module
        .borrow()
        .tableaddrs
        .get(idx as usize)
        .cloned()
        .ok_or(Trap)
}
