#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
mod db;
mod error;
mod group;
mod idx;
mod iter;
mod kind;
mod literal;
mod parse_utils;
#[cfg(test)]
mod tests;
mod tokenize;
mod utils;

pub use db::*;
pub use error::*;
pub use group::*;
use husky_vfs::{ModulePath, VfsResult};
pub use idx::*;
pub use iter::*;
pub use kind::*;
pub use literal::*;
pub use parse_utils::*;

use husky_text::{HasTextRange, RangedIdentifier, TextRange};
use husky_word::Identifier;
#[cfg(test)]
use tests::*;
use tokenize::*;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(token_sheet, reserved_words);

#[salsa::tracked(jar = TokenJar, return_ref)]
fn token_sheet(db: &dyn TokenDb, module_path: ModulePath) -> VfsResult<TokenSheet> {
    Ok(TokenSheet::new(tokenize::tokenize(
        db,
        db.module_content(module_path)?,
    )))
}

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

impl TokenSheet {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl Token {
    pub fn new(i: u32, start: u32, end: u32, kind: TokenKind) -> Token {
        Token {
            range: husky_text::new_same_line(i, start, end),
            kind,
        }
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
