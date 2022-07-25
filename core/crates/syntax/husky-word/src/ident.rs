mod contextual;
mod custom;
mod root;

pub use contextual::*;
pub use custom::*;
pub use root::*;

use core::hash::Hash;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use serde::Serialize;
use std::fmt::Write;
use std::{borrow::Borrow, ops::Deref};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(RootIdentifier),
    Custom(CustomIdentifier),
    Contextual(ContextualIdentifier),
}

impl HuskyDisplay for Identifier {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <10}{}",
                husky_print_utils::CYAN,
                self.as_str(),
                husky_print_utils::RESET
            )
            .unwrap();
        } else {
            write!(result, "{: <10}", self.as_str()).unwrap();
        }
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

pub fn default_func_type() -> RootIdentifier {
    RootIdentifier::Fp
}
