use alloc::vec::Vec;

use super::instances::{DataInst, ElemInst, FuncInst, GlobalInst, MemoryInst, TableInst};

/// According to WASM Core spec the store represents all global state that can be manipulated
/// by WebAssembly programs.
///
/// It consists of the runtime
/// representation of all instances of:
///  - functions,
///  - tables,
///  - memories,
///  - globals,
///  - element segments,
///  - data segments.
///
/// Those are the instances that have been allocated during the life time of the abstract machine.
pub struct Store {
    funcs: Vec<FuncInst>,
    tables: Vec<TableInst>,
    mems: Vec<MemoryInst>,
    globals: Vec<GlobalInst>,
    elems: Vec<ElemInst>,
    datas: Vec<DataInst>,
}

impl Store {
    /// Creates empty store.
    #[inline]
    pub fn empty() -> Store {
        Store {
            funcs: Vec::new(),
            tables: Vec::new(),
            mems: Vec::new(),
            globals: Vec::new(),
            elems: Vec::new(),
            datas: Vec::new(),
        }
    }
    /// Returns `Option` with a reference to a function by its function space address `addr`.
    #[inline]
    pub fn get_func(&self, addr: usize) -> Option<&FuncInst> {
        self.funcs.get(addr)
    }

    /// Returns mutable reference to a `MemoryInst` by its memaddress.
    #[inline]
    pub fn get_memory_mut(&mut self, memaddr: usize) -> Option<&mut MemoryInst> {
        self.mems.get_mut(memaddr)
    }

    /// Pushes provided memory instance to the Store and returns memory address
    /// that should be used to access this memory instance.
    #[inline]
    pub fn add_memory(&mut self, memory_inst: MemoryInst) -> usize {
        self.mems.push(memory_inst);
        self.mems.len() - 1
    }
}
