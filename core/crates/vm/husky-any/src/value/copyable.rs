use super::*;
use crate::*;
use core::hash::Hash;
use husky_entity_route::EntityRoutePtr;
use serde::{ser::SerializeStruct, Serialize};
use std::{any::TypeId, borrow::Cow};
use word::RootIdentifier;

// use husky_entity_route::ScopeId;
// use word::ReservedIdentifier;

#[derive(Debug, Clone, Copy)]
pub enum CopyableValue {
    I32(i32),
    F32(f32),
    B32(u32),
    B64(u64),
    Bool(bool),
    Void(()),
    EnumKind(EnumKindValue),
}

impl Into<PrimitiveValueData> for CopyableValue {
    fn into(self) -> PrimitiveValueData {
        match self {
            CopyableValue::I32(i) => PrimitiveValueData::I32(i),
            CopyableValue::F32(f) => PrimitiveValueData::F32(f),
            CopyableValue::B32(b) => PrimitiveValueData::B32(b),
            CopyableValue::B64(b) => PrimitiveValueData::B64(b),
            CopyableValue::Bool(b) => PrimitiveValueData::Bool(b),
            CopyableValue::Void(_) => PrimitiveValueData::Void(()),
            CopyableValue::EnumKind(_) => panic!(),
        }
    }
}

impl Serialize for CopyableValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("PrimitiveValue", 3)?;
        let kind = match self {
            CopyableValue::I32(value) => "I32",
            CopyableValue::F32(_) => "F32",
            CopyableValue::B32(_) => "B32",
            CopyableValue::B64(_) => "B64",
            CopyableValue::Bool(_) => "Bool",
            CopyableValue::Void(_) => "Void",
            CopyableValue::EnumKind(_) => "EnumKind",
        };
        let value: Cow<'static, str> = (*self).into();
        state.serialize_field("kind", &kind)?;
        state.serialize_field("value", &value)?;
        state.end()
    }
}

impl CopyableValue {
    pub fn ty(&self) -> EntityRoutePtr {
        match self {
            CopyableValue::I32(_) => RootIdentifier::I32.into(),
            CopyableValue::F32(_) => RootIdentifier::F32.into(),
            CopyableValue::B32(_) => RootIdentifier::B32.into(),
            CopyableValue::B64(_) => RootIdentifier::B64.into(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.route.parent(),
        }
    }

    pub fn take_i32(&self) -> i32 {
        if let CopyableValue::I32(i) = self {
            *i
        } else {
            panic!("expect I32, but get {:?} instead", self)
        }
    }

    pub fn take_f32(&self) -> f32 {
        if let CopyableValue::F32(f) = self {
            *f
        } else {
            panic!()
        }
    }

    pub fn take_b32(&self) -> u32 {
        if let CopyableValue::B32(b) = self {
            *b
        } else {
            panic!()
        }
    }

    pub fn take_b64(&self) -> u64 {
        if let CopyableValue::B64(b) = self {
            *b
        } else {
            panic!()
        }
    }

    pub fn take_bool(&self) -> bool {
        if let CopyableValue::Bool(b) = self {
            *b
        } else {
            panic!()
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            CopyableValue::I32(value) => *value != 0i32,
            CopyableValue::F32(value) => *value != 0.0f32,
            CopyableValue::B32(value) => *value != 0u32,
            CopyableValue::B64(value) => *value != 0u64,
            CopyableValue::Bool(value) => *value,
            CopyableValue::Void(_) | CopyableValue::EnumKind(_) => panic!(),
        }
    }

    pub fn take_enum_kind(self) -> EnumKindValue {
        if let CopyableValue::EnumKind(b) = self {
            b
        } else {
            panic!()
        }
    }

    pub fn any_ref<'eval>(&self) -> &(dyn __AnyValueDyn<'eval> + 'eval) {
        match self {
            CopyableValue::I32(value) => value,
            CopyableValue::F32(value) => value,
            CopyableValue::B32(value) => value,
            CopyableValue::B64(value) => value,
            CopyableValue::Bool(value) => value,
            CopyableValue::EnumKind(value) => value,
            CopyableValue::Void(value) => value,
        }
    }

