use std::ops::Deref;

use crate::{Keyword, Token};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeEntityKeyword {
    Extern,
    Struct,
    Enum,
    Record,
    Structure,
    Inductive,
}

impl TypeEntityKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            TypeEntityKeyword::Extern => "extern",
            TypeEntityKeyword::Struct => "struct",
            TypeEntityKeyword::Enum => "enum",
            TypeEntityKeyword::Record => "record",
            TypeEntityKeyword::Structure => "structure",
            TypeEntityKeyword::Inductive => "inductive",
        }
    }
}

impl Deref for TypeEntityKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl const From<TypeEntityKeyword> for Token {
    fn from(val: TypeEntityKeyword) -> Self {
        Token::Keyword(val.into())
    }
}
