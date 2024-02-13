use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FugitiveKeyword {
    Def,
    Theorem,
    Lemma,
    Proposition,
    Type,
    Val,
    Fn,
    Vn,
    Gn,
    Pn,
    Bn,
    Qn,
    Memo,
}

impl FugitiveKeyword {
    pub fn code(self) -> &'static str {
        match self {
            FugitiveKeyword::Def => "def",
            FugitiveKeyword::Theorem => "theorem",
            FugitiveKeyword::Lemma => "lemma",
            FugitiveKeyword::Proposition => "proposition",
            FugitiveKeyword::Type => "type",
            FugitiveKeyword::Val => "val",
            FugitiveKeyword::Memo => "memo",
            FugitiveKeyword::Fn => "fn",
            FugitiveKeyword::Gn => "gn",
            FugitiveKeyword::Vn => "vn",
            FugitiveKeyword::Pn => "pn",
            FugitiveKeyword::Qn => "qn",
            FugitiveKeyword::Bn => "bn",
        }
    }
}

impl Deref for FugitiveKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<FugitiveKeyword> for TokenData {
    fn from(kw: FugitiveKeyword) -> Self {
        TokenData::Keyword(kw.into())
    }
}
