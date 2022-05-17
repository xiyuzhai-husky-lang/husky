use crate::*;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Decorator {
    Pub,
    Private,
    Async,
    Static,
}

impl Deref for Decorator {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Decorator {
    pub fn as_str(self) -> &'static str {
        match self {
            Decorator::Pub => "pub",
            Decorator::Private => "private",
            Decorator::Async => "async",
            Decorator::Static => "static",
        }
    }
}

impl Into<WordPtr> for Decorator {
    fn into(self) -> WordPtr {
        WordPtr::Decorator(self)
    }
}
