use crate::{TokenData};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PronounKeyword {
    Crate,
    SelfType,
    SelfValue,
    Super,
}

impl PronounKeyword {
    pub fn code(self) -> &'static str {
        match self {
            PronounKeyword::Crate => "crate",
            PronounKeyword::SelfType => "Self",
            PronounKeyword::SelfValue => "self",
            PronounKeyword::Super => "super",
        }
    }
}

impl From<PronounKeyword> for TokenData {
    fn from(val: PronounKeyword) -> Self {
        TokenData::Keyword(val.into())
    }
}
