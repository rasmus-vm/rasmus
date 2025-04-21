use crate::{
    controller::run_func,
    instances::{stack::Stack, store::Store, value::Val},
    module_registry::ModuleRegistry,
};

mod address;
mod binary;
mod cli;
mod controller;
mod entities;
mod execute;
mod instances;
mod module_registry;
mod result;
pub mod sign;
mod validation;

#[cfg(test)]
mod execute_test;
#[cfg(test)]
mod test_utils;

const MAIN_MODULE: &'static str = "$MAIN";

fn main() {
    let input = cli::UserInput::parse_args();

    let mut store = Store::new();
    let mut stack = Stack::new();

    let mut module_registry = ModuleRegistry::new(&mut store, &mut stack);
    module_registry
        .register_module(MAIN_MODULE.into(), input.source_file_path.into())
        .expect("Unable register");

    module_registry
        .register_module(
            "factorial-lib".into(),
            "./rasmus/tests/files/factorial-lib.wasm".into(),
        )
        .expect("Unable register");

    let module_inst = module_registry
        .get_instance(&MAIN_MODULE.into())
        .expect("Unable resolve")
        .clone();
    let module = module_registry
        .get_module(&MAIN_MODULE.into())
        .expect("Unable to get module")
        .clone();

    let result = run_func(
        module_inst.clone(),
        &module,
        "factorial",
        vec![Val::I32(10)],
        &mut stack,
        &mut store,
    )
    .expect("finish without errors");

    println!("result >>> {:?}", result);
}
