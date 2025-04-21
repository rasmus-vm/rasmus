use crate::{binary::syntax_error::SyntaxError, module_registry::ModuleRegistryError};

pub type RResult<T> = Result<T, Trap>;

#[derive(Debug)]
pub struct Trap;

impl From<ModuleRegistryError> for Trap {
    fn from(_registry_error: ModuleRegistryError) -> Self {
        Trap
    }
}

impl From<SyntaxError> for Trap {
    fn from(_syntax_error: SyntaxError) -> Self {
        Trap
    }
}
