use super::frame::Frame;
use super::label::LabelInst;
use super::ref_inst::RefInst;
use super::value::Val;

#[derive(Debug, Clone)]
pub enum StackEntry {
    Value(Val),
    Label(LabelInst),
    Frame(Frame),
}

impl StackEntry {
    #[allow(dead_code)]
    pub fn is_value(&self) -> bool {
        match self {
            StackEntry::Value(_) => true,
            _ => false,
        }
    }

    pub fn is_label(&self) -> bool {
        match self {
            StackEntry::Label(_) => true,
            _ => false,
        }
    }

    pub fn is_frame(&self) -> bool {
        match self {
            StackEntry::Frame(_) => true,
            _ => false,
        }
    }
}

pub struct Stack {
    stack: Vec<StackEntry>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: vec![] }
    }

    pub fn pop(&mut self) -> Option<StackEntry> {
        self.stack.pop()
    }

    pub fn push_entry(&mut self, entry: StackEntry) {
        self.stack.push(entry);
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(StackEntry::Frame(frame));
    }

    pub fn push_value(&mut self, value: Val) {
        self.stack.push(StackEntry::Value(value));
    }

    pub fn push_label(&mut self, label: LabelInst) {
        self.stack.push(StackEntry::Label(label));
    }

    pub fn last(&self) -> Option<&StackEntry> {
        self.stack.last()
    }

    pub fn count_labels(&self) -> usize {
        self.stack.iter().rev().fold(0, |count, entry| match entry {
            StackEntry::Label(_) => count + 1,
            _ => count,
        })
    }

    pub fn get_label(&self, label_idx: usize) -> Option<&LabelInst> {
        let mut i = label_idx;

        for entry in self.stack.iter().rev() {
            if let StackEntry::Label(label) = entry {
                if i == 0 {
                    return Some(label);
                }
                i -= 1;
            }
        }

        None
    }

    pub fn pop_value(&mut self) -> Option<Val> {
        match self.stack.pop() {
            Some(StackEntry::Value(val)) => Some(val),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_i32(&mut self) -> Option<u32> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::I32(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_i64(&mut self) -> Option<u64> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::I64(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_f32(&mut self) -> Option<f32> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::F32(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_f64(&mut self) -> Option<f64> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::F64(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_v128(&mut self) -> Option<u128> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::Vec(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        if let Some(stack_entry) = self.stack.last() {
            if stack_entry.is_frame() {
                if let Some(StackEntry::Frame(frame)) = self.stack.pop() {
                    return Some(frame);
                }
            }
        }

        None
    }

    pub fn pop_ref(&mut self) -> Option<RefInst> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::Ref(r))) => Some(r),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_label(&mut self) -> Option<LabelInst> {
        if let Some(stack_entry) = self.stack.last() {
            if stack_entry.is_label() {
                if let Some(StackEntry::Label(label)) = self.stack.pop() {
                    return Some(label);
                }
            }
        }

        None
    }

    pub fn current_frame(&mut self) -> Option<&mut Frame> {
        self.stack.iter_mut().rev().find_map(|entry| match entry {
            StackEntry::Frame(frame) => Some(frame),
            _ => None,
        })
    }
}
