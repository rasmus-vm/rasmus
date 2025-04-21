use crate::entities::types::RefType;
use crate::{
    address::{ExternAddr, FuncAddr},
    result::{RResult, Trap},
};

#[derive(Debug, Clone, PartialEq)]
pub enum RefInst {
    Null(RefType),
    Func(FuncAddr),
    Extern(ExternAddr),
}

impl RefInst {
    pub fn is_null(&self) -> bool {
        if let RefInst::Null(_) = self {
            return true;
        }

        false
    }

    pub fn as_func(&self) -> RResult<FuncAddr> {
        if let RefInst::Func(func_addr) = self {
            return Ok(*func_addr);
        }

        Err(Trap)
    }
}
