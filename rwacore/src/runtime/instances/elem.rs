//! WebAssembly element instance.

use alloc::vec::Vec;

use crate::{runtime::values::Value, types::RType};

/// WebAssembly element instance.
pub struct ElemInst {
    // TODO: validate it is always one of
    // ref types.
    e_type: RType,
    // TODO: validate it is always ref value.
    elem: Vec<Value>
}
