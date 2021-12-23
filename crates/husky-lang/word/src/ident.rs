use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(BuiltinIdentifier),
    Custom(CustomIdentifier),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CustomIdentifier(u32);

impl From<CustomIdentifier> for Identifier {
    fn from(ident: CustomIdentifier) -> Self {
        Self::Custom(ident)
    }
}

impl From<u32> for CustomIdentifier {
    fn from(raw: u32) -> Self {
        CustomIdentifier(raw)
    }
}

impl From<u32> for Identifier {
    fn from(raw: u32) -> Self {
        CustomIdentifier(raw).into()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BuiltinIdentifier {
    Void,
    I32,
    F32,
    Vec,
    Tuple,
    Debug,
    Std,
    Core,
    Fp,
    Fn,
    FnMut,
    FnOnce,
    Array,
}

pub fn default_func_type() -> BuiltinIdentifier {
    BuiltinIdentifier::Fp
}
