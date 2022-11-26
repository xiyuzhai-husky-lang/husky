mod comment;
mod hex;
mod iter;
mod keylike;
mod string;
#[cfg(test)]
mod tests;
mod whitespace;

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

impl<'a> Iterator for TokenIter<'a> {
    type Item = TomlToken;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, c) = self.next_char()?;

        let variant = match c {
            '\n' => Ok(TomlTokenVariant::Newline),
            ' ' => Ok(self.next_whitespace_token(start)),
            '\t' => Ok(self.next_whitespace_token(start)),
            '#' => Ok(self.next_comment_token(start)),
            '=' => Ok(TomlSpecialToken::Equals.into()),
            '.' => Ok(TomlSpecialToken::Period.into()),
            ',' => Ok(TomlSpecialToken::Comma.into()),
            ':' => Ok(TomlSpecialToken::Colon.into()),
            '+' => Ok(TomlSpecialToken::Plus.into()),
            '{' => Ok(TomlSpecialToken::LeftCurly.into()),
            '}' => Ok(TomlSpecialToken::RightCurly.into()),
            '[' => Ok(TomlSpecialToken::LeftBox.into()),
            ']' => Ok(TomlSpecialToken::RightBox.into()),
            '\'' => self.next_literal_string(start),
            '"' => self.next_basic_string(start),
            ch if is_keylike(ch) => Ok(self.next_keylike(start)),
            ch => Err(TomlTokenError::Unexpected(start, ch)),
        };

        Some(self.emit_token(start, variant))
    }
}
