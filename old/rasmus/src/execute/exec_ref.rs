use crate::entities::types::RefType;
use crate::instances::ref_inst::RefInst;
use crate::instances::{stack::StackEntry, value::Val};

use crate::{
    instances::stack::Stack,
    result::{RResult, Trap},
};

pub fn ref_null(ref_type: &RefType, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::Ref(RefInst::Null(ref_type.clone()))));

    Ok(())
}

pub fn ref_func(stack: &mut Stack, func_idx: usize) -> RResult<()> {
    let funcaddr = match stack.current_frame() {
        Some(frame) => match frame.module.borrow().funcaddrs.get(func_idx) {
            Some(funcaddr) => funcaddr.clone(),
            None => {
                return Err(Trap);
            }
        },
        None => {
            return Err(Trap);
        }
    };

    stack.push_entry(StackEntry::Value(Val::Ref(RefInst::Func(funcaddr))));

    Ok(())
}

pub fn is_ref_null(stack: &mut Stack) -> RResult<()> {
    if let Some(Val::Ref(reference)) = stack.pop_value() {
        let is_null = match reference {
            RefInst::Null(_) => 1u32,
            _ => 0u32,
        };
        stack.push_entry(StackEntry::Value(Val::I32(is_null)));
    } else {
        return Err(Trap);
    }

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use std::{cell::RefCell, rc::Rc};

//     use crate::entities::{
//         module::{CodeType, ExpressionType, FuncCodeType, InstructionType, Module},
//         types::{FuncIdx, FuncType, RefType, TypeIdx, U32Type},
//     };

//     use crate::{
//         instances::{
//             frame::Frame,
//             module::ModuleInst,
//             ref_inst::RefInst,
//             stack::{Stack, StackEntry},
//             store::Store,
//             value::Val,
//         },
//         module_registry::ModuleRegistry,
//         test_utils::{test_instruction, test_instruction_with_stack_and_store},
//     };

//     #[test]
//     fn ref_null() {
//         test_instruction(
//             vec![],
//             InstructionType::RefNull(RefType::FuncRef),
//             Val::Ref(RefInst::Null(RefType::FuncRef)),
//         );
//     }

//     // TODO: unskip
//     // #[test]
//     // fn ref_func() {
//     //     let mut stack = Stack::new();
//     //     let mut store = Store::new();
//     //     let module_registry = Box::new(ModuleRegistry::new());
//     //     let module = Module {
//     //         types: vec![FuncType {
//     //             parameters: vec![],
//     //             results: vec![],
//     //         }],
//     //         imports: vec![],
//     //         funcs: vec![0u32]
//     //             .iter()
//     //             .map(|idx: &u32| TypeIdx(U32Type(*idx)))
//     //             .collect(),
//     //         tables: vec![],
//     //         mems: vec![],
//     //         globals: vec![],
//     //         exports: vec![],
//     //         start: None,
//     //         elems: vec![],
//     //         code: vec![CodeType {
//     //             size: U32Type(0),
//     //             code: FuncCodeType {
//     //                 locals: vec![],
//     //                 expression: ExpressionType {
//     //                     instructions: vec![],
//     //                 },
//     //             },
//     //         }],
//     //         datas: vec![],
//     //     };
//     //     let module_inst =
//     //         ModuleInst::instantiate(&mut store, &mut stack, &module, &module_registry)
//     //             .expect("unable to instantiate module");
//     //     stack.push_entry(StackEntry::Frame(Frame {
//     //         locals: Rc::new(RefCell::new(vec![])),
//     //         module: module_inst,
//     //         arity: None,
//     //     }));

//     //     test_instruction_with_stack_and_store(
//     //         &mut stack,
//     //         &mut store,
//     //         InstructionType::RefFunc(FuncIdx(U32Type(0))),
//     //         Val::Ref(RefInst::Func(1)),
//     //     );
//     // }
// }
