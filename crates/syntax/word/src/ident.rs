mod custom;

pub use custom::*;

use core::hash::Hash;
use serde::Serialize;
use std::fmt::Write;
use std::{borrow::Borrow, ops::Deref};
use test_utils::TestComparable;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(RootIdentifier),
    Custom(CustomIdentifier),
    Contextual(ContextualIdentifier),
}

impl TestComparable for Identifier {
    fn write_inherent(&self, result: &mut String) {
        write!(
            result,
            "{}{: <10}{}",
            print_utils::CYAN,
            self.as_str(),
            print_utils::RESET
        )
        .unwrap();
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self)
    }
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
            RootIdentifier::Type => "type",
            RootIdentifier::CloneTrait => "Clone",
            RootIdentifier::CopyTrait => "Copy",
            RootIdentifier::PartialEqTrait => "PartialEq",
            RootIdentifier::EqTrait => "Eq",
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
