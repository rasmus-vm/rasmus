use alloc::vec::Vec;

use crate::{
    runtime::trap::{RResult, Trap},
    types::MemType,
};

/// WebAssembly memory instance.
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
    /// It may return `Trap::MemoryExceededSytemLimit` if `min` exceeds
    /// `system_limit_max`.
    pub fn new(m_type: MemType, system_limit_max: usize) -> RResult<Self> {
        if system_limit_max < m_type.0.min as usize {
            return Err(Trap::MemoryExceededSytemLimit);
        }

        let capacity = m_type.0.min as usize;

        Ok(MemoryInst {
            m_type,
            data: Vec::with_capacity(capacity),
            system_limit_max,
        })
    }

    /// Allocates `np` memory pages (65,536 bytes) more for the current Memory Instance.
    ///
    /// Method may return `Trap::MemoryExceededSytemLimit` if the requested memory combined
    /// with already allocated memory will exceed `system_limit_max` that was used during
    /// current Memory Instance creation.
    pub fn allocate(&mut self, np: usize) -> RResult<()> {
        let bytes_to_allocate = np * Self::PAGE_SIZE;
        let current_capacity = self.data.capacity();
        let new_capacity = current_capacity + bytes_to_allocate;

        if new_capacity > self.system_limit_max
            || (self.m_type.0.max.is_some() && new_capacity > self.m_type.0.max.unwrap() as usize)
        {
            return Err(Trap::MemoryExceededSytemLimit);
        }

        self.data.reserve_exact(bytes_to_allocate);

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
}

#[cfg(test)]
mod test {
    use crate::{
        runtime::{instances::MemoryInst, trap::Trap},
        types::{LimitsType, MemType},
    };

    #[test]
    fn test_new() {
        let m_type = MemType(LimitsType {
            min: 1,
            max: Some(100),
        });
        let system_limit_max = 4 * 1024;
        let inst_res = MemoryInst::new(m_type, system_limit_max);
        assert!(inst_res.is_ok(), "should create MemoryInst");
        let m_type = MemType(LimitsType {
            min: system_limit_max as u32 + 1,
            max: None,
        });
        let inst_res = MemoryInst::new(m_type, system_limit_max);
        assert_eq!(inst_res.unwrap_err(), Trap::MemoryExceededSytemLimit);
    }

    #[test]
    fn test_write_ok() {}

    fn test_write_trap() {}
}
