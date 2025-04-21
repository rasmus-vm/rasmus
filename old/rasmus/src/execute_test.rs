use crate::entities::{
    module::InstructionType,
    types::{I32Type, I64Type},
};

use crate::instances::value::Val;

#[macro_export]
macro_rules! test_instruction {
    ($test_name: ident, $instruction: expr, $expected_val: expr) => {
        #[test]
        fn $test_name() {
            let mut store = crate::instances::store::Store::new();
            let mut stack = crate::instances::stack::Stack::new();

            crate::execute::execute_instruction(&$instruction, &mut stack, &mut store)
                .expect("should execute without errors");

            if let Some(val) = stack.pop_value() {
                assert_eq!(val, $expected_val);
            } else {
                assert!(false, "stack should contain value");
            }
        }
    };
    ($test_name: ident, $before_instructions: expr, $instruction: expr, $expected_val: expr) => {
        #[test]
        fn $test_name() {
            let mut store = crate::instances::store::Store::new();
            let mut stack = crate::instances::stack::Stack::new();

            for ref before in $before_instructions {
                crate::execute::execute_instruction(before, &mut stack, &mut store)
                    .expect("shouold execute before instruction wihtout errors");
            }

            crate::execute::execute_instruction(&$instruction, &mut stack, &mut store)
                .expect("should execute instruction without errors");

            if let Some(val) = stack.pop_value() {
                assert_eq!(val, $expected_val);
            } else {
                assert!(false, "stack should contain value");
            }
        }
    };
}

#[macro_export]
macro_rules! test_instruction_assert {
    ($test_name: ident, $before_instructions: expr, $instruction: expr, $assert: expr) => {
        #[test]
        fn $test_name() {
            let mut store = Store::new();
            let mut stack = Stack::new();

            for ref before in $before_instructions {
                assert!(execute_instruction(before, &mut stack, &mut store).is_ok());
            }

            assert!(execute_instruction(&$instruction, &mut stack, &mut store).is_ok());

            if let Some(val) = stack.pop_value() {
                $assert(val);
            } else {
                assert!(false, "stack should contain value");
            }
        }
    };
}

test_instruction!(
    i32_eqz_true,
    vec![InstructionType::I32Const(I32Type(0))],
    InstructionType::I32Eqz,
    Val::I32(1)
);

test_instruction!(
    i32_eqz_false,
    vec![InstructionType::I32Const(I32Type(1))],
    InstructionType::I32Eqz,
    Val::I32(0)
);

test_instruction!(
    i64_eqz_true,
    vec![InstructionType::I64Const(I64Type(0))],
    InstructionType::I64Eqz,
    Val::I32(1)
);

test_instruction!(
    i64_eqz_false,
    vec![InstructionType::I64Const(I64Type(1))],
    InstructionType::I64Eqz,
    Val::I32(0)
);

test_instruction!(
    i32_eq_true,
    vec![
        InstructionType::I32Const(I32Type(0)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Eq,
    Val::I32(1)
);

test_instruction!(
    i32_eq_false,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Eq,
    Val::I32(0)
);

test_instruction!(
    i64_eq_true,
    vec![
        InstructionType::I64Const(I64Type(0)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Eq,
    Val::I32(1)
);

test_instruction!(
    i64_eq_false,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Eq,
    Val::I32(0)
);

test_instruction!(
    i32_ne_false,
    vec![
        InstructionType::I32Const(I32Type(0)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Ne,
    Val::I32(0)
);

test_instruction!(
    i32_ne_true,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(0))
    ],
    InstructionType::I32Ne,
    Val::I32(1)
);

test_instruction!(
    i64_ne_false,
    vec![
        InstructionType::I64Const(I64Type(0)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Ne,
    Val::I32(0)
);

test_instruction!(
    i64_ne_true,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(0))
    ],
    InstructionType::I64Ne,
    Val::I32(1)
);

test_instruction!(
    i32_lts_true_positive,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32LtS,
    Val::I32(1)
);

test_instruction!(
    i32_lts_true_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
    ],
    InstructionType::I32LtS,
    Val::I32(1)
);

test_instruction!(
    i32_lts_false_positive,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(1)),
    ],
    InstructionType::I32LtS,
    Val::I32(0)
);

test_instruction!(
    i32_lts_false_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
    ],
    InstructionType::I32LtS,
    Val::I32(0)
);

