use crate::*;
use entity_route::EntityRoutePtr;
use vm::{EagerContract, LazyContract, PrimitiveValue};
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

    pub fn eager(self) -> EagerContract {
        match self {
            MatchContract::Pure => EagerContract::Pure,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CasePattern {
    pub ty: EntityRoutePtr,
    pub range: TextRange,
    pub variant: CasePatternVariant,
}

impl TextRanged for CasePattern {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CasePatternVariant {
    PrimitiveLiteral(PrimitiveValue),
    OneOf { patterns: Vec<CasePattern> },
}

impl CasePattern {
    pub fn primitive_literal(value: PrimitiveValue, range: TextRange) -> Self {
        Self {
            ty: value.ty().into(),
            variant: CasePatternVariant::PrimitiveLiteral(value),
            range,
        }
    }

    pub fn or(self, new_pattern: CasePattern) -> AstResult<Self> {
        if self.ty != new_pattern.ty {
            todo!()
        }
        let ty = self.ty;
        let range = self.text_range_to(&new_pattern);
        let patterns = match self.variant {
            CasePatternVariant::PrimitiveLiteral(_) => {
                vec![self, new_pattern]
            }
            CasePatternVariant::OneOf { mut patterns } => {
                patterns.push(new_pattern);
                patterns
            }
        };
        Ok(CasePattern {
            ty,
            variant: CasePatternVariant::OneOf { patterns },
            range,
        })
    }
}
