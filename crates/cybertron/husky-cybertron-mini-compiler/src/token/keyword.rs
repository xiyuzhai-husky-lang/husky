#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Let,
}

impl std::fmt::Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.repr()))
    }
}

impl Keyword {
    pub(super) fn new(s: &str) -> Option<Self> {
        match s {
            "let" => Some(Keyword::Let),
            _ => None,
        }
    }
}

impl Keyword {
    pub fn repr(self) -> &'static str {
        match self {
            Keyword::Let => "let",
        }
    }
}
