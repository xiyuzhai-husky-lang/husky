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

impl From<&CustomIdentifier> for Identifier {
    fn from(ident: &CustomIdentifier) -> Self {
        Self::Custom(*ident)
    }
}

impl From<u32> for CustomIdentifier {
    fn from(raw: u32) -> Self {
        CustomIdentifier(raw)
    }
}

impl From<u32> for Identifier {
    fn from(raw: u32) -> Self {
        Self::Custom(CustomIdentifier(raw))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BuiltinIdentifier {
    Void,
    I32,
    F32,
    Bool,
    Vector,
    Tuple,
    Debug,
    Std,
    Core,
    Fp,
    Fn,
    FnMut,
    FnOnce,
    Array,
    Input,
}

impl BuiltinIdentifier {
    pub fn code(self) -> &'static str {
        match self {
            BuiltinIdentifier::Void => "()",
            BuiltinIdentifier::I32 => "i32",
            BuiltinIdentifier::F32 => "f32",
            BuiltinIdentifier::Bool => "bool",
            BuiltinIdentifier::Vector => "Vec",
            BuiltinIdentifier::Tuple => "Tuple",
            BuiltinIdentifier::Debug => "debug",
            BuiltinIdentifier::Std => "std",
            BuiltinIdentifier::Core => "core",
            BuiltinIdentifier::Fp => "Fp",
            BuiltinIdentifier::Fn => "Fn",
            BuiltinIdentifier::FnMut => "FnMut",
            BuiltinIdentifier::FnOnce => "FnOnce",
            BuiltinIdentifier::Array => "Array",
            BuiltinIdentifier::Input => "Input",
        }
    }
}

pub fn default_func_type() -> BuiltinIdentifier {
    BuiltinIdentifier::Fp
}
