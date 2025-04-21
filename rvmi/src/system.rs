//! System calls.

use crate::{memory::LinearMemory, user_error::RUserError};

/// Rasmus Virtual Machine Interface.
pub trait RVMI<'a> {
    fn malloc(size: usize) -> Result<impl LinearMemory, RUserError>;
}
