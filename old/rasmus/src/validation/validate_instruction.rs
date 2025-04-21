use crate::entities::instructions::{InstructionType, InstructionType as I};

use super::{
    context::ValidationContext,
    instructions::{
        block, br, br_if, br_table, call, call_indirect, data_drop, drop_val, elem_drop,
        extract_line_f, extract_line_i, f32_binop, f32_const, f32_relop, f32_to_f64_cvtop,
        f32_to_i32_cvtop, f32_to_i64_cvtop, f32_unop, f32x4_splat, f64_binop, f64_const, f64_relop,
        f64_to_f32_cvtop, f64_to_i32_cvtop, f64_to_i64_cvtop, f64_unop, f64x2_splat, global_get,
        global_set, i32_binop, i32_const, i32_relop, i32_testop, i32_to_f32_cvtop,
        i32_to_f64_cvtop, i32_to_i32_cvtop, i32_to_i64_cvtop, i32_unop, i64_binop, i64_const,
        i64_relop, i64_testop, i64_to_f32_cvtop, i64_to_f64_cvtop, i64_to_i64_cvtop, i64_unop,
        i64x2_splat, i8x16_shuffle, i8x16_splat, i8x16_swizzle, if_else, ishape_bitmask, load_f32,
        load_f64, load_i32, load_i32_t, load_i64, load_i64_t, load_v128, load_vec_lane,
        load_vec_nm, load_vec_splat, local_get, local_set, local_tee, loop_instr, memory_fill,
        memory_grow, memory_init, memory_size, ref_func, ref_is_null, ref_null, replace_line_f,
        replace_line_i, return_instr, select, select_vec, shape_bitop, store_value, store_value_t,
        store_vec_lane, table_copy, table_fill, table_get, table_grow, table_init, table_set,
        table_size, unreachable_instr, v128_binop, v128_const, v128_relop, v128_ternop,
        v128_testop, v128_unop,
    },
    validation_error::ValidationResult,
    validation_stack::{ValidationStack, ValidationType},
};

