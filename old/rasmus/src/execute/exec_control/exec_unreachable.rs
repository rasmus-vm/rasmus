use crate::result::{RResult, Trap};

pub fn exec_unreachable() -> RResult<()> {
    Err(Trap)
}
