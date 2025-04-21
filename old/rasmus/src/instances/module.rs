use std::cell::RefCell;
use std::rc::Rc;

use crate::address::*;
use crate::entities::module::*;
use crate::entities::types::*;
use crate::execute::{execute_expression, execute_instruction};
use crate::instances::{frame::Frame, stack::Stack, stack::StackEntry, store::Store};
use crate::result::{RResult, Trap};

use super::export::ExportInst;
use super::value::Val;

#[derive(Debug, Default)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub funcaddrs: Vec<FuncAddr>,
    pub tableaddrs: Vec<TableAddr>,
    pub memaddrs: Vec<MemAddr>,
    pub globaladdrs: Vec<GlobalAddr>,
    pub elemaddrs: Vec<ElemAddr>,
    pub dataaddrs: Vec<DataAddr>,
    pub exports: Vec<ExportInst>,
    pub start: Option<StartType>,
}

#[derive(Debug)]
pub enum ExternalDependency {
    Func {
        func_addr: FuncAddr,
        func_type: FuncType,
    },
    Table {
        table_addr: TableAddr,
        table_type: TableType,
    },
    Mem {
        mem_addr: MemAddr,
        mem_type: MemType,
    },
    Global {
        global_addr: GlobalAddr,
        global_type: GlobalType,
    },
}

// Takes module declaration type,
// creates following inputs for module allocation
// - externals
// - vals
// - refs
// invokes module allocation process in the store
impl ModuleInst {
    pub fn instantiate(
        store: &mut Store,
        stack: &mut Stack,
        module: &Module,
        // externals: Vec<ExportInst>,
        externals: Vec<ExternalDependency>,
    ) -> RResult<Rc<RefCell<Self>>> {
        if !module.is_valid(&externals) {
            return Err(Trap);
        }

        let aux_module_raw = RefCell::new(ModuleInst {
            types: module.types.clone(),
            globaladdrs: externals
                .iter()
                .filter_map(|external| match external {
                    ExternalDependency::Global { global_addr, .. } => Some(*global_addr),
                    _ => None,
                })
                .collect(),
            ..Default::default()
        });

        let aux_module = Rc::new(aux_module_raw);

        let funcaddrs_aux: Vec<FuncAddr> = module
            .get_funcs()
            .ok_or(Trap)?
            .iter()
            .map(|func| store.allocate_local_func(func.clone(), aux_module.clone()))
            .collect();

        aux_module.borrow_mut().funcaddrs = funcaddrs_aux;

        stack.push_entry(StackEntry::Frame(Frame {
            module: aux_module.clone(),
            locals: Rc::new(RefCell::new(vec![])),
            arity: None,
        }));

        let mut vals = Vec::with_capacity(module.globals.len());
        for global in &module.globals {
            vals.push(execute_expression(&global.init, stack, store)?);
        }

        if !stack.last().map(|entry| entry.is_frame()).unwrap_or(false) {
            return Err(Trap);
        }

        let mut refs_refs = Vec::with_capacity(module.elems.len());
        for elem in &module.elems {
            let init = Self::resolve_element_segment_init(elem);

            let mut refs = Vec::with_capacity(init.len());

            for init_expr in init {
                let ref_instr = match execute_expression(&init_expr, stack, store)? {
                    Val::Ref(ref_instr) => ref_instr,
                    _ => {
                        // unreachable due to validation
                        return Err(Trap);
                    }
                };
                refs.push(ref_instr);
            }

            refs_refs.push(refs);
        }

        if stack.pop_frame().is_none() {
            return Err(Trap);
        }

        // let globals = externals
        //     .iter()
        //     .map(|export_inst| export_inst.value.clone())
        //     .collect();

        let module_inst_rc = store.allocate_module(&module, &externals, vals, refs_refs)?;

        stack.push_entry(StackEntry::Frame(Frame {
            module: module_inst_rc.clone(),
            locals: Rc::new(RefCell::new(vec![])),
            arity: None,
        }));

        Self::apply_elems(module, stack, store)?;
        Self::apply_active_datas(module, stack, store)?;
        Self::execute_module_start_fn(module, stack, store)?;

        if stack.pop_frame().is_none() {
            return Err(Trap);
        }

        return Ok(module_inst_rc);
    }

