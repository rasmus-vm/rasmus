use crate::entities::{
    instructions::InstructionType,
    types::{NumType, ValType},
};

use super::validation_error::{ValidationError, ValidationResult};

pub struct ValidationStack {
    vals: Vec<ValidationType>,
    ctrls: Vec<CtrlFrame>,
}

impl ValidationStack {
    pub fn new() -> Self {
        ValidationStack {
            vals: vec![],
            ctrls: vec![],
        }
    }

    pub fn push_val(&mut self, t: ValidationType) {
        self.vals.push(t);
    }

    pub fn pop_val(&mut self) -> ValidationResult<ValidationType> {
        let frame = self
            .ctrls
            .get(0)
            .ok_or_else(|| ValidationError::ControlFrameNotFound)?;

        if self.vals.len() == frame.height && frame.unreachable {
            return Ok(ValidationType::Unknown);
        }

        if self.vals.len() == frame.height {
            return Err(ValidationError::InsufficientOperandStackForInstruction);
        }

        self.vals
            .pop()
            .ok_or(ValidationError::InsufficientOperandStackForInstruction)
    }

    pub fn pop_val_expect(&mut self, expected: ValidationType) -> ValidationResult<ValidationType> {
        let actual = self.pop_val()?;

        if actual != expected
            && actual != ValidationType::Unknown
            && expected != ValidationType::Unknown
        {
            return Err(ValidationError::UnexpectedType { actual, expected });
        }

        Ok(actual)
    }

    pub fn push_vals(&mut self, types: Vec<VType>) {
        for t in types {
            self.push_val(ValidationType::Known(t));
        }
    }

    pub fn push_vals_2(&mut self, types: Vec<ValidationType>) {
        for t in types {
            self.push_val(t);
        }
    }

    pub fn pop_vals(
        &mut self,
        expected: &Vec<ValidationType>,
    ) -> ValidationResult<Vec<ValidationType>> {
        let mut popped = Vec::with_capacity(expected.len());

        for t in expected.iter().rev() {
            popped.push(self.pop_val_expect(t.clone())?);
        }

        popped.reverse();

        Ok(popped)
    }

    pub fn push_ctrl(
        &mut self,
        opcode: InstructionType,
        start_types: Vec<ValType>,
        end_types: Vec<ValType>,
        unreachable: bool,
    ) {
        self.ctrls.push(CtrlFrame {
            opcode,
            start_types: start_types.iter().map(From::from).collect(),
            end_types: end_types.iter().map(From::from).collect(),
            height: self.vals.len(),
            unreachable,
        });

        self.push_vals(start_types.iter().map(From::from).collect());
    }

    pub fn pop_ctrl(&mut self) -> ValidationResult<CtrlFrame> {
        if self.ctrls.is_empty() {
            return Err(super::validation_error::ValidationError::FrameNotFound);
        }

        let frame = self
            .ctrls
            .get(0)
            .expect("Should return CtrlFrame due to the previous check");

        let end_types = frame.end_types.clone();
        let height = frame.height;

        self.pop_vals(&end_types)?;

        if self.vals.len() != height {
            return Err(ValidationError::InsufficientOperandStackForInstruction);
        }

        Ok(self
            .ctrls
            .pop()
            .expect("Should return CtrlFrame due to the previous check"))
    }

    pub fn unreachable(&mut self) -> ValidationResult<()> {
        let height = self
            .ctrls
            .get(0)
            .map(|f| f.height)
            .ok_or(ValidationError::FrameNotFound)?;

        self.vals.truncate(height);

        match self.ctrls.get_mut(0) {
            Some(ref mut frame) => {
                frame.unreachable = true;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn ctrl_len(&self) -> usize {
        self.ctrls.len()
    }

    pub fn get_ctrl(&self, i: usize) -> Option<&CtrlFrame> {
        self.ctrls.get(i)
    }
}

pub fn label_types(frame: &CtrlFrame) -> &Vec<ValidationType> {
    if let InstructionType::Loop(_) = frame.opcode {
        &frame.start_types
    } else {
        &frame.end_types
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationType {
    Known(VType),
    Unknown,
}

impl ValidationType {
    pub fn is_num(&self) -> bool {
        match self {
            ValidationType::Known(val) => {
                val == &VType::I32 || val == &VType::I64 || val == &VType::F32 || val == &VType::F64
            }
            ValidationType::Unknown => true,
        }
    }

    pub fn is_vec(&self) -> bool {
        match self {
            ValidationType::Known(val) => val == &VType::V128,
            ValidationType::Unknown => true,
        }
    }

    #[allow(dead_code)]
    pub fn is_ref(&self) -> bool {
        match self {
            ValidationType::Known(val) => val == &VType::Ref,
            ValidationType::Unknown => true,
        }
    }

    pub fn is_unknown(&self) -> bool {
        self == &ValidationType::Unknown
    }

    pub fn i32() -> Self {
        ValidationType::Known(VType::I32)
    }

    pub fn i64() -> Self {
        ValidationType::Known(VType::I64)
    }

    pub fn f32() -> Self {
        ValidationType::Known(VType::F32)
    }

    pub fn f64() -> Self {
        ValidationType::Known(VType::F64)
    }

    pub fn v128() -> Self {
        ValidationType::Known(VType::V128)
    }

    pub fn reference() -> Self {
        ValidationType::Known(VType::Ref)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref,
}

impl<T: Into<VType>> From<T> for ValidationType {
    fn from(t: T) -> Self {
        ValidationType::Known(t.into())
    }
}

impl From<ValType> for VType {
    fn from(val_type: ValType) -> Self {
        match val_type {
            ValType::NumType(NumType::I32) => VType::I32,
            ValType::NumType(NumType::I64) => VType::I64,
            ValType::NumType(NumType::F32) => VType::F32,
            ValType::NumType(NumType::F64) => VType::F64,
            ValType::VecType(_) => VType::V128,
            ValType::RefType(_) => VType::Ref,
        }
    }
}

impl From<&ValType> for VType {
    fn from(val_type: &ValType) -> Self {
        match val_type {
            &ValType::NumType(NumType::I32) => VType::I32,
            &ValType::NumType(NumType::I64) => VType::I64,
            &ValType::NumType(NumType::F32) => VType::F32,
            &ValType::NumType(NumType::F64) => VType::F64,
            &ValType::VecType(_) => VType::V128,
            &ValType::RefType(_) => VType::Ref,
        }
    }
}

#[derive(Debug)]
pub struct CtrlFrame {
    pub opcode: InstructionType,
    pub start_types: Vec<ValidationType>,
    pub end_types: Vec<ValidationType>,
    pub height: usize,
    pub unreachable: bool,
}
