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

pub use error::*;
pub use kind::HuskyTokenKind;
pub use query::*;
pub use semantic_token::*;
pub use special::SpecialToken;
pub use stream::*;
pub use tokenized_text::{TokenGroupIter, TokenizedText};

use husky_text::{RangedCustomIdentifier, TextIndent, TextRange, TextRanged};
use scanner::TokenScanner;
use word::Identifier;

#[derive(PartialEq, Eq)]
pub struct HuskyToken {
    pub range: TextRange,
    pub kind: HuskyTokenKind,
}

impl std::fmt::Debug for HuskyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Token {{{:?}, {:?}}}", self.kind, self.range))
    }
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
                ident: ident,
                range: self.range,
            }),
            _ => todo!(),
        }
    }
}

impl TextRanged for HuskyToken {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
