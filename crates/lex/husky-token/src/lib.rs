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

use husky_doc::{HasTextRange, TextRange};
use husky_vfs::{ModulePath, VfsResult};
use husky_word::Identifier;
#[cfg(test)]
use tests::*;
use tokenize::*;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(
    TokenSheet,
    StringLiteral,
    Snippet,
    ranged_token_sheet,
    token_sheet,
    reserved_words,
    tokenize_snippet,
);
