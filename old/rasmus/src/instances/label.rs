use std::rc::Rc;

use crate::entities::module::InstructionType;

#[derive(Debug, Clone)]
pub struct LabelInst {
    pub arity: usize,
    pub instructions: Rc<Vec<InstructionType>>,
}
