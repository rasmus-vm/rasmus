use crate::runtime::{
    stack::Stack, stack::StackEntity, store::Store, trap::RResult, values::Value,
};

/// WebAssembly `i32.const val` instruction.
/// Push value to the Stack.
pub fn i32_const(_store: &mut Store, stack: &mut Stack, val: u32) -> RResult<()> {
    stack.push(StackEntity::Val(Value::I32(val)))
}

/// WebAssembly `i64.const val` instruction.
/// Push value to the Stack.
pub fn i64_const(_store: &mut Store, stack: &mut Stack, val: u64) -> RResult<()> {
    stack.push(StackEntity::Val(Value::I64(val)))
}

/// WebAssembly `f32.const val` instruction.
/// Push value to the Stack.
pub fn f32_const(_store: &mut Store, stack: &mut Stack, val: f32) -> RResult<()> {
    stack.push(StackEntity::Val(Value::F32(val)))
}

/// WebAssembly `f64.const val` instruction.
/// Push value to the Stack.
pub fn f64_const(_store: &mut Store, stack: &mut Stack, val: f64) -> RResult<()> {
    stack.push(StackEntity::Val(Value::F64(val)))
}
