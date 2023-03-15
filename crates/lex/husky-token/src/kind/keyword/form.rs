use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum FormKeyword {
    Def,
    Func,
    Proc,
    Fn,
    Function,
    Theorem,
    Lemma,
    Proposition,
    Type,
    Const,
    Mm,
}

impl FormKeyword {
    pub const fn code(self) -> &'static str {
        match self {
            FormKeyword::Proc => "proc",
            FormKeyword::Func => "func",
            FormKeyword::Def => "def",
            FormKeyword::Fn => "fn",
            FormKeyword::Function => "function",
            FormKeyword::Theorem => "theorem",
            FormKeyword::Lemma => "lemma",
            FormKeyword::Proposition => "proposition",
            FormKeyword::Type => "type",
            FormKeyword::Const => "const",
            FormKeyword::Mm => "mm",
        }
    }

    pub fn is_lazy(self) -> bool {
        match self {
            FormKeyword::Def => true,
            _ => false,
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
