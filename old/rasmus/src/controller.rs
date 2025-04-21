use std::{cell::RefCell, rc::Rc};

use crate::{
    entities::{
        instructions::InstructionType,
        module::{ExportDescription, Module},
        types::NameType,
    },
    execute::{execute_instruction, pop_values_original_order},
    instances::{frame::Frame, module::ModuleInst, stack::Stack, store::Store, value::Val},
    result::{RResult, Trap},
};

pub fn run_func(
    module_inst: Rc<RefCell<ModuleInst>>,
    module: &Module,
    func_name: &str,
    mut values: Vec<Val>,
    stack: &mut Stack,
    store: &mut Store,
) -> RResult<Vec<Val>> {
    let export = module
        .exports
        .iter()
        .find_map(|current_export| {
            let NameType(ref current_name) = current_export.name;
            if current_name == func_name {
                Some(&current_export.desc)
            } else {
                None
            }
        })
        .ok_or(Trap)?;

    let func_idx = match &export {
        ExportDescription::Func(ref func_idx) => func_idx.clone(),
        _ => {
            return Err(Trap);
        }
    };

    let func_addr = module_inst
        .borrow()
        .funcaddrs
        .get(func_idx.0 .0 as usize)
        .ok_or(Trap)?
        .clone();

    let return_arity = store
        .funcs
        .get(func_addr)
        .ok_or(Trap)?
        .get_type()
        .results
        .len();

    values.reverse();

    stack.push_frame(Frame {
        module: module_inst.clone(),
        locals: Rc::new(RefCell::new(vec![])),
        arity: None,
    });

    for val in values {
        stack.push_value(val);
    }

    execute_instruction(&InstructionType::Call(func_idx), stack, store)?;

    let values = pop_values_original_order(stack, return_arity)?;

    // stack.pop_frame().ok_or(Trap);

    Ok(values)
}
