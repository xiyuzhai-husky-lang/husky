use crate::*;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeKeyword {
    Pub,
    Protected,
    Private,
    Async,
    Static,
}

impl std::fmt::Display for AttributeKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Deref for AttributeKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AttributeKeyword {
    pub const fn as_str(self) -> &'static str {
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
