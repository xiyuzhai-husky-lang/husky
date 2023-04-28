use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FormKeyword {
    Def,
    Fn,
    Theorem,
    Lemma,
    Proposition,
    Type,
    Const,
    Val,
    Constexpr,
    Gn,
    Memo,
}

impl FormKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            FormKeyword::Def => "def",
            FormKeyword::Fn => "fn",
            FormKeyword::Theorem => "theorem",
            FormKeyword::Lemma => "lemma",
            FormKeyword::Proposition => "proposition",
            FormKeyword::Type => "type",
            FormKeyword::Const => "const",
            FormKeyword::Val => "val",
            FormKeyword::Constexpr => "constexpr",
            FormKeyword::Gn => "gn",
            FormKeyword::Memo => "memo",
        }
    }
}

impl Deref for FormKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl From<FormKeyword> for Token {
    fn from(kw: FormKeyword) -> Self {
        Token::Keyword(kw.into())
    }
}
