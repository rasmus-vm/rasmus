use std::{cell::RefCell, rc::Rc};

use crate::entities::module::InstructionType;

use crate::execute::executor::ExitType;
use crate::{
    instances::{frame::Frame, label::LabelInst, stack::Stack, store::Store, value::Val},
    result::{RResult, Trap},
};

pub fn pop_values_original_order(stack: &mut Stack, m: usize) -> RResult<Vec<Val>> {
    let mut values: Vec<Val> = vec![];

    for _ in 0..m {
        values.push(stack.pop_value().ok_or(Trap)?);
    }

    values.reverse();
    Ok(values)
}

pub fn invoke(
    stack: &mut Stack,
    store: &mut Store,
    function_addr: usize,
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<ExitType>
        + Copy,
) -> RResult<ExitType> {
    let function = store.funcs.get(function_addr).cloned().ok_or(Trap)?;
    let func_type = function.get_type();
    let arity = func_type.results.len();

    let values = pop_values_original_order(stack, func_type.parameters.len())?;
    let activation_frame = Frame {
        arity: Some(arity),
        module: function.get_module(),
        locals: Rc::new(RefCell::new(values)),
    };

    stack.push_frame(activation_frame);

    let label = LabelInst {
        arity,
        instructions: Rc::new(vec![]),
    };
    stack.push_label(label);

    function.invoke(stack, store, execute_instruction_fn)
}
