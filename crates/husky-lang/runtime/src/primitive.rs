#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveValue {
    I32(i32),
    F32(f32),
    B32(u32),
    Bool(bool),
}

impl From<i32> for PrimitiveValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<f32> for PrimitiveValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<u32> for PrimitiveValue {
    fn from(value: u32) -> Self {
        Self::B32(value)
    }
}

impl From<bool> for PrimitiveValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
