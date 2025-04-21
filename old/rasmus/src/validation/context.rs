use crate::entities::{
    module::{DataType, Module},
    types::*,
};

#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    pub types: Vec<FuncType>,
    pub funcs: Vec<FuncType>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<GlobalType>,
    pub elems: Vec<RefType>,
    pub datas: Vec<DataType>,
    pub locals: Vec<ValType>,
    pub labels: Vec<ResultType>,
    pub maybe_return: Option<ResultType>,
    pub refs: Vec<FuncIdx>,
}

impl From<&Module> for ValidationContext {
    fn from(module: &Module) -> Self {
        ValidationContext {
            types: module.types.clone(),
            funcs: module
                .funcs
                .iter()
                .map(|idx| module.types[idx.0 .0 as usize].clone())
                .collect(),
            tables: module.tables.clone(),
            mems: module.mems.clone(),
            globals: module
                .globals
                .iter()
                .map(|g| g.global_type.clone())
                .collect(),
            elems: module.elems.iter().map(|e| e.get_type()).collect(),
            datas: module.datas.clone(),
            locals: vec![],
            labels: Vec::new(),
            maybe_return: None,
            refs: vec![],
        }
    }
}
