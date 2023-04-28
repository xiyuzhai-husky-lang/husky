use crate::{Keyword, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EndKeyword {
    With,
    Into,
}

impl EndKeyword {
    pub fn code(self) -> &'static str {
        match self {
            EndKeyword::With => "with",
            EndKeyword::Into => "into",
        }
    }
}

impl From<EndKeyword> for Token {
    fn from(kw: EndKeyword) -> Self {
        Token::Keyword(kw.into())
    }
}
