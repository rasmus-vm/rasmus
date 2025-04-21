use crate::entities::module::{BlockInstructionType, IfElseInstructionType, InstructionType};

use crate::execute::executor::ExitType;
use crate::{
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

pub fn exec_ifelse(
    stack: &mut Stack,
    store: &mut Store,
    &IfElseInstructionType {
        ref blocktype,
        ref if_instructions,
        ref else_instructions,
    }: &IfElseInstructionType,
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<ExitType>
        + Copy,
) -> RResult<ExitType> {
    let c = stack.pop_i32().ok_or(Trap)?;

    if c != 0 {
        execute_instruction_fn(
            &InstructionType::Block(BlockInstructionType {
                blocktype: blocktype.clone(),
                instructions: if_instructions.clone(),
            }),
            stack,
            store,
        )
    } else {
        execute_instruction_fn(
            &InstructionType::Block(BlockInstructionType {
                blocktype: blocktype.clone(),
                instructions: else_instructions.clone(),
            }),
            stack,
            store,
        )
    }
}