pub fn validate_instruction(
    instruction: &InstructionType,
    ctx: &ValidationContext,
    val_stack: &mut ValidationStack,
) -> ValidationResult<()> {
    match instruction {
        // t.const
        I::I32Const(_) => i32_const(val_stack)?,
        I::I64Const(_) => i64_const(val_stack)?,
        I::F32Const(_) => f32_const(val_stack)?,
        I::F64Const(_) => f64_const(val_stack)?,

        // t.unop
        I::I32Clz | I::I32Ctz | I::I32Popcnt => i32_unop(val_stack)?,
        I::I64Clz | I::I64Ctz | I::I64Popcnt => i64_unop(val_stack)?,
        I::F32Abs
        | I::F32Neg
        | I::F32Ceil
        | I::F32Floor
        | I::F32Trunc
        | I::F32Nearest
        | I::F32Sqrt => f32_unop(val_stack)?,
        I::F64Abs
        | I::F64Neg
        | I::F64Ceil
        | I::F64Floor
        | I::F64Trunc
        | I::F64Nearest
        | I::F64Sqrt => f64_unop(val_stack)?,

        // t.testop
        I::I32Eqz => i32_testop(val_stack)?,
        I::I64Eqz => i64_testop(val_stack)?,

        // t.binop
        I::I32Add
        | I::I32Sub
        | I::I32Mul
        | I::I32DivS
        | I::I32DivU
        | I::I32RemS
        | I::I32RemU
        | I::I32And
        | I::I32Or
        | I::I32Xor
        | I::I32Shl
        | I::I32ShrS
        | I::I32ShrU
        | I::I32Rotl
        | I::I32Rotr => i32_binop(val_stack)?,
        I::I64Add
        | I::I64Sub
        | I::I64Mul
        | I::I64DivS
        | I::I64DivU
        | I::I64RemS
        | I::I64RemU
        | I::I64And
        | I::I64Or
        | I::I64Xor
        | I::I64Shl
        | I::I64ShrS
        | I::I64ShrU
        | I::I64Rotl
        | I::I64Rotr => i64_binop(val_stack)?,
        I::F32Add | I::F32Sub | I::F32Mul | I::F32Div | I::F32Min | I::F32Max | I::F32Copysign => {
            f32_binop(val_stack)?
        }
        I::F64Add | I::F64Sub | I::F64Mul | I::F64Div | I::F64Min | I::F64Max | I::F64Copysign => {
            f64_binop(val_stack)?
        }

        // t.relop
        I::I32Eq
        | I::I32Ne
        | I::I32LtS
        | I::I32LtU
        | I::I32GtS
        | I::I32GtU
        | I::I32LeS
        | I::I32LeU
        | I::I32GeS
        | I::I32GeU => i32_relop(val_stack)?,
        I::I64Eq
        | I::I64Ne
        | I::I64LtS
        | I::I64LtU
        | I::I64GtS
        | I::I64GtU
        | I::I64LeS
        | I::I64LeU
        | I::I64GeS
        | I::I64GeU => i64_relop(val_stack)?,
        I::F32Eq | I::F32Ne | I::F32Lt | I::F32Gt | I::F32Le | I::F32Ge => f32_relop(val_stack)?,
        I::F64Eq | I::F64Ne | I::F64Lt | I::F64Gt | I::F64Le | I::F64Ge => f64_relop(val_stack)?,

        // t.cvtop
        I::I32WrapI64 => i32_to_i64_cvtop(val_stack)?,
        I::I32Extend8S | I::I32Extend16S => i32_to_i32_cvtop(val_stack)?,
        I::I64Extend8S
        | I::I64Extend32S
        | I::I64ExtendI32S
        | I::I64ExtendI32U
        | I::I64Extend16S => i64_to_i64_cvtop(val_stack)?,
        I::I32TruncF32S | I::I32TruncF32U | I::I32TruncSatF32S | I::I32TruncSatF32U => {
            f32_to_i32_cvtop(val_stack)?
        }
        I::I32TruncF64S | I::I32TruncF64U | I::I32TruncSatF64S | I::I32TruncSatF64U => {
            f64_to_i32_cvtop(val_stack)?
        }
        I::I64TruncF32S | I::I64TruncF32U | I::I64TruncSatF32S | I::I64TruncSatF32U => {
            f32_to_i64_cvtop(val_stack)?
        }
        I::I64TruncF64S | I::I64TruncF64U | I::I64TruncSatF64S | I::I64TruncSatF64U => {
            f64_to_i64_cvtop(val_stack)?
        }
        I::F32ConvertI32S | I::F32ConvertI32U => i32_to_f32_cvtop(val_stack)?,
        I::F32ConvertI64S | I::F32ConvertI64U => i64_to_f32_cvtop(val_stack)?,
        I::F64ConvertI32S | I::F64ConvertI32U => i32_to_f64_cvtop(val_stack)?,
        I::F64ConvertI64S | I::F64ConvertI64U => i64_to_f64_cvtop(val_stack)?,
        I::F32DemoteF64 => f64_to_f32_cvtop(val_stack)?,
        I::F64PromoteF32 => f32_to_f64_cvtop(val_stack)?,
        I::I32ReinterpretF32 => f32_to_i32_cvtop(val_stack)?,
        I::I64ReinterpretF64 => f64_to_i64_cvtop(val_stack)?,
        I::F32ReinterpretI32 => i32_to_f32_cvtop(val_stack)?,
        I::F64ReinterpretI64 => i64_to_f64_cvtop(val_stack)?,

        // Ref instructions
        I::RefNull(_) => ref_null(val_stack)?,
        I::RefIsNull => ref_is_null(val_stack)?,
        I::RefFunc(ref func_idx) => ref_func(func_idx, val_stack, ctx)?,

        // Vector instructions

        // v128.const
        I::V128Const(_) => v128_const(val_stack)?,

        // v128.vvunop
        I::V128Not => v128_unop(val_stack)?,

        // v128.vvbinop
        I::V128And | I::V128AndNot | I::V128Or | I::V128Xor => v128_binop(val_stack)?,

        // v128.vvternop
        I::V128Bitselect => v128_ternop(val_stack)?,

        // v128.vvtestop
        I::V128AnyTrue => v128_testop(val_stack)?,

        // i8x16.swizzle
        I::I8x16Swizzle => i8x16_swizzle(val_stack)?,

        // i8x16.shuffle
        I::I8x16Shuffle(lane_indexes) => i8x16_shuffle(lane_indexes, val_stack)?,

        // shape.splat (more details about 'shape' - https://webassembly.github.io/spec/core/valid/instructions.html#vector-instructions)
        I::I8x16Splat | I::I16x8Splat | I::I32x4Splat => i8x16_splat(val_stack)?,
        I::I64x2Splat => i64x2_splat(val_stack)?,
        I::F32x4Splat => f32x4_splat(val_stack)?,
        I::F64x2Splat => f64x2_splat(val_stack)?,

        // shape.extract_lane_sx
        I::I8x16ExtractLaneS(ref lane_idx) | I::I8x16ExtractLaneU(ref lane_idx) => {
            extract_line_i(16, lane_idx, val_stack)?
        }
        I::I16x8ExtractLaneS(ref lane_idx) | I::I16x8ExtractLaneU(ref lane_idx) => {
            extract_line_i(8, lane_idx, val_stack)?
        }
        I::I32x4ExtractLane(lane_idx) => extract_line_i(4, lane_idx, val_stack)?,
        I::I64x2ExtractLane(lane_idx) => extract_line_i(2, lane_idx, val_stack)?,
        I::F32x4ExtractLane(lane_idx) => extract_line_f(4, lane_idx, val_stack)?,
        I::F64x2ExtractLane(lane_idx) => extract_line_f(2, lane_idx, val_stack)?,

        // shape.replace_lane
        I::I8x16ReplaceLane(ref lane_idx) => replace_line_i(16, lane_idx, val_stack)?,
        I::I16x8ReplaceLane(ref lane_idx) => replace_line_i(8, lane_idx, val_stack)?,
        I::I32x4ReplaceLane(ref lane_idx) => replace_line_i(4, lane_idx, val_stack)?,
        I::I64x2ReplaceLane(ref lane_idx) => replace_line_i(2, lane_idx, val_stack)?,
        I::F32x4ReplaceLane(ref lane_idx) => replace_line_f(4, lane_idx, val_stack)?,
        I::F64x2ReplaceLane(ref lane_idx) => replace_line_f(2, lane_idx, val_stack)?,

        I::I8x16Eq
        | I::I8x16Ne
        | I::I8x16LtS
        | I::I8x16LtU
        | I::I8x16GtS
        | I::I8x16GtU
        | I::I8x16LeS
        | I::I8x16LeU
        | I::I8x16GeS
        | I::I8x16GeU
        | I::I16x8Eq
        | I::I16x8Ne
        | I::I16x8LtS
        | I::I16x8LtU
        | I::I16x8GtS
        | I::I16x8GtU
        | I::I16x8LeS
        | I::I16x8LeU
        | I::I16x8GeS
        | I::I16x8GeU
        | I::I32x4Eq
        | I::I32x4Ne
        | I::I32x4LtS
        | I::I32x4LtU
        | I::I32x4GtS
        | I::I32x4GtU
        | I::I32x4LeS
        | I::I32x4LeU
        | I::I32x4GeS
        | I::I32x4GeU
        | I::I64x2Eq
        | I::I64x2Ne
        | I::I64x2LtS
        | I::I64x2GtS
        | I::I64x2LeS
        | I::I64x2GeS
        | I::F32x4Eq
        | I::F32x4Ne
        | I::F32x4Lt
        | I::F32x4Gt
        | I::F32x4Le
        | I::F32x4Ge
        | I::F64x2Eq
        | I::F64x2Ne
        | I::F64x2Lt
        | I::F64x2Gt
        | I::F64x2Le
        | I::F64x2Ge => v128_binop(val_stack)?,

        I::I8x16Shl
        | I::I8x16ShrS
        | I::I8x16ShrU
        | I::I16x8Shl
        | I::I16x8ShrS
        | I::I16x8ShrU
        | I::I32x4Shl
        | I::I32x4ShrS
        | I::I32x4ShrU
        | I::I64x2Shl
        | I::I64x2ShrS
        | I::I64x2ShrU => shape_bitop(val_stack)?,

        I::I8x16Abs
        | I::I8x16Neg
        | I::I16x8Abs
        | I::I16x8Neg
        | I::I32x4Abs
        | I::I32x4Neg
        | I::I64x2Abs
        | I::I64x2Neg
        | I::F32x4Abs
        | I::F32x4Neg
        | I::F64x2Abs
        | I::F64x2Neg
        | I::F32x4Sqrt
        | I::F64x2Sqrt
        | I::F32x4Ceil
        | I::F64x2Ceil
        | I::F32x4Floor
        | I::F64x2Floor
        | I::F32x4Trunc
        | I::F64x2Trunc
        | I::F32x4Nearest
        | I::F64x2Nearest
        | I::I8x16Popcnt => v128_unop(val_stack)?,

        I::I8x16Add
        | I::I8x16Sub
        | I::I16x8Add
        | I::I16x8Sub
        | I::I32x4Add
        | I::I32x4Sub
        | I::I64x2Add
        | I::I64x2Sub
        | I::F32x4Add
        | I::F32x4Sub
        | I::F32x4Mul
        | I::F32x4Div
        | I::F32x4Min
        | I::F32x4Max
        | I::F32x4Pmin
        | I::F32x4Pmax
        | I::F64x2Add
        | I::F64x2Sub
        | I::F64x2Mul
        | I::F64x2Div
        | I::F64x2Min
        | I::F64x2Max
        | I::F64x2Pmin
        | I::F64x2Pmax
        | I::I8x16MinS
        | I::I8x16MinU
        | I::I8x16MaxS
        | I::I8x16MaxU
        | I::I16x8MinS
        | I::I16x8MinU
        | I::I16x8MaxS
        | I::I16x8MaxU
        | I::I32x4MinS
        | I::I32x4MinU
        | I::I32x4MaxS
        | I::I32x4MaxU
        | I::I8x16AddSatS
        | I::I8x16AddSatU
        | I::I8x16SubSatS
        | I::I8x16SubSatU
        | I::I16x8AddSatS
        | I::I16x8AddSatU
        | I::I16x8SubSatS
        | I::I16x8SubSatU
        | I::I16x8Mul
        | I::I32x4Mul
        | I::I64x2Mul
        | I::I8x16AvgrU
        | I::I16x8AvgrU
        | I::I16x8Q15MulrSatS => v128_binop(val_stack)?,

        I::I8x16AllTrue | I::I16x8AllTrue | I::I32x4AllTrue | I::I64x2AllTrue => {
            v128_relop(val_stack)?
        }

        I::I16x8ExtendLowI8x16S
        | I::I16x8ExtendHighI8x16S
        | I::I16x8ExtendLowI8x16U
        | I::I16x8ExtendHighI8x16U
        | I::I32x4ExtendLowI16x8S
        | I::I32x4ExtendHighI16x8S
        | I::I32x4ExtendLowI16x8U
        | I::I32x4ExtendHighI16x8U
        | I::I32x4TruncSatF32x4S
        | I::I32x4TruncSatF32x4U
        | I::I64x2ExtendLowI32x4S
        | I::I64x2ExtendHighI32x4S
        | I::I64x2ExtendLowI32x4U
        | I::I64x2ExtendHighI32x4U
        | I::F32x4ConvertI32x4S
        | I::F32x4ConvertI32x4U
        | I::I32x4TruncSatF64x2SZero
        | I::I32x4TruncSatF64x2UZero
        | I::F64x2ConvertLowI32x4S
        | I::F64x2ConvertLowI32x4U
        | I::F32x4DemoteF64x2Zero
        | I::F64x2PromoteLowF32x4 => v128_unop(val_stack)?,

        // ishape.narrow_ishape_sx
        I::I16x8NarrowI32x4S
        | I::I16x8NarrowI32x4U
        | I::I8x16NarrowI16x8S
        | I::I8x16NarrowI16x8U => v128_binop(val_stack)?,

        // ishape.bitmask
        I::I8x16Bitmask | I::I16x8Bitmask | I::I32x4Bitmask | I::I64x2Bitmask => {
            ishape_bitmask(val_stack)?
        }

        // ishape.dot_ishape_s
        I::I32x4DotI16x8S => v128_binop(val_stack)?,

        // ishape.extmul_half_ishape_sx
        I::I16x8ExtmulLowI8x16S
        | I::I16x8ExtmulHighI8x16S
        | I::I16x8ExtmulLowI8x16U
        | I::I16x8ExtmulHighI8x16U
        | I::I32x4ExtmulLowI16x8S
        | I::I32x4ExtmulHighI16x8S
        | I::I32x4ExtmulLowI16x8U
        | I::I32x4ExtmulHighI16x8U
        | I::I64x2ExtmulLowI32x4S
        | I::I64x2ExtmulHighI32x4S
        | I::I64x2ExtmulLowI32x4U
        | I::I64x2ExtmulHighI32x4U => v128_binop(val_stack)?,

        // ishape.extadd_pairwise_ishape_sx
        I::I16x8ExtaddPairwiseI8x16S
        | I::I16x8ExtaddPairwiseI8x16U
        | I::I32x4ExtaddPairwiseI16x8S
        | I::I32x4ExtaddPairwiseI16x8U => v128_unop(val_stack)?,

        // Parametric instructions
        I::Drop => drop_val(val_stack)?,
        I::Select => select(val_stack)?,
        I::SelectVec(val_types) => select_vec(val_stack, val_types)?,

        // Variable instructions
        I::LocalGet(local_idx) => local_get(val_stack, ctx, local_idx)?,
        I::LocalSet(local_idx) => local_set(val_stack, ctx, local_idx)?,
        I::LocalTee(local_idx) => local_tee(val_stack, ctx, local_idx)?,
        I::GlobalGet(global_idx) => global_get(val_stack, ctx, global_idx)?,
        I::GlobalSet(global_idx) => global_set(val_stack, ctx, global_idx)?,

        // Table instructions
        I::TableGet(table_idx) => table_get(val_stack, ctx, table_idx)?,
        I::TableSet(table_idx) => table_set(val_stack, ctx, table_idx)?,
        I::TableSize(table_idx) => table_size(val_stack, ctx, table_idx)?,
        I::TableGrow(table_idx) => table_grow(val_stack, ctx, table_idx)?,
        I::TableFill(table_idx) => table_fill(val_stack, ctx, table_idx)?,
        I::TableCopy(copy_args) => table_copy(val_stack, ctx, copy_args)?,
        I::TableInit(init_args) => table_init(val_stack, ctx, init_args)?,
        I::ElemDrop(elem_idx) => elem_drop(ctx, elem_idx)?,

        // Memory instructions
        I::I32Load(memarg) => load_i32(val_stack, ctx, memarg)?,
        I::I64Load(memarg) => load_i64(val_stack, ctx, memarg)?,
        I::F32Load(memarg) => load_f32(val_stack, ctx, memarg)?,
        I::F64Load(memarg) => load_f64(val_stack, ctx, memarg)?,
        I::V128Load(memarg) => load_v128(val_stack, ctx, memarg)?,
        I::I32Load8S(memarg) | I::I32Load8U(memarg) => load_i32_t(val_stack, ctx, memarg, 8)?,
        I::I32Load16S(memarg) | I::I32Load16U(memarg) => load_i32_t(val_stack, ctx, memarg, 16)?,
        I::I64Load8S(memarg) | I::I64Load8U(memarg) => load_i64_t(val_stack, ctx, memarg, 8)?,
        I::I64Load16S(memarg) | I::I64Load16U(memarg) => load_i64_t(val_stack, ctx, memarg, 16)?,
        I::I64Load32S(memarg) | I::I64Load32U(memarg) => load_i64_t(val_stack, ctx, memarg, 32)?,
        I::I32Store(memarg) => store_value(val_stack, ctx, memarg, ValidationType::i32())?,
        I::I64Store(memarg) => store_value(val_stack, ctx, memarg, ValidationType::i64())?,
        I::F32Store(memarg) => store_value(val_stack, ctx, memarg, ValidationType::f32())?,
        I::F64Store(memarg) => store_value(val_stack, ctx, memarg, ValidationType::f64())?,
        I::V128Store(memarg) => store_value(val_stack, ctx, memarg, ValidationType::v128())?,
        I::I32Store8(memarg) => store_value_t(val_stack, ctx, memarg, ValidationType::i32(), 8)?,
        I::I32Store16(memarg) => store_value_t(val_stack, ctx, memarg, ValidationType::i32(), 16)?,
        I::I64Store8(memarg) => store_value_t(val_stack, ctx, memarg, ValidationType::i64(), 8)?,
        I::I64Store16(memarg) => store_value_t(val_stack, ctx, memarg, ValidationType::i64(), 16)?,
        I::I64Store32(memarg) => store_value_t(val_stack, ctx, memarg, ValidationType::i64(), 32)?,
        I::V128Load8x8S(memarg) | I::V128Load8x8U(memarg) => {
            load_vec_nm(val_stack, ctx, memarg, (8, 8))?
        }
        I::V128Load16x4S(memarg) | I::V128Load16x4U(memarg) => {
            load_vec_nm(val_stack, ctx, memarg, (16, 4))?
        }
        I::V128Load32x2S(memarg) | I::V128Load32x2U(memarg) => {
            load_vec_nm(val_stack, ctx, memarg, (32, 2))?
        }
        I::V128Load8Splat(memarg) => load_vec_splat(val_stack, ctx, memarg, 8)?,
        I::V128Load16Splat(memarg) => load_vec_splat(val_stack, ctx, memarg, 16)?,
        I::V128Load32Splat(memarg) | I::V128Load32Zero(memarg) => {
            load_vec_splat(val_stack, ctx, memarg, 32)?
        }
        I::V128Load64Splat(memarg) | I::V128Load64Zero(memarg) => {
            load_vec_splat(val_stack, ctx, memarg, 64)?
        }
        I::V128Load8Lane((memarg, lane_idx)) => load_vec_lane(val_stack, ctx, memarg, lane_idx, 8)?,
        I::V128Load16Lane((memarg, lane_idx)) => {
            load_vec_lane(val_stack, ctx, memarg, lane_idx, 16)?
        }
        I::V128Load32Lane((memarg, lane_idx)) => {
            load_vec_lane(val_stack, ctx, memarg, lane_idx, 32)?
        }
        I::V128Load64Lane((memarg, lane_idx)) => {
            load_vec_lane(val_stack, ctx, memarg, lane_idx, 64)?
        }
        I::V128Store8Lane((memarg, lane_idx)) => {
            store_vec_lane(val_stack, ctx, memarg, lane_idx, 8)?
        }
        I::V128Store16Lane((memarg, lane_idx)) => {
            store_vec_lane(val_stack, ctx, memarg, lane_idx, 16)?
        }
        I::V128Store32Lane((memarg, lane_idx)) => {
            store_vec_lane(val_stack, ctx, memarg, lane_idx, 32)?
        }
        I::V128Store64Lane((memarg, lane_idx)) => {
            store_vec_lane(val_stack, ctx, memarg, lane_idx, 64)?
        }
        I::MemorySize => memory_size(val_stack, ctx)?,
        I::MemoryGrow => memory_grow(val_stack, ctx)?,
        I::MemoryFill | I::MemoryCopy => memory_fill(val_stack, ctx)?,
        I::MemoryInit(data_idx) => memory_init(val_stack, ctx, data_idx)?,
        I::DataDrop(data_idx) => data_drop(val_stack, ctx, data_idx)?,

        // Control instructions
        I::Nop => {
            // always valid
        }
        I::Unreachable => unreachable_instr(val_stack)?,
        I::Block(block_instruction) => {
            block(val_stack, ctx, block_instruction, validate_instruction)?
        }
        I::Loop(loop_instruction) => {
            loop_instr(val_stack, ctx, loop_instruction, validate_instruction)?
        }
        I::IfElse(ifelse_instruction) => {
            if_else(val_stack, ctx, ifelse_instruction, validate_instruction)?
        }
        I::Br(label_idx) => br(val_stack, label_idx)?,
        I::BrIf(label_idx) => br_if(val_stack, label_idx)?,
        I::BrTable(br_table_arg) => br_table(val_stack, br_table_arg)?,
        I::Return => return_instr(val_stack, ctx)?,
        I::Call(func_idx) => call(val_stack, ctx, func_idx)?,
        I::CallIndirect(call_indirect_arg) => call_indirect(val_stack, ctx, call_indirect_arg)?,
    }

    Ok(())
}
