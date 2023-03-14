use crate::*;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = TokenDb)]
pub enum AttributeKeyword {
    Pub,
    Protected,
    Private,
    Async,
    Static,
}

impl std::fmt::Display for AttributeKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

impl Deref for AttributeKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl AttributeKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            AttributeKeyword::Pub => "pub",
            AttributeKeyword::Protected => "protected",
            AttributeKeyword::Private => "private",
            AttributeKeyword::Async => "async",
            AttributeKeyword::Static => "static",
        }
    }
}

impl const Into<Token> for AttributeKeyword {
    fn into(self) -> Token {
        Token::Attr(self)
    }
}