test_instruction!(
    i64_lts_true_positive,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64LtS,
    Val::I32(1)
);

test_instruction!(
    i64_lts_true_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
    ],
    InstructionType::I64LtS,
    Val::I32(1)
);

test_instruction!(
    i64_lts_false_positive,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(1)),
    ],
    InstructionType::I64LtS,
    Val::I32(0)
);

test_instruction!(
    i64_lts_false_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
    ],
    InstructionType::I64LtS,
    Val::I32(0)
);

test_instruction!(
    i32_ltu_true_positive,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32LtU,
    Val::I32(1)
);

test_instruction!(
    i32_ltu_true_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
    ],
    InstructionType::I32LtU,
    Val::I32(1)
);

test_instruction!(
    i32_ltu_false_positive,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(1)),
    ],
    InstructionType::I32LtU,
    Val::I32(0)
);

test_instruction!(
    i32_ltu_false_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
    ],
    InstructionType::I32LtU,
    Val::I32(0)
);

test_instruction!(
    i64_ltu_true_positive,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64LtU,
    Val::I32(1)
);

test_instruction!(
    i64_ltu_true_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
    ],
    InstructionType::I64LtU,
    Val::I32(1)
);

test_instruction!(
    i64_ltu_false_positive,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(1)),
    ],
    InstructionType::I64LtU,
    Val::I32(0)
);

test_instruction!(
    i64_ltu_false_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
    ],
    InstructionType::I64LtU,
    Val::I32(0)
);

test_instruction!(
    i32_gts_false_positive,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32GtS,
    Val::I32(0)
);

test_instruction!(
    i32_gts_false_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
    ],
    InstructionType::I32GtS,
    Val::I32(0)
);

test_instruction!(
    i32_gts_true_positive,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(1)),
    ],
    InstructionType::I32GtS,
    Val::I32(1)
);

test_instruction!(
    i32_gts_true_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
    ],
    InstructionType::I32GtS,
    Val::I32(1)
);

test_instruction!(
    i64_gts_false_positive,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64GtS,
    Val::I32(0)
);

test_instruction!(
    i64_gts_false_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
    ],
    InstructionType::I64GtS,
    Val::I32(0)
);

test_instruction!(
    i64_gts_true_positive,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(1)),
    ],
    InstructionType::I64GtS,
    Val::I32(1)
);

test_instruction!(
    i64_gts_true_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
    ],
    InstructionType::I64GtS,
    Val::I32(1)
);

test_instruction!(
    i32_gtu_false_positive,
    vec![
        InstructionType::I32Const(I32Type(1)),
        InstructionType::I32Const(I32Type(2))
    ],
    InstructionType::I32GtU,
    Val::I32(0)
);

test_instruction!(
    i32_gtu_false_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
    ],
    InstructionType::I32GtU,
    Val::I32(0)
);

test_instruction!(
    i32_gtu_true_positive,
    vec![
        InstructionType::I32Const(I32Type(2)),
        InstructionType::I32Const(I32Type(1)),
    ],
    InstructionType::I32GtU,
    Val::I32(1)
);

test_instruction!(
    i32_gtu_true_negative,
    vec![
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111111u32)),
        InstructionType::I32Const(I32Type(0b11111111111111111111111111111110u32)),
    ],
    InstructionType::I32GtU,
    Val::I32(1)
);

test_instruction!(
    i64_gtu_false_positive,
    vec![
        InstructionType::I64Const(I64Type(1)),
        InstructionType::I64Const(I64Type(2))
    ],
    InstructionType::I64GtU,
    Val::I32(0)
);

test_instruction!(
    i64_gtu_false_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
    ],
    InstructionType::I64GtU,
    Val::I32(0)
);

test_instruction!(
    i64_gtu_true_positive,
    vec![
        InstructionType::I64Const(I64Type(2)),
        InstructionType::I64Const(I64Type(1)),
    ],
    InstructionType::I64GtU,
    Val::I32(1)
);

test_instruction!(
    i64_gtu_true_negative,
    vec![
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111111u64
        )),
        InstructionType::I64Const(I64Type(
            0b1111111111111111111111111111111111111111111111111111111111111110u64
        )),
    ],
    InstructionType::I64GtU,
    Val::I32(1)
);
