mod crlf_fold;
mod iter;
mod keylike;
mod string;
#[cfg(test)]
mod tests;

use husky_text_span::TextSpan;
use husky_toml_token::*;
use husky_word::{Word, WordDb};
use iter::*;
use keylike::is_keylike;
use std::sync::Arc;

pub trait TomlTokenizeDb: WordDb {
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken>;
}

impl<T> TomlTokenizeDb for T
where
    T: WordDb,
{
    fn toml_tokenize(&self, input: &str) -> Vec<TomlToken> {
        TokenIter::new(self, input).collect()
    }
}
