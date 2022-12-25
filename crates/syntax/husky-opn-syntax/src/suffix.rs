use std::borrow::Cow;

use super::*;

impl From<RawSuffixOpr> for Opn {
    fn from(suffix: RawSuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawSuffixOpr {
    Incr, // ++
    Decr, // --
    Unveil,
}

impl RawSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            RawSuffixOpr::Incr => "++".into(),
            RawSuffixOpr::Decr => "--".into(),
            RawSuffixOpr::Unveil => "?".into(),
        }
    }
}
