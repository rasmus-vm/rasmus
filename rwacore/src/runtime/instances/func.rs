//! Function instance implementation.

use crate::{
    module::Func,
    runtime::{
        instances::exec_instr,
        stack::Stack,
        store::Store,
        trap::{RResult, Trap},
    },
    types::FuncType,
};
use alloc::boxed::Box;

use super::ModuleInst;

/// Function instance as it is defined in WebAssembly
/// Core specification.
pub enum FuncInst {
    /// "Local" function, declared in one of .wasm modules.
    Local(LocalFunc),
    /// Host function, provided by the embedder.
    Host(HostFunc),
}

impl Callable for FuncInst {
    fn call(
        &self,
        store: &mut Store,
        stack: &mut Stack,
        maybe_module: Option<&ModuleInst>,
    ) -> RResult<()> {
        match self {
            FuncInst::Local(local) => local.call(store, stack, maybe_module),
            FuncInst::Host(host) => host.call(store, stack, None),
        }
    }
}

/// "Local" function, declared in one of .wasm modules.
pub struct LocalFunc {
    /// Function type (signature).
    pub f_type: FuncType,
    module: &'static ModuleInst,
    code: Func,
}

impl Callable for LocalFunc {
    fn call(
        &self,
        store: &mut Store,
        stack: &mut Stack,
        maybe_module: Option<&ModuleInst>,
    ) -> RResult<()> {
        if maybe_module.is_none() {
            return Err(Trap::MissingExecutionModule);
        }
        // execute code instructions list
        for instr in self.code.instructions() {
            exec_instr(&instr, store, stack, maybe_module.unwrap())?;
        }
        todo!()
    }
}

/// Host function, provided by the embedder.
pub struct HostFunc {
    /// Function type (signature).
    pub f_type: FuncType,
    host_code: Box<dyn Callable>,
}

impl Callable for HostFunc {
    fn call(
        &self,
        store: &mut Store,
        stack: &mut Stack,
        maybe_module: Option<&ModuleInst>,
    ) -> RResult<()> {
        self.host_code.call(store, stack, maybe_module)
    }
}

pub trait Callable {
    fn call(
        &self,
        store: &mut Store,
        stack: &mut Stack,
        maybe_module: Option<&ModuleInst>,
    ) -> RResult<()>;
}

#[cfg(test)]
mod test {
    extern crate std;

    use alloc::boxed::Box;
    use alloc::vec;

    use super::Callable;
    use super::FuncInst;
    use crate::runtime::instances::ModuleInst;
    use crate::runtime::stack::Stack;
    use crate::runtime::stack::StackEntity;
    use crate::runtime::store::Store;
    use crate::runtime::trap::RResult;
    use crate::runtime::values::Value;
    use crate::types::FuncType;
    use crate::types::ResType;
    use std::println;

    struct HostLogU32 {}

    impl Callable for HostLogU32 {
        fn call(
            &self,
            _store: &mut Store,
            stack: &mut Stack,
            _maybe_module: Option<&ModuleInst>,
        ) -> RResult<()> {
            println!("even logs");
            stack
                .push(StackEntity::Val(Value::U32(256)))
                .expect("should push to stack (test)");
            Ok(())
        }
    }

    #[test]
    fn test_host_func() {
        let host_f = FuncInst::Host(super::HostFunc {
            f_type: FuncType {
                args: ResType(vec![]),
                ret: ResType(vec![]),
            },
            host_code: Box::new(HostLogU32 {}),
        });

        let mut stack = Stack::new(4).expect("should create stack (test)");
        let mut store = Store::empty();

        host_f.call(&mut store, &mut stack, None).unwrap();

        assert_eq!(stack.dump(), vec![StackEntity::Val(Value::U32(256))]);
    }
}
