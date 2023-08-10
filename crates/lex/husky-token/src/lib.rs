#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
mod convexity;
mod db;
mod error;
mod helpers;
mod sheet;
mod snippet;
mod stream;
#[cfg(feature = "test-utils")]
pub mod test_utils;
#[cfg(test)]
mod tests;
mod token;
mod token_visibility;
mod tokenize;

pub use self::convexity::*;
pub use self::db::*;
pub use self::error::*;
pub use self::helpers::*;
pub use self::sheet::*;
pub use self::snippet::*;
pub use self::stream::*;
pub use self::token::*;
pub use self::token_visibility::*;

use husky_coword::Ident;
use husky_term_prelude::*;
use husky_text::{HasTextRange, TextRange};
use husky_vfs::{ModulePath, VfsResult};
#[cfg(test)]
use tests::*;
use tokenize::*;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(
    TokenSheet,
    Snippet,
    UnspecifiedFloatLiteral,
    ranged_token_sheet,
    token_sheet,
    reserved_cowords,
    tokenize_snippet,
);
