use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FugitiveKeyword {
    Def,
    Fn,
    Theorem,
    Lemma,
    Proposition,
    Type,
    Val,
    Constexpr,
    Gn,
}

impl FugitiveKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            FugitiveKeyword::Def => "def",
            FugitiveKeyword::Fn => "fn",
            FugitiveKeyword::Theorem => "theorem",
            FugitiveKeyword::Lemma => "lemma",
            FugitiveKeyword::Proposition => "proposition",
            FugitiveKeyword::Type => "type",
            FugitiveKeyword::Val => "val",
            FugitiveKeyword::Constexpr => "constexpr",
            FugitiveKeyword::Gn => "gn",
        }
    }
}

impl Deref for FugitiveKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<FugitiveKeyword> for Token {
    fn from(kw: FugitiveKeyword) -> Self {
        Token::Keyword(kw.into())
    }
}
