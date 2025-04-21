use super::ref_inst::RefInst;
use super::value::Val;
use crate::entities::types::{LimitsType, TableType, U32Type};
use crate::result::{RResult, Trap};
use crate::validation::types_validation::is_table_type_valid;

pub struct TableInst {
    pub table_type: TableType,
    pub elem: Vec<RefInst>,
}

impl TableInst {
    pub const MAX_TABLE_SIZE: usize = 2usize.pow(32);

    #[inline]
    #[allow(dead_code)]
    pub fn grow(&mut self, n_val: &Val, reference: RefInst) -> RResult<()> {
        if let Val::I32(n) = n_val {
            return self.grow_n(*n, reference);
        }

        Err(Trap)
    }

    #[inline]
    pub fn grow_n(&mut self, n: u32, reference: RefInst) -> RResult<()> {
        let num = n as usize;
        let len = num + self.elem.len();
        if len > Self::MAX_TABLE_SIZE {
            return Err(Trap);
        }

        let new_limits = LimitsType {
            min: U32Type(len as u32),
            max: self.table_type.limits.max.clone(),
        };
        let new_table_type = TableType {
            limits: new_limits,
            element_ref_type: self.table_type.element_ref_type.clone(),
        };

        if !is_table_type_valid(&new_table_type) {
            return Err(Trap);
        }

        self.elem.append(&mut vec![reference; num]);
        self.table_type = new_table_type;

        return Ok(());
    }
}
