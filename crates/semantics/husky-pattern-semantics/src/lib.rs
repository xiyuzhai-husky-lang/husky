use husky_entity_route::EntityRoutePtr;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use infer_total::InferQueryGroup;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PurePattern {
    pub ty: EntityRoutePtr,
    pub variant: PurePatternVariant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PurePatternVariant {
    PrimitiveLiteral(PrimitiveLiteralData),
    OneOf { subpatterns: Vec<PurePattern> },
    EnumLiteral(EntityRoutePtr),
    Some,
    None,
}

impl PurePattern {
    pub fn from_raw(db: &dyn InferQueryGroup, raw_patt: &RawPattern, ty: EntityRoutePtr) -> Self {
        let variant = match raw_patt.variant {
            RawPatternVariant::PrimitiveLiteral(data) => PurePatternVariant::PrimitiveLiteral(data),
            RawPatternVariant::OneOf { ref subpatterns } => PurePatternVariant::OneOf {
                subpatterns: subpatterns
                    .iter()
                    .map(|subpattern| PurePattern::from_raw(db, subpattern, ty))
                    .collect(),
            },
            RawPatternVariant::EnumLiteral(_) => todo!(),
            RawPatternVariant::Some => PurePatternVariant::Some,
            RawPatternVariant::None => PurePatternVariant::None,
        };
        Self { ty, variant }
    }
}
