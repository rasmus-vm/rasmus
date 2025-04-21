use crate::entities::{
    instructions::InstructionType,
    types::{MemType, RefType, TableType},
};

use super::validation_stack::ValidationType;

// TODO: try to add more debugging information to each option
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    InsufficientOperandStackForInstruction,
    CannotFindRefFuncInValidationContext,
    LaneIndexIsOutOfRange {
        value: u8,
        max_allowed: u8,
    },
    // Length of SelectVec argument sequence should be equal to 1
    InvalidSelectVecOperandSequence,
    // When branches are neither both numbers nor both vectors
    InvalidSelectBranchTypes,
    LocalNotFound,
    GlobalNotFound,
    UnableToSetToConstGlobal,
    TableNotFound,
    // When do table.copy x y, table types of x and y must be the same
    UnableToCopyIncosistentTableTypes,
    ElemNotFound,
    // When init table elem type should be the same as a table's ref type
    WrongElemType,
    // When load a value memarg align should not be bigger than th bit width divided by 8
    MemargAlignTooBig,
    MemNotFound,
    LaneIdxTooBix,
    DataNotFound,
    TypeNotFound,
    ControlFrameNotFound,
    FrameNotFound,
    UnexpectedType {
        actual: ValidationType,
        expected: ValidationType,
    },
    UnexpectedRefType {
        actual: RefType,
        expected: RefType,
    },
    NotConsistentArity,
    ReturnNotFoundInContext,
    FuncTypeNotFound {
        func_idx: usize,
    },
    CodeNotFound,
    IfControlFrameIsExpected,
    InvalidTableType {
        table_type: TableType,
    },
    InvalidMemoryType {
        memory_type: MemType,
    },
    InvalidStartFunctionType,
    NonConstantInstruction {
        instruction: InstructionType,
    },
}

pub type ValidationResult<T> = Result<T, ValidationError>;
