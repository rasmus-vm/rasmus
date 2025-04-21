use crate::address::*;
use crate::entities::types::NameType;

#[derive(Debug, PartialEq, Clone)]
pub struct ExportInst {
    pub name: NameType,
    pub value: ExternVal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}
