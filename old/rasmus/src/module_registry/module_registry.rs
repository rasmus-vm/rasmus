use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read;
use std::rc::Rc;

use crate::binary::module_parser::ModuleParser;
use crate::binary::parse_trait::ParseBin;
use crate::entities::module::Module;
use crate::instances::export::ExternVal;
use crate::instances::module::{ExternalDependency, ModuleInst};
use crate::instances::stack::Stack;
use crate::instances::store::Store;
use crate::module_registry::ModuleRegistryError;
use crate::result::Trap;

type ModuleName = String;

type ModulePath = String;

pub struct ModuleRegistry<'a> {
    modules: HashMap<ModuleName, Module>,
    instances: RefCell<HashMap<ModuleName, Rc<RefCell<ModuleInst>>>>,
    store: &'a mut Store,
    stack: &'a mut Stack,
}

impl<'a> ModuleRegistry<'a> {
    pub fn new(store: &'a mut Store, stack: &'a mut Stack) -> Self {
        ModuleRegistry {
            modules: HashMap::new(),
            instances: RefCell::new(HashMap::new()),
            store,
            stack,
        }
    }

    pub fn register_module(&mut self, name: ModuleName, path: ModulePath) -> Result<(), Trap> {
        if self.module_exists(&name) {
            // TODO: ModuleRegistryError::ModuleAlreadyRegistered { name }
            return Err(Trap);
        }

        self.modules.insert(name, parse_module(&path)?);

        Ok(())
    }

    pub fn get_instance(&mut self, name: &ModuleName) -> Result<Rc<RefCell<ModuleInst>>, Trap> {
        if self.instances.borrow().get(name).is_none() {
            self.instantiate_module(name)?;
        }

        Ok(self.instances.borrow().get(name).unwrap().clone())
    }

    pub fn get_module(&self, name: &ModuleName) -> Option<&Module> {
        self.modules.get(name)
    }

    fn module_exists(&self, name: &String) -> bool {
        self.modules.get(name).is_some()
    }

    fn instantiate_module(&mut self, name: &ModuleName) -> Result<(), Trap> {
        if self.instances.borrow().get(name).is_some() {
            return Ok(());
        }

        let dependency_names: Vec<String> = self
            .modules
            .get(name)
            .ok_or_else(|| {
                Trap::from(ModuleRegistryError::ModuleNotRegistered { name: name.clone() })
            })?
            .imports
            .iter()
            .map(|import| import.module.0.clone())
            .collect();

        for dep_name in &dependency_names {
            // iteratively instantiate module until either:
            // 1. a module has no dependencies
            // 2. its dependencies have been already instantiated
            // TODO: detect circular dependencies
            self.instantiate_module(dep_name)?;
        }

        let module = self.modules.get(name).ok_or_else(|| {
            Trap::from(ModuleRegistryError::ModuleNotRegistered { name: name.clone() })
        })?;

        let resolved_imports: Vec<Option<ExternalDependency>> = module
            .imports
            .iter()
            .map(|import| {
                let module_name = &import.module.0;
                let val_name = &import.name;

                self.instances
                    .borrow()
                    .get(module_name)
                    .and_then(|module_ref| {
                        let module = module_ref.clone();
                        let exports = &module.borrow().exports;

                        exports
                            .iter()
                            .cloned()
                            .find(|export| &export.name == val_name)
                            .map(|export_inst| match export_inst.value {
                                ExternVal::Func(func_addr) => ExternalDependency::Func {
                                    func_addr,
                                    func_type: self.store.funcs[func_addr].get_type().clone(),
                                },
                                ExternVal::Table(table_addr) => ExternalDependency::Table {
                                    table_addr,
                                    table_type: self.store.tables[table_addr].table_type.clone(),
                                },
                                ExternVal::Mem(mem_addr) => ExternalDependency::Mem {
                                    mem_addr,
                                    mem_type: self.store.mems[mem_addr].mem_type.clone(),
                                },
                                ExternVal::Global(global_addr) => ExternalDependency::Global {
                                    global_addr,
                                    global_type: self.store.globals[global_addr]
                                        .global_type
                                        .clone(),
                                },
                            })
                    })
            })
            .collect();

        let mut externals: Vec<ExternalDependency> = Vec::with_capacity(resolved_imports.len());

        for resolved_import in resolved_imports {
            match resolved_import {
                Some(external) => {
                    externals.push(external);
                }
                None => {
                    // TODO: iterate through extern_vals and throw error if any is none
                    // add import module and name to error
                }
            }
        }

        let inst = ModuleInst::instantiate(self.store, self.stack, module, externals)?;
        self.instances.borrow_mut().insert(name.clone(), inst);

        Ok(())
    }
}

fn parse_module(path: &String) -> Result<Module, Trap> {
    let file_content = read(path)
        .map_err(|_| Trap::from(ModuleRegistryError::UnableToReadModule { path: path.clone() }))?;

    ModuleParser::parse(&file_content)
        .map_err(Into::<Trap>::into)
        .map(|res| res.1)
}
