use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                    // ++
    Decr,                    // --
    AsTy(RangedEntityRoute), // :
    BePattern(ExprPattern),
}

impl SuffixOpr {
    pub fn code(&self) -> Cow<'static, str> {
        match self {
            SuffixOpr::Incr => "++".into(),
            SuffixOpr::Decr => "--".into(),
            SuffixOpr::AsTy(ty) => format!(" as {}", ty.route).into(),
            SuffixOpr::BePattern(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprPattern {
    pub ty: EntityRoutePtr,
    pub variant: ExprPatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprPatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<ExprPattern> },
    EnumLiteral(EntityRoutePtr),
}
