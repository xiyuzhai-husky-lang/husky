use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RootIdentifier {
    Void,
    I32,
    I64,
    F32,
    F64,
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
    Mor,
    Fp,
    Fn,
    FnMut,
    FnOnce,
    Array,
    Domains,
    DatasetType,
    VisualType,
    TypeType,
    TraitType,
    ModuleType,
    CloneTrait,
    CopyTrait,
    PartialEqTrait,
    EqTrait,
    Ref,
    Option,
}

impl std::fmt::Display for RootIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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
            RootIdentifier::I64 => "i64",
            RootIdentifier::F32 => "f32",
            RootIdentifier::F64 => "f64",
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
            RootIdentifier::Mor => "Mor",
            RootIdentifier::Fp => "Fp",
            RootIdentifier::Fn => "Fn",
            RootIdentifier::FnMut => "FnMut",
            RootIdentifier::FnOnce => "FnOnce",
            RootIdentifier::Array => "Array",
            RootIdentifier::Domains => "domains",
            RootIdentifier::DatasetType => "Dataset",
            RootIdentifier::VisualType => "Visual",
            RootIdentifier::TypeType => "Type",
            RootIdentifier::TraitType => "Trait",
            RootIdentifier::ModuleType => "Module",
            RootIdentifier::CloneTrait => "Clone",
            RootIdentifier::CopyTrait => "Copy",
            RootIdentifier::PartialEqTrait => "PartialEq",
            RootIdentifier::EqTrait => "Eq",
            RootIdentifier::Ref => "Ref",
            RootIdentifier::Option => "Option",
        }
    }
}

impl Borrow<str> for RootIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
