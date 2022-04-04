use core::hash::Hash;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(BuiltinIdentifier),
    Custom(CustomIdentifier),
    Implicit(ImplicitIdentifier),
    This,
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Identifier::Builtin(ident) => ident.deref(),
            Identifier::Custom(ident) => ident.deref(),
            Identifier::Implicit(ident) => ident.deref(),
            Identifier::This => "this",
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

impl From<CustomIdentifier> for Identifier {
    fn from(ident: CustomIdentifier) -> Self {
        Self::Custom(ident)
    }
}

impl From<ImplicitIdentifier> for Identifier {
    fn from(ident: ImplicitIdentifier) -> Self {
        Self::Implicit(ident)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BuiltinIdentifier {
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
}

impl Deref for BuiltinIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            BuiltinIdentifier::Void => "()",
            BuiltinIdentifier::I32 => "i32",
            BuiltinIdentifier::F32 => "f32",
            BuiltinIdentifier::B32 => "b32",
            BuiltinIdentifier::B64 => "b64",
            BuiltinIdentifier::Bool => "bool",
            BuiltinIdentifier::True => "true",
            BuiltinIdentifier::False => "false",
            BuiltinIdentifier::Vec => "Vec",
            BuiltinIdentifier::Tuple => "Tuple",
            BuiltinIdentifier::Debug => "debug",
            BuiltinIdentifier::Std => "std",
            BuiltinIdentifier::Core => "core",
            BuiltinIdentifier::Fp => "Fp",
            BuiltinIdentifier::Fn => "Fn",
            BuiltinIdentifier::FnMut => "FnMut",
            BuiltinIdentifier::FnOnce => "FnOnce",
            BuiltinIdentifier::Array => "Array",
            BuiltinIdentifier::Datasets => "datasets",
            BuiltinIdentifier::DatasetType => "Dataset",
            BuiltinIdentifier::Type => "type",
        }
    }
}

impl Borrow<str> for BuiltinIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ImplicitIdentifier {
    Input,
}

impl Deref for ImplicitIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            ImplicitIdentifier::Input => "input",
        }
    }
}

impl Borrow<str> for ImplicitIdentifier {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

pub fn default_func_type() -> BuiltinIdentifier {
    BuiltinIdentifier::Fp
}
