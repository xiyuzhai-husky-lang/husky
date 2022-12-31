use std::borrow::Cow;

use super::*;

impl From<SuffixPunctuation> for Opn {
    fn from(suffix: SuffixPunctuation) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixPunctuation {
    Incr, // ++
    Decr, // --
    Unveil,
}

impl SuffixPunctuation {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            SuffixPunctuation::Incr => "++".into(),
            SuffixPunctuation::Decr => "--".into(),
            SuffixPunctuation::Unveil => "?".into(),
        }
    }
}
