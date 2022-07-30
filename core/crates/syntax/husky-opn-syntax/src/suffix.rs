use std::borrow::Cow;

use husky_entity_route::RangedEntityRoute;
use husky_pattern_syntax::RawPattern;
use husky_word::RootIdentifier;

use super::*;

impl From<RawSuffixOpr> for RawOpnVariant {
    fn from(suffix: RawSuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawSuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    AsTy(RangedEntityRoute), // :
    BePattern(RawPattern),
}

impl RawSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            RawSuffixOpr::Incr => "++".into(),
            RawSuffixOpr::Decr => "--".into(),
            RawSuffixOpr::AsTy(ty) => format!(" as {}", ty.route).into(),
            RawSuffixOpr::BePattern(_) => todo!(),
        }
    }
}
