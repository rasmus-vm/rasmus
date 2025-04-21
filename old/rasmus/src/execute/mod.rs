mod as_float_trait;
mod as_signed_trait;
mod exec_binop;
mod exec_const;
mod exec_control;
mod exec_cvtop;
mod exec_memory;
mod exec_parametric;
mod exec_ref;
mod exec_table;
mod exec_unop;
mod exec_variable;
mod exec_vec;
mod exec_vector;
pub mod executor;

pub use exec_control::pop_values_original_order;

use crate::result::{RResult, Trap};

use crate::entities::instructions::{ExpressionType, InstructionType};
use crate::entities::types::{F32Type, F64Type, FuncIdx, I32Type, I64Type, LaneIdx, U32Type};
use crate::instances::instruction::{
    bitselect, eq, eqz, ges, geu, gts, gtu, les, leu, lts, ltu, neq,
};
use crate::instances::stack::{Stack, StackEntry};
use crate::instances::store::Store;
use crate::instances::value::Val;
use crate::sign::Sign;
use crate::{relop_impl, testop_impl};

use self::exec_binop::{
    f32_add, f32_copysign, f32_div, f32_max, f32_min, f32_mul, f32_sub, f64_add, f64_copysign,
    f64_div, f64_max, f64_min, f64_mul, f64_sub, i32_add, i32_and, i32_div_s, i32_div_u, i32_mul,
    i32_or, i32_rem_s, i32_rem_u, i32_rotl, i32_rotr, i32_shl, i32_shr_s, i32_shr_u, i32_sub,
    i32_xor, i64_add, i64_and, i64_div_s, i64_div_u, i64_mul, i64_or, i64_rem_s, i64_rem_u,
    i64_rotl, i64_rotr, i64_shl, i64_shr_s, i64_shr_u, i64_sub, i64_xor,
};
use self::exec_const::{f32_const, f64_const, i32_const, i64_const, v128_const};
use self::exec_control::{
    block, exec_br, exec_brif, exec_brtable, exec_call, exec_call_indirect, exec_ifelse, exec_loop,
    exec_return, exec_unreachable,
};
use self::exec_cvtop::{
    f32_convert_i32_s, f32_convert_i32_u, f32_convert_i64_s, f32_convert_i64_u, f32_demote_f64,
    f32_reinterpret_i32, f64_convert_i32_s, f64_convert_i32_u, f64_convert_i64_s,
    f64_convert_i64_u, f64_promote_f32, f64_reinterpret_i64, i32_reinterpret_f32, i32_trunc_f32_s,
    i32_trunc_f32_u, i32_trunc_f64_s, i32_trunc_f64_u, i32_trunc_sat_f32_s, i32_trunc_sat_f32_u,
    i32_trunc_sat_f64_s, i32_trunc_sat_f64_u, i32_wrap_i64, i64_extend_i32, i64_reinterpret_f64,
    i64_trunc_f32_s, i64_trunc_f32_u, i64_trunc_f64_s, i64_trunc_f64_u, i64_trunc_sat_f32_s,
    i64_trunc_sat_f32_u, i64_trunc_sat_f64_s, i64_trunc_sat_f64_u,
};
use self::exec_memory::{
    data_drop, f32_load, f32_store, f64_load, f64_store, i32_load, i32_load_16, i32_load_8,
    i32_store, i32_store16, i32_store8, i64_load, i64_load_16, i64_load_32, i64_load_8, i64_store,
    i64_store16, i64_store32, i64_store8, memory_copy, memory_fill, memory_grow, memory_init,
    memory_size, v128_load, v128_load16_lane, v128_load16_splat, v128_load32_lane,
    v128_load32_splat, v128_load32_zero, v128_load64_lane, v128_load64_splat, v128_load64_zero,
    v128_load8_lane, v128_load8_splat, v128_load_16x4, v128_load_32x2, v128_load_8x8, v128_store,
    v128_store16_lane, v128_store32_lane, v128_store64_lane, v128_store8_lane,
};
use self::exec_parametric::{exec_drop, exec_select, exec_select_vec};
use self::exec_ref::{is_ref_null, ref_func, ref_null};
use self::exec_table::{
    elem_drop, table_copy, table_fill, table_get, table_grow, table_init, table_set, table_size,
};
use self::exec_unop::{
    f32_abs, f32_ceil, f32_floor, f32_nearest, f32_neg, f32_sqrt, f32_trunc, f64_abs, f64_ceil,
    f64_floor, f64_nearest, f64_neg, f64_sqrt, f64_trunc, i32_clz, i32_ctz, i32_extend_16s,
    i32_extend_8s, i32_popcnt, i64_clz, i64_ctz, i64_extend_16s, i64_extend_32s, i64_extend_8s,
    i64_popcnt,
};
use self::exec_variable::{global_get, global_set, local_get, local_set, local_tee};
use self::exec_vec::{
    binop_16x8, binop_32x4, binop_64x2, binop_8x16, f32x4_extract_lane, f32x4_replace_lane,
    f32x4_splat, f64x2_extract_lane, f64x2_replace_lane, f64x2_splat, i16x8_extract_lane_s,
    i16x8_extract_lane_u, i16x8_replace_lane, i16x8_splat, i32x4_extract_lane, i32x4_replace_lane,
    i32x4_splat, i64x2_extract_lane, i64x2_replace_lane, i64x2_splat, i8x16_extract_lane_s,
    i8x16_extract_lane_u, i8x16_replace_lane, i8x16_shuffle, i8x16_splat, i8x16_swizzle,
    make_shape_shl, make_shape_shr_s, make_shape_shr_u, relop_16x8, relop_32x4, relop_64x2,
    relop_8x16, shape_eq, shape_f32_abs, shape_f32_add, shape_f32_ceil, shape_f32_div,
    shape_f32_floor, shape_f32_max, shape_f32_min, shape_f32_mul, shape_f32_nearest, shape_f32_neg,
    shape_f32_pmax, shape_f32_pmin, shape_f32_sqrt, shape_f32_sub, shape_f32_trunc, shape_f64_abs,
    shape_f64_add, shape_f64_ceil, shape_f64_div, shape_f64_floor, shape_f64_max, shape_f64_min,
    shape_f64_mul, shape_f64_nearest, shape_f64_neg, shape_f64_pmax, shape_f64_pmin,
    shape_f64_sqrt, shape_f64_sub, shape_f64_trunc, shape_ge_s, shape_ge_u, shape_gt_s, shape_gt_u,
    shape_i16_abs, shape_i16_add, shape_i16_avgr_u, shape_i16_max_s, shape_i16_max_u,
    shape_i16_min_s, shape_i16_min_u, shape_i16_mul, shape_i16_mulr_sat_s, shape_i16_neg,
    shape_i16_sat_add_s, shape_i16_sat_add_u, shape_i16_sat_sub_s, shape_i16_sat_sub_u,
    shape_i16_sub, shape_i32_abs, shape_i32_add, shape_i32_max_s, shape_i32_max_u, shape_i32_min_s,
    shape_i32_min_u, shape_i32_mul, shape_i32_neg, shape_i32_sub, shape_i64_abs, shape_i64_add,
    shape_i64_mul, shape_i64_neg, shape_i64_sub, shape_i8_abs, shape_i8_add, shape_i8_avgr_u,
    shape_i8_max_s, shape_i8_max_u, shape_i8_min_s, shape_i8_min_u, shape_i8_neg, shape_i8_popcnt,
    shape_i8_sat_add_s, shape_i8_sat_add_u, shape_i8_sat_sub_s, shape_i8_sat_sub_u, shape_i8_sub,
    shape_le_s, shape_le_u, shape_lt_s, shape_lt_u, shape_ne, shapef_ge, shapef_gt, shapef_le,
    shapef_lt, shiftop_16x8, shiftop_32x4, shiftop_64x2, shiftop_8x16, unop_16x8, unop_32x4,
    unop_64x2, unop_8x16, v128_and, v128_andnot, v128_anytrue, v128_or, v128_xor, vternop, vvunop,
};
use self::exec_vector::{
    all_true_16x8, all_true_32x4, all_true_64x2, all_true_8x16, bitmask_16x8, bitmask_32x4,
    bitmask_64x2, bitmask_8x16, f32_promote_f64, f32x4_vcvtop_i32x4, i16_extend_i32_s,
    i16_extend_i32_u, i16x8_extadd_pairwise_i8x16, i16x8_extmul_half_i8x16,
    i16x8_vcvtop_half_i8x16, i32_convert_f64_s, i32_convert_f64_u, i32_extend_i64_s,
    i32_extend_i64_u, i32x4_dot_i16x8s, i32x4_extadd_pairwise_i16x8, i32x4_extmul_half_i16x8,
    i32x4_vcvtop_f32x4, i32x4_vcvtop_half_i16x8, i64x2_extmul_half_i32x4, i64x2_vcvtop_half_i32x4,
    i8_extend_i16_s, i8_extend_i16_u, shape_16x8_narrow_32x4_s, shape_16x8_narrow_32x4_u,
    shape_32x4_vcvtop_64x2_zero, shape_8x16_narrow_16x8_s, shape_8x16_narrow_16x8_u,
    shape_f32_convert_i32_s, shape_f32_convert_i32_u, shape_f32_demote_f64, shape_i32_trunc_f32_s,
    shape_i32_trunc_f32_u, shape_i32_trunc_f64_s, shape_i32_trunc_f64_u, Half,
};
use self::executor::ExitType;

