use crate::*;
use entity_route::EntityRoutePtr;
use vm::{LazyContract, PrimitiveValue};
use word::RootIdentifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MatchContract {
    Pure,
}

impl MatchContract {
    pub fn lazy(self) -> LazyContract {
        match self {
            MatchContract::Pure => LazyContract::Pure,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MatchPattern {
    pub ty: EntityRoutePtr,
    pub range: TextRange,
    pub variant: MatchPatternVariant,
}

impl TextRanged for MatchPattern {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MatchPatternVariant {
    PrimitiveLiteral(PrimitiveValue),
    OneOf { patterns: Vec<MatchPattern> },
}

impl MatchPattern {
    pub fn primitive_literal(value: PrimitiveValue, range: TextRange) -> Self {
        Self {
            ty: value.ty().into(),
            variant: MatchPatternVariant::PrimitiveLiteral(value),
            range,
        }
    }

    pub fn or(self, new_pattern: MatchPattern) -> AstResult<Self> {
        if self.ty != new_pattern.ty {
            todo!()
        }
        let ty = self.ty;
        let range = self.text_range_to(&new_pattern);
        let patterns = match self.variant {
            MatchPatternVariant::PrimitiveLiteral(_) => {
                vec![self, new_pattern]
            }
            MatchPatternVariant::OneOf { mut patterns } => {
                patterns.push(new_pattern);
                patterns
            }
        };
        Ok(MatchPattern {
            ty,
            variant: MatchPatternVariant::OneOf { patterns },
            range,
        })
    }
}
