mod error;
mod kind;
mod line_token_iter;
mod query;
mod scanner;
mod special;
#[cfg(test)]
mod tests;
mod tokenized_text;

pub use error::LexError;
pub use kind::TokenKind;
pub use query::{TokenQueryGroup, TokenQueryGroupStorage};
pub use special::Special;
pub use tokenized_text::{TokenGroupIter, TokenizedText};

use common::*;

use scanner::TokenScanner;
use text::{Indent, TextRange, TextRanged};

#[derive(PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Token {{{:?}, {:?}}}", self.kind, self.range))
    }
}

impl Token {
    pub fn new(i: usize, start: usize, end: usize, kind: TokenKind) -> Token {
        Token {
            range: text::new_same_line(i, start, end),
            kind,
        }
    }
}

impl TextRanged for Token {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}
