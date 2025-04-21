use crate::entities::types::ValType;

use crate::{
    instances::stack::{Stack, StackEntry},
    result::{RResult, Trap},
};

pub fn exec_drop(stack: &mut Stack) -> RResult<()> {
    stack.pop_value().ok_or(Trap).map(|_| ())
}

pub fn exec_select(stack: &mut Stack) -> RResult<()> {
    let c = stack.pop_i32().ok_or(Trap)?;
    let val_1 = stack.pop_value().ok_or(Trap)?;
    let val_2 = stack.pop_value().ok_or(Trap)?;

    if c != 0 {
        stack.push_entry(StackEntry::Value(val_1));
    } else {
        stack.push_entry(StackEntry::Value(val_2))
    }

    Ok(())
}

pub fn exec_select_vec(_stack: &mut Stack, _vector: &Vec<ValType>) -> RResult<()> {
    todo!()
}
