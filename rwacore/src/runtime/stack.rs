//! Virtual Stack.

use super::{
    trap::{RResult, Trap},
    values::Value,
};
use alloc::vec::Vec;

/// Entity that can be pushed an popped from
/// the Virtual Stack.
#[derive(Clone, Debug, PartialEq)]
pub enum StackEntity {
    Val(Value),
}

/// Virtual Stack used by a program.
pub struct Stack {
    inner: Vec<StackEntity>,
}

impl Stack {
    /// Method that allocates a `Stack` for a program.
    pub fn new(max_capacity: usize) -> RResult<Self> {
        // TODO: add check with VM memory allocator
        // if stack can be allocated
        Ok(Stack {
            inner: Vec::with_capacity(max_capacity),
        })
    }

    /// Push `StackEntry` to `Stack`.
    pub fn push(&mut self, stack_entity: StackEntity) -> RResult<()> {
        if self.inner.len() == self.inner.capacity() {
            return Err(Trap::StackOverflow);
        }
        self.inner.push(stack_entity);
        Ok(())
    }

    /// Pop `StackEntry` from `Stack`.
    pub fn pop(&mut self) -> RResult<StackEntity> {
        self.inner.pop().ok_or_else(|| Trap::EmptyStackOnPop)
    }

    #[cfg(test)]
    pub fn dump(&self) -> Vec<StackEntity> {
        self.inner.clone()
    }
}
