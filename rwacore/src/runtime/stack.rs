//! Virtual Stack.

use crate::{instructions::Instr, types::{NumType, RType}};

use super::{
    Frame,
    trap::{RResult, Trap},
    values::Value,
};
use alloc::vec::Vec;

/// Entity that can be pushed an popped from
/// the Virtual Stack.
#[derive(Clone, Debug)]
pub enum StackEntity<'a> {
    Val(Value),
    Label { arity: usize, instrs: Vec<Instr> },
    Frame(Frame<'a>),
}

/// Virtual Stack used by a program.
pub struct Stack<'a> {
    inner: Vec<StackEntity<'a>>,
    capacity: usize,
}

impl<'a> Stack<'a> {
    /// Method that allocates a `Stack` for a program.
    #[inline]
    pub fn new(capacity: usize) -> RResult<Self> {
        Ok(Stack {
            inner: Vec::with_capacity(capacity),
            capacity,
        })
    }

    /// Push `StackEntry` to `Stack`.
    #[inline]
    pub fn push(&mut self, stack_entity: StackEntity<'a>) -> RResult<()> {
        if self.inner.len() == self.capacity {
            return Err(Trap::StackOverflow);
        }
        self.inner.push(stack_entity);
        Ok(())
    }

    /// Pop `StackEntry` from `Stack`.
    #[inline]
    pub fn pop(&mut self) -> RResult<StackEntity> {
        // FIXME: double check if it should be Trap.
        self.inner.pop().ok_or_else(|| Trap::EmptyStackOnPop)
    }

    /// Pops `StackEntry` from `Stack` and ensures it is `I32`.
    /// Traps otherwise.
    #[inline]
    pub fn pop_i32(&mut self) -> RResult<u32> {
        match self.pop()? {
            StackEntity::Val(Value::I32(v)) => Ok(v),
            _ => Err(Trap::UnexpectedStackEntity(RType::I32)),
        }
    }

    /// Pops `StackEntry` from `Stack` and ensures it is `I64`.
    /// Traps otherwise.
    #[inline]
    pub fn pop_i64(&mut self) -> RResult<u64> {
        match self.pop()? {
            StackEntity::Val(Value::I64(v)) => Ok(v),
            _ => Err(Trap::UnexpectedStackEntity(RType::I64)),
        }
    }

    /// Pops `StackEntry` from `Stack` and ensures it is `F32`.
    /// Traps otherwise.
    #[inline]
    pub fn pop_f32(&mut self) -> RResult<f32> {
        match self.pop()? {
            StackEntity::Val(Value::F32(v)) => Ok(v),
            _ => Err(Trap::UnexpectedStackEntity(RType::F32)),
        }
    }

    /// Pops `StackEntry` from `Stack` and ensures it is `F64`.
    /// Traps otherwise.
    #[inline]
    pub fn pop_f64(&mut self) -> RResult<f64> {
        match self.pop()? {
            StackEntity::Val(Value::F64(v)) => Ok(v),
            _ => Err(Trap::UnexpectedStackEntity(RType::F64)),
        }
    }

    /// Method that tries to find current activation frame.
    #[inline]
    pub fn get_current_frame(&self) -> Option<&'a Frame> {
        for se in self.inner.iter().rev() {
            if let StackEntity::Frame(frame) = se {
                return Some(&frame);
            }
        }
        None
    }
}

#[cfg(test)]
pub fn dump<'a>(stack: &'a Stack) -> Vec<StackEntity<'a>> {
    stack.inner.clone()
}
