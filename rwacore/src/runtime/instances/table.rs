//! WebAssembly Table instance

use alloc::vec::Vec;

use crate::{runtime::values::Value, types::TableType};

/// WebAssembly Table instance
pub struct TableInst {
    t_type: TableType,
    // TODO: should be validated that the elem
    // is one of following `Value::RefNull`, `Value::Ref`,
    // `Value::RefExtern`.
    elem: Vec<Value>
}

