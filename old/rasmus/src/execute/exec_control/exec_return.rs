use crate::{
    execute::executor::ExitType,
    instances::stack::{Stack, StackEntry},
    result::{RResult, Trap},
};

use super::utils::pop_values_original_order;

pub fn exec_return(stack: &mut Stack) -> RResult<ExitType> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    let frame_arity = current_frame.arity.unwrap_or(0);
    let values = pop_values_original_order(stack, frame_arity)?;

    loop {
        if let StackEntry::Frame(_) = stack.pop().ok_or(Trap)? {
            break;
        }
    }

    for value in values {
        stack.push_value(value);
    }

    Ok(ExitType::Returned)
}
