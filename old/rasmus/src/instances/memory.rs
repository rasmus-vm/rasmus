use super::value::Val;
use crate::entities::types::{Byte, LimitsType, MemType, U32Type};
use crate::result::{RResult, Trap};
use crate::validation::types_validation::is_memory_type_valid;

pub struct MemInst {
    pub mem_type: MemType,
    pub data: Vec<Byte>,
}

impl MemInst {
    pub const PAGE_SIZE: usize = 2usize.pow(16);

    #[inline]
    #[allow(dead_code)]
    pub fn grow(&mut self, n_val: &Val) -> RResult<()> {
        if let Val::I32(n) = n_val {
            return self.grow_n(*n);
        }

        Err(Trap)
    }

    pub fn grow_n(&mut self, n: u32) -> RResult<()> {
        let num = n as usize;
        if self.data.len().checked_rem(Self::PAGE_SIZE).is_some() {
            return Err(Trap);
        }

        let len = num + self.data.len() / Self::PAGE_SIZE;

        if len > 2usize.pow(16) {
            return Err(Trap);
        }

        let new_limits = LimitsType {
            min: U32Type(len as u32),
            max: self.mem_type.limits.max.clone(),
        };
        let new_mem_type = MemType { limits: new_limits };

        if !is_memory_type_valid(&new_mem_type) {
            return Err(Trap);
        }

        self.data.append(&mut vec![0x00; num]);
        self.mem_type = new_mem_type;

        return Ok(());
    }

    pub fn size(&self) -> u32 {
        (self.data.len() as u32) / (Self::PAGE_SIZE as u32)
    }
}
