//! WebAssembly Memory instructions
use crate::runtime::{
    instances::MemoryInst,
    stack::Stack,
    store::Store,
    trap::{RResult, Trap},
    values::{N32_BYTES_LEN, N64_BYTES_LEN},
};

use super::numeric::i32_const;

const ERR_RESP: u32 = -1i32 as u32;

/// Memory argument used in `t.store memarg` and other instrucitons.
pub struct MemArg {
    pub offset: u32,
    pub align: u32,
}

/// WebAssembly `memory.size` instruction.
/// [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#xref-syntax-instructions-syntax-instr-memory-mathsf-memory-grow).
pub fn size(store: &mut Store, stack: &mut Stack) -> RResult<()> {
    let memory_inst = get_memory_inst(store, stack)?;
    let size = memory_inst.capacity();
    i32_const(store, stack, size as u32)
}

/// WebAssembly `memory.grow` instruction.
/// [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#xref-syntax-instructions-syntax-instr-memory-mathsf-memory-grow).
pub fn grow(store: &mut Store, stack: &mut Stack) -> RResult<()> {
    let n = stack.pop_i32()?;
    let memory_inst = get_memory_inst(store, stack)?;
    let old_size = memory_inst.capacity();
    let res = match memory_inst.allocate(n as usize) {
        Ok(_) => old_size as u32,
        Err(_) => ERR_RESP,
    };
    i32_const(store, stack, res)
}

/// Stores `I32` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i32_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_i32()?;
    let i = stack.pop_i32()?;
    let ea = (i + memarg.offset) as usize;
    let n = N32_BYTES_LEN;
    let memory_inst = get_memory_inst(store, stack)?;

    if ea + n > memory_inst.capacity() {
        return Err(Trap::NotEnoughMemory);
    }

    let b = c.to_le_bytes();
    memory_inst.write(&b, ea).map(|_| (()))
}

/// Stores `I64` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i64_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_i64()?;
    let i = stack.pop_i32()?;
    let ea = (i + memarg.offset) as usize;
    let n = N64_BYTES_LEN;
    let memory_inst = get_memory_inst(store, stack)?;

    if ea + n > memory_inst.capacity() {
        return Err(Trap::NotEnoughMemory);
    }

    let b = c.to_le_bytes();
    memory_inst.write(&b, ea).map(|_| (()))
}

/// Stores `F32` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn f32_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_f32()?;
    let i = stack.pop_i32()?;
    let ea = (i + memarg.offset) as usize;
    let n = N32_BYTES_LEN;
    let memory_inst = get_memory_inst(store, stack)?;

    if ea + n > memory_inst.capacity() {
        return Err(Trap::NotEnoughMemory);
    }

    let b = c.to_le_bytes();
    memory_inst.write(&b, ea).map(|_| (()))
}

/// Stores `F64` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn f64_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_f64()?;
    let i = stack.pop_i32()?;
    let ea = (i + memarg.offset) as usize;
    let n = N64_BYTES_LEN;
    let memory_inst = get_memory_inst(store, stack)?;

    if ea + n > memory_inst.capacity() {
        return Err(Trap::NotEnoughMemory);
    }

    let b = c.to_le_bytes();
    memory_inst.write(&b, ea).map(|_| (()))
}

fn get_memory_inst<'a>(store: &'a mut Store, stack: &'a mut Stack) -> RResult<&'a mut MemoryInst> {
    let current_frame = stack
        .get_current_frame()
        .ok_or(Trap::CurrentFrameNotFound)?;
    let memory_address = current_frame
        .module
        .get_memaddr(0)
        .ok_or(Trap::MemoryNotFound)?;
    store
        .get_memory_mut(memory_address)
        .ok_or(Trap::MemoryNotFound)
}

#[cfg(test)]
mod test {

    use crate::instructions::memory::grow;
    use crate::instructions::numeric::i32_const;
    use crate::mem_instr_test_bed;
    use crate::{
        runtime::{
            instances::MemoryInst,
            stack::{Stack, StackEntity, dump},
            store::Store,
            values::Value,
        },
        types::{LimitsType, MemType},
    };

    use super::size;

    #[test]
    fn test_size() {
        // TODO: make it reusable
        // Test Bed preparation
        let memory_size = 9;
        let system_limit_max = 9;
        let memory_inst = MemoryInst::new(
            MemType(LimitsType {
                min: memory_size,
                max: None,
            }),
            system_limit_max,
        )
        .expect("should create memory instance");
        let mut store = Store::empty();
        let mut stack = Stack::new(3).expect("should create stack instance");
        mem_instr_test_bed!(store, stack, memory_inst);

        // test execution
        size(&mut store, &mut stack).expect("should execute instruction without error");
        let stack_dump = dump(&stack);
        assert_eq!(
            stack_dump.len(),
            2,
            "should push to stack single entry (Frame is kept)"
        );
        match stack_dump.last() {
            Some(StackEntity::Val(Value::I32(s))) => {
                assert_eq!(*s, memory_size, "should properly return size")
            }
            ref entry => assert!(
                false,
                "should properly return size (wrong stack entry type {entry:?})"
            ),
        }
    }

    #[test]
    fn test_grow() {
        // TODO: make it reusable
        // Test Bed preparation
        let memory_size = 0;
        let system_limit_max = MemoryInst::PAGE_SIZE * 1;
        let memory_inst = MemoryInst::new(
            MemType(LimitsType {
                min: memory_size,
                max: None,
            }),
            system_limit_max,
        )
        .expect("should create memory instance");
        let mut store = Store::empty();
        let mut stack = Stack::new(3).expect("should create stack instance");
        mem_instr_test_bed!(store, stack, memory_inst);
        // successfull grow
        i32_const(&mut store, &mut stack, 1).expect("should push without errors");
        grow(&mut store, &mut stack).expect("should execute grow without errors");
        let stack_dump = dump(&stack);
        match stack_dump.last() {
            Some(StackEntity::Val(Value::I32(s))) => {
                assert!(*s as i32 >= 0, "should grow without errors")
            }
            ref entry => assert!(
                false,
                "should properly return size (wrong stack entry type {entry:?})"
            ),
        }
        // error
        i32_const(&mut store, &mut stack, 1).expect("should push without errors");
        grow(&mut store, &mut stack).expect("should execute grow without errors");
        let stack_dump = dump(&stack);
        match stack_dump.last() {
            Some(StackEntity::Val(Value::I32(s))) => {
                assert_eq!(*s as i32, -1, "should return error")
            }
            ref entry => assert!(
                false,
                "should properly return size (wrong stack entry type {entry:?})"
            ),
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! mem_instr_test_bed {
    ($store: ident, $stack: ident, $memory_inst: ident) => {
        let mem_addr = $store.add_memory($memory_inst);
        let mut mem_addrs = alloc::vec::Vec::with_capacity(1);
        mem_addrs.push(mem_addr);
        let module_inst = $crate::runtime::instances::ModuleInstBuilder::new()
            .with_memory(mem_addrs)
            .build();
        let stack_frame = $crate::runtime::stack::StackEntity::Frame($crate::runtime::Frame {
            arity: 0,
            locals: alloc::vec::Vec::new(),
            module: &module_inst,
        });
        $stack.push(stack_frame).expect("should push to stack");
    };
}
