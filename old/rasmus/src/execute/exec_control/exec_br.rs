use crate::{
    entities::{
        module::InstructionType,
        types::{LabelIdx, U32Type},
    },
    execute::executor::ExitType,
};

use crate::{
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::utils::pop_values_original_order;

pub fn exec_br(
    stack: &mut Stack,
    store: &mut Store,
    &LabelIdx(U32Type(label_idx)): &LabelIdx,
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<ExitType>
        + Copy,
) -> RResult<ExitType> {
    if stack.count_labels() < (label_idx + 1) as usize {
        return Err(Trap);
    }

    let label = stack.get_label(label_idx as usize).ok_or(Trap)?;
    let label_instructions = label.instructions.clone();
    let n = label.arity;

    let values = pop_values_original_order(stack, n)?;

    for _ in 0..(label_idx + 1) {
        loop {
            if stack.pop_value().is_none() {
                break;
            }
        }
        stack.pop_label().ok_or(Trap)?;
    }

    for value in values {
        stack.push_value(value);
    }

    for ref instruction in label_instructions.iter() {
        if execute_instruction_fn(instruction, stack, store)? == ExitType::Returned {
            return Ok(ExitType::Returned);
        }
    }

    Ok(ExitType::Completed)
}
