use crate::*;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Decorator {
    Pub,
    Protected,
    Private,
    Async,
    Static,
}

impl std::fmt::Display for Decorator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Deref for Decorator {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Decorator {
    pub const fn as_str(self) -> &'static str {
        match self {
            Decorator::Pub => "pub",
            Decorator::Protected => "protected",
            Decorator::Private => "private",
            Decorator::Async => "async",
            Decorator::Static => "static",
        }
    }
}

impl const Into<TokenKind> for Decorator {
    fn into(self) -> TokenKind {
        TokenKind::Decorator(self)
    }
}
