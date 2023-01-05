use std::ops::Deref;

use crate::{Keyword, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeKeyword {
    Type,
    Struct,
    Enum,
    Record,
    Structure,
    Inductive,
}

impl TypeKeyword {
    pub const fn as_str(&self) -> &'static str {
        match self {
            TypeKeyword::Type => "type",
            TypeKeyword::Struct => "struct",
            TypeKeyword::Enum => "enum",
            TypeKeyword::Record => "record",
            TypeKeyword::Structure => "structure",
            TypeKeyword::Inductive => "inductive",
        }
    }
}

impl Deref for TypeKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl const Into<Keyword> for TypeKeyword {
    fn into(self) -> Keyword {
        Keyword::Type(self)
    }
}

impl const Into<Token> for TypeKeyword {
    fn into(self) -> Token {
        Token::Keyword(self.into())
    }
}
