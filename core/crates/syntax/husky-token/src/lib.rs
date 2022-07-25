mod convexity;
mod error;
mod kind;
mod line_token_iter;
mod query;
mod scanner;
mod semantic_token;
mod special;
mod stream;
#[cfg(test)]
mod tests;
mod tokenized_text;
mod utils;

pub use convexity::*;
pub use error::*;
pub use kind::HuskyTokenKind;
pub use query::*;
pub use semantic_token::*;
pub use special::SpecialToken;
pub use stream::*;
pub use tokenized_text::{TokenGroupIter, TokenizedText};

use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_text::{RangedCustomIdentifier, TextIndent, TextRange, TextRanged};
use husky_word::Identifier;
use scanner::TokenScanner;

#[derive(PartialEq, Eq)]
pub struct HuskyToken {
    pub range: TextRange,
    pub kind: HuskyTokenKind,
}

impl HuskyToken {
    pub fn new(i: usize, start: usize, end: usize, kind: HuskyTokenKind) -> HuskyToken {
        HuskyToken {
            range: husky_text::new_same_line(i, start, end),
            kind,
        }
    }

    pub fn ranged_custom_ident(&self) -> Option<RangedCustomIdentifier> {
        match self.kind {
            HuskyTokenKind::Identifier(Identifier::Custom(ident)) => Some(RangedCustomIdentifier {
                ident,
                range: self.range,
            }),
            _ => todo!(),
        }
    }

    pub fn left_convexity(&self) -> Option<Convexity> {
        self.kind.left_convexity()
    }
}

impl std::fmt::Debug for HuskyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Token {{{:?}, {:?}}}", self.kind, self.range))
    }
}

impl TextRanged for HuskyToken {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
