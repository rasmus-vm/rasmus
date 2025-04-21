use std::ops::Neg;

use crate::entities::types::LaneIdx;

use crate::nearest;
use crate::result::{RResult, Trap};

use crate::instances::{
    stack::{Stack, StackEntry},
    value::Val,
};

use super::as_float_trait::AsFloat;
use super::as_signed_trait::AsSigned;
use super::exec_binop::{
    fadd, fdiv, fmul, fsub, iadd_32, iadd_64, iand, iandnot, imul_16, imul_32, imul_64, ior,
    ishr_s_32, isub_32, isub_64, ixor, max, min,
};
use super::exec_vector::{
    to_lanes_16x8, to_lanes_32x4, to_lanes_64x2, to_lanes_8x16, vec_from_lanes,
};

pub fn vbinop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128, u128) -> RResult<u128>,
{
    let first = stack.pop_v128().ok_or(Trap)?;
    let second = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(first, second)?)));

    Ok(())
}

pub fn vbinop_with_value<Op, V>(stack: &mut Stack, operation: Op, value: V) -> RResult<()>
where
    Op: Fn(u128, u128, V) -> RResult<u128>,
{
    let first = stack.pop_v128().ok_or(Trap)?;
    let second = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(
        first, second, value,
    )?)));

    Ok(())
}

pub fn vvunop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128) -> u128,
{
    let v128 = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(v128))));

    Ok(())
}

pub fn vternop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128, u128, u128) -> RResult<u128>,
{
    let first = stack.pop_v128().ok_or(Trap)?;
    let second = stack.pop_v128().ok_or(Trap)?;
    let third = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(
        first, second, third,
    )?)));

    Ok(())
}

pub fn v128_and(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, iand)
}

pub fn v128_andnot(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, iandnot)
}

pub fn v128_or(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, ior)
}

pub fn v128_xor(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, ixor)
}

pub fn v128_anytrue(stack: &mut Stack) -> RResult<()> {
    if let Some(Val::Vec(vector)) = stack.pop_value() {
        let res = if vector != 0 { 1u32 } else { 0u32 };
        stack.push_entry(StackEntry::Value(Val::I32(res)));
    } else {
        return Err(Trap);
    }

    Ok(())
}

pub fn i8x16_swizzle(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, inner_swizzle_i8x16)
}

fn inner_swizzle_i8x16(left: u128, right: u128) -> RResult<u128> {
    let left_lines = to_lanes_8x16(left);
    let right_lines = to_lanes_8x16(right);

    let mut new_lines = [0u8; 16];
    for (index, select_index) in right_lines.iter().enumerate() {
        new_lines[index] = *left_lines.get(*select_index as usize).unwrap_or(&0u8);
    }

    Ok(vec_from_lanes(new_lines.to_vec()))
}

pub fn i8x16_shuffle(stack: &mut Stack, value: &Vec<LaneIdx>) -> RResult<()> {
    vbinop_with_value(stack, inner_shuffle_i8x16, value)
}

fn inner_shuffle_i8x16(left: u128, right: u128, lane_idx: &Vec<LaneIdx>) -> RResult<u128> {
    if lane_idx.iter().any(|LaneIdx(idx)| *idx > 32) {
        return Err(Trap);
    }
    let left_lanes = to_lanes_8x16(left);
    let right_lanes = to_lanes_8x16(right);
    let concatenated_lanes: Vec<&u8> = left_lanes.iter().chain(right_lanes.iter()).collect();

    let mut new_lines = [0u8; 16];
    for idx in 0..16 {
        let LaneIdx(i) = lane_idx[idx];
        new_lines[idx] = *concatenated_lanes[i as usize];
    }

    Ok(vec_from_lanes(new_lines.to_vec()))
}

pub fn i8x16_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)? as u8;
    let shape_dim = 16usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i16x8_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)? as u16;
    let shape_dim = 8usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i32x4_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)?;
    let shape_dim = 4usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i64x2_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i64().ok_or(Trap)?;
    let shape_dim = 2usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn f32x4_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)?;
    let new_lane = u32::from_be_bytes(base.to_be_bytes());
    let shape_dim = 4usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(new_lane);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn f64x2_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i64().ok_or(Trap)?;
    let new_lane = u64::from_be_bytes(base.to_be_bytes());
    let shape_dim = 2usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(new_lane);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i8x16_extract_lane_s(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 16usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as i8 as u32)));

    Ok(())
}

