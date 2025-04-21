use std::cell::RefCell;
use std::rc::Rc;

use super::data::DataInst;
use super::elem::ElemInst;
use super::export::{ExportInst, ExternVal};
use super::func::{FuncInst, FuncInstLocal, HostCode, HostFunc};
use super::global::GlobalInst;
use super::memory::MemInst;
use super::module::{ExternalDependency, ModuleInst};
use super::ref_inst::RefInst;
use super::table::TableInst;
use super::value::Val;
use crate::entities::{
    module::{ExportDescription, Module},
    types::{Byte, Func, FuncType, GlobalType, MemType, RefType, TableType},
};
use crate::validation::types_validation::{is_memory_type_valid, is_table_type_valid};
use crate::{
    address::*,
    result::{RResult, Trap},
};

// TODO: consider using HashMap instead of Vec,
// encapsulate this detail to make store content not public
// and provide only neccessary methods to work with the Store content

// #[derive(Debug)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    // TODO: rewrite to HashMap<usize, ElemInst> ?
    pub elems: Vec<Option<ElemInst>>,
    // TODO: rewrite to HashMap<usize, DataInst> ?
    pub datas: Vec<Option<DataInst>>,
}

impl Store {
    /// Create empy store
    pub fn new() -> Store {
        Store {
            funcs: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
        }
    }

    pub fn drop_elem(&mut self, e: ElemAddr) -> RResult<()> {
        self.elems
            .get(e)
            // check if elem with index e exists
            .ok_or(Trap)?
            .as_ref()
            // check if elem with index e is not None
            .ok_or(Trap)?;

        self.elems[e] = None;

        Ok(())
    }

    pub fn drop_data(&mut self, e: DataAddr) -> RResult<()> {
        self.datas
            .get(e)
            // check if elem with index e exists
            .ok_or(Trap)?
            .as_ref()
            // check if elem with index e is not None
            .ok_or(Trap)?;

        self.datas[e] = None;

        Ok(())
    }

    pub fn allocate_local_func(
        &mut self,
        func: Func,
        module_inst: Rc<RefCell<ModuleInst>>,
    ) -> FuncAddr {
        let func_type = module_inst.borrow().types[func.func_type.0 .0 as usize].clone();
        let func_inst = FuncInst::FuncInst(FuncInstLocal {
            func_type,
            module: module_inst.clone(),
            code: func,
        });
        self.funcs.push(func_inst);

        self.funcs.len() - 1 as FuncAddr
    }

    #[allow(dead_code)]
    pub fn allocate_host_func(&mut self, func_type: FuncType, host_code: HostCode) -> FuncAddr {
        let func_inst = FuncInst::HostFunc(HostFunc {
            func_type,
            host_code,
        });

        self.funcs.push(func_inst);

        self.funcs.len() - 1 as FuncAddr
    }

    pub fn allocate_table(&mut self, table_type: TableType, elem: RefInst) -> TableAddr {
        let len = table_type.limits.min.0 as usize;
        let table_inst = TableInst {
            table_type,
            elem: vec![elem; len],
        };
        self.tables.push(table_inst);

        self.tables.len() - 1 as TableAddr
    }

    pub fn allocate_mem(&mut self, mem_type: MemType) -> MemAddr {
        let size = (mem_type.limits.min.0.clone() as usize) * 2usize.pow(16);
        let mem_inst = MemInst {
            mem_type,
            data: vec![0x00; size],
        };

        self.mems.push(mem_inst);

        self.mems.len() - 1 as MemAddr
    }

    pub fn allocate_global(&mut self, global_type: GlobalType, value: Val) -> GlobalAddr {
        let global_inst = GlobalInst { global_type, value };
        self.globals.push(global_inst);

        self.globals.len() - 1 as GlobalAddr
    }

    pub fn allocate_elem(&mut self, elem_type: RefType, elem: Vec<RefInst>) -> ElemAddr {
        let elem_inst = ElemInst { elem, elem_type };
        self.elems.push(Some(elem_inst));

        self.elems.len() - 1 as ElemAddr
    }

    pub fn allocate_data(&mut self, data: Vec<Byte>) -> DataAddr {
        let data_inst = DataInst { data };
        self.datas.push(Some(data_inst));

        self.datas.len() - 1 as DataAddr
    }

