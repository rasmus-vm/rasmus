//! Execution result module.

use super::{values::Value, trap::RResult};

/// Runtime exectution result.
pub type ExecRes = RResult<Value>;
