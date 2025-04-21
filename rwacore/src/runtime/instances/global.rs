//! WebAssembly global value instance.

use crate::{runtime::values::Value, types::GlobalType};

/// WebAssembly global value instance.
pub struct GlobalInst {
    g_type: GlobalType,
    val: Value
}
