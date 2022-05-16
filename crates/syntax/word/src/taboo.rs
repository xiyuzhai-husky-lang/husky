use crate::*;
use std::ops::Deref;

// words that are forbidden due to compatibility with Rust or other reasons
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Taboo {
    Vec,
    Crate,
}

impl Deref for Taboo {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Taboo {
    pub fn as_str(&self) -> &'static str {
        match self {
            Taboo::Vec => "Vec",
            Taboo::Crate => "crate",
        }
    }
}

impl From<Taboo> for WordPtr {
    fn from(ident: Taboo) -> Self {
        Self::Taboo(ident)
    }
}
