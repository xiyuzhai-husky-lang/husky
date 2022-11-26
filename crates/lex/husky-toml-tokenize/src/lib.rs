mod error;
mod keylike;
mod string;
#[cfg(tests)]
mod tests;
mod tokenizer;

pub use error::*;
use keylike::is_keylike;

use std::sync::Arc;

use husky_text_span::TextSpan;
use husky_toml_token::{StringValue, TomlSpecialToken, TomlToken, TomlTokenVariant};
use husky_word::{Word, WordDb};
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
struct Tokenizer<'a> {
    db: &'a dyn WordDb,
    input: &'a str,
    chars: CrlfFold<'a>,
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

impl<'a> Tokenizer<'a> {
    fn new(db: &'a dyn WordDb, input: &'a str) -> Self {
        let mut t = Tokenizer {
            db,
            input,
            chars: CrlfFold {
                chars: input.char_indices(),
            },
        };
        // Eat utf-8 BOM
        t.eatc('\u{feff}');
        t
    }

    fn next(&mut self) -> TomlTokenizeResult<Option<TomlToken>> {
        let (start, variant) = match self.take_one_char() {
            Some((start, '\n')) => (start, TomlTokenVariant::Newline),
            Some((start, ' ')) => (start, self.next_whitespace_token(start)),
            Some((start, '\t')) => (start, self.next_whitespace_token(start)),
            Some((start, '#')) => (start, self.next_comment_token(start)),
            Some((start, '=')) => (start, TomlSpecialToken::Equals.into()),
            Some((start, '.')) => (start, TomlSpecialToken::Period.into()),
            Some((start, ',')) => (start, TomlSpecialToken::Comma.into()),
            Some((start, ':')) => (start, TomlSpecialToken::Colon.into()),
            Some((start, '+')) => (start, TomlSpecialToken::Plus.into()),
            Some((start, '{')) => (start, TomlSpecialToken::LeftCurly.into()),
            Some((start, '}')) => (start, TomlSpecialToken::RightCurly.into()),
            Some((start, '[')) => (start, TomlSpecialToken::LeftBox.into()),
            Some((start, ']')) => (start, TomlSpecialToken::RightBox.into()),
            Some((start, '\'')) => {
                return self.next_literal_string(start).map(|variant| {
                    Some(TomlToken {
                        span: self.calc_span(start),
                        variant,
                    })
                })
            }
            Some((start, '"')) => {
                return self.next_basic_string(start).map(|variant| {
                    Some(TomlToken {
                        span: self.calc_span(start),
                        variant,
                    })
                })
            }
            Some((start, ch)) if is_keylike(ch) => (start, self.next_keylike(start)),
            Some((start, ch)) => return Err(TomlTokenizeError::Unexpected(start, ch)),
            None => return Ok(None),
        };

        let span = self.calc_span(start);
        Ok(Some(TomlToken { span, variant }))
    }

    fn peek(&mut self) -> TomlTokenizeResult<Option<TomlToken>> {
        self.clone().next()
    }

    fn eatc(&mut self, ch: char) -> bool {
        match self.chars.clone().next() {
            Some((_, ch2)) if ch == ch2 => {
                self.take_one_char();
                true
            }
            _ => false,
        }
    }

    fn current(&self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }

    fn next_whitespace_token(&mut self, start: usize) -> TomlTokenVariant {
        while self.eatc(' ') || self.eatc('\t') {
            // ...
        }
        TomlTokenVariant::Whitespace
    }

    fn next_comment_token(&mut self, start: usize) -> TomlTokenVariant {
        while let Some((_, ch)) = self.chars.clone().next() {
            if ch != '\t' && !('\u{20}'..='\u{10ffff}').contains(&ch) {
                break;
            }
            self.take_one_char();
        }
        TomlTokenVariant::Comment
    }

    fn next_hex(&mut self, start: usize, i: usize, len: usize) -> TomlTokenizeResult<char> {
        let mut buf = String::with_capacity(len);
        for _ in 0..len {
            match self.take_one_char() {
                Some((_, ch)) if ch as u32 <= 0x7F && ch.is_ascii_hexdigit() => buf.push(ch),
                Some((i, ch)) => return Err(TomlTokenizeError::InvalidHexEscape(i, ch)),
                None => return Err(TomlTokenizeError::UnterminatedString(start)),
            }
        }
        let val = u32::from_str_radix(&buf, 16).unwrap();
        match char::from_u32(val) {
            Some(ch) => Ok(ch),
            None => Err(TomlTokenizeError::InvalidEscapeValue(i, val)),
        }
    }

    /// Calculate the span from start to next char
    fn calc_span(&mut self, start: usize) -> TextSpan {
        let end = self
            .peek_one()
            .map(|t| t.0)
            .unwrap_or_else(|| self.input.len());
        TextSpan { start, end }
    }

    /// Peek one char without consuming it.
    fn peek_one(&self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    /// Take one char.
    fn take_one_char(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }
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
