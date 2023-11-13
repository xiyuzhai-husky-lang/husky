use crate::{TokenData};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EndKeyword {
    With,
}

impl EndKeyword {
    pub fn code(self) -> &'static str {
        match self {
            EndKeyword::With => "with",
        }
    }
}

impl From<EndKeyword> for TokenData {
    fn from(kw: EndKeyword) -> Self {
        TokenData::Keyword(kw.into())
    }
}
