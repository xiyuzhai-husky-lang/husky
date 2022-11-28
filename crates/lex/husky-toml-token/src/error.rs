use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Hash)]
pub enum TomlTokenError {
    #[error("invalid char in string {0} {1}")]
    InvalidCharInString(usize, char),
    #[error("todo")]
    InvalidEscape(usize, char),
    #[error("todo")]
    InvalidHexEscape(usize, char),
    #[error("todo")]
    InvalidEscapeValue(usize, u32),
    #[error("todo")]
    NewlineInString(usize),
    #[error("todo")]
    UnexpectedChar(char),
    #[error("todo")]
    UnterminatedString,
    #[error("todo")]
    NewlineInTableKey(usize),
    #[error("todo")]
    MultilineStringKey(usize),
    #[error("todo")]
    Wanted {
        at: usize,
        expected: &'static str,
        found: &'static str,
    },
}

pub type TomlTokenResult<T> = Result<T, TomlTokenError>;
