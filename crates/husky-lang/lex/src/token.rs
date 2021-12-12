use common::*;

use text::{CharIter, GetTextRange, Indent, TextRange};

use crate::{line_token_iter::LineTokenIter, *};

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Keyword(word::Keyword),
    Identifier(word::Identifier),
    Special(Special),
    I32Literal(i32),
    F32Literal(f32),
}
impl Eq for TokenKind {}
impl From<word::Word> for TokenKind {
    fn from(word: word::Word) -> Self {
        match word {
            word::Word::Keyword(keyword) => TokenKind::Keyword(keyword),
            word::Word::Identifier(ident) => TokenKind::Identifier(ident),
        }
    }
}
impl From<f32> for TokenKind {
    fn from(f: f32) -> Self {
        TokenKind::F32Literal(f)
    }
}
impl From<i32> for TokenKind {
    fn from(i: i32) -> Self {
        TokenKind::I32Literal(i)
    }
}
