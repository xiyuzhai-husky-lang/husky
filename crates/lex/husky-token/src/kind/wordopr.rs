use std::ops::Deref;

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordOpr {
    And,
    Or,
    As,
    Be,
}

impl const From<WordOpr> for Token {
    fn from(wordopr: WordOpr) -> Self {
        Token::WordOpr(wordopr)
    }
}

impl Deref for WordOpr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl WordOpr {
    pub const fn code(&self) -> &'static str {
        match self {
            WordOpr::And => "and",
            WordOpr::Or => "or",
            WordOpr::As => "as",
            WordOpr::Be => "be",
        }
    }
}