    // TODO: implement resolve_imports to get extern_vals (implement module registry)
    // TODO: implement resolve_globals to get globals values (according to the spec init of a global must be a single const instruction, take value from there)
    // TODO: implement resolve_elems to get refs vector of module's element segments
    pub fn allocate_module(
        &mut self,
        module: &Module,
        extern_vals: &Vec<ExternalDependency>,
        mut globals: Vec<Val>,
        mut refs: Vec<Vec<RefInst>>,
    ) -> RResult<Rc<RefCell<ModuleInst>>> {
        let mut module_inst = ModuleInst {
            types: module.types.clone(),
            tableaddrs: vec![],
            globaladdrs: vec![],
            funcaddrs: vec![],
            memaddrs: vec![],
            elemaddrs: vec![],
            dataaddrs: vec![],
            exports: vec![],
            start: module.start.clone(),
        };

        // table allocations
        for table_type in &module.tables {
            if !is_table_type_valid(&table_type) {
                return Err(Trap);
            }
        }
        for table_type in &module.tables {
            let elem = RefInst::Null(table_type.element_ref_type.clone());
            module_inst
                .tableaddrs
                .push(self.allocate_table(table_type.clone(), elem));
        }
        module_inst
            .tableaddrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternalDependency::Table { table_addr, .. } => Some(*table_addr),
                _ => None,
            }));

        // mem allocations
        for mem_type in &module.mems {
            if !is_memory_type_valid(&mem_type) {
                return Err(Trap);
            }
        }
        for mem_type in &module.mems {
            module_inst
                .memaddrs
                .push(self.allocate_mem(mem_type.clone()));
        }
        module_inst
            .memaddrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternalDependency::Mem { mem_addr, .. } => Some(*mem_addr),
                _ => None,
            }));

        // global allocations
        for global in &module.globals {
            globals.rotate_left(1);
            let val = globals.pop().ok_or(Trap)?;
            module_inst
                .globaladdrs
                .push(self.allocate_global(global.global_type.clone(), val));
        }
        module_inst
            .globaladdrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternalDependency::Global { global_addr, .. } => Some(*global_addr),
                _ => None,
            }));

        // elem allocation
        for element_segment in &module.elems {
            let elem_type = element_segment.get_type();
            refs.rotate_left(1);
            let elem = refs.pop().ok_or(Trap)?;
            self.allocate_elem(elem_type, elem);
        }

        // data allocation
        for data in &module.datas {
            self.allocate_data(data.clone_data());
        }

        let module_inst_rc = Rc::new(RefCell::new(module_inst));

        // func allocations
        let funcs = module.get_funcs().ok_or(Trap)?;
        for func in funcs {
            let func_addr = self.allocate_local_func(func, module_inst_rc.clone());
            module_inst_rc.borrow_mut().funcaddrs.push(func_addr);
        }
        module_inst_rc
            .borrow_mut()
            .funcaddrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternalDependency::Func { func_addr, .. } => Some(*func_addr),
                _ => None,
            }));

        // exports instantiation
        for export_declaration in &module.exports {
            let export_inst = ExportInst {
                name: export_declaration.name.clone(),
                value: match &export_declaration.desc {
                    ExportDescription::Func(func_idx) => {
                        let funcaddr = module_inst_rc
                            .borrow()
                            .funcaddrs
                            .get(func_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Func(funcaddr)
                    }
                    ExportDescription::Global(global_idx) => {
                        let globaladdr = module_inst_rc
                            .borrow()
                            .globaladdrs
                            .get(global_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Global(globaladdr)
                    }
                    ExportDescription::Mem(mem_idx) => {
                        let memaddr = module_inst_rc
                            .borrow()
                            .memaddrs
                            .get(mem_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Mem(memaddr)
                    }
                    ExportDescription::Table(table_idx) => {
                        let tableaddr = module_inst_rc
                            .borrow()
                            .tableaddrs
                            .get(table_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Table(tableaddr)
                    }
                },
            };
            module_inst_rc.borrow_mut().exports.push(export_inst);
        }

        Ok(module_inst_rc)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::entities::{
        module::ExpressionType,
        types::{Func, FuncType, TypeIdx, U32Type},
    };

    use crate::instances::{
        func::{FuncInst, FuncInstLocal, HostCode, HostFunc},
        module::ModuleInst,
    };

    use super::Store;

    #[test]
    fn allocate_local_func() {
        let mut store = Store::new();

        let module_inst = Rc::new(RefCell::new(ModuleInst {
            types: vec![FuncType {
                parameters: vec![],
                results: vec![],
            }],
            funcaddrs: vec![],
            ..Default::default()
        }));

        let func = Func {
            func_type: TypeIdx(U32Type(0)),
            locals: vec![],
            body: ExpressionType {
                instructions: vec![],
            },
        };

        let module_func_type = module_inst.borrow().types[func.func_type.0 .0 as usize].clone();
        let expected_code = func.clone();
        let func_addr = store.allocate_local_func(func, module_inst);

        assert_eq!(func_addr, 0);

        if let Some(FuncInst::FuncInst(FuncInstLocal {
            func_type, code, ..
        })) = store.funcs.get(0)
        {
            assert_eq!(
                func_type, &module_func_type,
                "allocated local function type should match"
            );
            assert_eq!(
                code, &expected_code,
                "allocated local function code should match"
            );
        } else {
            panic!("local function should be allocated");
        }
    }

    #[test]
    fn allocate_host_func() {
        let mut store = Store::new();

        let func_type = FuncType {
            parameters: vec![],
            results: vec![],
        };
        let expected_func_type = func_type.clone();

        let host_hode = HostCode;
        let expected_host_code = host_hode.clone();

        let func_addr = store.allocate_host_func(func_type, host_hode);

        assert_eq!(func_addr, 0);

        if let Some(FuncInst::HostFunc(HostFunc {
            func_type,
            host_code,
        })) = store.funcs.get(0)
        {
            assert_eq!(
                func_type, &expected_func_type,
                "allocated host function type should match"
            );
            assert_eq!(
                host_code, &expected_host_code,
                "allocated host function code should match"
            );
        } else {
            panic!("host function should be allocated");
        }
    }

    #[test]
    fn allocate_module() {}
}
