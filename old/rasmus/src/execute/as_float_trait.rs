pub trait AsFloat<T>: Copy {
    fn as_float(&self) -> T;
}

impl AsFloat<f32> for u32 {
    fn as_float(&self) -> f32 {
        f32::from_be_bytes(self.to_be_bytes())
    }
}

impl AsFloat<f64> for u64 {
    fn as_float(&self) -> f64 {
        f64::from_be_bytes(self.to_be_bytes())
    }
}
