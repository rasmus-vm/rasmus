mod control;
pub mod memory;
pub mod numeric;
mod parametric;
mod reference;
mod table;
mod variable;

#[cfg(test)]
pub mod test_utils;

pub use control::*;
use memory::MemArg;
pub use parametric::*;
pub use reference::*;
pub use table::*;
pub use variable::*;

use crate::runtime::{ExecRes, Frame, stack::Stack, store::Store};

/// Trait to be implmented by instructions.
pub trait Exuctable {
    fn exec(&self, store: &mut Store, stack: &mut Stack)-> ExecRes;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    // Memory Instructions
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,
    MemoryInit,
    DataDrop,
    MemoryCopy,
    MemoryFill,
}
