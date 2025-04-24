use crate::runtime::{stack::Stack, store::Store, trap::RResult, stack::StackEntity, values::Value};

/// WebAssembly `i32.const val` instruction.
/// Push value to the Stack.
pub fn i32_const(_store: &mut Store, stack: &mut Stack, val: u32) -> RResult<()> {
    stack.push(StackEntity::Val(Value::I32(val)))
}
