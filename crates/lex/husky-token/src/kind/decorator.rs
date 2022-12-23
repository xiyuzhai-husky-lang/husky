use crate::*;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttrKeyword {
    Pub,
    Protected,
    Private,
    Async,
    Static,
}

impl std::fmt::Display for AttrKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Deref for AttrKeyword {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AttrKeyword {
    pub const fn as_str(self) -> &'static str {
        match self {
            AttrKeyword::Pub => "pub",
            AttrKeyword::Protected => "protected",
            AttrKeyword::Private => "private",
            AttrKeyword::Async => "async",
            AttrKeyword::Static => "static",
        }
    }
}

impl const Into<TokenKind> for AttrKeyword {
    fn into(self) -> TokenKind {
        TokenKind::Attr(self)
    }
}
