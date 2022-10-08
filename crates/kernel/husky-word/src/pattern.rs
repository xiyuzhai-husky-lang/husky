use std::ops::Deref;

use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum WordPattern {
    Some,
    None,
}

impl From<WordPattern> for WordPtr {
    fn from(ident: WordPattern) -> Self {
        WordPtr::Pattern(ident)
    }
}

impl WordPattern {
    pub fn as_str(self) -> &'static str {
        match self {
            WordPattern::Some => "some",
            WordPattern::None => "none",
        }
    }
}

impl Deref for WordPattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