    fn resolve_element_segment_init<'a>(elem: &ElementSegmentType) -> Vec<ExpressionType> {
        match elem {
            ElementSegmentType::Active0Expr(active0_expr) => active0_expr.init.clone(),
            ElementSegmentType::Active0Functions(active0_funcs) => active0_funcs
                .init
                .iter()
                .map(|func_idx| {
                    ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                })
                .collect(),
            ElementSegmentType::ActiveRef(active0_ref) => active0_ref.init.clone(),
            ElementSegmentType::DeclarativeRef(declarative_ref) => declarative_ref.init.clone(),
            ElementSegmentType::ElemKindActiveFunctions(elemkind_active_funcs) => {
                elemkind_active_funcs
                    .init
                    .iter()
                    .map(|func_idx| {
                        ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                    })
                    .collect()
            }
            ElementSegmentType::ElemKindDeclarativeFunctions(elemkind_declarative_funcs) => {
                elemkind_declarative_funcs
                    .init
                    .iter()
                    .map(|func_idx| {
                        ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                    })
                    .collect()
            }
            ElementSegmentType::ElemKindPassiveFunctions(elemkind_passive_funcs) => {
                elemkind_passive_funcs
                    .init
                    .iter()
                    .map(|func_idx| {
                        ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                    })
                    .collect()
            }
            ElementSegmentType::PassiveRef(passive_ref) => passive_ref.init.clone(),
        }
    }

    fn apply_elems(module: &Module, stack: &mut Stack, store: &mut Store) -> RResult<()> {
        for (i, elem) in module.elems.iter().enumerate() {
            let init_len = elem.get_init().len() as u32;
            match elem {
                ElementSegmentType::Active0Expr(segment_type) => {
                    let offset_instructions = &segment_type.mode.offset;
                    let table_idx = TableIdx(U32Type(0));
                    Self::apply_active_element_segment(
                        init_len,
                        i as u32,
                        offset_instructions,
                        table_idx,
                        stack,
                        store,
                    )?;
                }
                ElementSegmentType::Active0Functions(segment_type) => {
                    let offset_instructions = &segment_type.mode.offset;
                    let table_idx = TableIdx(U32Type(0));
                    Self::apply_active_element_segment(
                        init_len,
                        i as u32,
                        offset_instructions,
                        table_idx,
                        stack,
                        store,
                    )?;
                }
                ElementSegmentType::ActiveRef(segment_type) => {
                    let offset_instructions = &segment_type.mode.offset;
                    let table_idx = segment_type.mode.table_idx.clone();
                    Self::apply_active_element_segment(
                        init_len,
                        i as u32,
                        offset_instructions,
                        table_idx,
                        stack,
                        store,
                    )?;
                }
                ElementSegmentType::ElemKindActiveFunctions(segment_type) => {
                    let offset_instructions = &segment_type.mode.offset;
                    let table_idx = segment_type.mode.table_idx.clone();
                    Self::apply_active_element_segment(
                        init_len,
                        i as u32,
                        offset_instructions,
                        table_idx,
                        stack,
                        store,
                    )?;
                }
                ElementSegmentType::DeclarativeRef(_)
                | ElementSegmentType::ElemKindDeclarativeFunctions(_) => {
                    Self::apply_declarative_element_segment(i as u32, stack, store)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn apply_active_element_segment(
        n: u32,
        i: u32,
        offset_instructions: &ExpressionType,
        table_idx: TableIdx,
        stack: &mut Stack,
        store: &mut Store,
    ) -> RResult<()> {
        execute_expression(offset_instructions, stack, store)?;
        execute_instruction(&InstructionType::I32Const(I32Type(0)), stack, store)?;
        execute_instruction(&InstructionType::I32Const(I32Type(n)), stack, store)?;
        execute_instruction(
            &InstructionType::TableInit((table_idx, ElemIdx(U32Type(i)))),
            stack,
            store,
        )?;
        execute_instruction(
            &InstructionType::ElemDrop(ElemIdx(U32Type(i))),
            stack,
            store,
        )?;

        Ok(())
    }

    fn apply_declarative_element_segment(
        i: u32,
        stack: &mut Stack,
        store: &mut Store,
    ) -> RResult<()> {
        execute_instruction(
            &InstructionType::ElemDrop(ElemIdx(U32Type(i))),
            stack,
            store,
        )?;

        Ok(())
    }

    fn apply_active_datas(module: &Module, stack: &mut Stack, store: &mut Store) -> RResult<()> {
        for (i, data) in module.datas.iter().enumerate() {
            match data {
                DataType::Active0(data_active) => {
                    let n = data_active.init.len() as u32;
                    execute_expression(&data_active.mode.offset, stack, store)?;
                    execute_instruction(&InstructionType::I32Const(I32Type(0)), stack, store)?;
                    execute_instruction(&InstructionType::I32Const(I32Type(n)), stack, store)?;
                    execute_instruction(
                        &InstructionType::MemoryInit(DataIdx(U32Type(i as u32))),
                        stack,
                        store,
                    )?;
                }
                DataType::Active(data_active) => {
                    if data_active.mode.memory.0 .0 != 0 {
                        return Err(Trap);
                    }
                    let n = data_active.init.len() as u32;
                    execute_expression(&data_active.mode.offset, stack, store)?;
                    execute_instruction(&InstructionType::I32Const(I32Type(0)), stack, store)?;
                    execute_instruction(&InstructionType::I32Const(I32Type(n)), stack, store)?;
                    execute_instruction(
                        &InstructionType::MemoryInit(DataIdx(U32Type(i as u32))),
                        stack,
                        store,
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn execute_module_start_fn(
        module: &Module,
        stack: &mut Stack,
        store: &mut Store,
    ) -> RResult<()> {
        if let Some(ref start_fn) = module.start {
            execute_instruction(&InstructionType::Call(start_fn.func.clone()), stack, store)?;
        }
        Ok(())
    }
}

// FIXME:
// #[cfg(test)]
// mod test {
//     use crate::entities::module::Module;

//     use crate::{
//         instances::{stack::Stack, store::Store},
//         module_registry::ModuleRegistry,
//     };

//     use super::ModuleInst;

//     #[test]
//     fn instantiate_empty() {
//         let registry = ModuleRegistry::new();
//         let empty_module = Module::default();
//         let mut store = Store::new();
//         let mut stack = Stack::new();

//         let instance = ModuleInst::instantiate(&mut store, &mut stack, &empty_module, &registry)
//             .expect("should instantiate empty instance");
//     }
// }
