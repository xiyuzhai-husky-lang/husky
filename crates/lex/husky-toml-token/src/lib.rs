#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
mod error;
mod special;
#[cfg(test)]
mod tests;

pub use error::*;
use husky_text::TextRange;
pub use special::*;

use husky_print_utils::p;
use husky_text_span::TextSpan;
use husky_word::{Word, WordDb};
use std::char;
use std::str;
use std::string;
use std::{borrow::Cow, sync::Arc};

pub type StringValue = Arc<String>;

/// tokens in toml file
#[derive(Debug, PartialEq, Eq)]
pub struct TomlToken {
    span: TextSpan,
    range: TextRange,
    variant: TomlTokenVariant,
}

impl TomlToken {
    pub fn new(span: TextSpan, range: TextRange, variant: TomlTokenVariant) -> Self {
        Self {
            span,
            range,
            variant,
        }
    }

    pub fn span(&self) -> TextSpan {
        self.span
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn variant(&self) -> &TomlTokenVariant {
        &self.variant
    }
}

/// variants for tokens in toml file
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TomlTokenVariant {
    Comment,
    Special(TomlSpecialToken),
    Keylike(Word),
    StringLiteral { val: StringValue, multiline: bool },
    Err(TomlTokenError),
}

impl TomlTokenVariant {
    pub fn describe(&self) -> &'static str {
        match *self {
            TomlTokenVariant::Keylike(_) => "an keylike",
            TomlTokenVariant::Comment => "a comment",
            TomlTokenVariant::Special(special) => special.describe(),
            TomlTokenVariant::StringLiteral { multiline, .. } => {
                if multiline {
                    "a multiline string"
                } else {
                    "a string"
                }
            }
            TomlTokenVariant::Err(_) => todo!(),
        }
    }
}

impl const std::ops::FromResidual<Result<core::convert::Infallible, TomlTokenError>>
    for TomlTokenVariant
{
    fn from_residual(residual: Result<core::convert::Infallible, TomlTokenError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => TomlTokenVariant::Err(e),
        }
    }
}
