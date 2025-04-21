use super::ref_inst::RefInst;
use crate::entities::types::RefType;

#[derive(Debug, Clone)]
pub struct ElemInst {
    pub elem_type: RefType,
    pub elem: Vec<RefInst>,
}
