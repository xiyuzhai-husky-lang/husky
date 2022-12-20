#![feature(const_trait_impl)]
#![feature(const_convert)]
mod db;
mod error;
mod group;
mod idx;
mod iter;
mod kind;
#[cfg(test)]
mod tests;
mod tokenize;
mod utils;

pub use db::*;
pub use error::*;
pub use group::*;
pub use idx::*;
pub use iter::*;
pub use kind::*;
pub use tokenize::Tokenize;

use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::{HasTextRange, RangedIdentifier, TextRange};
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenSheet {
    tokens: Vec<Token>,
    group_starts: Vec<usize>,
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

    pub fn identify(&self) -> Option<Identifier> {
        match self.kind {
            TokenKind::Identifier(ident) => Some(ident),
            _ => None,
        }
    }
}

impl HasTextRange for Token {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
