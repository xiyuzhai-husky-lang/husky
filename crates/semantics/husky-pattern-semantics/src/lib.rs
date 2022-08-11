use husky_entity_route::EntityRoutePtr;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use infer_total::InferQueryGroup;

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
    Some,
    None,
}

impl ExprPattern {
    pub fn from_raw(db: &dyn InferQueryGroup, raw_patt: &RawPattern, ty: EntityRoutePtr) -> Self {
        let variant = match raw_patt.variant {
            RawPatternVariant::PrimitiveLiteral(_) => todo!(),
            RawPatternVariant::OneOf { ref subpatterns } => todo!(),
            RawPatternVariant::EnumLiteral(_) => todo!(),
            RawPatternVariant::Some => ExprPatternVariant::Some,
            RawPatternVariant::None => ExprPatternVariant::None,
        };
        Self { ty, variant }
    }
}
