use std::ops::Deref;

use crate::{Keyword, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeKeyword {
    Extern,
    Struct,
    Enum,
    Record,
    Structure,
    Inductive,
}

impl TypeKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            TypeKeyword::Extern => "extern",
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
        self.code()
    }
}

impl const From<TypeKeyword> for Token {
    fn from(val: TypeKeyword) -> Self {
        Token::Keyword(val.into())
    }
}
