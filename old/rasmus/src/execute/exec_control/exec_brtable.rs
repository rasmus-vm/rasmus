use crate::entities::{module::InstructionType, types::LabelIdx};

use crate::execute::executor::ExitType;
use crate::{
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::exec_br;

pub fn exec_brtable(
    stack: &mut Stack,
    store: &mut Store,
    brtable_arg: &(Vec<LabelIdx>, LabelIdx),
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<ExitType>
        + Copy,
) -> RResult<ExitType> {
    let i = stack.pop_i32().ok_or(Trap)?;

    let idx = brtable_arg.0.get(i as usize).unwrap_or(&brtable_arg.1);

    exec_br(stack, store, idx, execute_instruction_fn)
}
