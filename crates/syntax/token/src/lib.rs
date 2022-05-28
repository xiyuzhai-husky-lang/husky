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
pub use kind::TokenKind;
pub use query::*;
pub use semantic_token::*;
pub use special::Special;
pub use stream::*;
pub use tokenized_text::{TokenGroupIter, TokenizedText};

use scanner::TokenScanner;
use text::{RangedCustomIdentifier, TextIndent, TextRange, TextRanged};
use word::Identifier;

#[derive(PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    pub fn ranged_custom_ident(&self) -> Option<RangedCustomIdentifier> {
        match self.kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => Some(RangedCustomIdentifier {
                ident: ident,
                range: self.range,
            }),
            _ => todo!(),
        }
    }
}

impl TextRanged for Token {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
