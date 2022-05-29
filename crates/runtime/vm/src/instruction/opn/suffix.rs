use entity_route::RangedEntityRoute;
use word::RootIdentifier;

use super::*;

impl From<SuffixOpr> for RawOpnVariant {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    WithTy(EntityRoutePtr),  // :
    AsTy(RangedEntityRoute), // :
}

impl SuffixOpr {
    pub fn code(self) -> String {
        match self {
            SuffixOpr::Incr => "++".into(),
            SuffixOpr::Decr => "--".into(),
            SuffixOpr::WithTy(ty) => format!(": {}", ty),
            SuffixOpr::AsTy(ty) => format!(" as {}", ty.route),
        }
    }
}
