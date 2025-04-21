use crate::{
    entities::{
        module::InstructionType,
        types::{FuncIdx, U32Type},
    },
    execute::executor::ExitType,
};

use crate::{
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::utils::invoke;

pub fn exec_call(
    stack: &mut Stack,
    store: &mut Store,
    &FuncIdx(U32Type(func_idx)): &FuncIdx,
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<ExitType>
        + Copy,
) -> RResult<ExitType> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    let function_addr = current_frame
        .module
        .borrow()
        .funcaddrs
        .get(func_idx as usize)
        .cloned()
        .ok_or(Trap)?;

    invoke(stack, store, function_addr, execute_instruction_fn)
}
