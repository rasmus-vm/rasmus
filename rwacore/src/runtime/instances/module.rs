//! Module that contains runtime WebAssembly module instance
//! as it is defined in the Core specification.

use alloc::vec::Vec;

use crate::types::FuncType;

use super::ExportInst;

/// WebAssembly Module instance used in runtime.
pub struct ModuleInst {
    types: Vec<FuncType>,
    funcaddrs: Vec<usize>,
    tableaddrs: Vec<usize>,
    memaddrs: Vec<usize>,
    globaladdrs: Vec<usize>,
    elemaddrs: Vec<usize>,
    dataaddrs: Vec<usize>,
    exports: Vec<ExportInst>
}
