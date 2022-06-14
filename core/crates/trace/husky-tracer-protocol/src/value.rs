use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PrimitiveValueData {
    I32(i32),
    F32(f32),
    B32(u32),
    B64(u64),
    Bool(bool),
    Void(()),
}

impl From<()> for PrimitiveValueData {
    fn from(_: ()) -> Self {
        Self::Void(())
    }
}

impl From<i32> for PrimitiveValueData {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<&i32> for PrimitiveValueData {
    fn from(value: &i32) -> Self {
        Self::I32(*value)
    }
}

impl From<f32> for PrimitiveValueData {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<&f32> for PrimitiveValueData {
    fn from(value: &f32) -> Self {
        Self::F32(*value)
    }
}

impl From<u32> for PrimitiveValueData {
    fn from(value: u32) -> Self {
        Self::B32(value)
    }
}

impl From<&u32> for PrimitiveValueData {
    fn from(value: &u32) -> Self {
        Self::B32(*value)
    }
}

impl From<u64> for PrimitiveValueData {
    fn from(value: u64) -> Self {
        Self::B64(value)
    }
}

impl From<&u64> for PrimitiveValueData {
    fn from(value: &u64) -> Self {
        Self::B64(*value)
    }
}

impl From<bool> for PrimitiveValueData {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&bool> for PrimitiveValueData {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}
