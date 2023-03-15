use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FormKeyword {
    Def,
    Fn,
    Theorem,
    Lemma,
    Proposition,
    TypeAlias,
    Const,
    Mm,
}

impl FormKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            FormKeyword::Def => "def",
            FormKeyword::Fn => "fn",
            FormKeyword::Theorem => "theorem",
            FormKeyword::Lemma => "lemma",
            FormKeyword::Proposition => "proposition",
            FormKeyword::TypeAlias => "type",
            FormKeyword::Const => "const",
            FormKeyword::Mm => "mm",
        }
    }
}

impl Deref for FormKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.code()
    }
}

impl const From<FormKeyword> for Token {
    fn from(kw: FormKeyword) -> Self {
        Token::Keyword(kw.into())
    }
}
