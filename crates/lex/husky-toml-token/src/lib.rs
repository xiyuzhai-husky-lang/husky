mod special;
#[cfg(feature = "storage")]
mod storage;
#[cfg(test)]
mod tests;

pub use special::*;

use husky_print_utils::p;
use husky_text_span::TextSpan;
use husky_word::{Word, WordDb};
use std::char;
use std::str;
use std::string;
use std::{borrow::Cow, sync::Arc};

pub type StringValue = Arc<String>;

/// tokens in toml file
#[derive(Eq, PartialEq, Debug)]
pub struct TomlToken {
    pub span: TextSpan,
    pub variant: TomlTokenVariant,
}

/// variants for tokens in toml file
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TomlTokenVariant {
    Whitespace,
    Newline,
    Comment,
    Special(TomlSpecialToken),
    Keylike(Word),
    StringLiteral { val: StringValue, multiline: bool },
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    InvalidCharInString(usize, char),
    InvalidEscape(usize, char),
    InvalidHexEscape(usize, char),
    InvalidEscapeValue(usize, u32),
    NewlineInString(usize),
    Unexpected(usize, char),
    UnterminatedString(usize),
    NewlineInTableKey(usize),
    MultilineStringKey(usize),
    Wanted {
        at: usize,
        expected: &'static str,
        found: &'static str,
    },
}

impl TomlTokenVariant {
    pub fn describe(&self) -> &'static str {
        match *self {
            TomlTokenVariant::Keylike(_) => "an keylike",
            TomlTokenVariant::Comment => "a comment",
            TomlTokenVariant::Newline => "a newline",
            TomlTokenVariant::Special(special) => special.describe(),
            TomlTokenVariant::Whitespace => "whitespace",
            TomlTokenVariant::StringLiteral { multiline, .. } => {
                if multiline {
                    "a multiline string"
                } else {
                    "a string"
                }
            }
        }
    }
}

impl TomlSpecialToken {
    pub fn describe(self) -> &'static str {
        match self {
            TomlSpecialToken::Equals => "an equals",
            TomlSpecialToken::Period => "a period",
            TomlSpecialToken::Comma => "a comma",
            TomlSpecialToken::RightCurly => "a right brace",
            TomlSpecialToken::LeftCurly => "a left brace",
            TomlSpecialToken::RightBox => "a right bracket",
            TomlSpecialToken::LeftBox => "a left bracket",
            TomlSpecialToken::Colon => "a colon",
            TomlSpecialToken::Plus => "a plus",
        }
    }
}
