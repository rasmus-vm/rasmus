use crate::{
    instructions::Instr,
    runtime::{stack::Stack, store::Store, trap::RResult},
};

use super::ModuleInst;

pub fn exec_instr(
    instr: &Instr,
    store: &mut Store,
    stack: &mut Stack,
    module: &ModuleInst,
) -> RResult<()> {
    todo!()
}
