mod error;
mod kind;
mod line_token_iter;
mod query;
mod scanner;
mod special;
mod tokenized_text;

pub use error::LexError;
pub use kind::TokenKind;
pub use query::{TokenQuery, TokenQueryStorage};
pub use special::Special;
pub use tokenized_text::{TokenGroupFoldedIter, TokenizedText};

use common::*;

use scanner::TokenScanner;
use text::{GetTextRange, Indent, TextRange};

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(i: usize, start: usize, end: usize, kind: TokenKind) -> Token {
        Token {
            range: TextRange::new_same_line(i, start, end),
            kind,
        }
    }
}

impl GetTextRange for Token {
    fn get_text_range(&self) -> &TextRange {
        &self.range
    }
}
