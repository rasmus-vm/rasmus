use alloc::string::String;

use crate::runtime::values::ExternValue;

/// Runtime representation of an export.
#[derive(Debug)]
pub struct ExportInst {
    name: String,
    value: ExternValue
}
