use super::value::Val;
use crate::entities::types::GlobalType;

pub struct GlobalInst {
    pub global_type: GlobalType,
    pub value: Val,
}
