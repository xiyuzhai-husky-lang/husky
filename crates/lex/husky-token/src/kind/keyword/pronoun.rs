use crate::{Keyword, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PronounKeyword {
    Crate,
    Super,
    SelfValue,
    SelfType,
}

impl PronounKeyword {
    pub fn code(self) -> &'static str {
        match self {
            PronounKeyword::Crate => "crate",
            PronounKeyword::SelfValue => "self",
            PronounKeyword::SelfType => "Self",
            PronounKeyword::Super => "super",
        }
    }
}

impl From<PronounKeyword> for Keyword {
    fn from(v: PronounKeyword) -> Self {
        Self::Pronoun(v)
    }
}

impl From<PronounKeyword> for Token {
    fn from(val: PronounKeyword) -> Self {
        Token::Keyword(val.into())
    }
}
