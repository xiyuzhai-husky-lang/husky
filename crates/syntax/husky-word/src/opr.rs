use std::ops::Deref;

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordOpr {
    And,
    Or,
    As,
    Be,
}

impl From<WordOpr> for WordPtr {
    fn from(ident: WordOpr) -> Self {
        WordPtr::Opr(ident)
    }
}

impl Deref for WordOpr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl WordOpr {
    pub fn as_str(&self) -> &'static str {
        match self {
            WordOpr::And => "and",
            WordOpr::Or => "or",
            WordOpr::As => "as",
            WordOpr::Be => "be",
        }
    }
}
