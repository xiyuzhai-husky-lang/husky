use std::borrow::Cow;

use husky_pattern_syntax::RawPattern;

use super::*;

impl From<RawSuffixOpr> for RawOpnVariant {
    fn from(suffix: RawSuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawSuffixOpr {
    Incr, // ++
    Decr, // --
    BePattern(RawPattern),
    Unveil,
}

impl RawSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            RawSuffixOpr::Incr => "++".into(),
            RawSuffixOpr::Decr => "--".into(),
            RawSuffixOpr::BePattern(_) => todo!(),
            RawSuffixOpr::Unveil => "?".into(),
        }
    }
}