#[allow(dead_code)]
pub fn execute_expression(
    expr: &ExpressionType,
    stack: &mut Stack,
    store: &mut Store,
) -> RResult<Val> {
    // let frame = match stack.last().cloned() {
    //     Some(StackEntry::Frame(frame)) => frame,
    //     _ => return Err(Trap),
    // };

    for ref instr in &expr.instructions {
        execute_instruction(instr, stack, store)?;
    }

    // TODO: recheck what does it mean
    stack.pop_value().ok_or(Trap)
}

// TODO: rewrite remaining singed instructions using AsSigned trait
pub fn execute_instruction(
    instr: &InstructionType,
    stack: &mut Stack,
    store: &mut Store,
    // frame_ref: &Frame,
) -> RResult<ExitType> {
    match instr {
        InstructionType::I32Const(I32Type(num_val)) => i32_const(num_val, stack)?,
        InstructionType::I64Const(I64Type(num_val)) => i64_const(num_val, stack)?,
        InstructionType::F32Const(F32Type(num_val)) => f32_const(num_val, stack)?,
        InstructionType::F64Const(F64Type(num_val)) => f64_const(num_val, stack)?,
        InstructionType::V128Const(v128) => v128_const(v128, stack)?,
        // iunop
        InstructionType::I32Clz => i32_clz(stack)?,
        InstructionType::I64Clz => i64_clz(stack)?,
        InstructionType::I32Ctz => i32_ctz(stack)?,
        InstructionType::I64Ctz => i64_ctz(stack)?,
        InstructionType::I32Popcnt => i32_popcnt(stack)?,
        InstructionType::I64Popcnt => i64_popcnt(stack)?,
        // funop
        InstructionType::F32Abs => f32_abs(stack)?,
        InstructionType::F64Abs => f64_abs(stack)?,
        InstructionType::F32Neg => f32_neg(stack)?,
        InstructionType::F64Neg => f64_neg(stack)?,
        InstructionType::F32Sqrt => f32_sqrt(stack)?,
        InstructionType::F64Sqrt => f64_sqrt(stack)?,
        InstructionType::F32Ceil => f32_ceil(stack)?,
        InstructionType::F64Ceil => f64_ceil(stack)?,
        InstructionType::F32Floor => f32_floor(stack)?,
        InstructionType::F64Floor => f64_floor(stack)?,
        InstructionType::F32Trunc => f32_trunc(stack)?,
        InstructionType::F64Trunc => f64_trunc(stack)?,
        InstructionType::F32Nearest => f32_nearest(stack)?,
        InstructionType::F64Nearest => f64_nearest(stack)?,
        // extendN_s
        InstructionType::I32Extend8S => i32_extend_8s(stack)?,
        InstructionType::I32Extend16S => i32_extend_16s(stack)?,
        InstructionType::I64Extend8S => i64_extend_8s(stack)?,
        InstructionType::I64Extend16S => i64_extend_16s(stack)?,
        InstructionType::I64Extend32S => i64_extend_32s(stack)?,
        // binop
        InstructionType::I32Add => i32_add(stack)?,
        InstructionType::I64Add => i64_add(stack)?,
        InstructionType::I32Sub => i32_sub(stack)?,
        InstructionType::I64Sub => i64_sub(stack)?,
        InstructionType::I32Mul => i32_mul(stack)?,
        InstructionType::I64Mul => i64_mul(stack)?,
        InstructionType::I32DivU => i32_div_u(stack)?,
        InstructionType::I32DivS => i32_div_s(stack)?,
        InstructionType::I64DivU => i64_div_u(stack)?,
        InstructionType::I64DivS => i64_div_s(stack)?,
        InstructionType::I32RemU => i32_rem_u(stack)?,
        InstructionType::I32RemS => i32_rem_s(stack)?,
        InstructionType::I64RemU => i64_rem_u(stack)?,
        InstructionType::I64RemS => i64_rem_s(stack)?,
        InstructionType::I32And => i32_and(stack)?,
        InstructionType::I64And => i64_and(stack)?,
        InstructionType::I32Or => i32_or(stack)?,
        InstructionType::I64Or => i64_or(stack)?,
        InstructionType::I32Xor => i32_xor(stack)?,
        InstructionType::I64Xor => i64_xor(stack)?,
        InstructionType::I32Shl => i32_shl(stack)?,
        InstructionType::I64Shl => i64_shl(stack)?,
        InstructionType::I32ShrU => i32_shr_u(stack)?,
        InstructionType::I64ShrU => i64_shr_u(stack)?,
        InstructionType::I32ShrS => i32_shr_s(stack)?,
        InstructionType::I64ShrS => i64_shr_s(stack)?,
        InstructionType::I32Rotl => i32_rotl(stack)?,
        InstructionType::I64Rotl => i64_rotl(stack)?,
        InstructionType::I32Rotr => i32_rotr(stack)?,
        InstructionType::I64Rotr => i64_rotr(stack)?,
        // fbinop
        InstructionType::F32Add => f32_add(stack)?,
        InstructionType::F64Add => f64_add(stack)?,
        InstructionType::F32Sub => f32_sub(stack)?,
        InstructionType::F64Sub => f64_sub(stack)?,
        InstructionType::F32Mul => f32_mul(stack)?,
        InstructionType::F64Mul => f64_mul(stack)?,
        InstructionType::F32Div => f32_div(stack)?,
        InstructionType::F64Div => f64_div(stack)?,
        InstructionType::F32Min => f32_min(stack)?,
        InstructionType::F64Min => f64_min(stack)?,
        InstructionType::F32Max => f32_max(stack)?,
        InstructionType::F64Max => f64_max(stack)?,
        InstructionType::F32Copysign => f32_copysign(stack)?,
        InstructionType::F64Copysign => f64_copysign(stack)?,
        // testop
        InstructionType::I32Eqz => i32_testop(eqz, stack)?,
        InstructionType::I64Eqz => i64_testop(eqz, stack)?,
        // relop
        InstructionType::I32Eq => i32_relop(eq, stack)?,
        InstructionType::I64Eq => i64_relop(eq, stack)?,
        InstructionType::I32Ne => i32_relop(neq, stack)?,
        InstructionType::I64Ne => i64_relop(neq, stack)?,
        InstructionType::I32LtS => i32_relop(lts, stack)?,
        InstructionType::I64LtS => i64_relop(lts, stack)?,
        InstructionType::I32LtU => i32_relop(ltu, stack)?,
        InstructionType::I64LtU => i64_relop(ltu, stack)?,
        InstructionType::I32GtS => i32_relop(gts, stack)?,
        InstructionType::I64GtS => i64_relop(gts, stack)?,
        InstructionType::I32GtU => i32_relop(gtu, stack)?,
        InstructionType::I64GtU => i64_relop(gtu, stack)?,
        InstructionType::I32LeS => i32_relop(les, stack)?,
        InstructionType::I64LeS => i64_relop(les, stack)?,
        InstructionType::I32LeU => i32_relop(leu, stack)?,
        InstructionType::I64LeU => i64_relop(leu, stack)?,
        InstructionType::I32GeS => i32_relop(ges, stack)?,
        InstructionType::I64GeS => i64_relop(ges, stack)?,
        InstructionType::I32GeU => i32_relop(geu, stack)?,
        InstructionType::I64GeU => i64_relop(geu, stack)?,
        InstructionType::F32Eq => f32_relop(eq, stack)?,
        InstructionType::F64Eq => f64_relop(eq, stack)?,
        InstructionType::F32Ne => f32_relop(neq, stack)?,
        InstructionType::F64Ne => f64_relop(neq, stack)?,
        InstructionType::F32Lt => f32_relop(ltu, stack)?,
        InstructionType::F64Lt => f64_relop(ltu, stack)?,
        InstructionType::F32Gt => f32_relop(gtu, stack)?,
        InstructionType::F64Gt => f64_relop(gtu, stack)?,
        InstructionType::F32Le => f32_relop(leu, stack)?,
        InstructionType::F64Le => f64_relop(leu, stack)?,
        InstructionType::F32Ge => f32_relop(geu, stack)?,
        InstructionType::F64Ge => f64_relop(geu, stack)?,
        // cvtop
        InstructionType::I32WrapI64 => i32_wrap_i64(stack)?,
        InstructionType::I32TruncF32U => i32_trunc_f32_u(stack)?,
        InstructionType::I32TruncF64U => i32_trunc_f64_u(stack)?,
        InstructionType::I32TruncF32S => i32_trunc_f32_s(stack)?,
        InstructionType::I32TruncF64S => i32_trunc_f64_s(stack)?,
        InstructionType::I64TruncF32U => i64_trunc_f32_u(stack)?,
        InstructionType::I64TruncF64U => i64_trunc_f64_u(stack)?,
        InstructionType::I64TruncF32S => i64_trunc_f32_s(stack)?,
        InstructionType::I64TruncF64S => i64_trunc_f64_s(stack)?,
        InstructionType::I32TruncSatF32U => i32_trunc_sat_f32_u(stack)?,
        InstructionType::I32TruncSatF64U => i32_trunc_sat_f64_u(stack)?,
        InstructionType::I32TruncSatF32S => i32_trunc_sat_f32_s(stack)?,
        InstructionType::I32TruncSatF64S => i32_trunc_sat_f64_s(stack)?,
        InstructionType::I64TruncSatF32U => i64_trunc_sat_f32_u(stack)?,
        InstructionType::I64TruncSatF64U => i64_trunc_sat_f64_u(stack)?,
        InstructionType::I64TruncSatF32S => i64_trunc_sat_f32_s(stack)?,
        InstructionType::I64TruncSatF64S => i64_trunc_sat_f64_s(stack)?,
        InstructionType::F32ConvertI32S => f32_convert_i32_s(stack)?,
        InstructionType::F32ConvertI32U => f32_convert_i32_u(stack)?,
        InstructionType::F32ConvertI64S => f32_convert_i64_s(stack)?,
        InstructionType::F32ConvertI64U => f32_convert_i64_u(stack)?,
        InstructionType::F64ConvertI32S => f64_convert_i32_s(stack)?,
        InstructionType::F64ConvertI32U => f64_convert_i32_u(stack)?,
        InstructionType::F64ConvertI64S => f64_convert_i64_s(stack)?,
        InstructionType::F64ConvertI64U => f64_convert_i64_u(stack)?,
        InstructionType::F32DemoteF64 => f32_demote_f64(stack)?,
        InstructionType::F64PromoteF32 => f64_promote_f32(stack)?,
        InstructionType::F32ReinterpretI32 => f32_reinterpret_i32(stack)?,
        InstructionType::F64ReinterpretI64 => f64_reinterpret_i64(stack)?,
        InstructionType::I32ReinterpretF32 => i32_reinterpret_f32(stack)?,
        InstructionType::I64ReinterpretF64 => i64_reinterpret_f64(stack)?,
        InstructionType::I64ExtendI32S => i64_extend_i32(stack, Sign::Signed)?,
        InstructionType::I64ExtendI32U => i64_extend_i32(stack, Sign::Unsigned)?,
        // reference instructions
        InstructionType::RefNull(ref_type) => ref_null(ref_type, stack)?,
        InstructionType::RefIsNull => is_ref_null(stack)?,
        InstructionType::RefFunc(FuncIdx(U32Type(func_idx))) => {
            ref_func(stack, *func_idx as usize)?
        }
        // vector instructions
        InstructionType::V128Not => {
            vvunop(stack, ::std::ops::Not::not)?;
        }
        InstructionType::V128And => v128_and(stack)?,
        InstructionType::V128AndNot => v128_andnot(stack)?,
        InstructionType::V128Or => v128_or(stack)?,
        InstructionType::V128Xor => v128_xor(stack)?,
        InstructionType::V128Bitselect => {
            vternop(stack, bitselect)?;
        }
        InstructionType::V128AnyTrue => v128_anytrue(stack)?,
        InstructionType::I8x16Swizzle => i8x16_swizzle(stack)?,
        InstructionType::I8x16Shuffle(lane_idx) => i8x16_shuffle(stack, lane_idx)?,
        InstructionType::I8x16Splat => i8x16_splat(stack)?,
        InstructionType::I16x8Splat => i16x8_splat(stack)?,
        InstructionType::I32x4Splat => i32x4_splat(stack)?,
        InstructionType::I64x2Splat => i64x2_splat(stack)?,
        InstructionType::F32x4Splat => f32x4_splat(stack)?,
        InstructionType::F64x2Splat => f64x2_splat(stack)?,
        InstructionType::I8x16ExtractLaneS(LaneIdx(lane_idx)) => {
            i8x16_extract_lane_s(stack, *lane_idx)?
        }
        InstructionType::I8x16ExtractLaneU(LaneIdx(lane_idx)) => {
            i8x16_extract_lane_u(stack, *lane_idx)?
        }
        InstructionType::I16x8ExtractLaneS(LaneIdx(lane_idx)) => {
            i16x8_extract_lane_s(stack, *lane_idx)?
        }
        InstructionType::I16x8ExtractLaneU(LaneIdx(lane_idx)) => {
            i16x8_extract_lane_u(stack, *lane_idx)?
        }
        InstructionType::I32x4ExtractLane(LaneIdx(lane_idx)) => {
            i32x4_extract_lane(stack, *lane_idx)?
        }
        InstructionType::I64x2ExtractLane(LaneIdx(lane_idx)) => {
            i64x2_extract_lane(stack, *lane_idx)?
        }
        InstructionType::F32x4ExtractLane(LaneIdx(lane_idx)) => {
            f32x4_extract_lane(stack, *lane_idx)?
        }
        InstructionType::F64x2ExtractLane(LaneIdx(lane_idx)) => {
            f64x2_extract_lane(stack, *lane_idx)?
        }
        InstructionType::I8x16ReplaceLane(LaneIdx(lane_idx)) => {
            i8x16_replace_lane(stack, *lane_idx)?
        }
        InstructionType::I16x8ReplaceLane(LaneIdx(lane_idx)) => {
            i16x8_replace_lane(stack, *lane_idx)?
        }
        InstructionType::I32x4ReplaceLane(LaneIdx(lane_idx)) => {
            i32x4_replace_lane(stack, *lane_idx)?
        }
        InstructionType::I64x2ReplaceLane(LaneIdx(lane_idx)) => {
            i64x2_replace_lane(stack, *lane_idx)?
        }
        InstructionType::F32x4ReplaceLane(LaneIdx(lane_idx)) => {
            f32x4_replace_lane(stack, *lane_idx)?
        }
        InstructionType::F64x2ReplaceLane(LaneIdx(lane_idx)) => {
            f64x2_replace_lane(stack, *lane_idx)?
        }
        // shape.vunop
        InstructionType::I8x16Abs => unop_8x16(stack, shape_i8_abs)?,
        InstructionType::I16x8Abs => unop_16x8(stack, shape_i16_abs)?,
        InstructionType::I32x4Abs => unop_32x4(stack, shape_i32_abs)?,
        InstructionType::I64x2Abs => unop_64x2(stack, shape_i64_abs)?,
        InstructionType::F32x4Abs => unop_32x4(stack, shape_f32_abs)?,
        InstructionType::F64x2Abs => unop_64x2(stack, shape_f64_abs)?,
        InstructionType::I8x16Neg => unop_8x16(stack, shape_i8_neg)?,
        InstructionType::I16x8Neg => unop_16x8(stack, shape_i16_neg)?,
        InstructionType::I32x4Neg => unop_32x4(stack, shape_i32_neg)?,
        InstructionType::I64x2Neg => unop_64x2(stack, shape_i64_neg)?,
        InstructionType::F32x4Neg => unop_32x4(stack, shape_f32_neg)?,
        InstructionType::F64x2Neg => unop_64x2(stack, shape_f64_neg)?,
        InstructionType::F32x4Sqrt => unop_32x4(stack, shape_f32_sqrt)?,
        InstructionType::F64x2Sqrt => unop_64x2(stack, shape_f64_sqrt)?,
        InstructionType::F32x4Ceil => unop_32x4(stack, shape_f32_ceil)?,
        InstructionType::F64x2Ceil => unop_64x2(stack, shape_f64_ceil)?,
        InstructionType::F32x4Floor => unop_32x4(stack, shape_f32_floor)?,
        InstructionType::F64x2Floor => unop_64x2(stack, shape_f64_floor)?,
        InstructionType::F32x4Trunc => unop_32x4(stack, shape_f32_trunc)?,
        InstructionType::F64x2Trunc => unop_64x2(stack, shape_f64_trunc)?,
        InstructionType::F32x4Nearest => unop_32x4(stack, shape_f32_nearest)?,
        InstructionType::F64x2Nearest => unop_64x2(stack, shape_f64_nearest)?,
        InstructionType::I8x16Popcnt => unop_8x16(stack, shape_i8_popcnt)?,
        // shape.vbinop
        InstructionType::I8x16Add => binop_8x16(stack, shape_i8_add)?,
        InstructionType::I16x8Add => binop_16x8(stack, shape_i16_add)?,
        InstructionType::I32x4Add => binop_32x4(stack, shape_i32_add)?,
        InstructionType::I64x2Add => binop_64x2(stack, shape_i64_add)?,
        InstructionType::I8x16Sub => binop_8x16(stack, shape_i8_sub)?,
        InstructionType::I16x8Sub => binop_16x8(stack, shape_i16_sub)?,
        InstructionType::I32x4Sub => binop_32x4(stack, shape_i32_sub)?,
        InstructionType::I64x2Sub => binop_64x2(stack, shape_i64_sub)?,
        InstructionType::F32x4Add => binop_32x4(stack, shape_f32_add)?,
        InstructionType::F64x2Add => binop_64x2(stack, shape_f64_add)?,
        InstructionType::F32x4Sub => binop_32x4(stack, shape_f32_sub)?,
        InstructionType::F64x2Sub => binop_64x2(stack, shape_f64_sub)?,
        InstructionType::I16x8Mul => binop_16x8(stack, shape_i16_mul)?,
        InstructionType::I32x4Mul => binop_32x4(stack, shape_i32_mul)?,
        InstructionType::I64x2Mul => binop_64x2(stack, shape_i64_mul)?,
        InstructionType::F32x4Mul => binop_32x4(stack, shape_f32_mul)?,
        InstructionType::F64x2Mul => binop_64x2(stack, shape_f64_mul)?,
        InstructionType::F32x4Div => binop_32x4(stack, shape_f32_div)?,
        InstructionType::F64x2Div => binop_64x2(stack, shape_f64_div)?,
        InstructionType::F32x4Min => binop_32x4(stack, shape_f32_min)?,
        InstructionType::F64x2Min => binop_64x2(stack, shape_f64_min)?,
        InstructionType::F32x4Max => binop_32x4(stack, shape_f32_max)?,
        InstructionType::F64x2Max => binop_64x2(stack, shape_f64_max)?,
        InstructionType::F32x4Pmin => binop_32x4(stack, shape_f32_pmin)?,
        InstructionType::F64x2Pmin => binop_64x2(stack, shape_f64_pmin)?,
        InstructionType::F32x4Pmax => binop_32x4(stack, shape_f32_pmax)?,
        InstructionType::F64x2Pmax => binop_64x2(stack, shape_f64_pmax)?,
        InstructionType::I8x16MinU => binop_8x16(stack, shape_i8_min_u)?,
        InstructionType::I8x16MinS => binop_8x16(stack, shape_i8_min_s)?,
        InstructionType::I16x8MinU => binop_16x8(stack, shape_i16_min_u)?,
        InstructionType::I16x8MinS => binop_16x8(stack, shape_i16_min_s)?,
        InstructionType::I32x4MinU => binop_32x4(stack, shape_i32_min_u)?,
        InstructionType::I32x4MinS => binop_32x4(stack, shape_i32_min_s)?,
        InstructionType::I8x16MaxU => binop_8x16(stack, shape_i8_max_u)?,
        InstructionType::I8x16MaxS => binop_8x16(stack, shape_i8_max_s)?,
        InstructionType::I16x8MaxU => binop_16x8(stack, shape_i16_max_u)?,
        InstructionType::I16x8MaxS => binop_16x8(stack, shape_i16_max_s)?,
        InstructionType::I32x4MaxU => binop_32x4(stack, shape_i32_max_u)?,
        InstructionType::I32x4MaxS => binop_32x4(stack, shape_i32_max_s)?,
        InstructionType::I8x16AddSatU => binop_8x16(stack, shape_i8_sat_add_u)?,
        InstructionType::I16x8AddSatU => binop_16x8(stack, shape_i16_sat_add_u)?,
        InstructionType::I8x16AddSatS => binop_8x16(stack, shape_i8_sat_add_s)?,
        InstructionType::I16x8AddSatS => binop_16x8(stack, shape_i16_sat_add_s)?,
        InstructionType::I8x16SubSatU => binop_8x16(stack, shape_i8_sat_sub_u)?,
        InstructionType::I16x8SubSatU => binop_16x8(stack, shape_i16_sat_sub_u)?,
        InstructionType::I8x16SubSatS => binop_8x16(stack, shape_i8_sat_sub_s)?,
        InstructionType::I16x8SubSatS => binop_16x8(stack, shape_i16_sat_sub_s)?,
        InstructionType::I8x16AvgrU => binop_8x16(stack, shape_i8_avgr_u)?,
        InstructionType::I16x8AvgrU => binop_16x8(stack, shape_i16_avgr_u)?,
        InstructionType::I16x8Q15MulrSatS => binop_16x8(stack, shape_i16_mulr_sat_s)?,
        InstructionType::I8x16Eq => relop_8x16(stack, shape_eq)?,
        InstructionType::I16x8Eq => relop_16x8(stack, shape_eq)?,
        InstructionType::I32x4Eq => relop_32x4(stack, shape_eq)?,
        InstructionType::I64x2Eq => relop_64x2(stack, shape_eq)?,
        InstructionType::I8x16Ne => relop_8x16(stack, shape_ne)?,
        InstructionType::I16x8Ne => relop_16x8(stack, shape_ne)?,
        InstructionType::I32x4Ne => relop_32x4(stack, shape_ne)?,
        InstructionType::I64x2Ne => relop_64x2(stack, shape_ne)?,
        InstructionType::I8x16LtU => relop_8x16(stack, shape_lt_u)?,
        InstructionType::I16x8LtU => relop_16x8(stack, shape_lt_u)?,
        InstructionType::I32x4LtU => relop_32x4(stack, shape_lt_u)?,
        InstructionType::I8x16LtS => relop_8x16(stack, shape_lt_s)?,
        InstructionType::I16x8LtS => relop_16x8(stack, shape_lt_s)?,
        InstructionType::I32x4LtS => relop_32x4(stack, shape_lt_s)?,
        InstructionType::I64x2LtS => relop_64x2(stack, shape_lt_s)?,
        InstructionType::I8x16GtU => relop_8x16(stack, shape_gt_u)?,
        InstructionType::I16x8GtU => relop_16x8(stack, shape_gt_u)?,
        InstructionType::I32x4GtU => relop_32x4(stack, shape_gt_u)?,
        InstructionType::I8x16GtS => relop_8x16(stack, shape_gt_s)?,
        InstructionType::I16x8GtS => relop_16x8(stack, shape_gt_s)?,
        InstructionType::I32x4GtS => relop_32x4(stack, shape_gt_s)?,
        InstructionType::I64x2GtS => relop_64x2(stack, shape_gt_s)?,
        InstructionType::I8x16LeU => relop_8x16(stack, shape_le_u)?,
        InstructionType::I16x8LeU => relop_16x8(stack, shape_le_u)?,
        InstructionType::I32x4LeU => relop_32x4(stack, shape_le_u)?,
        InstructionType::I8x16LeS => relop_8x16(stack, shape_le_s)?,
        InstructionType::I16x8LeS => relop_16x8(stack, shape_le_s)?,
        InstructionType::I32x4LeS => relop_32x4(stack, shape_le_s)?,
        InstructionType::I64x2LeS => relop_64x2(stack, shape_le_s)?,
        InstructionType::I8x16GeU => relop_8x16(stack, shape_ge_u)?,
        InstructionType::I16x8GeU => relop_16x8(stack, shape_ge_u)?,
        InstructionType::I32x4GeU => relop_32x4(stack, shape_ge_u)?,
        InstructionType::I8x16GeS => relop_8x16(stack, shape_ge_s)?,
        InstructionType::I16x8GeS => relop_16x8(stack, shape_ge_s)?,
        InstructionType::I32x4GeS => relop_32x4(stack, shape_ge_s)?,
        InstructionType::I64x2GeS => relop_64x2(stack, shape_ge_s)?,
        InstructionType::F32x4Eq => relop_32x4(stack, shape_eq)?,
        InstructionType::F64x2Eq => relop_64x2(stack, shape_eq)?,
        InstructionType::F32x4Ne => relop_32x4(stack, shape_ne)?,
        InstructionType::F64x2Ne => relop_64x2(stack, shape_ne)?,
        InstructionType::F32x4Lt => relop_32x4(stack, shapef_lt)?,
        InstructionType::F64x2Lt => relop_64x2(stack, shapef_lt)?,
        InstructionType::F32x4Gt => relop_32x4(stack, shapef_gt)?,
        InstructionType::F64x2Gt => relop_64x2(stack, shapef_gt)?,
        InstructionType::F32x4Le => relop_32x4(stack, shapef_le)?,
        InstructionType::F64x2Le => relop_64x2(stack, shapef_le)?,
        InstructionType::F32x4Ge => relop_32x4(stack, shapef_ge)?,
        InstructionType::F64x2Ge => relop_64x2(stack, shapef_ge)?,
        InstructionType::I8x16Shl => shiftop_8x16(stack, make_shape_shl(8))?,
        InstructionType::I16x8Shl => shiftop_16x8(stack, make_shape_shl(16))?,
        InstructionType::I32x4Shl => shiftop_32x4(stack, make_shape_shl(32))?,
        InstructionType::I64x2Shl => shiftop_64x2(stack, make_shape_shl(64))?,
        InstructionType::I8x16ShrU => shiftop_8x16(stack, make_shape_shr_u(8))?,
        InstructionType::I16x8ShrU => shiftop_16x8(stack, make_shape_shr_u(16))?,
        InstructionType::I32x4ShrU => shiftop_32x4(stack, make_shape_shr_u(32))?,
        InstructionType::I64x2ShrU => shiftop_64x2(stack, make_shape_shr_u(64))?,
        InstructionType::I8x16ShrS => shiftop_8x16(stack, make_shape_shr_s(8))?,
        InstructionType::I16x8ShrS => shiftop_16x8(stack, make_shape_shr_s(16))?,
        InstructionType::I32x4ShrS => shiftop_32x4(stack, make_shape_shr_s(32))?,
        InstructionType::I64x2ShrS => shiftop_64x2(stack, make_shape_shr_s(64))?,
        InstructionType::I8x16AllTrue => all_true_8x16(stack)?,
        InstructionType::I16x8AllTrue => all_true_16x8(stack)?,
        InstructionType::I32x4AllTrue => all_true_32x4(stack)?,
        InstructionType::I64x2AllTrue => all_true_64x2(stack)?,
        InstructionType::I8x16Bitmask => bitmask_8x16(stack)?,
        InstructionType::I16x8Bitmask => bitmask_16x8(stack)?,
        InstructionType::I32x4Bitmask => bitmask_32x4(stack)?,
        InstructionType::I64x2Bitmask => bitmask_64x2(stack)?,
        InstructionType::I8x16NarrowI16x8U => shape_8x16_narrow_16x8_u(stack)?,
        InstructionType::I8x16NarrowI16x8S => shape_8x16_narrow_16x8_s(stack)?,
        InstructionType::I16x8NarrowI32x4U => shape_16x8_narrow_32x4_u(stack)?,
        InstructionType::I16x8NarrowI32x4S => shape_16x8_narrow_32x4_s(stack)?,
        InstructionType::I32x4TruncSatF32x4S => i32x4_vcvtop_f32x4(stack, shape_i32_trunc_f32_s)?,
        InstructionType::I32x4TruncSatF32x4U => i32x4_vcvtop_f32x4(stack, shape_i32_trunc_f32_u)?,
        InstructionType::F32x4ConvertI32x4S => f32x4_vcvtop_i32x4(stack, shape_f32_convert_i32_s)?,
        InstructionType::F32x4ConvertI32x4U => f32x4_vcvtop_i32x4(stack, shape_f32_convert_i32_u)?,
        InstructionType::I16x8ExtendLowI8x16U => {
            i16x8_vcvtop_half_i8x16(stack, i8_extend_i16_u, Half::Low)?
        }
        InstructionType::I16x8ExtendLowI8x16S => {
            i16x8_vcvtop_half_i8x16(stack, i8_extend_i16_s, Half::Low)?
        }
        InstructionType::I16x8ExtendHighI8x16U => {
            i16x8_vcvtop_half_i8x16(stack, i8_extend_i16_u, Half::High)?
        }
        InstructionType::I16x8ExtendHighI8x16S => {
            i16x8_vcvtop_half_i8x16(stack, i8_extend_i16_s, Half::High)?
        }
        InstructionType::I32x4ExtendLowI16x8U => {
            i32x4_vcvtop_half_i16x8(stack, i16_extend_i32_u, Half::Low)?
        }
        InstructionType::I32x4ExtendLowI16x8S => {
            i32x4_vcvtop_half_i16x8(stack, i16_extend_i32_s, Half::Low)?
        }
        InstructionType::I32x4ExtendHighI16x8U => {
            i32x4_vcvtop_half_i16x8(stack, i16_extend_i32_u, Half::High)?
        }
        InstructionType::I32x4ExtendHighI16x8S => {
            i32x4_vcvtop_half_i16x8(stack, i16_extend_i32_s, Half::High)?
        }
        InstructionType::I64x2ExtendLowI32x4U => {
            i64x2_vcvtop_half_i32x4(stack, i32_extend_i64_u, Half::Low)?
        }
        InstructionType::I64x2ExtendLowI32x4S => {
            i64x2_vcvtop_half_i32x4(stack, i32_extend_i64_s, Half::Low)?
        }
        InstructionType::I64x2ExtendHighI32x4U => {
            i64x2_vcvtop_half_i32x4(stack, i32_extend_i64_u, Half::High)?
        }
        InstructionType::I64x2ExtendHighI32x4S => {
            i64x2_vcvtop_half_i32x4(stack, i32_extend_i64_s, Half::High)?
        }
        InstructionType::F64x2ConvertLowI32x4U => {
            i64x2_vcvtop_half_i32x4(stack, i32_convert_f64_u, Half::Low)?
        }
        InstructionType::F64x2ConvertLowI32x4S => {
            i64x2_vcvtop_half_i32x4(stack, i32_convert_f64_s, Half::Low)?
        }
        InstructionType::F64x2PromoteLowF32x4 => {
            i64x2_vcvtop_half_i32x4(stack, f32_promote_f64, Half::Low)?
        }
        InstructionType::I32x4TruncSatF64x2UZero => {
            shape_32x4_vcvtop_64x2_zero(stack, shape_i32_trunc_f64_u)?
        }
        InstructionType::I32x4TruncSatF64x2SZero => {
            shape_32x4_vcvtop_64x2_zero(stack, shape_i32_trunc_f64_s)?
        }
        InstructionType::F32x4DemoteF64x2Zero => {
            shape_32x4_vcvtop_64x2_zero(stack, shape_f32_demote_f64)?
        }
        InstructionType::I32x4DotI16x8S => i32x4_dot_i16x8s(stack)?,
        InstructionType::I16x8ExtmulLowI8x16U => i16x8_extmul_half_i8x16(stack, Half::Low, false)?,
        InstructionType::I16x8ExtmulLowI8x16S => i16x8_extmul_half_i8x16(stack, Half::Low, true)?,
        InstructionType::I16x8ExtmulHighI8x16U => {
            i16x8_extmul_half_i8x16(stack, Half::High, false)?
        }
        InstructionType::I16x8ExtmulHighI8x16S => i16x8_extmul_half_i8x16(stack, Half::High, true)?,
        InstructionType::I32x4ExtmulLowI16x8U => i32x4_extmul_half_i16x8(stack, Half::Low, false)?,
        InstructionType::I32x4ExtmulLowI16x8S => i32x4_extmul_half_i16x8(stack, Half::Low, true)?,
        InstructionType::I32x4ExtmulHighI16x8U => {
            i32x4_extmul_half_i16x8(stack, Half::High, false)?
        }
        InstructionType::I32x4ExtmulHighI16x8S => i32x4_extmul_half_i16x8(stack, Half::High, true)?,
        InstructionType::I64x2ExtmulLowI32x4U => i64x2_extmul_half_i32x4(stack, Half::Low, false)?,
        InstructionType::I64x2ExtmulLowI32x4S => i64x2_extmul_half_i32x4(stack, Half::Low, true)?,
        InstructionType::I64x2ExtmulHighI32x4U => {
            i64x2_extmul_half_i32x4(stack, Half::High, false)?
        }
        InstructionType::I64x2ExtmulHighI32x4S => i64x2_extmul_half_i32x4(stack, Half::High, true)?,
        InstructionType::I16x8ExtaddPairwiseI8x16U => i16x8_extadd_pairwise_i8x16(stack, false)?,
        InstructionType::I16x8ExtaddPairwiseI8x16S => i16x8_extadd_pairwise_i8x16(stack, true)?,
        InstructionType::I32x4ExtaddPairwiseI16x8U => i32x4_extadd_pairwise_i16x8(stack, false)?,
        InstructionType::I32x4ExtaddPairwiseI16x8S => i32x4_extadd_pairwise_i16x8(stack, true)?,

        // parametric instructions
        InstructionType::Drop => exec_drop(stack)?,
        InstructionType::Select => exec_select(stack)?,
        InstructionType::SelectVec(vector) => exec_select_vec(stack, vector)?,

        // variable instructions
        InstructionType::LocalGet(local_idx) => local_get(stack, local_idx)?,
        InstructionType::LocalSet(local_idx) => local_set(stack, local_idx)?,
        InstructionType::LocalTee(local_idx) => local_tee(stack, local_idx)?,
        InstructionType::GlobalGet(global_idx) => global_get(stack, store, global_idx)?,
        InstructionType::GlobalSet(global_idx) => global_set(stack, store, global_idx)?,

        // table instructions
        InstructionType::TableGet(table_idx) => table_get(stack, store, table_idx)?,
        InstructionType::TableSet(table_idx) => table_set(stack, store, table_idx)?,
        InstructionType::TableSize(table_idx) => table_size(stack, store, table_idx)?,
        InstructionType::TableGrow(table_idx) => table_grow(stack, store, table_idx)?,
        InstructionType::TableFill(table_idx) => table_fill(stack, store, table_idx)?,
        InstructionType::TableCopy(table_idxes) => table_copy(stack, store, table_idxes)?,
        InstructionType::TableInit(table_idxes) => table_init(stack, store, table_idxes)?,
        InstructionType::ElemDrop(elem_idx) => elem_drop(stack, store, elem_idx)?,

        // memory instructions
        InstructionType::I32Load(mem_arg) => i32_load(stack, store, mem_arg)?,
        InstructionType::I64Load(mem_arg) => i64_load(stack, store, mem_arg)?,
        InstructionType::F32Load(mem_arg) => f32_load(stack, store, mem_arg)?,
        InstructionType::F64Load(mem_arg) => f64_load(stack, store, mem_arg)?,
        InstructionType::V128Load(mem_arg) => v128_load(stack, store, mem_arg)?,
        InstructionType::I32Load8U(mem_arg) => i32_load_8(stack, store, mem_arg, Sign::Unsigned)?,
        InstructionType::I32Load8S(mem_arg) => i32_load_8(stack, store, mem_arg, Sign::Signed)?,
        InstructionType::I32Load16U(mem_arg) => i32_load_16(stack, store, mem_arg, Sign::Unsigned)?,
        InstructionType::I32Load16S(mem_arg) => i32_load_16(stack, store, mem_arg, Sign::Signed)?,
        InstructionType::I64Load8U(mem_arg) => i64_load_8(stack, store, mem_arg, Sign::Unsigned)?,
        InstructionType::I64Load8S(mem_arg) => i64_load_8(stack, store, mem_arg, Sign::Signed)?,
        InstructionType::I64Load16U(mem_arg) => i64_load_16(stack, store, mem_arg, Sign::Unsigned)?,
        InstructionType::I64Load16S(mem_arg) => i64_load_16(stack, store, mem_arg, Sign::Signed)?,
        InstructionType::I64Load32U(mem_arg) => i64_load_32(stack, store, mem_arg, Sign::Unsigned)?,
        InstructionType::I64Load32S(mem_arg) => i64_load_32(stack, store, mem_arg, Sign::Signed)?,
        InstructionType::V128Load8x8U(mem_arg) => {
            v128_load_8x8(stack, store, mem_arg, Sign::Unsigned)?
        }
        InstructionType::V128Load8x8S(mem_arg) => {
            v128_load_8x8(stack, store, mem_arg, Sign::Signed)?
        }
        InstructionType::V128Load16x4U(mem_arg) => {
            v128_load_16x4(stack, store, mem_arg, Sign::Unsigned)?
        }
        InstructionType::V128Load16x4S(mem_arg) => {
            v128_load_16x4(stack, store, mem_arg, Sign::Signed)?
        }
        InstructionType::V128Load32x2U(mem_arg) => {
            v128_load_32x2(stack, store, mem_arg, Sign::Unsigned)?
        }
        InstructionType::V128Load32x2S(mem_arg) => {
            v128_load_32x2(stack, store, mem_arg, Sign::Signed)?
        }
        InstructionType::V128Load8Splat(mem_arg) => v128_load8_splat(stack, store, mem_arg)?,
        InstructionType::V128Load16Splat(mem_arg) => v128_load16_splat(stack, store, mem_arg)?,
        InstructionType::V128Load32Splat(mem_arg) => v128_load32_splat(stack, store, mem_arg)?,
        InstructionType::V128Load64Splat(mem_arg) => v128_load64_splat(stack, store, mem_arg)?,
        InstructionType::V128Load32Zero(mem_arg) => v128_load32_zero(stack, store, mem_arg)?,
        InstructionType::V128Load64Zero(mem_arg) => v128_load64_zero(stack, store, mem_arg)?,
        InstructionType::V128Load8Lane((mem_arg, lane_idx)) => {
            v128_load8_lane(stack, store, mem_arg, lane_idx)?
        }
        InstructionType::V128Load16Lane((mem_arg, lane_idx)) => {
            v128_load16_lane(stack, store, mem_arg, lane_idx)?
        }
        InstructionType::V128Load32Lane((mem_arg, lane_idx)) => {
            v128_load32_lane(stack, store, mem_arg, lane_idx)?
        }
        InstructionType::V128Load64Lane((mem_arg, lane_idx)) => {
            v128_load64_lane(stack, store, mem_arg, lane_idx)?
        }
        InstructionType::V128Store(mem_arg) => v128_store(stack, store, mem_arg)?,
        InstructionType::I32Store(mem_arg) => i32_store(stack, store, mem_arg)?,
        InstructionType::I64Store(mem_arg) => i64_store(stack, store, mem_arg)?,
        InstructionType::F32Store(mem_arg) => f32_store(stack, store, mem_arg)?,
        InstructionType::F64Store(mem_arg) => f64_store(stack, store, mem_arg)?,
        InstructionType::I32Store8(mem_arg) => i32_store8(stack, store, mem_arg)?,
        InstructionType::I64Store8(mem_arg) => i64_store8(stack, store, mem_arg)?,
        InstructionType::I32Store16(mem_arg) => i32_store16(stack, store, mem_arg)?,
        InstructionType::I64Store16(mem_arg) => i64_store16(stack, store, mem_arg)?,
        InstructionType::I64Store32(mem_arg) => i64_store32(stack, store, mem_arg)?,
        InstructionType::V128Store8Lane(arg) => v128_store8_lane(stack, store, arg)?,
        InstructionType::V128Store16Lane(arg) => v128_store16_lane(stack, store, arg)?,
        InstructionType::V128Store32Lane(arg) => v128_store32_lane(stack, store, arg)?,
        InstructionType::V128Store64Lane(arg) => v128_store64_lane(stack, store, arg)?,
        InstructionType::MemorySize => memory_size(stack, store)?,
        InstructionType::MemoryGrow => memory_grow(stack, store)?,
        InstructionType::MemoryFill => memory_fill(stack, store)?,
        InstructionType::MemoryCopy => memory_copy(stack, store)?,
        InstructionType::MemoryInit(data_idx) => memory_init(stack, store, data_idx)?,
        InstructionType::DataDrop(data_idx) => data_drop(stack, store, data_idx)?,

        // control instructions
        InstructionType::Nop => {}
        InstructionType::Unreachable => exec_unreachable()?,
        InstructionType::Block(block_instruction) => {
            return block(stack, store, block_instruction, execute_instruction);
        }
        InstructionType::Loop(loop_instruction) => {
            return exec_loop(stack, store, loop_instruction, execute_instruction);
        }
        InstructionType::IfElse(ifelse_instruction) => {
            return exec_ifelse(stack, store, ifelse_instruction, execute_instruction);
        }
        InstructionType::Br(label_idx) => {
            return exec_br(stack, store, label_idx, execute_instruction);
        }
        InstructionType::BrIf(label_idx) => {
            return exec_brif(stack, store, label_idx, execute_instruction);
        }
        InstructionType::BrTable(brtable_arg) => {
            return exec_brtable(stack, store, brtable_arg, execute_instruction);
        }
        InstructionType::Return => {
            return exec_return(stack);
        }
        InstructionType::Call(func_idx) => {
            return exec_call(stack, store, func_idx, execute_instruction);
        }
        InstructionType::CallIndirect(call_indirect_args) => {
            return exec_call_indirect(stack, store, call_indirect_args, execute_instruction);
        }
    };

    Ok(ExitType::Completed)
}

testop_impl!(i32_testop, Val::I32, u32);
testop_impl!(i64_testop, Val::I64, u64);

relop_impl!(i32_relop, Val::I32, u32);
relop_impl!(i64_relop, Val::I64, u64);
relop_impl!(f32_relop, Val::F32, f32);
relop_impl!(f64_relop, Val::F64, f64);
