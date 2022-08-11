use husky_entity_route::EntityRoutePtr;
use husky_primitive_literal_syntax::PrimitiveLiteralData;

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
