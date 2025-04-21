pub trait AsSigned<T>: Copy {
    fn as_signed(&self) -> T;
}

impl AsSigned<i8> for u8 {
    fn as_signed(&self) -> i8 {
        *self as i8
    }
}

impl AsSigned<i16> for u16 {
    fn as_signed(&self) -> i16 {
        *self as i16
    }
}

impl AsSigned<i32> for u32 {
    fn as_signed(&self) -> i32 {
        *self as i32
    }
}

impl AsSigned<i64> for u64 {
    fn as_signed(&self) -> i64 {
        *self as i64
    }
}
