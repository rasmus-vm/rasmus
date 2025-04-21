use alloc::vec::Vec;

use crate::{
    runtime::trap::{RResult, Trap},
    types::MemType,
};

/// WebAssembly memory instance.
///
/// Internally this structure uses `alloc::vec::Vec` which means
/// that to start using in binary crates [`#[global_allocator]`](https://doc.rust-lang.org/stable/alloc/alloc/trait.GlobalAlloc.html)
/// must be defined.
#[derive(Debug)]
pub struct MemoryInst {
    m_type: MemType,
    data: Vec<u8>,
    system_limit_max: usize,
}

impl MemoryInst {
    /// Memory page size in bytes.
    pub const PAGE_SIZE: usize = 65_536;

    /// Creates new memory instance.
    /// It allocates memory provided as `min` value of `MemType`.
    ///
    /// System limit is typical provided as a virtual machine configuration
    /// and is common for all WASM module instances.
    ///
    /// It may return `Trap::MemoryExceededSytemLimit` if `min` exceeds
    /// `system_limit_max`.
    pub fn new(m_type: MemType, system_limit_max: usize) -> RResult<Self> {
        if system_limit_max < m_type.0.min as usize {
            return Err(Trap::MemoryExceededSytemLimit);
        }

        let capacity = m_type.0.min as usize;
        let mut data = Vec::with_capacity(capacity);
        data.resize(capacity, 0);

        Ok(MemoryInst {
            m_type,
            data,
            system_limit_max,
        })
    }

    /// Allocates `np` memory pages (65,536 bytes each) more for the current Memory Instance.
    ///
    /// Method may return `Trap::MemoryExceededSytemLimit` if the requested memory combined
    /// with already allocated memory will exceed `system_limit_max that was used during
    /// current Memory Instance creation or it will exceed `max` from the `m_type` if it contains
    /// one.
    pub fn allocate(&mut self, np: usize) -> RResult<()> {
        let bytes_to_allocate = np * Self::PAGE_SIZE;
        let new_capacity = self.data.len() + bytes_to_allocate;

        if new_capacity > self.system_limit_max
            || (self.m_type.0.max.is_some() && new_capacity > self.m_type.0.max.unwrap() as usize)
        {
            return Err(Trap::MemoryExceededSytemLimit);
        }

        self.data.resize(new_capacity, 0);

        Ok(())
    }

    /// Writes bytes from `data` to the linear memory. It will start writing new
    /// data starting from `offset` position. Old data if there is any will be
    /// overrited.
    ///
    /// Method may return `Trap::NotEnoughMemory` in case if internal storage capacity
    /// minus the offset is less then number of bytes in `data` buffer.
    pub fn write(&mut self, data: &[u8], offset: usize) -> RResult<()> {
        let bytes_to_write = data.len();

        if self.data.len() < offset + bytes_to_write {
            return Err(Trap::NotEnoughMemory);
        }

        for i in 0..bytes_to_write {
            self.data[offset + i] = data[i];
        }

        Ok(())
    }

    /// Reads bytes from memory starting from the `offset` bytes. Result is being
    /// written to `read_buf`. Method will try to read `read_buf.len()` number of
    /// bytes. It will retun number of bytes that were actually read and written
    /// to the buffer as a result.
    #[inline]
    pub fn read(&self, read_buf: &mut [u8], offset: usize) -> RResult<usize> {
        if self.data.len() < offset {
            return Ok(0);
        }

        let memory_has_bytes = self.data.len() - offset;
        let bytes_requested = read_buf.len();
        let bytes_to_read = memory_has_bytes.min(bytes_requested);

        for i in 0..bytes_to_read {
            read_buf[i] = self.data[offset + i];
        }

        Ok(bytes_to_read)
    }

    /// Returns Memory instance capacity.
    pub fn capacity(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        runtime::{instances::MemoryInst, trap::Trap},
        types::{LimitsType, MemType},
    };

    #[test]
    fn test_new() {
        let min = 1;
        let mem_type = MemType(LimitsType { min, max: None });
        let system_limit_max = 10;
        let mem_inst_res = MemoryInst::new(mem_type, system_limit_max);
        assert!(
            mem_inst_res.is_ok(),
            "should create MemoryInst without errors"
        );
        assert_eq!(mem_inst_res.unwrap().data.capacity(), min as usize);

        let system_limit_max = 10;
        let mem_type = MemType(LimitsType {
            min: system_limit_max + 1,
            max: None,
        });
        let mem_inst_res = MemoryInst::new(mem_type, system_limit_max as usize);
        assert_eq!(
            mem_inst_res.expect_err("should return error if min > system_limit_max"),
            Trap::MemoryExceededSytemLimit
        );
    }

    #[test]
    fn test_allocate() {
        // bellow system_limit_max and max None
        let mem_type = MemType(LimitsType {
            min: MemoryInst::PAGE_SIZE as u32,
            max: None,
        });
        let system_limit_max = MemoryInst::PAGE_SIZE * 2;
        let mut mem_inst = MemoryInst::new(mem_type, system_limit_max)
            .expect("shuld create memory without errors");
        mem_inst.allocate(1).expect("should allocate 1 memory page without errors when capacity is bellow system_limit_max and MemType max");
        // above system limit
        mem_inst
            .allocate(1)
            .expect_err("should return error when goes above system_limit_max");

        // above max Some (system_limit_max > max)
        let mem_type = MemType(LimitsType {
            min: MemoryInst::PAGE_SIZE as u32,
            max: Some(MemoryInst::PAGE_SIZE as u32 * 2),
        });
        let system_limit_max = MemoryInst::PAGE_SIZE * 3;
        let mut mem_inst = MemoryInst::new(mem_type, system_limit_max)
            .expect("shuld create memory without errors");
        mem_inst
            .allocate(2)
            .expect_err("should return error when capacity goes above MemType max value");
    }

    #[test]
    fn test_write() {
        let mem_type = MemType(LimitsType { min: 10, max: None });
        let system_limit_max = MemoryInst::PAGE_SIZE * 2;
        let mut mem_inst = MemoryInst::new(mem_type, system_limit_max)
            .expect("shuld create memory without errors");

        let to_write = [2u8; 5];
        mem_inst
            .write(&to_write, 0)
            .expect("should write data without error when there is enough capacity");
        assert_eq!(
            to_write,
            mem_inst.data[0..to_write.len()],
            "memory should have data written"
        );

        mem_inst.write(&to_write, 8).expect_err(
            "should return error when there is not enough remaining capacity to write data",
        );
    }

    #[test]
    fn test_read() {
        let mem_type = MemType(LimitsType { min: 10, max: Some(10) });
        let system_limit_max = MemoryInst::PAGE_SIZE * 2;
        let mut mem_inst = MemoryInst::new(mem_type, system_limit_max)
            .expect("shuld create memory without errors");

        let to_write = [2u8; 5];
        mem_inst
            .write(&to_write, 5)
            .expect("should write data without error when there is enough capacity");

        // written bytes is less then the read buffer capacity
        let mut to_read = [0u8; 6];
        assert_eq!(mem_inst.read(&mut to_read, 5).expect("should read without errors"), 5);
        assert_eq!([2u8, 2u8, 2u8, 2u8, 2u8, 0u8], to_read);

        // written bytes is same as the read buffer capacity
        let mut to_read = [0u8; 2];
        assert_eq!(mem_inst.read(&mut to_read, 5).expect("should read without errors"), 2);
        assert_eq!([2u8, 2u8], to_read);
    }
}
