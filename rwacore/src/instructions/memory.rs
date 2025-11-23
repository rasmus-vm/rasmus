//! WebAssembly Memory instructions
use crate::runtime::{
    instances::MemoryInst,
    stack::Stack,
    store::Store,
    trap::{RResult, Trap},
};

use super::numeric::{f32_const, f64_const, i32_const, i64_const};

const ERR_RESP: u32 = -1i32 as u32;

/// Memory argument used in `t.store memarg` and other instrucitons.
#[derive(PartialEq, Debug, Clone)]
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

pub fn fill(store: &mut Store, stack: &mut Stack) -> RResult<()> {
    let n = stack.pop_i32()?;
    let val = stack.pop_i32()?;
    let d = stack.pop_i32()?;

    let memory_inst = get_memory_inst(store, stack)?;

    if (d + n) as usize > memory_inst.capacity() {
        return Err(Trap::NotEnoughMemory);
    }

    if n == 0 {
        return Ok(());
    }

    i32_const(store, stack, d)?;
    i32_const(store, stack, val)?;

    i32_store8(
        store,
        stack,
        &MemArg {
            offset: 0,
            align: 0,
        },
    )?;

    if d.checked_add(1).is_none() {
        return Err(Trap::ValueOverflow);
    }

    i32_const(store, stack, d + 1)?;
    i32_const(store, stack, val)?;
    i32_const(store, stack, n - 1)?;

    fill(store, stack)
}

pub fn mem_copy(store: &mut Store, stack: &mut Stack) -> RResult<()> {
    let n = stack.pop_i32()?;
    let s = stack.pop_i32()?;
    let d = stack.pop_i32()?;
    let memory_inst = get_memory_inst(store, stack)?;
    let capacity = memory_inst.capacity();

    if ((s + n) as usize > capacity) || ((d + n) as usize > capacity) {
        return Err(Trap::NotEnoughMemory);
    }

    if n == 0 {
        return Ok(());
    }

    if d <= s {
        i32_const(store, stack, d)?;
        i32_const(store, stack, s)?;

        let zero_offset = MemArg {
            offset: 0,
            align: 0,
        };
        i32_load8u(store, stack, &zero_offset)?;
        i32_store8(store, stack, &zero_offset)?;

        if d.checked_add(1).is_none() {
            return Err(Trap::ValueOverflow);
        }

        i32_const(store, stack, d + 1)?;

        if s.checked_add(1).is_none() {
            return Err(Trap::ValueOverflow);
        }

        i32_const(store, stack, s + 1)?;
    } else {
        if d.checked_add(n - 1).is_none() {
            return Err(Trap::ValueOverflow);
        }
        i32_const(store, stack, d + n - 1)?;

        if s.checked_add(n - 1).is_none() {
            return Err(Trap::ValueOverflow);
        }
        i32_const(store, stack, s + n - 1)?;

        let zero_offset = MemArg {
            offset: 0,
            align: 0,
        };
        i32_load8u(store, stack, &zero_offset)?;
        i32_store8(store, stack, &zero_offset)?;

        i32_const(store, stack, d)?;
        i32_const(store, stack, s)?;
    }

    i32_const(store, stack, n - 1)?;
    mem_copy(store, stack)
}

macro_rules! store {
    ($c: expr, $store: expr, $stack: expr, $memarg: expr) => {{
        let i = $stack.pop_i32()?;
        let ea = (i + $memarg.offset) as usize;
        let b = $c.to_le_bytes();
        let n = b.len();
        let memory_inst = get_memory_inst($store, $stack)?;

        if ea + n > memory_inst.capacity() {
            return Err(Trap::NotEnoughMemory);
        }

        memory_inst.write(&b, ea).map(|_| (()))
    }};
}

/// Stores `I32` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i32_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_i32()?;
    store!(c, store, stack, memarg)
}

/// Stores `I64` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i64_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_i64()?;
    store!(c, store, stack, memarg)
}

/// Stores `F32` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn f32_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_f32()?;
    store!(c, store, stack, memarg)
}

/// Stores `F64` to memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn f64_store(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c = stack.pop_f64()?;
    store!(c, store, stack, memarg)
}

/// Stores `I32` to memory 8 bits wrapped. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i32_store8(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c_i32 = stack.pop_i32()?;
    store!(c_i32 % 2u32.pow(8), store, stack, memarg)
}

/// Stores `I32` to memory 16 bits wrapped. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i32_store16(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c_i32 = stack.pop_i32()?;
    store!(c_i32 % 2u32.pow(16), store, stack, memarg)
}

/// Stores `I64` to memory 8 bits wrapped. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i64_store8(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c_i64 = stack.pop_i64()?;
    store!(c_i64 % 2u64.pow(8), store, stack, memarg)
}

/// Stores `I64` to memory 16 bits wrapped. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i64_store16(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c_i64 = stack.pop_i64()?;
    store!(c_i64 % 2u64.pow(16), store, stack, memarg)
}

/// Stores `I64` to memory 32 bits wrapped. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-store-n-xref-syntax-instructions-syntax-memarg-mathit-memarg).
pub fn i64_store32(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let c_i64 = stack.pop_i64()?;
    store!(c_i64 % 2u64.pow(32), store, stack, memarg)
}

