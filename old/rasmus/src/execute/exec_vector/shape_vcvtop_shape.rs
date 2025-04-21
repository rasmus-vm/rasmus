use crate::{
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

use super::{to_lanes_16x8, to_lanes_32x4, to_lanes_64x2, to_lanes_8x16, vec_from_lanes};

pub fn i32x4_vcvtop_f32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(f32) -> u32 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    let mut new_lanes: Vec<u32> = Vec::with_capacity(4);

    for lane in lanes {
        let float = f32::from_be_bytes(lane.to_be_bytes());
        new_lanes.push(func(float));
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub fn shape_i32_trunc_f32_s(f: f32) -> u32 {
    if f.is_nan() {
        return 0;
    }

    if f.is_infinite() {
        if f.is_sign_positive() {
            return i32::MAX as u32;
        } else {
            return i32::MIN as u32;
        }
    }

    let trunced = f.trunc() as i128;

    if trunced > i32::MAX as i128 {
        return i32::MAX as u32;
    }

    if trunced < i32::MIN as i128 {
        return i32::MIN as u32;
    }

    trunced as u32
}

pub fn shape_i32_trunc_f32_u(f: f32) -> u32 {
    if f.is_nan() {
        return 0;
    }

    if f.is_infinite() {
        if f.is_sign_positive() {
            return u32::MAX;
        } else {
            return 0;
        }
    }

    let trunced = f.trunc() as u128;

    if trunced > u32::MAX as u128 {
        return u32::MAX;
    }

    trunced as u32
}

pub fn f32x4_vcvtop_i32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u32) -> u32 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    let mut new_lanes: Vec<u32> = Vec::with_capacity(4);

    for lane in lanes {
        new_lanes.push(func(lane));
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub enum Half {
    Low,
    High,
}

pub fn i16x8_vcvtop_half_i8x16<F>(stack: &mut Stack, func: F, half: Half) -> RResult<()>
where
    F: FnOnce(u8) -> u16 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let range = match half {
        Half::Low => 0..8,
        Half::High => 8..16,
    };
    let lanes = &to_lanes_8x16(vector)[range];
    let new_lanes = vec_from_lanes(lanes.iter().map(|l| func(*l)).collect());

    stack.push_entry(StackEntry::Value(Val::Vec(new_lanes)));
    Ok(())
}

pub fn i32x4_vcvtop_half_i16x8<F>(stack: &mut Stack, func: F, half: Half) -> RResult<()>
where
    F: FnOnce(u16) -> u32 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let range = match half {
        Half::Low => 0..4,
        Half::High => 4..8,
    };
    let lanes = &to_lanes_16x8(vector)[range];
    let new_lanes = vec_from_lanes(lanes.iter().map(|l| func(*l)).collect());

    stack.push_entry(StackEntry::Value(Val::Vec(new_lanes)));
    Ok(())
}

pub fn i64x2_vcvtop_half_i32x4<F>(stack: &mut Stack, func: F, half: Half) -> RResult<()>
where
    F: FnOnce(u32) -> u64 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let range = match half {
        Half::Low => 0..2,
        Half::High => 2..4,
    };
    let lanes = &to_lanes_32x4(vector)[range];
    let new_lanes = vec_from_lanes(lanes.iter().map(|l| func(*l)).collect());

    stack.push_entry(StackEntry::Value(Val::Vec(new_lanes)));
    Ok(())
}

pub fn shape_32x4_vcvtop_64x2_zero<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u64) -> u32 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;

    let lanes = to_lanes_64x2(vector);
    let new_lanes = vec_from_lanes(
        lanes
            .iter()
            .map(|l| func(*l))
            .rev()
            .chain(vec![0u32, 0u32].iter().cloned())
            .collect(),
    );

    stack.push_entry(StackEntry::Value(Val::Vec(new_lanes)));
    Ok(())
}

pub fn shape_i32_trunc_f64_u(f: u64) -> u32 {
    let float = f64::from_be_bytes(f.to_be_bytes());
    if float.is_nan() {
        return 0;
    }

    if float.is_infinite() {
        if float.is_sign_positive() {
            return u32::MAX;
        } else {
            return 0;
        }
    }

    let trunced = float.trunc() as u128;

    if trunced > u32::MAX as u128 {
        return u32::MAX;
    }

    if (trunced as i32) < 0 {
        return 0;
    }

    trunced as u32
}

pub fn shape_i32_trunc_f64_s(f: u64) -> u32 {
    let float = f64::from_be_bytes(f.to_be_bytes());
    if float.is_nan() {
        return 0;
    }

    if float.is_infinite() {
        if float.is_sign_positive() {
            return i32::MAX as u32;
        } else {
            return i32::MIN as u32;
        }
    }

    let trunced = float.trunc() as i128;

    if trunced > i32::MAX as i128 {
        return i32::MAX as u32;
    }

    if trunced < i32::MIN as i128 {
        return i32::MIN as u32;
    }

    trunced as u32
}

pub fn shape_f32_demote_f64(v: u64) -> u32 {
    let float_64 = f64::from_be_bytes(v.to_be_bytes());
    let float_32 = float_64 as f32;

    if float_64.is_nan() || float_32.is_nan() {
        return u32::from_be_bytes(f32::NAN.to_be_bytes());
    }

    if float_64.is_infinite() {
        if float_64.is_sign_positive() {
            return u32::from_be_bytes(f32::INFINITY.to_be_bytes());
        } else {
            return u32::from_be_bytes(f32::NEG_INFINITY.to_be_bytes());
        }
    }

    if float_64 == 0.0 {
        return u32::from_be_bytes(0.0f32.to_be_bytes());
    }

    if float_64 == -0.0 {
        return u32::from_be_bytes((-0.0f32).to_be_bytes());
    }

    u32::from_be_bytes(float_32.to_be_bytes())
}

pub fn shape_f32_convert_i32_u(v: u32) -> u32 {
    u32::from_be_bytes((v as f32).to_be_bytes())
}

pub fn shape_f32_convert_i32_s(v: u32) -> u32 {
    u32::from_be_bytes((v as i32 as f32).to_be_bytes())
}

pub fn i8_extend_i16_u(v: u8) -> u16 {
    v as u16
}

pub fn i8_extend_i16_s(v: u8) -> u16 {
    v as i8 as u16
}

pub fn i16_extend_i32_u(v: u16) -> u32 {
    v as u32
}

pub fn i16_extend_i32_s(v: u16) -> u32 {
    v as i16 as u32
}

pub fn i32_extend_i64_u(v: u32) -> u64 {
    v as u64
}

pub fn i32_extend_i64_s(v: u32) -> u64 {
    v as i32 as u64
}

pub fn i32_convert_f64_s(v: u32) -> u64 {
    u64::from_be_bytes((v as i32 as f64).to_be_bytes())
}

pub fn i32_convert_f64_u(v: u32) -> u64 {
    u64::from_be_bytes((v as f64).to_be_bytes())
}

pub fn f32_promote_f64(v: u32) -> u64 {
    u64::from_be_bytes((f32::from_be_bytes(v.to_be_bytes()) as f64).to_be_bytes())
}
