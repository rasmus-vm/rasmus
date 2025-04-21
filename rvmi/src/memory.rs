use crate::user_error::RUserError;

pub trait LinearMemory: Sized {
    /// Method to write binary data into memory using provided offset.
    /// Can return `RUserError::NotEnoughMemory` if the linear memory capacity
    /// is not enough to write `data` with a provided `offset`.
    ///
    /// `write` will override existing data if ther is any at the moment of write.
    fn write(&mut self, data: &[u8], offset: usize) -> Result<(), RUserError>;
    
    /// Method to read data from linear memory. It will try to read as much data
    /// as `buffer.len()` and write it to the `buffer`. When reading it will start
    /// from a provided `offset`.
    ///
    /// As a retult it will return a number of bytes that were read and written into
    /// the buffer in case of `Ok`.
    fn read(&mut self, buffer: &mut [u8], offset: usize) -> Result<usize, RUserError>;
}
