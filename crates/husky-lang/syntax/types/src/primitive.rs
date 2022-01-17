use core::hash::Hash;

use scope::ScopeId;
use word::ReservedIdentifier;

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveValue {
    I32(i32),
    F32(f32),
    B32(u32),
    B64(u64),
    Bool(bool),
    Void,
}

impl PrimitiveValue {
    pub fn ty(&self) -> ScopeId {
        match self {
            PrimitiveValue::I32(_) => ReservedIdentifier::I32,
            PrimitiveValue::F32(_) => ReservedIdentifier::F32,
            PrimitiveValue::B32(_) => ReservedIdentifier::B32,
            PrimitiveValue::B64(_) => ReservedIdentifier::B64,
            PrimitiveValue::Bool(_) => todo!(),
            PrimitiveValue::Void => todo!(),
        }
        .into()
    }
}

impl Hash for PrimitiveValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            PrimitiveValue::I32(i) => i.hash(state),
            PrimitiveValue::F32(f) => f.to_bits().hash(state),
            PrimitiveValue::B32(b) => b.hash(state),
            PrimitiveValue::B64(b) => b.hash(state),
            PrimitiveValue::Bool(b) => b.hash(state),
            PrimitiveValue::Void => ().hash(state),
        }
    }
}

impl From<i32> for PrimitiveValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<&i32> for PrimitiveValue {
    fn from(value: &i32) -> Self {
        Self::I32(*value)
    }
}

impl From<f32> for PrimitiveValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<&f32> for PrimitiveValue {
    fn from(value: &f32) -> Self {
        Self::F32(*value)
    }
}

impl From<u32> for PrimitiveValue {
    fn from(value: u32) -> Self {
        Self::B32(value)
    }
}

impl From<&u32> for PrimitiveValue {
    fn from(value: &u32) -> Self {
        Self::B32(*value)
    }
}

impl From<bool> for PrimitiveValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&bool> for PrimitiveValue {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl PartialEq for PrimitiveValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::F32(l0), Self::F32(r0)) => l0.to_bits() == r0.to_bits(),
            (Self::B32(l0), Self::B32(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for PrimitiveValue {}

#[test]
fn compare_floats() {
    use common::*;
    let a = f64::NAN;
    should_eq!(a == a, false);
    should_eq!(a.to_bits() == a.to_bits(), true);
}
