use alloc::vec::Vec;

use super::{instances::ModuleInst, values::Value};

#[derive(Clone, Debug)]
pub struct Frame<'a> {
    pub arity: usize,
    pub locals: Vec<Value>,
    pub module: &'a ModuleInst,
}
