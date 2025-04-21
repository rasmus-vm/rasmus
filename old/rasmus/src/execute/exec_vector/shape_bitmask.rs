use crate::{
    instances::{stack::Stack, value::Val},
    result::{RResult, Trap},
};

use super::{to_lanes_16x8, to_lanes_32x4, to_lanes_64x2, to_lanes_8x16};

pub fn bitmask_8x16(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(v);

    let bits = lanes
        .iter()
        .map(|l| (*l as i8) < 0)
        .rev()
        .chain((0..16).map(|_| false))
        .fold(0i32, |n, is_set| {
            if is_set {
                (n | 1).rotate_right(1)
            } else {
                n.rotate_right(1)
            }
        });

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        bits as u32,
    )));

    Ok(())
}

pub fn bitmask_16x8(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(v);

    let bits = lanes
        .iter()
        .map(|l| (*l as i16) < 0)
        .rev()
        .chain((0..8).map(|_| false))
        .fold(0i32, |n, is_set| {
            if is_set {
                (n | 1).rotate_right(1)
            } else {
                n.rotate_right(1)
            }
        });

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        bits as u32,
    )));

    Ok(())
}

pub fn bitmask_32x4(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(v);

    let bits = lanes
        .iter()
        .map(|l| (*l as i32) < 0)
        .rev()
        .chain((0..4).map(|_| false))
        .fold(0i32, |n, is_set| {
            if is_set {
                (n | 1).rotate_right(1)
            } else {
                n.rotate_right(1)
            }
        });

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        bits as u32,
    )));

    Ok(())
}

pub fn bitmask_64x2(stack: &mut Stack) -> RResult<()> {
    let v = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_64x2(v);

    let bits = lanes
        .iter()
        .map(|l| (*l as i64) < 0)
        .rev()
        .chain((0..4).map(|_| false))
        .fold(0i32, |n, is_set| {
            if is_set {
                (n | 1).rotate_right(1)
            } else {
                n.rotate_right(1)
            }
        });

    stack.push_entry(crate::instances::stack::StackEntry::Value(Val::I32(
        bits as u32,
    )));

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::entities::module::InstructionType;

    use crate::{execute::exec_vector::vec_from_lanes, test_utils::test_instruction};

    use super::*;

    #[test]
    fn bitmask_8x16() {
        test_instruction(
            vec![InstructionType::V128Const(
                vec_from_lanes(vec![
                    0, -1i8 as u8, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ])
                .to_be_bytes()
                .to_vec(),
            )],
            InstructionType::I8x16Bitmask,
            Val::I32(2i32 as u32),
        );

        test_instruction(
            vec![InstructionType::V128Const(
                vec_from_lanes(vec![
                    0, -1i8 as u8, 1, 0, 0, -1i8 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ])
                .to_be_bytes()
                .to_vec(),
            )],
            InstructionType::I8x16Bitmask,
            Val::I32(34i32 as u32),
        );
    }
}
