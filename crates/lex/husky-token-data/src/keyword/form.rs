use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FormKeyword {
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
    Qn,
    Bn,
    Sn,
    Tn,
    Memo,
}

impl FormKeyword {
    pub fn code(self) -> &'static str {
        match self {
            FormKeyword::Def => "def",
            FormKeyword::Theorem => "theorem",
            FormKeyword::Lemma => "lemma",
            FormKeyword::Proposition => "proposition",
            FormKeyword::Type => "type",
            FormKeyword::Val => "val",
            FormKeyword::Memo => "memo",
            FormKeyword::Fn => "fn",
            FormKeyword::Gn => "gn",
            FormKeyword::Vn => "vn",
            FormKeyword::Pn => "pn",
            FormKeyword::Qn => "qn",
            FormKeyword::Bn => "bn",
            FormKeyword::Sn => "sn",
            FormKeyword::Tn => "tn",
        }
    }
}

impl Deref for FormKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<FormKeyword> for TokenData {
    fn from(kw: FormKeyword) -> Self {
        TokenData::Keyword(kw.into())
    }
}
