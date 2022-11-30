mod comment;
mod db;
mod hex;
mod iter;
mod keylike;
mod sheet;
mod string;
#[cfg(test)]
mod tests;
mod whitespace;

pub use db::TomlTokenizeDb;
pub use sheet::{TomlTokenGroup, TomlTokenSheet, TomlTokens};

use husky_text_span::TextSpan;
use husky_toml_token::*;
use husky_word::{Word, WordDb};
use iter::*;
use keylike::is_keylike;
use sheet::package_manifest_toml_token_sheet;
use std::sync::Arc;

#[salsa::jar(db = TomlTokenizeDb)]
pub struct TomlTokenizeJar(TomlTokenSheet, package_manifest_toml_token_sheet);
