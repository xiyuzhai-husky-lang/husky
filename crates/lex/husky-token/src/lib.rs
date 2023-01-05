#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
mod convexity;
mod db;
mod error;
mod kind;
mod literal;
mod parse_utils;
mod sheet;
mod snippet;
mod stream;
#[cfg(test)]
mod tests;
mod token_accessibility;
mod tokenize;

pub use convexity::*;
pub use db::*;
pub use error::*;
pub use kind::*;
pub use literal::*;
pub use parse_utils::*;
pub use sheet::*;
pub use snippet::*;
pub use stream::*;
pub use token_accessibility::*;

use husky_text::{HasTextRange, TextRange};
use husky_vfs::{ModulePath, VfsResult};
use husky_word::Identifier;
#[cfg(test)]
use tests::*;
use tokenize::*;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(
    StringLiteral,
    Snippet,
    token_sheet,
    reserved_words,
    tokenize_snippet,
);

impl Token {
    // pub fn new(i: u32, start: u32, end: u32, kind: Token) -> Token {
    //     Token {
    //         range: husky_text::new_same_line(i, start, end),
    //         kind,
    //     }
    // }

    pub fn identify(&self) -> Option<Identifier> {
        match self {
            Token::Identifier(ident) => Some(*ident),
            _ => None,
        }
    }
}

// impl HasTextRange for Token {
//     fn text_range(&self) -> TextRange {
//         self.range
//     }
// }
