use core::iter::Iterator;

use crate::instructions::Instr;

pub struct Func {}

impl Func {
    pub fn instructions(&self) -> impl Iterator<Item = Instr> {
        [].into_iter()
    }
}
