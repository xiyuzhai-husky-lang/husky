use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum RootBuiltinIdentifier {
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
    ThickFp,
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
    RefMut,
    Option,
}

impl std::fmt::Display for RootBuiltinIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<RootBuiltinIdentifier> for Identifier {
    fn from(ident: RootBuiltinIdentifier) -> Self {
        Self::Root(ident)
    }
}

impl Deref for RootBuiltinIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl RootBuiltinIdentifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            RootBuiltinIdentifier::Void => "void",
            RootBuiltinIdentifier::I32 => "i32",
            RootBuiltinIdentifier::I64 => "i64",
            RootBuiltinIdentifier::F32 => "f32",
            RootBuiltinIdentifier::F64 => "f64",
            RootBuiltinIdentifier::B32 => "b32",
            RootBuiltinIdentifier::B64 => "b64",
            RootBuiltinIdentifier::Bool => "bool",
            RootBuiltinIdentifier::True => "true",
            RootBuiltinIdentifier::False => "false",
            RootBuiltinIdentifier::Vec => "Vec",
            RootBuiltinIdentifier::Tuple => "Tuple",
            RootBuiltinIdentifier::Debug => "debug",
            RootBuiltinIdentifier::Std => "std",
            RootBuiltinIdentifier::Core => "core",
            RootBuiltinIdentifier::Mor => "Mor",
            RootBuiltinIdentifier::ThickFp => "ThickFp",
            RootBuiltinIdentifier::Fn => "Fn",
            RootBuiltinIdentifier::FnMut => "FnMut",
            RootBuiltinIdentifier::FnOnce => "FnOnce",
            RootBuiltinIdentifier::Array => "Array",
            RootBuiltinIdentifier::Domains => "domains",
            RootBuiltinIdentifier::DatasetType => "Dataset",
            RootBuiltinIdentifier::VisualType => "Visual",
            RootBuiltinIdentifier::TypeType => "Type",
            RootBuiltinIdentifier::TraitType => "Trait",
            RootBuiltinIdentifier::ModuleType => "Module",
            RootBuiltinIdentifier::CloneTrait => "Clone",
            RootBuiltinIdentifier::CopyTrait => "Copy",
            RootBuiltinIdentifier::PartialEqTrait => "PartialEq",
            RootBuiltinIdentifier::EqTrait => "Eq",
            RootBuiltinIdentifier::Ref => "Ref",
            RootBuiltinIdentifier::RefMut => "RefMut",
            RootBuiltinIdentifier::Option => "Option",
        }
    }
}

impl Borrow<str> for RootBuiltinIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}
