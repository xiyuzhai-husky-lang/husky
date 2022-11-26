mod keylike;
mod string;
#[cfg(tests)]
mod tests;
mod tokenizer;

use husky_text_span::TextSpan;
use husky_toml_token::*;
use husky_word::{Word, WordDb};
use keylike::is_keylike;
use std::sync::Arc;
use tokenizer::*;

pub trait TomlTokenizeDb: WordDb {
    fn toml_tokenize(&self, input: &str) -> (Vec<TomlToken>, Vec<TomlTokenizeError>);
}

impl<T> TomlTokenizeDb for T
where
    T: WordDb,
{
    fn toml_tokenize(&self, input: &str) -> (Vec<TomlToken>, Vec<TomlTokenizeError>) {
        let tokenizer = Tokenizer::new(self, input);
        todo!()
    }
}

#[derive(Clone)]
struct CrlfFold<'a> {
    chars: std::str::CharIndices<'a>,
}

#[derive(Debug)]
enum MaybeString {
    NotEscaped(usize),
    Owned(std::string::String),
}

impl<'a> Iterator for CrlfFold<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(i, c)| {
            if c == '\r' {
                let mut attempt = self.chars.clone();
                if let Some((_, '\n')) = attempt.next() {
                    self.chars = attempt;
                    return (i, '\n');
                }
            }
            (i, c)
        })
    }
}

impl MaybeString {
    fn push(&mut self, ch: char) {
        match *self {
            MaybeString::NotEscaped(..) => {}
            MaybeString::Owned(ref mut s) => s.push(ch),
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_owned(&mut self, input: &str) {
        match *self {
            MaybeString::NotEscaped(start) => {
                *self = MaybeString::Owned(input[start..].to_owned());
            }
            MaybeString::Owned(..) => {}
        }
    }

    fn into_cow(self, input: &str) -> StringValue {
        match self {
            MaybeString::NotEscaped(start) => Arc::new(input[start..].to_owned()),
            MaybeString::Owned(s) => Arc::new(s),
        }
    }
}
