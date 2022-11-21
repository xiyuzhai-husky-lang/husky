use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LiasonKeyword {
    Mut,
}

impl From<LiasonKeyword> for Keyword {
    fn from(kw: LiasonKeyword) -> Self {
        Self::Liason(kw)
    }
}

impl LiasonKeyword {
    pub fn as_str(self) -> &'static str {
        match self {
            LiasonKeyword::Mut => "mut",
        }
    }
}
