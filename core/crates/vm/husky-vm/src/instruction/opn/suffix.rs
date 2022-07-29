use std::borrow::Cow;

use husky_entity_route::RangedEntityRoute;
use husky_word::RootIdentifier;

use super::*;

impl From<SuffixOpr> for RawOpnVariant {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    AsTy(RangedEntityRoute), // :
}

impl SuffixOpr {
    pub fn code(self) -> Cow<'static, str> {
        match self {
            SuffixOpr::Incr => "++".into(),
            SuffixOpr::Decr => "--".into(),
            SuffixOpr::AsTy(ty) => format!(" as {}", ty.route).into(),
        }
    }
}
