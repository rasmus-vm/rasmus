//! Module that contains runtime WebAssembly module instance
//! as it is defined in the Core specification.

use alloc::vec::Vec;

use crate::types::FuncType;

use super::ExportInst;

/// WebAssembly Module instance used in runtime.
#[derive(Debug)]
pub struct ModuleInst {
    types: Vec<FuncType>,
    funcaddrs: Vec<usize>,
    tableaddrs: Vec<usize>,
    memaddrs: Vec<usize>,
    globaladdrs: Vec<usize>,
    elemaddrs: Vec<usize>,
    dataaddrs: Vec<usize>,
    exports: Vec<ExportInst>,
}

impl ModuleInst {
    /// Returns memory address by its index.
    pub fn get_memaddr(&self, idx: usize) -> Option<usize> {
        self.memaddrs.get(idx).copied()
    }
}

pub struct ModuleInstBuilder {
    inner: ModuleInst,
}

impl ModuleInstBuilder {
    pub fn new() -> Self {
        ModuleInstBuilder {
            inner: ModuleInst {
                types: Vec::new(),
                funcaddrs: Vec::new(),
                tableaddrs: Vec::new(),
                memaddrs: Vec::new(),
                globaladdrs: Vec::new(),
                elemaddrs: Vec::new(),
                dataaddrs: Vec::new(),
                exports: Vec::new(),
            },
        }
    }

    pub fn with_memory(mut self, mem_addrs: Vec<usize>) -> Self {
        self.inner.memaddrs = mem_addrs;
        self
    } 

    pub fn build(self) -> ModuleInst {
        self.inner
    }
}
