#![feature(const_trait_impl)]
#![feature(const_convert)]
mod db;
mod error;
mod group;
mod kind;
mod semantic_token;
#[cfg(feature = "storage")]
mod storage;
mod stream;
#[cfg(test)]
mod tests;
mod tokenize;
mod utils;

pub use db::*;
pub use error::*;
pub use group::*;
pub use kind::*;
pub use semantic_token::*;
pub use stream::*;
pub use tokenize::Tokenize;

use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::{HasTextRange, RangedIdentifier, TextIndent, TextRange};
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(i: u32, start: u32, end: u32, kind: TokenKind) -> Token {
        Token {
            range: husky_text::new_same_line(i, start, end),
            kind,
        }
    }

    pub fn ranged_custom_ident(&self) -> Option<RangedIdentifier> {
        todo!()
        // match self.kind {
        //     TokenKind::Identifier(Identifier::Custom(ident)) => Some(RangedIdentifier {
        //         ident,
        //         range: self.range,
        //     }),
        //     _ => todo!(),
        // }
    }
}

impl HasTextRange for Token {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
