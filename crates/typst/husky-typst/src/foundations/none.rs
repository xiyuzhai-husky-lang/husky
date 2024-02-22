use std::fmt::{self, Debug, Formatter};

use ecow::EcoString;
use serde::{Serialize, Serializer};

use crate::diag::StrResult;
use crate::foundations::{
    cast, ty, CastInfo, FromTypstValue, IntoTypstValue, Reflect, Type, TypstValue, TypstValueRepr,
};

/// A value that indicates the absence of any other value.
///
/// The none type has exactly one value: `{none}`.
///
/// When inserted into the document, it is not visible. This is also the value
/// that is produced by empty code blocks. It can be
/// [joined]($scripting/#blocks) with any value, yielding the other value.
///
/// # Example
/// ```example
/// Not visible: #none
/// ```
#[ty(cast, name = "none")]
#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NoneTypstValue;

impl Reflect for NoneTypstValue {
    fn input() -> CastInfo {
        CastInfo::Type(Type::of::<Self>())
    }

    fn output() -> CastInfo {
        CastInfo::Type(Type::of::<Self>())
    }

    fn castable(value: &TypstValue) -> bool {
        matches!(value, TypstValue::None)
    }
}

impl IntoTypstValue for NoneTypstValue {
    fn into_value(self) -> TypstValue {
        TypstValue::None
    }
}

impl FromTypstValue for NoneTypstValue {
    fn from_value(value: TypstValue) -> StrResult<Self> {
        match value {
            TypstValue::None => Ok(Self),
            _ => Err(Self::error(&value)),
        }
    }
}

impl Debug for NoneTypstValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("None")
    }
}

impl TypstValueRepr for NoneTypstValue {
    fn repr(&self) -> EcoString {
        "none".into()
    }
}

impl Serialize for NoneTypstValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

cast! {
    (),
    self => TypstValue::None,
    _: NoneTypstValue => (),
}

impl<T: Reflect> Reflect for Option<T> {
    fn input() -> CastInfo {
        T::input() + NoneTypstValue::input()
    }

    fn output() -> CastInfo {
        T::output() + NoneTypstValue::output()
    }

    fn castable(value: &TypstValue) -> bool {
        NoneTypstValue::castable(value) || T::castable(value)
    }
}

impl<T: IntoTypstValue> IntoTypstValue for Option<T> {
    fn into_value(self) -> TypstValue {
        match self {
            Some(v) => v.into_value(),
            None => TypstValue::None,
        }
    }
}

impl<T: FromTypstValue> FromTypstValue for Option<T> {
    fn from_value(value: TypstValue) -> StrResult<Self> {
        match value {
            TypstValue::None => Ok(None),
            v if T::castable(&v) => Ok(Some(T::from_value(v)?)),
            _ => Err(Self::error(&value)),
        }
    }
}
