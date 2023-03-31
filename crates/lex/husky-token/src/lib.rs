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
#[cfg(feature = "test-utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod token_visibility;
mod tokenize;

pub use self::convexity::*;
pub use self::db::*;
pub use self::error::*;
pub use self::kind::*;
pub use self::literal::*;
pub use self::parse_utils::*;
pub use self::sheet::*;
pub use self::snippet::*;
pub use self::stream::*;
pub use self::token_visibility::*;

use husky_text::{HasTextRange, TextRange};
use husky_vfs::{ModulePath, VfsResult};
use husky_word::Ident;
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
