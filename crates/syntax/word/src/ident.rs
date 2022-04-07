use core::hash::Hash;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(RootIdentifier),
    Custom(CustomIdentifier),
    Contextual(ContextualIdentifier),
}

impl Identifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Identifier::Builtin(ident) => ident.as_str(),
            Identifier::Custom(ident) => ident.as_str(),
            Identifier::Contextual(ident) => ident.as_str(),
        }
    }

    pub fn custom(&self) -> CustomIdentifier {
        match self {
            Identifier::Custom(ident) => *ident,
            _ => panic!(""),
        }
    }

    pub fn opt_custom(&self) -> Option<CustomIdentifier> {
        match self {
            Identifier::Custom(ident) => Some(*ident),
            _ => None,
        }
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Identifier::Builtin(ident) => ident.deref(),
            Identifier::Custom(ident) => ident.deref(),
            Identifier::Contextual(ident) => ident.deref(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct CustomIdentifier(pub &'static str);

impl CustomIdentifier {
    pub fn snake_name(&self) -> String {
        let mut snake_name = String::new();
        for c in self.0.chars() {
            if c.is_alphanumeric() {
                snake_name.push(c)
            } else {
                todo!()
            }
        }
        snake_name
    }
    pub fn dash_name(&self) -> String {
        todo!()
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Debug for CustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Display for CustomIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(self.0)
    }
}

impl PartialEq for CustomIdentifier {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as *const str) == (other.0 as *const str)
    }
}

impl Eq for CustomIdentifier {}

impl Hash for CustomIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const str).hash(state);
    }
}

impl Deref for CustomIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Borrow<str> for CustomIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl From<RootIdentifier> for Identifier {
    fn from(ident: RootIdentifier) -> Self {
        Self::Builtin(ident)
    }
}

impl From<CustomIdentifier> for Identifier {
    fn from(ident: CustomIdentifier) -> Self {
        Self::Custom(ident)
    }
}

impl From<ContextualIdentifier> for Identifier {
    fn from(ident: ContextualIdentifier) -> Self {
        Self::Contextual(ident)
    }
}

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
    Type,
    CloneTrait,
    CopyTrait,
    PartialEqTrait,
    EqTrait,
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
            RootIdentifier::Void => "()",
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
            RootIdentifier::Type => "type",
            RootIdentifier::CloneTrait => todo!(),
            RootIdentifier::CopyTrait => todo!(),
            RootIdentifier::PartialEqTrait => todo!(),
            RootIdentifier::EqTrait => todo!(),
        }
    }
}

impl Borrow<str> for RootIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ContextualIdentifier {
    Input,
    ThisData,
    ThisType,
}

impl ContextualIdentifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContextualIdentifier::Input => "input",
            ContextualIdentifier::ThisData => "this",
            ContextualIdentifier::ThisType => "This",
        }
    }
}

impl Deref for ContextualIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for ContextualIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

pub fn default_func_type() -> RootIdentifier {
    RootIdentifier::Fp
}
