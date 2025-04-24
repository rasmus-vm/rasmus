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
pub use parametric::*;
pub use reference::*;
pub use table::*;
pub use variable::*;

use crate::runtime::{ExecRes, Frame, stack::Stack, store::Store};

/// Trait to be implmented by instructions.
pub trait Exuctable {
    fn exec(&self, store: &mut Store, stack: &mut Stack, frame: &mut Frame) -> ExecRes;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instr {}
