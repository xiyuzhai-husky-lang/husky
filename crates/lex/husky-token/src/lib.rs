mod convexity;
mod decorator;
mod keyword;
mod kind;
mod semantic_token;
mod special;
#[cfg(test)]
mod tests;
mod utils;
mod wordopr;

pub use convexity::*;
pub use decorator::Decorator;
pub use keyword::*;
pub use semantic_token::*;
pub use special::SpecialToken;
pub use wordopr::WordOpr;

use husky_identifier::Identifier;
use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_text::{HasTextRange, RangedIdentifier, TextIndent, TextRange};

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(i: usize, start: usize, end: usize, kind: TokenKind) -> Token {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Decorator(Decorator),
    Keyword(Keyword),
    Identifier(Identifier),
    Special(SpecialToken),
    WordOpr(WordOpr),
    Literal(RawLiteralData),
    Unrecognized(char),
    IllFormedLiteral(RawLiteralData),
}
