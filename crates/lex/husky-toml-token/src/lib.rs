#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
mod db;
mod error;
mod line_group;
mod sheet;
mod special;
mod stream;
#[cfg(test)]
mod tests;
mod tokenize;

pub use db::*;
pub use error::*;
use husky_text::TextRange;
pub use line_group::*;
pub use sheet::*;
pub use special::*;
pub use stream::*;

use tokenize::*;

#[salsa::jar(db = TomlTokenDb)]
pub struct TomlTokenJar(package_manifest_toml_token_sheet);

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
    Word(Word),
    StringLiteral { val: StringValue, multiline: bool },
    Err(TomlTokenError),
}

impl TomlTokenVariant {
    pub fn describe(&self) -> &'static str {
        match *self {
            TomlTokenVariant::Word(_) => "a word",
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
