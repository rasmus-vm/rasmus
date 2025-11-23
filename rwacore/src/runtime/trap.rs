//! Rasmus `Trap` and `Result`.

use crate::types::RType;
pub type RResult<T> = Result<T, Trap>;

/// Rasmus Trap enum.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Trap {
    /// Rasmus is unable to allocate stack. Probably because all memory allocated
    /// for the VM is already used.
    UnableToAllocateStack,
    /// Unable to push `StackEntry` to `Stack`. Stack overflow.
    StackOverflow,
    /// Unable to pop `StackEntry` from `Stack`. Stack is empty.
    EmptyStackOnPop,
    /// No execution module was provided during local function execution.
    MissingExecutionModule,
    /// Not allowed memory was attempted to reach.
    SegmentationFault,
    /// Memory was attempted to initialise or grow beyond system limit of
    /// max allowed memory.
    MemoryExceededSytemLimit,
    /// Not enough free memory to write to a Memory.
    NotEnoughMemory,
    /// Current Frame was not found during an instruction execution.
    CurrentFrameNotFound,
    /// Unable to find memory address by its index in the Module instance.
    MemoryNotFound,
    /// Value update out of type range.
    ValueOverflow,
    /// Unexpected stack entity. Expected entity type is provided.
    UnexpectedStackEntity(RType)
}
