#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Let,
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
    pub fn data(self) -> &'static str {
        match self {
            Keyword::Let => "let",
        }
    }
}