pub fn i8x16_extract_lane_u(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 16usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as u32)));

    Ok(())
}

pub fn i16x8_extract_lane_s(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as i16 as u32)));

    Ok(())
}

pub fn i16x8_extract_lane_u(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as u32)));

    Ok(())
}

pub fn i32x4_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 4usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i])));

    Ok(())
}

pub fn i64x2_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_64x2(vector);

    stack.push_entry(StackEntry::Value(Val::I64(lanes[lane_i])));

    Ok(())
}

pub fn f32x4_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 4usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    let float = f32::from_be_bytes(lanes[lane_i].to_be_bytes());
    stack.push_entry(StackEntry::Value(Val::F32(float)));

    Ok(())
}

pub fn f64x2_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_64x2(vector);

    let float = f64::from_be_bytes(lanes[lane_i].to_be_bytes());
    stack.push_entry(StackEntry::Value(Val::F64(float)));

    Ok(())
}

pub fn i8x16_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 16usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i32().ok_or(Trap)? as u8;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_8x16(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn i16x8_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i32().ok_or(Trap)? as u16;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_16x8(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn i32x4_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 4usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i32().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_32x4(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn i64x2_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i64().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_64x2(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn f32x4_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = u32::from_be_bytes(stack.pop_f32().ok_or(Trap)?.to_be_bytes());
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_32x4(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn f64x2_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = u64::from_be_bytes(stack.pop_f64().ok_or(Trap)?.to_be_bytes());
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_64x2(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn shape_i8_abs(v: &u8) -> u8 {
    (*v as i8).abs() as u8
}

pub fn shape_i16_abs(v: &u16) -> u16 {
    (*v as i16).abs() as u16
}

pub fn shape_i32_abs(v: &u32) -> u32 {
    (*v as i32).abs() as u32
}

pub fn shape_i64_abs(v: &u64) -> u64 {
    (*v as i64).abs() as u64
}

pub fn shape_f32_abs(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes()).abs();
    u32::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f64_abs(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).abs();
    u64::from_be_bytes(float.to_be_bytes())
}

pub fn shape_i8_neg(v: &u8) -> u8 {
    (*v as i8).neg() as u8
}

pub fn shape_i16_neg(v: &u16) -> u16 {
    (*v as i16).neg() as u16
}

pub fn shape_i32_neg(v: &u32) -> u32 {
    (*v as i32).neg() as u32
}

pub fn shape_i64_neg(v: &u64) -> u64 {
    (*v as i64).neg() as u64
}

pub fn shape_f32_neg(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes()).neg();
    u32::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f64_neg(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).neg();
    u64::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f32_sqrt(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes()).sqrt();
    u32::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f64_sqrt(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).sqrt();
    u64::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f32_ceil(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes()).ceil();
    u32::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f64_ceil(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).ceil();
    u64::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f32_floor(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes()).floor();
    u32::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f64_floor(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).floor();
    u64::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f32_trunc(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes()).trunc();
    u32::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f64_trunc(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).trunc();
    u64::from_be_bytes(float.to_be_bytes())
}

pub fn shape_f32_nearest(v: &u32) -> u32 {
    let float = f32::from_be_bytes(v.to_be_bytes());
    u32::from_be_bytes(nearest!(f32)(float).to_be_bytes())
}

pub fn shape_f64_nearest(v: &u64) -> u64 {
    let float = f64::from_be_bytes(v.to_be_bytes()).trunc();
    u64::from_be_bytes(nearest!(f64)(float).to_be_bytes())
}

pub fn shape_i8_popcnt(v: &u8) -> u8 {
    (*v as i8).count_ones() as u8
}

pub fn shape_i8_add((left, right): (&u8, &u8)) -> u8 {
    ((*left as u128) + (*right as u128)).rem_euclid(2u128.pow(8)) as u8
}

pub fn shape_i16_add((left, right): (&u16, &u16)) -> u16 {
    ((*left as u128) + (*right as u128)).rem_euclid(2u128.pow(16)) as u16
}

pub fn shape_i32_add((left, right): (&u32, &u32)) -> u32 {
    // it is safe to unwrap because iadd_32 just wraps addition result into Ok
    iadd_32(*left, *right).unwrap()
}

pub fn shape_i64_add((left, right): (&u64, &u64)) -> u64 {
    // it is safe to unwrap because iadd_64 just wraps addition result into Ok
    iadd_64(*left, *right).unwrap()
}

pub fn shape_i8_sub((left, right): (&u8, &u8)) -> u8 {
    ((*left as u128) - (*right as u128)).rem_euclid(2u128.pow(8)) as u8
}

pub fn shape_i16_sub((left, right): (&u16, &u16)) -> u16 {
    ((*left as u128) - (*right as u128)).rem_euclid(2u128.pow(16)) as u16
}

pub fn shape_i32_sub((left, right): (&u32, &u32)) -> u32 {
    // it is safe to unwrap because isub_32 just wraps substraction result into Ok
    isub_32(*left, *right).unwrap()
}

pub fn shape_i64_sub((left, right): (&u64, &u64)) -> u64 {
    // it is safe to unwrap because isub_64 just wraps substraction result into Ok
    isub_64(*left, *right).unwrap()
}

pub fn shape_f32_add((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    u32::from_be_bytes((fadd(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f64_add((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    u64::from_be_bytes((fadd(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f32_sub((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    u32::from_be_bytes((fsub(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f64_sub((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    u64::from_be_bytes((fsub(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_i16_mul((left, right): (&u16, &u16)) -> u16 {
    imul_16(*left, *right)
}

pub fn shape_i32_mul((left, right): (&u32, &u32)) -> u32 {
    imul_32(*left, *right).unwrap()
}

pub fn shape_i64_mul((left, right): (&u64, &u64)) -> u64 {
    imul_64(*left, *right).unwrap()
}

pub fn shape_f32_mul((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    u32::from_be_bytes((fmul(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f64_mul((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    u64::from_be_bytes((fmul(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f32_div((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    u32::from_be_bytes((fdiv(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f64_div((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    u64::from_be_bytes((fdiv(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f32_min((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    u32::from_be_bytes((min(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f64_min((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    u64::from_be_bytes((min(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f32_max((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    u32::from_be_bytes((max(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f64_max((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    u64::from_be_bytes((max(f_left, f_right).unwrap()).to_be_bytes())
}

pub fn shape_f32_pmin((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    let min_val = if f_left > f_right { f_right } else { f_left };
    u32::from_be_bytes(min_val.to_be_bytes())
}

pub fn shape_f64_pmin((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    let min_val = if f_left > f_right { f_right } else { f_left };
    u64::from_be_bytes(min_val.to_be_bytes())
}

pub fn shape_f32_pmax((left, right): (&u32, &u32)) -> u32 {
    let f_left = f32::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f32::from_be_bytes(right.to_be_bytes()).trunc();
    let max_val = if f_left < f_right { f_right } else { f_left };
    u32::from_be_bytes(max_val.to_be_bytes())
}

pub fn shape_f64_pmax((left, right): (&u64, &u64)) -> u64 {
    let f_left = f64::from_be_bytes(left.to_be_bytes()).trunc();
    let f_right = f64::from_be_bytes(right.to_be_bytes()).trunc();
    let max_val = if f_left < f_right { f_right } else { f_left };
    u64::from_be_bytes(max_val.to_be_bytes())
}

pub fn shape_i8_min_u((left, right): (&u8, &u8)) -> u8 {
    *left.min(right)
}

pub fn shape_i8_min_s((left, right): (&u8, &u8)) -> u8 {
    (*left as i8).min(*right as i8) as u8
}

pub fn shape_i16_min_u((left, right): (&u16, &u16)) -> u16 {
    *left.min(right)
}

pub fn shape_i16_min_s((left, right): (&u16, &u16)) -> u16 {
    (*left as i16).min(*right as i16) as u16
}

pub fn shape_i32_min_u((left, right): (&u32, &u32)) -> u32 {
    *left.min(right)
}

pub fn shape_i32_min_s((left, right): (&u32, &u32)) -> u32 {
    (*left as i32).min(*right as i32) as u32
}

pub fn shape_i8_max_u((left, right): (&u8, &u8)) -> u8 {
    *left.max(right)
}

pub fn shape_i8_max_s((left, right): (&u8, &u8)) -> u8 {
    (*left as i8).max(*right as i8) as u8
}

pub fn shape_i16_max_u((left, right): (&u16, &u16)) -> u16 {
    *left.max(right)
}

pub fn shape_i16_max_s((left, right): (&u16, &u16)) -> u16 {
    (*left as i16).max(*right as i16) as u16
}

pub fn shape_i32_max_u((left, right): (&u32, &u32)) -> u32 {
    *left.max(right)
}

pub fn shape_i32_max_s((left, right): (&u32, &u32)) -> u32 {
    (*left as i32).max(*right as i32) as u32
}

pub fn shape_i8_sat_add_u((left, right): (&u8, &u8)) -> u8 {
    left.checked_add(*right).unwrap_or(u8::MAX)
}

pub fn shape_i16_sat_add_u((left, right): (&u16, &u16)) -> u16 {
    left.checked_add(*right).unwrap_or(u16::MAX)
}

pub fn shape_i8_sat_add_s((left, right): (&u8, &u8)) -> u8 {
    let left_signed = *left as i8;
    let right_signed = *right as i8;
    left_signed.checked_add(right_signed).unwrap_or_else(|| {
        let left_sign = left_signed.signum();
        let right_sign = right_signed.signum();
        if left_sign > 0 && right_sign > 0 {
            i8::MAX
        } else {
            i8::MIN
        }
    }) as u8
}

pub fn shape_i16_sat_add_s((left, right): (&u16, &u16)) -> u16 {
    let left_signed = *left as i16;
    let right_signed = *right as i16;
    left_signed.checked_add(right_signed).unwrap_or_else(|| {
        let left_sign = left_signed.signum();
        let right_sign = right_signed.signum();
        if left_sign > 0 && right_sign > 0 {
            i16::MAX
        } else {
            i16::MIN
        }
    }) as u16
}

pub fn shape_i8_sat_sub_u((left, right): (&u8, &u8)) -> u8 {
    left.checked_sub(*right).unwrap_or(u8::MAX)
}

pub fn shape_i16_sat_sub_u((left, right): (&u16, &u16)) -> u16 {
    left.checked_sub(*right).unwrap_or(u16::MAX)
}

pub fn shape_i8_sat_sub_s((left, right): (&u8, &u8)) -> u8 {
    let left_signed = *left as i8;
    let right_signed = *right as i8;
    left_signed.checked_sub(right_signed).unwrap_or_else(|| {
        let left_sign = left_signed.signum();
        let right_sign = right_signed.signum();
        if left_sign > 0 && right_sign < 0 {
            i8::MAX
        } else {
            i8::MIN
        }
    }) as u8
}

pub fn shape_i16_sat_sub_s((left, right): (&u16, &u16)) -> u16 {
    let left_signed = *left as i16;
    let right_signed = *right as i16;
    left_signed.checked_sub(right_signed).unwrap_or_else(|| {
        let left_sign = left_signed.signum();
        let right_sign = right_signed.signum();
        if left_sign > 0 && right_sign < 0 {
            i16::MAX
        } else {
            i16::MIN
        }
    }) as u16
}

pub fn shape_i8_avgr_u(args: (&u8, &u8)) -> u8 {
    shape_i8_add(args) / 2
}

pub fn shape_i16_avgr_u(args: (&u16, &u16)) -> u16 {
    shape_i16_add(args) / 2
}

pub fn shape_i16_mulr_sat_s((left, right): (&u16, &u16)) -> u16 {
    let l = *left as u32;
    let r = *right as u32;
    let mulr = ishr_s_32(l * r + 2u32.pow(14), 15).unwrap() as i32;

    if mulr > i16::MAX as i32 {
        i16::MAX as u16
    } else if mulr < i16::MIN as i32 {
        i16::MIN as u16
    } else {
        mulr as u16
    }
}

pub fn shape_eq<T>((lhs, rhs): (&T, &T)) -> bool
where
    T: std::cmp::PartialEq,
{
    lhs == rhs
}

pub fn shape_ne<T>((lhs, rhs): (&T, &T)) -> bool
where
    T: std::cmp::PartialEq,
{
    lhs != rhs
}

pub fn shape_lt_u<T>((lhs, rhs): (&T, &T)) -> bool
where
    T: std::cmp::PartialOrd,
{
    lhs < rhs
}

pub fn shape_lt_s<T, S>((lhs, rhs): (&T, &T)) -> bool
where
    T: AsSigned<S>,
    S: std::cmp::PartialOrd,
{
    lhs.as_signed() < rhs.as_signed()
}

pub fn shape_gt_u<T>((lhs, rhs): (&T, &T)) -> bool
where
    T: std::cmp::PartialOrd,
{
    lhs > rhs
}

pub fn shape_gt_s<T, S>((lhs, rhs): (&T, &T)) -> bool
where
    T: AsSigned<S>,
    S: std::cmp::PartialOrd,
{
    lhs.as_signed() > rhs.as_signed()
}

pub fn shape_le_u<T>((lhs, rhs): (&T, &T)) -> bool
where
    T: std::cmp::PartialOrd,
{
    lhs <= rhs
}

pub fn shape_le_s<T, S>((lhs, rhs): (&T, &T)) -> bool
where
    T: AsSigned<S>,
    S: std::cmp::PartialOrd,
{
    lhs.as_signed() <= rhs.as_signed()
}

pub fn shape_ge_u<T>((lhs, rhs): (&T, &T)) -> bool
where
    T: std::cmp::PartialOrd,
{
    lhs >= rhs
}

pub fn shape_ge_s<T, S>((lhs, rhs): (&T, &T)) -> bool
where
    T: AsSigned<S>,
    S: std::cmp::PartialOrd,
{
    lhs.as_signed() >= rhs.as_signed()
}

pub fn shapef_lt<T, F>((lhs, rhs): (&T, &T)) -> bool
where
    F: std::cmp::PartialOrd,
    T: AsFloat<F>,
{
    lhs.as_float() < rhs.as_float()
}

pub fn shapef_gt<T, F>((lhs, rhs): (&T, &T)) -> bool
where
    F: std::cmp::PartialOrd,
    T: AsFloat<F>,
{
    lhs.as_float() > rhs.as_float()
}

pub fn shapef_le<T, F>((lhs, rhs): (&T, &T)) -> bool
where
    F: std::cmp::PartialOrd,
    T: AsFloat<F>,
{
    lhs.as_float() <= rhs.as_float()
}

pub fn shapef_ge<T, F>((lhs, rhs): (&T, &T)) -> bool
where
    F: std::cmp::PartialOrd,
    T: AsFloat<F>,
{
    lhs.as_float() >= rhs.as_float()
}

pub fn make_shape_shl<T>(size: u32) -> impl FnOnce(T, u32) -> T + Copy
where
    T: std::ops::Shl<u32, Output = T>,
{
    move |lane: T, shift: u32| {
        let k = shift.rem_euclid(size);
        lane << k
    }
}

pub fn make_shape_shr_u<T>(size: u32) -> impl FnOnce(T, u32) -> T + Copy
where
    T: std::ops::Shr<u32, Output = T>,
{
    move |lane: T, shift: u32| {
        let k = shift.rem_euclid(size);
        lane >> k
    }
}

pub fn make_shape_shr_s<T>(size: u32) -> impl FnOnce(T, u32) -> T + Copy
where
    T: std::ops::Shr<u32, Output = T>
        + std::ops::BitOrAssign
        + std::ops::BitAnd<T, Output = T>
        + From<u8>
        + std::ops::Div<T, Output = T>
        + std::ops::Shl<u32, Output = T>
        + Copy,
{
    move |lane: T, shift: u32| {
        let k = shift.rem_euclid(size);
        let mut shifted = lane >> k;
        // lane / lane is a generic one
        let most_significant_byte = lane & (lane / lane);
        let mut byte_propagator = (most_significant_byte) << (size - 1);
        for _ in 0..k {
            shifted |= byte_propagator;
            byte_propagator = byte_propagator >> 1;
        }

        shifted
    }
}

pub fn unop_8x16<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut(&u8) -> u8 + Copy,
{
    vvunop(stack, |v| {
        let lanes = to_lanes_8x16(v);
        vec_from_lanes(lanes.iter().map(func).collect())
    })
}

pub fn unop_16x8<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut(&u16) -> u16 + Copy,
{
    vvunop(stack, |v| {
        let lanes = to_lanes_16x8(v);
        vec_from_lanes(lanes.iter().map(func).collect())
    })
}

pub fn unop_32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut(&u32) -> u32 + Copy,
{
    vvunop(stack, |v| {
        let lanes = to_lanes_32x4(v);
        vec_from_lanes(lanes.iter().map(func).collect())
    })
}

pub fn unop_64x2<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut(&u64) -> u64 + Copy,
{
    vvunop(stack, |v| {
        let lanes = to_lanes_64x2(v);
        vec_from_lanes(lanes.iter().map(func).collect())
    })
}

pub fn binop_8x16<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut((&u8, &u8)) -> u8 + Copy,
{
    let left = stack.pop_v128().ok_or(Trap)?;
    let right = stack.pop_v128().ok_or(Trap)?;
    let lanes_left = to_lanes_8x16(left);
    let lanes_right = to_lanes_8x16(right);
    let result_vec = vec_from_lanes(
        lanes_left
            .iter()
            .zip(lanes_right.iter())
            .map(func)
            .collect(),
    );

    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn binop_16x8<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut((&u16, &u16)) -> u16 + Copy,
{
    let left = stack.pop_v128().ok_or(Trap)?;
    let right = stack.pop_v128().ok_or(Trap)?;
    let lanes_left = to_lanes_16x8(left);
    let lanes_right = to_lanes_16x8(right);
    let result_vec = vec_from_lanes(
        lanes_left
            .iter()
            .zip(lanes_right.iter())
            .map(func)
            .collect(),
    );

    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn binop_32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut((&u32, &u32)) -> u32 + Copy,
{
    let left = stack.pop_v128().ok_or(Trap)?;
    let right = stack.pop_v128().ok_or(Trap)?;
    let lanes_left = to_lanes_32x4(left);
    let lanes_right = to_lanes_32x4(right);
    let result_vec = vec_from_lanes(
        lanes_left
            .iter()
            .zip(lanes_right.iter())
            .map(func)
            .collect(),
    );

    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn binop_64x2<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnMut((&u64, &u64)) -> u64 + Copy,
{
    let left = stack.pop_v128().ok_or(Trap)?;
    let right = stack.pop_v128().ok_or(Trap)?;
    let lanes_left = to_lanes_64x2(left);
    let lanes_right = to_lanes_64x2(right);
    let result_vec = vec_from_lanes(
        lanes_left
            .iter()
            .zip(lanes_right.iter())
            .map(func)
            .collect(),
    );

    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn relop_8x16<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce((&u8, &u8)) -> bool + Copy,
{
    binop_8x16(stack, |values| relop_8(func(values)))
}

pub fn relop_16x8<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce((&u16, &u16)) -> bool + Copy,
{
    binop_16x8(stack, |values| relop_16(func(values)))
}

pub fn relop_32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce((&u32, &u32)) -> bool + Copy,
{
    binop_32x4(stack, |values| relop_32(func(values)))
}

pub fn relop_64x2<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce((&u64, &u64)) -> bool + Copy,
{
    binop_64x2(stack, |values| relop_64(func(values)))
}

fn relop_8(b: bool) -> u8 {
    if b {
        u8::MAX
    } else {
        0
    }
}

fn relop_16(b: bool) -> u16 {
    if b {
        u16::MAX
    } else {
        0
    }
}

fn relop_32(b: bool) -> u32 {
    if b {
        u32::MAX
    } else {
        0
    }
}

fn relop_64(b: bool) -> u64 {
    if b {
        u64::MAX
    } else {
        0
    }
}

pub fn shiftop_8x16<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u8, u32) -> u8 + Copy,
{
    let shift = stack.pop_i32().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;

    let lanes = to_lanes_8x16(vector);

    let result_vec = vec_from_lanes(lanes.iter().map(|l| func(*l, shift)).collect());
    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn shiftop_16x8<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u16, u32) -> u16 + Copy,
{
    let shift = stack.pop_i32().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;

    let lanes = to_lanes_16x8(vector);

    let result_vec = vec_from_lanes(lanes.iter().map(|l| func(*l, shift)).collect());
    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn shiftop_32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u32, u32) -> u32 + Copy,
{
    let shift = stack.pop_i32().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;

    let lanes = to_lanes_32x4(vector);

    let result_vec = vec_from_lanes(lanes.iter().map(|l| func(*l, shift)).collect());
    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

pub fn shiftop_64x2<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u64, u32) -> u64 + Copy,
{
    let shift = stack.pop_i32().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;

    let lanes = to_lanes_64x2(vector);

    let result_vec = vec_from_lanes(lanes.iter().map(|l| func(*l, shift)).collect());
    stack.push_entry(StackEntry::Value(Val::Vec(result_vec)));

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use syntax::types::LaneIdx;

//     use super::*;

//     #[test]
//     fn vec_from_lanes_test() {
//         // i8x16
//         assert_eq!(
//             vec_from_lanes(vec![
//                 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
//             ],),
//             0
//         );

//         assert_eq!(
//         vec_from_lanes(vec![
//             0b00010001u8,
//             0b00010001u8,
//             0b00010001u8,
//             0b00010001u8,
//             0b00010001u8,
//             0b00010001u8,
//             0b00010001u8,
//             0b00010001u8,
//             0u8,
//             0u8,
//             0u8,
//             0u8,
//             0u8,
//             0u8,
//             0u8,
//             0u8,
//         ],),
//         0b00010001000100010001000100010001000100010001000100010001000100010000000000000000000000000000000000000000000000000000000000000000u128
//     );

//         assert_eq!(
//             vec_from_lanes(vec![
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//                 u8::MAX,
//             ],),
//             u128::MAX
//         );

//         // i16x8
//         assert_eq!(
//             vec_from_lanes(vec![0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16]),
//             0,
//         );

//         assert_eq!(
//             vec_from_lanes(vec![
//                 u16::MAX,
//                 u16::MAX,
//                 u16::MAX,
//                 u16::MAX,
//                 u16::MAX,
//                 u16::MAX,
//                 u16::MAX,
//                 u16::MAX,
//             ]),
//             u128::MAX,
//         );

//         // i32x4
//         assert_eq!(vec_from_lanes(vec![0u32, 0u32, 0u32, 0u32,]), 0);
//         assert_eq!(
//             vec_from_lanes(vec![u32::MAX, u32::MAX, u32::MAX, u32::MAX,]),
//             u128::MAX
//         );

//         // i64x2
//         assert_eq!(vec_from_lanes(vec![0u64, 0u64]), 0);
//         assert_eq!(vec_from_lanes(vec![u64::MAX, u64::MAX]), u128::MAX);
//     }

//     #[test]
//     fn vec_to_lanes_16x8() {
//         let init_lanes = vec![1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16];
//         let vector = vec_from_lanes(init_lanes.clone());

//         let result_lanes = to_lanes_16x8(vector);

//         assert_eq!(
//             result_lanes.to_vec(),
//             init_lanes,
//             "should properly return 16x8 lanes"
//         );
//     }

//     #[test]
//     fn vec_to_lanes_32x4() {
//         let init_lanes = vec![1u32, 2u32, 3u32, 4u32];
//         let vector = vec_from_lanes(init_lanes.clone());

//         let result_lanes = to_lanes_32x4(vector);

//         assert_eq!(
//             result_lanes.to_vec(),
//             init_lanes,
//             "should properly return 16x8 lanes"
//         );
//     }

//     #[test]
//     fn vec_to_lanes_64x2() {
//         let init_lanes = vec![1u64, 2u64];
//         let vector = vec_from_lanes(init_lanes.clone());

//         let result_lanes = to_lanes_64x2(vector);

//         assert_eq!(
//             result_lanes.to_vec(),
//             init_lanes,
//             "should properly return 16x8 lanes"
//         );
//     }

//     #[test]
//     fn swizzle_i8x16_test() {
//         let src = vec_from_lanes(vec![
//             1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
//         ]);
//         let pick = vec_from_lanes(vec![0u8, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15]);
//         let expected = vec_from_lanes(vec![
//             1u8, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16,
//         ]);
//         assert_eq!(
//             inner_swizzle_i8x16(src, pick).expect("should swizzle without errors"),
//             expected
//         );
//     }

//     #[test]
//     fn shuffle_i8x16_test() {
//         let left = vec_from_lanes(vec![
//             1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
//         ]);
//         let right = vec_from_lanes(vec![
//             17u8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
//         ]);

//         assert!(
//             inner_shuffle_i8x16(
//                 left,
//                 right,
//                 &([33u8; 16].iter().map(|v| LaneIdx(*v)).collect())
//             )
//             .is_err(),
//             "should return error if any of lane_idx is more or equal 32"
//         );

//         assert_eq!(
//             inner_shuffle_i8x16(
//                 left,
//                 right,
//                 &(vec![0u8, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32]
//                     .iter()
//                     .map(|v| LaneIdx(*v))
//                     .collect())
//             )
//             .expect("should shuffle without errors"),
//             vec_from_lanes(vec![
//                 1u8, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31
//             ]),
//             "should return error if any of lane_idx is more or equal 32"
//         );
//     }

//     #[test]
//     fn shape_splat_i8x16_test() {
//         let mut stack = Stack::new();
//         stack.push_entry(StackEntry::Value(Val::I32(1)));
//         i8x16_splat(&mut stack).expect("should splat without errors");
//         let val = stack.pop_value();
//         let expected = vec_from_lanes(vec![1u8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
//         assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
//     }

//     #[test]
//     fn shape_splat_i16x8_test() {
//         let mut stack = Stack::new();
//         stack.push_entry(StackEntry::Value(Val::I32(1)));
//         i16x8_splat(&mut stack).expect("should splat without errors");
//         let val = stack.pop_value();
//         let expected = vec_from_lanes(vec![1u16, 1, 1, 1, 1, 1, 1, 1]);
//         assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
//     }

//     #[test]
//     fn shape_splat_i32x4_test() {
//         let mut stack = Stack::new();
//         stack.push_entry(StackEntry::Value(Val::I32(1)));
//         i32x4_splat(&mut stack).expect("should splat without errors");
//         let val = stack.pop_value();
//         let expected = vec_from_lanes(vec![1u32, 1u32, 1u32, 1u32]);
//         assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
//     }

//     #[test]
//     fn shape_splat_i64x2_test() {
//         let mut stack = Stack::new();
//         stack.push_entry(StackEntry::Value(Val::I64(2)));
//         i64x2_splat(&mut stack).expect("should splat without errors");
//         let val = stack.pop_value();
//         let expected = vec_from_lanes(vec![2u64, 2u64]);
//         assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
//     }

//     #[test]
//     fn shape_extract_lane_s_i8x16_test() {
//         let mut stack = Stack::new();
//         stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(vec![
//             255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         ]))));

//         i8x16_extract_lane_s(&mut stack, 0).expect("should splat without errors");

//         let val = stack.pop_value();
//         assert_eq!(val, Some(Val::I32(-1i32 as u32)), "should property splat");
//     }

//     #[test]
//     fn shape_extract_lane_u_i8x16_test() {
//         let mut stack = Stack::new();
//         stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(vec![
//             255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//         ]))));

//         i8x16_extract_lane_u(&mut stack, 0).expect("should splat without errors");

//         let val = stack.pop_value();
//         assert_eq!(val, Some(Val::I32(255)), "should property splat");
//     }
// }
