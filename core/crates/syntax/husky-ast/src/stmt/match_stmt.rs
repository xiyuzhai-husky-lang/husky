use crate::*;
use husky_entity_route::EntityRoutePtr;
use husky_token::PrimitiveLiteralData;
use map_collect::MapCollect;
use vm::{PrimitiveValueData, VMCasePattern};
use word::RootIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MatchLiason {
    Pure,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawCasePattern {
    pub range: TextRange,
    pub variant: RawCasePatternVariant,
}

impl RawCasePattern {
    pub fn compile(&self) -> VMCasePattern {
        match self.variant {
            RawCasePatternVariant::PrimitiveValue(value) => {
                todo!()
                // VMCasePattern::Primitive(primitive_value_from_literal(self.ty, value))
            }
            RawCasePatternVariant::OneOf { ref patterns } => {
                VMCasePattern::OneOf(patterns.map(|pattern| pattern.compile()))
            }
            RawCasePatternVariant::EnumLiteral(entity_route) => {
                VMCasePattern::EnumKindLiteral(entity_route)
            }
        }
    }
}

impl TextRanged for RawCasePattern {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawCasePatternVariant {
    PrimitiveValue(PrimitiveLiteralData),
    OneOf { patterns: Vec<RawCasePattern> },
    EnumLiteral(EntityRoutePtr),
}

impl RawCasePattern {
    pub fn primitive_literal(value: PrimitiveLiteralData, range: TextRange) -> Self {
        Self {
            variant: RawCasePatternVariant::PrimitiveValue(value),
            range,
        }
    }

    pub fn enum_literal(value: EntityRoutePtr, range: TextRange) -> Self {
        Self {
            variant: RawCasePatternVariant::EnumLiteral(value),
            range,
        }
    }

    pub fn or(self, new_pattern: RawCasePattern) -> AstResult<Self> {
        let range = self.text_range_to(&new_pattern);
        let patterns = match self.variant {
            RawCasePatternVariant::PrimitiveValue(_) | RawCasePatternVariant::EnumLiteral(_) => {
                vec![self, new_pattern]
            }
            RawCasePatternVariant::OneOf { mut patterns } => {
                patterns.push(new_pattern);
                patterns
            }
        };
        Ok(RawCasePattern {
            variant: RawCasePatternVariant::OneOf { patterns },
            range,
        })
    }
}

fn primitive_value_from_literal(
    ty: EntityRoutePtr,
    value: PrimitiveLiteralData,
) -> PrimitiveValueData {
    todo!()
}
