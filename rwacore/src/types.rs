//! Webassembly Core types.
//! Types are checked during validation and instantiation.

use alloc::vec::Vec;

/// Webassembly basic type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RType {
    /// 32-bit integer.
    I32,
    /// 64-bit integer.
    I64,
    /// 32-bit float.
    F32,
    /// 64-bit float.
    F64,
    /// Classify vectors of numeric values
    Vec,
    /// Reference to a function.
    FuncRef,
    /// Reference to objects owned by the embedder.
    ExternRef,
}

/// Webassembly [number type](https://webassembly.github.io/spec/core/syntax/types.html#syntax-numtype).
pub enum NumType {
    I32,
    I64,
    F32,
    F64
}

/// Type that classifies the result of function or instruction execution.
#[derive(Clone, Debug, PartialEq)]
pub struct ResType(pub Vec<RType>);

/// Type that classifies the signature of a fuction.
#[derive(Clone, Debug, PartialEq)]
pub struct FuncType {
    /// Function arguments types.
    pub args: ResType,
    /// Function execution result type.
    pub ret: ResType,
}

/// Limits classify the size range of resizeable storage 
/// associated with memory types and table types.
#[derive(Clone, Debug, PartialEq)]
pub struct LimitsType {
    pub min: u32,
    pub max: Option<u32>
}

/// Memory types classify linear memories and their size range.
#[derive(Clone, Debug, PartialEq)]
pub struct MemType(pub LimitsType);

/// Table types classify tables over elements of reference type within a size range.
/// The limits are given in number of entries.
#[derive(Clone, Debug, PartialEq)]
pub struct TableType {
    pub limits: LimitsType,
    /// Reference type.
    ///
    /// **Before creating `TableType` `RType` must be validated
    /// that it is one of either `RType::FuncRef` or `RType::ExternRef`.**
    pub reference: RType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mutability {
    Const,
    Mut
}

/// Global types classify global variables, which hold a value and can either be mutable or immutable.
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalType {
    pub mutability: Mutability,
    pub val_type: RType
}

/// External types classify imports and external values with their respective types.
#[derive(Clone, Debug, PartialEq)]
pub enum ExternalType {
    Fun(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType)
}
