use crate::entities::types::Byte;

use crate::{
    instances::{
        ref_inst::RefInst,
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

pub fn i32_const(v: &u32, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::I32(*v)));
    Ok(())
}

pub fn i64_const(v: &u64, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::I64(*v)));
    Ok(())
}

pub fn f32_const(v: &f32, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::F32(*v)));
    Ok(())
}

pub fn f64_const(v: &f64, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::F64(*v)));
    Ok(())
}

pub fn v128_const(v: &Vec<u8>, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::Vec(v128_from_vec(v)?)));
    Ok(())
}

pub fn ref_const(r: RefInst, stack: &mut Stack) -> RResult<()> {
    stack.push_entry(StackEntry::Value(Val::Ref(r)));
    Ok(())
}

fn v128_from_vec(v: &Vec<Byte>) -> RResult<u128> {
    let slice: &[u8] = v.as_ref();
    let bytes: [u8; 16] = slice.try_into().map_err(|_| Trap)?;

    Ok(u128::from_le_bytes(bytes))
}

#[cfg(test)]
mod test {
    use crate::entities::{
        module::InstructionType,
        types::{F32Type, F64Type, I32Type, I64Type},
    };

    use crate::{instances::value::Val, test_utils::test_instruction};

    use super::v128_from_vec;

    #[test]
    fn i32_const() {
        test_instruction(vec![], InstructionType::I32Const(I32Type(1)), Val::I32(1));
    }

    #[test]
    fn i64_const() {
        test_instruction(vec![], InstructionType::I64Const(I64Type(1)), Val::I64(1));
    }

    #[test]
    fn f32_const() {
        test_instruction(
            vec![],
            InstructionType::F32Const(F32Type(1.0)),
            Val::F32(1.0),
        );
    }

    #[test]
    fn f64_const() {
        test_instruction(
            vec![],
            InstructionType::F64Const(F64Type(1.0)),
            Val::F64(1.0),
        );
    }

    #[test]
    fn v128_const() {
        test_instruction(
            vec![],
            InstructionType::V128Const(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            Val::Vec(v128_from_vec(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]).unwrap()),
        );
    }
}
