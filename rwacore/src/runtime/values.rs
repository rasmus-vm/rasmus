#![allow(unused)]
//! Rasmus structures representing WebAssembly Values
//! as they are defined in
//! [Core Specficication](https://webassembly.github.io/spec/core/syntax/values.html).

use crate::types::RType;

/// Bytes length of 32-bit numerical type values (`I32`, `F32`).
pub const N32_BYTES_LEN: usize = 32/8;
/// Bytes length of 64-bit numerical type values (`I64`, `F64`).
pub const N64_BYTES_LEN: usize = 64/8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Value {
    /// Unsigned 32-bit integer.
    U32(u32),
    /// Unsigned 64-bit integer.
    U64(u64),
    /// Signed 32-bit integer.
    S32(i32),
    /// Signed 64-bit integer.
    S64(i64),
    /// Uniterpreted 8-bit integer.
    I8(u8),
    /// Uniterpreted 16-bit integer.
    I16(u16),
    /// Uniterpreted 32-bit integer.
    I32(u32),
    /// Uniterpreted 64-bit integer.
    I64(u64),
    /// Numeric vectors are 128-bit values that
    /// are processed by vector instructions
    Vec([u8; 16]),
    /// 32-bit floating point number.
    /// [IEEE 754](https://ieeexplore.ieee.org/document/8766229).
    F32(f32),
    /// 64-bit floating point number.
    /// [IEEE 754](https://ieeexplore.ieee.org/document/8766229).
    F64(f64),
    /// Null reference.
    /// Must be validated that value type is either
    /// `RType::FunRef` or `RType::ExternRef`.
    RefNull(RType),
    /// Reference to function. Inner value is a *funcaddr*.
    Ref(usize),
    /// External reference. Inner value is an *externaddr*.
    RefExtern(usize)
}

/// An external value is the runtime representation of an
/// entity that can be imported or exported.
#[derive(Debug)]
pub enum ExternValue {
    /// funcaddr
    Func(usize),
    /// tableaddr
    Table(usize),
    /// memaddr
    Mem(usize),
    /// globaladdr
    Global(usize)
}