macro_rules! load {
    ($t: ty, $store: expr, $stack: expr, $memarg: expr) => {{
        let i = $stack.pop_i32()?;
        let ea = (i + $memarg.offset) as usize;
        let n = (<$t>::BITS / 8) as usize;
        let memory_inst = get_memory_inst($store, $stack)?;

        if ea + n > memory_inst.capacity() {
            return Err(Trap::NotEnoughMemory);
        }

        let mut buf = [0u8; (<$t>::BITS / 8) as usize];
        memory_inst.read(&mut buf, ea)?;

        <$t>::from_le_bytes(buf)
    }};
    ($t: ty, $bits: expr, $store: expr, $stack: expr, $memarg: expr) => {{
        let i = $stack.pop_i32()?;
        let ea = (i + $memarg.offset) as usize;
        let memory_inst = get_memory_inst($store, $stack)?;

        if ea + $bits > memory_inst.capacity() {
            return Err(Trap::NotEnoughMemory);
        }

        let mut buf = [0u8; $bits];
        memory_inst.read(&mut buf, ea)?;

        <$t>::from_le_bytes(buf)
    }};
    ($t: ty, $s: ty, $bits: expr, $store: expr, $stack: expr, $memarg: expr) => {{
        let i = $stack.pop_i32()?;
        let ea = (i + $memarg.offset) as usize;
        let memory_inst = get_memory_inst($store, $stack)?;

        if ea + $bits > memory_inst.capacity() {
            return Err(Trap::NotEnoughMemory);
        }

        let mut buf = [0u8; $bits];
        memory_inst.read(&mut buf, ea)?;

        <$t>::from_le_bytes(buf)
    }};
}

/// Loads `I32` from memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-n-mathsf-xref-syntax-instructions-syntax-sx-mathit-sx-xref-syntax-instructions-syntax-memarg-mathit-memarg)
pub fn i32_load(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n = load!(u32, store, stack, memarg);
    i32_const(store, stack, n)
}

/// Loads `I64` from memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-n-mathsf-xref-syntax-instructions-syntax-sx-mathit-sx-xref-syntax-instructions-syntax-memarg-mathit-memarg)
pub fn i64_load(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n = load!(u64, store, stack, memarg);
    i64_const(store, stack, n)
}

/// Loads `F32` from memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-n-mathsf-xref-syntax-instructions-syntax-sx-mathit-sx-xref-syntax-instructions-syntax-memarg-mathit-memarg)
pub fn f32_load(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n = load!(f32, 4, store, stack, memarg);
    f32_const(store, stack, n)
}

/// Loads `F64` from memory. [Spec](https://webassembly.github.io/spec/core/exec/instructions.html#t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-xref-syntax-instructions-syntax-memarg-mathit-memarg-and-t-mathsf-xref-syntax-instructions-syntax-instr-memory-mathsf-load-n-mathsf-xref-syntax-instructions-syntax-sx-mathit-sx-xref-syntax-instructions-syntax-memarg-mathit-memarg)
pub fn f64_load(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n = load!(f64, 8, store, stack, memarg);
    f64_const(store, stack, n)
}

/// Loads signed 8 bytes from memory and pushes its `I32` interpretation to the stack.
pub fn i32_load8s(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n8s = load!(i8, store, stack, memarg);
    i32_const(store, stack, n8s as u32)
}

/// Loads unsigned 8 bytes from memory and pushes its `I32` interpretation to the stack.
pub fn i32_load8u(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n8s = load!(u8, store, stack, memarg);
    i32_const(store, stack, n8s as u32)
}

/// Loads signed 16 bytes from memory and pushes its `I32` interpretation to the stack.
pub fn i32_load16s(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n16s = load!(i16, store, stack, memarg);
    i32_const(store, stack, n16s as u32)
}

/// Loads unsigned 16 bytes from memory and pushes its `I32` interpretation to the stack.
pub fn i32_load16u(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n16s = load!(u16, store, stack, memarg);
    i32_const(store, stack, n16s as u32)
}

/// Loads signed 8 bytes from memory and pushes its `I64` interpretation to the stack.
pub fn i64_load8s(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n8s = load!(i8, store, stack, memarg);
    i64_const(store, stack, n8s as u64)
}

/// Loads unsigned 8 bytes from memory and pushes its `I64` interpretation to the stack.
pub fn i64_load8u(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n8s = load!(u8, store, stack, memarg);
    i64_const(store, stack, n8s as u64)
}

/// Loads signed 16 bytes from memory and pushes its `I64` interpretation to the stack.
pub fn i64_load16s(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n16s = load!(i16, store, stack, memarg);
    i64_const(store, stack, n16s as u64)
}

/// Loads unsigned 16 bytes from memory and pushes its `I64` interpretation to the stack.
pub fn i64_load16u(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n16s = load!(u16, store, stack, memarg);
    i64_const(store, stack, n16s as u64)
}

/// Loads signed 32 bytes from memory and pushes its `I64` interpretation to the stack.
pub fn i64_load32s(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n32s = load!(i32, store, stack, memarg);
    i64_const(store, stack, n32s as u64)
}

/// Loads unsigned 32 bytes from memory and pushes its `I64` interpretation to the stack.
pub fn i64_load32u(store: &mut Store, stack: &mut Stack, memarg: &MemArg) -> RResult<()> {
    let n32s = load!(u32, store, stack, memarg);
    i64_const(store, stack, n32s as u64)
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
