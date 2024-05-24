use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FormKeyword {
    Type,
    Fn,
    Vn,
    Gn,
    Pn,
    Qn,
    Bn,
    Sn,
    Tn,
    Memo,
    Static,
    Compterm,
    Val,
    Def,
    Theorem,
    Lemma,
    Proposition,
}

impl FormKeyword {
    pub fn code(self) -> &'static str {
        match self {
            FormKeyword::Memo => "memo",
            FormKeyword::Fn => "fn",
            FormKeyword::Gn => "gn",
            FormKeyword::Vn => "vn",
            FormKeyword::Pn => "pn",
            FormKeyword::Qn => "qn",
            FormKeyword::Bn => "bn",
            FormKeyword::Sn => "sn",
            FormKeyword::Tn => "tn",
            FormKeyword::Static => "static",
            FormKeyword::Compterm => "compterm",
            FormKeyword::Val => "val",
            FormKeyword::Type => "type",
            FormKeyword::Def => "def",
            FormKeyword::Theorem => "theorem",
            FormKeyword::Lemma => "lemma",
            FormKeyword::Proposition => "proposition",
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
