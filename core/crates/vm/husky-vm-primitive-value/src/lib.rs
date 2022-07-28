#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum PrimitiveValueData {
    I32(i32),
    I64(i64),
    F32(f32),
    B32(u32),
    B64(u64),
    Bool(bool),
    Void(()),
}

impl PartialEq for PrimitiveValueData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => *l0 as u32 == *r0 as u32,
            (Self::B32(l0), Self::B32(r0)) => l0 == r0,
            (Self::B64(l0), Self::B64(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Void(l0), Self::Void(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl std::hash::Hash for PrimitiveValueData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            PrimitiveValueData::I32(i) => i.hash(state),
            PrimitiveValueData::I64(_) => todo!(),
            PrimitiveValueData::F32(f) => {
                assert!(!f.is_nan());
                (*f as u64).hash(state)
            }
            PrimitiveValueData::B32(_) => todo!(),
            PrimitiveValueData::B64(_) => todo!(),
            PrimitiveValueData::Bool(_) => todo!(),
            PrimitiveValueData::Void(_) => todo!(),
        }
    }
}
impl Eq for PrimitiveValueData {}

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

impl From<i64> for PrimitiveValueData {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<&i64> for PrimitiveValueData {
    fn from(value: &i64) -> Self {
        Self::I64(*value)
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

impl From<&f64> for PrimitiveValueData {
    fn from(value: &f64) -> Self {
        todo!()
        // Self::F64(*value)
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
