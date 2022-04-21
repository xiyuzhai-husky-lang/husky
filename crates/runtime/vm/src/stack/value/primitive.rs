use core::hash::Hash;
use std::borrow::Cow;

use serde::{ser::SerializeStruct, Serialize};
use word::RootIdentifier;

use crate::*;

// use entity_route::ScopeId;
// use word::ReservedIdentifier;

#[derive(Clone, Copy)]
pub enum PrimitiveValue {
    I32(i32),
    F32(f32),
    B32(u32),
    B64(u64),
    Bool(bool),
    Void,
}

impl Serialize for PrimitiveValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("PrimitiveValue", 3)?;
        let kind = match self {
            PrimitiveValue::I32(value) => "I32",
            PrimitiveValue::F32(_) => "F32",
            PrimitiveValue::B32(_) => "B32",
            PrimitiveValue::B64(_) => "B64",
            PrimitiveValue::Bool(_) => "Bool",
            PrimitiveValue::Void => "Void",
        };
        let value: Cow<'static, str> = (*self).into();
        state.serialize_field("kind", &kind)?;
        state.serialize_field("value", &value)?;
        state.end()
    }
}

impl From<()> for PrimitiveValue {
    fn from(_: ()) -> Self {
        Self::Void
    }
}

impl std::fmt::Debug for PrimitiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => {
                arg0.fmt(f)?;
                f.write_str("(i32)")
            }
            Self::F32(arg0) => f.debug_tuple("F32").field(arg0).finish(),
            Self::B32(arg0) => f.debug_tuple("B32").field(arg0).finish(),
            Self::B64(arg0) => f.debug_tuple("B64").field(arg0).finish(),
            Self::Bool(arg0) => f.debug_tuple("Bool").field(arg0).finish(),
            Self::Void => write!(f, "Void"),
        }
    }
}

impl PrimitiveValue {
    pub fn ty(&self) -> RootIdentifier {
        match self {
            PrimitiveValue::I32(_) => RootIdentifier::I32,
            PrimitiveValue::F32(_) => RootIdentifier::F32,
            PrimitiveValue::B32(_) => RootIdentifier::B32,
            PrimitiveValue::B64(_) => RootIdentifier::B64,
            PrimitiveValue::Bool(_) => todo!(),
            PrimitiveValue::Void => todo!(),
        }
    }

    pub fn as_i32(&self) -> VMResult<i32> {
        if let PrimitiveValue::I32(i) = self {
            Ok(*i)
        } else {
            Err(VMError::TypeMismatch(format!(
                "expect I32, but get {:?} instead",
                self
            )))
        }
    }

    pub fn as_f32(&self) -> VMResult<f32> {
        if let PrimitiveValue::F32(f) = self {
            Ok(*f)
        } else {
            todo!()
        }
    }

    pub fn as_b32(&self) -> VMResult<u32> {
        if let PrimitiveValue::B32(b) = self {
            Ok(*b)
        } else {
            todo!()
        }
    }

    pub(crate) fn as_bool(&self) -> VMResult<bool> {
        if let PrimitiveValue::Bool(b) = self {
            Ok(*b)
        } else {
            todo!()
        }
    }

    pub(crate) fn to_bool(&self) -> VMResult<bool> {
        Ok(match self {
            PrimitiveValue::I32(value) => *value != 0i32,
            PrimitiveValue::F32(value) => *value != 0.0f32,
            PrimitiveValue::B32(value) => *value != 0u32,
            PrimitiveValue::B64(value) => *value != 0u64,
            PrimitiveValue::Bool(value) => *value,
            PrimitiveValue::Void => panic!(),
        })
    }

    pub fn any_ref<'eval>(&self) -> &dyn AnyValueDyn<'eval> {
        match self {
            PrimitiveValue::I32(value) => value,
            PrimitiveValue::F32(value) => value,
            PrimitiveValue::B32(value) => value,
            PrimitiveValue::B64(value) => value,
            PrimitiveValue::Bool(value) => value,
            PrimitiveValue::Void => panic!(),
        }
    }

    pub fn any_mut<'eval>(&mut self) -> &mut dyn AnyValueDyn<'eval> {
        match self {
            PrimitiveValue::I32(value) => value,
            PrimitiveValue::F32(value) => value,
            PrimitiveValue::B32(value) => value,
            PrimitiveValue::B64(value) => value,
            PrimitiveValue::Bool(value) => value,
            PrimitiveValue::Void => panic!(),
        }
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

impl From<u64> for PrimitiveValue {
    fn from(value: u64) -> Self {
        Self::B64(value)
    }
}

impl From<&u64> for PrimitiveValue {
    fn from(value: &u64) -> Self {
        Self::B64(*value)
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

impl Into<String> for PrimitiveValue {
    fn into(self) -> String {
        match self {
            PrimitiveValue::I32(i) => format!("{}", i),
            PrimitiveValue::F32(f) => format!("{}", f),
            PrimitiveValue::B32(b) => format!("{}", b),
            PrimitiveValue::B64(b) => format!("{}", b),
            PrimitiveValue::Bool(b) => format!("{}", b),
            PrimitiveValue::Void => "()".into(),
        }
    }
}

impl Into<Cow<'static, str>> for PrimitiveValue {
    fn into(self) -> Cow<'static, str> {
        match self {
            PrimitiveValue::I32(i) => format!("{}", i).into(),
            PrimitiveValue::F32(f) => format!("{}", f).into(),
            PrimitiveValue::B32(b) => format!("{:#032b}", b).into(),
            PrimitiveValue::B64(b) => format!("{:#064b}", b).into(),
            PrimitiveValue::Bool(b) => format!("{}", b).into(),
            PrimitiveValue::Void => "()".into(),
        }
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
    use check_utils::*;
    let a = f64::NAN;
    should_eq!(a == a, false);
    should_eq!(a.to_bits() == a.to_bits(), true);
}
