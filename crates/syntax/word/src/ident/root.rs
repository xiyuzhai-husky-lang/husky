use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RootIdentifier {
    Void,
    I32,
    F32,
    B32,
    B64,
    Bool,
    True,
    False,
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
    Datasets,
    DatasetType,
    TypeType,
    ModuleType,
    CloneTrait,
    CopyTrait,
    PartialEqTrait,
    EqTrait,
    Ref,
}

impl From<RootIdentifier> for Identifier {
    fn from(ident: RootIdentifier) -> Self {
        Self::Builtin(ident)
    }
}

impl Deref for RootIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl RootIdentifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            RootIdentifier::Void => "void",
            RootIdentifier::I32 => "i32",
            RootIdentifier::F32 => "f32",
            RootIdentifier::B32 => "b32",
            RootIdentifier::B64 => "b64",
            RootIdentifier::Bool => "bool",
            RootIdentifier::True => "true",
            RootIdentifier::False => "false",
            RootIdentifier::Vec => "Vec",
            RootIdentifier::Tuple => "Tuple",
            RootIdentifier::Debug => "debug",
            RootIdentifier::Std => "std",
            RootIdentifier::Core => "core",
            RootIdentifier::Fp => "Fp",
            RootIdentifier::Fn => "Fn",
            RootIdentifier::FnMut => "FnMut",
            RootIdentifier::FnOnce => "FnOnce",
            RootIdentifier::Array => "Array",
            RootIdentifier::Datasets => "datasets",
            RootIdentifier::DatasetType => "Dataset",
            RootIdentifier::TypeType => "Type",
            RootIdentifier::ModuleType => "Module",
            RootIdentifier::CloneTrait => "Clone",
            RootIdentifier::CopyTrait => "Copy",
            RootIdentifier::PartialEqTrait => "PartialEq",
            RootIdentifier::EqTrait => "Eq",
            RootIdentifier::Ref => "Ref",
        }
    }
}

impl Borrow<str> for RootIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
