use super::*;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum PrimitiveValueData {
    I32(i32),
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
        core::mem::discriminant(self).hash(state);
        match self {
            PrimitiveValueData::I32(_) => todo!(),
            PrimitiveValueData::F32(_) => todo!(),
            PrimitiveValueData::B32(_) => todo!(),
            PrimitiveValueData::B64(_) => todo!(),
            PrimitiveValueData::Bool(_) => todo!(),
            PrimitiveValueData::Void(_) => todo!(),
        }
    }
}
impl Eq for PrimitiveValueData {}

impl PrimitiveValueData {
    pub fn take_i32(self) -> i32 {
        if let PrimitiveValueData::I32(i) = self {
            i
        } else {
            panic!("expect I32, but get {:?} instead", self)
        }
    }

    pub fn take_f32(self) -> f32 {
        if let PrimitiveValueData::F32(f) = self {
            f
        } else {
            panic!()
        }
    }

    pub fn take_b32(self) -> u32 {
        if let PrimitiveValueData::B32(b) = self {
            b
        } else {
            panic!()
        }
    }

    pub fn take_b64(self) -> u64 {
        if let PrimitiveValueData::B64(b) = self {
            b
        } else {
            panic!()
        }
    }

    pub fn take_bool(self) -> bool {
        if let PrimitiveValueData::Bool(b) = self {
            b
        } else {
            panic!()
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            PrimitiveValueData::I32(value) => *value != 0i32,
            PrimitiveValueData::F32(value) => *value != 0.0f32,
            PrimitiveValueData::B32(value) => *value != 0u32,
            PrimitiveValueData::B64(value) => *value != 0u64,
            PrimitiveValueData::Bool(value) => *value,
            PrimitiveValueData::Void(_) => panic!(),
        }
    }

    pub fn to_register<'eval>(self) -> __Register<'eval> {
        unsafe {
            match self {
                PrimitiveValueData::I32(value) => value.__to_register__(),
                PrimitiveValueData::F32(value) => value.__to_register__(),
                PrimitiveValueData::B32(value) => value.__to_register__(),
                PrimitiveValueData::B64(value) => value.__to_register__(),
                PrimitiveValueData::Bool(value) => value.__to_register__(),
                PrimitiveValueData::Void(value) => value.__to_register__(),
            }
        }
    }
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
