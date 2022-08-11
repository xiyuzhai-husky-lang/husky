use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_pattern_semantics::PurePattern;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerSuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    AsTy(RangedEntityRoute), // :
    BePattern(PurePattern),
}

impl EagerSuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            EagerSuffixOpr::Incr => "++".into(),
            EagerSuffixOpr::Decr => "--".into(),
            EagerSuffixOpr::AsTy(ty) => format!(" as {}", ty.route).into(),
            EagerSuffixOpr::BePattern(_) => todo!(),
        }
    }
}
