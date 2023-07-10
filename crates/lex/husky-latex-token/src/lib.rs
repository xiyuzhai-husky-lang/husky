#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
mod db;
mod error;
mod paragraph;
mod sheet;
mod special;
mod stream;
#[cfg(test)]
mod tests;
mod tokenize;

pub use db::*;
pub use error::*;
use husky_text::TextRange;
pub use paragraph::*;
pub use sheet::*;
pub use special::*;
pub use stream::*;

use tokenize::*;

#[salsa::jar(db = LaTexTokenDb)]
pub struct LaTexTokenJar();

use husky_coword::Coword;
use husky_text_span::DocumentSpan;
use std::char;
use std::str;

use std::sync::Arc;

pub type StringValue = Arc<String>;

/// tokens in toml file
#[derive(Debug, PartialEq, Eq)]
pub struct EnglishToken {
    span: DocumentSpan,
    range: TextRange,
    variant: EnglishTokenVariant,
}

impl EnglishToken {
    pub fn new(span: DocumentSpan, range: TextRange, variant: EnglishTokenVariant) -> Self {
        Self {
            span,
            range,
            variant,
        }
    }

    pub fn span(&self) -> DocumentSpan {
        self.span
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn variant(&self) -> &EnglishTokenVariant {
        &self.variant
    }
}

/// variants for tokens in toml file
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EnglishTokenVariant {
    Comment,
    Special(EnglishSpecialToken),
    Word(Coword),
    StringLiteral { val: StringValue, multiline: bool },
    Err(EnglishTokenError),
}

impl EnglishTokenVariant {
    pub fn describe(&self) -> &'static str {
        match *self {
            EnglishTokenVariant::Word(_) => "a word",
            EnglishTokenVariant::Comment => "a comment",
            EnglishTokenVariant::Special(special) => special.describe(),
            EnglishTokenVariant::StringLiteral { multiline, .. } => {
                if multiline {
                    "a multiline string"
                } else {
                    "a string"
                }
            }
            EnglishTokenVariant::Err(_) => todo!(),
        }
    }
}

impl std::ops::FromResidual<Result<core::convert::Infallible, EnglishTokenError>>
    for EnglishTokenVariant
{
    fn from_residual(residual: Result<core::convert::Infallible, EnglishTokenError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => EnglishTokenVariant::Err(e),
        }
    }
}