    pub fn any_mut<'eval>(&mut self) -> &mut (dyn __AnyValueDyn<'eval> + 'eval) {
        match self {
            CopyableValue::I32(value) => value,
            CopyableValue::F32(value) => value,
            CopyableValue::B32(value) => value,
            CopyableValue::B64(value) => value,
            CopyableValue::Bool(value) => value,
            CopyableValue::EnumKind(value) => value,
            CopyableValue::Void(_) => todo!(),
        }
    }
    // pub fn to_json_value(self) -> serde_json::Value {
    //     match self {
    //         CopyableValue::I32(value) => serde_json::to_value(value).unwrap(),
    //         CopyableValue::F32(value) => serde_json::to_value(value).unwrap(),
    //         CopyableValue::B32(value) => serde_json::to_value(value).unwrap(),
    //         CopyableValue::B64(value) => serde_json::to_value(value).unwrap(),
    //         CopyableValue::Bool(value) => serde_json::to_value(value).unwrap(),
    //         CopyableValue::Void(value) => serde_json::to_value(value).unwrap(),
    //         CopyableValue::EnumKind(value) => todo!(),
    //     }
    // }
}

impl Hash for CopyableValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            CopyableValue::I32(i) => i.hash(state),
            CopyableValue::F32(f) => f.to_bits().hash(state),
            CopyableValue::B32(b) => b.hash(state),
            CopyableValue::B64(b) => b.hash(state),
            CopyableValue::Bool(b) => b.hash(state),
            CopyableValue::Void(_) => ().hash(state),
            CopyableValue::EnumKind(enum_kind) => enum_kind.hash(state),
        }
    }
}

impl From<()> for CopyableValue {
    fn from(_: ()) -> Self {
        Self::Void(())
    }
}

impl From<i32> for CopyableValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<&i32> for CopyableValue {
    fn from(value: &i32) -> Self {
        Self::I32(*value)
    }
}

impl From<f32> for CopyableValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<&f32> for CopyableValue {
    fn from(value: &f32) -> Self {
        Self::F32(*value)
    }
}

impl From<u32> for CopyableValue {
    fn from(value: u32) -> Self {
        Self::B32(value)
    }
}

impl From<&u32> for CopyableValue {
    fn from(value: &u32) -> Self {
        Self::B32(*value)
    }
}

impl From<u64> for CopyableValue {
    fn from(value: u64) -> Self {
        Self::B64(value)
    }
}

impl From<&u64> for CopyableValue {
    fn from(value: &u64) -> Self {
        Self::B64(*value)
    }
}

impl From<bool> for CopyableValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&bool> for CopyableValue {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl From<EnumKindValue> for CopyableValue {
    fn from(value: EnumKindValue) -> Self {
        CopyableValue::EnumKind(value)
    }
}

impl Into<String> for CopyableValue {
    fn into(self) -> String {
        match self {
            CopyableValue::I32(i) => format!("{}", i),
            CopyableValue::F32(f) => format!("{}", f),
            CopyableValue::B32(b) => format!("{}", b),
            CopyableValue::B64(b) => format!("{}", b),
            CopyableValue::Bool(b) => format!("{}", b),
            CopyableValue::Void(_) => "()".into(),
            CopyableValue::EnumKind(enum_kind) => format!("{:?}", enum_kind.route),
        }
    }
}

impl Into<Cow<'static, str>> for CopyableValue {
    fn into(self) -> Cow<'static, str> {
        match self {
            CopyableValue::I32(i) => format!("{}", i).into(),
            CopyableValue::F32(f) => format!("{}", f).into(),
            CopyableValue::B32(b) => format!("{}", b).into(),
            CopyableValue::B64(b) => format!("{}", b).into(),
            CopyableValue::Bool(b) => format!("{}", b).into(),
            CopyableValue::Void(_) => "()".into(),
            CopyableValue::EnumKind(enum_kind) => format!("{:?}", enum_kind.route).into(),
        }
    }
}

impl PartialEq for CopyableValue {
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

impl Eq for CopyableValue {}

#[test]
fn compare_floats() {
    use husky_check_utils::*;
    let a = f64::NAN;
    should_eq!(a == a, false);
    should_eq!(a.to_bits() == a.to_bits(), true);
}
