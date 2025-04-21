pub trait AsSigned {
    type Output;

    fn as_signed(&self) -> Self::Output;
}

impl AsSigned for u32 {
    type Output = i32;

    fn as_signed(&self) -> Self::Output {
        return *self as i32;
    }
}

impl AsSigned for u64 {
    type Output = i64;

    fn as_signed(&self) -> Self::Output {
        return *self as i64;
    }
}

pub trait Min {
    fn get_min(&self, rhs: Self) -> Self;
}

impl Min for f32 {
    fn get_min(&self, rhs: Self) -> Self {
        if self.is_nan() || rhs.is_nan() {
            return f32::NAN;
        }
        self.min(rhs)
    }
}

impl Min for f64 {
    fn get_min(&self, rhs: Self) -> Self {
        if self.is_nan() || rhs.is_nan() {
            return f64::NAN;
        }
        self.min(rhs)
    }
}

pub trait Max {
    fn get_max(&self, rhs: Self) -> Self;
}

impl Max for f32 {
    fn get_max(&self, rhs: Self) -> Self {
        if self.is_nan() || rhs.is_nan() {
            return f32::NAN;
        }
        self.max(rhs)
    }
}

impl Max for f64 {
    fn get_max(&self, rhs: Self) -> Self {
        if self.is_nan() || rhs.is_nan() {
            return f64::NAN;
        }
        self.max(rhs)
    }
}
