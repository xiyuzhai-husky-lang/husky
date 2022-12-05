use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum ParadigmKeyword {
    Def,
    Func,
    Proc,
    Fn,
}

impl ParadigmKeyword {
    pub const fn as_str(self) -> &'static str {
        match self {
            ParadigmKeyword::Proc => "proc",
            ParadigmKeyword::Func => "func",
            ParadigmKeyword::Def => "def",
            ParadigmKeyword::Fn => "fn",
        }
    }

    pub fn is_lazy(self) -> bool {
        match self {
            ParadigmKeyword::Def => true,
            _ => false,
        }
    }
}

impl Deref for ParadigmKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl const From<ParadigmKeyword> for Keyword {
    fn from(kw: ParadigmKeyword) -> Self {
        Keyword::Paradigm(kw)
    }
}

impl const From<ParadigmKeyword> for TokenKind {
    fn from(kw: ParadigmKeyword) -> Self {
        TokenKind::Keyword(kw.into())
    }
}
