use crate::*;
use husky_text::{HasTextRange, TextRange};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawPattern {
    pub range: TextRange,
    pub variant: PatternToken,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PatternToken {
    PrimitiveLiteral(LiteralToken),
    OneOf { subpatterns: Vec<RawPattern> },
    Some,
    None,
}

impl HasTextRange for RawPattern {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl RawPattern {
    pub fn primitive_literal(value: LiteralToken, range: TextRange) -> Self {
        Self {
            variant: PatternToken::PrimitiveLiteral(value),
            range,
        }
    }

    pub fn or(self, new_pattern: RawPattern) -> Self {
        let range = self.text_range_to(&new_pattern);
        let patterns = match self.variant {
            PatternToken::PrimitiveLiteral(_) => {
                vec![self, new_pattern]
            }
            PatternToken::OneOf {
                subpatterns: mut patterns,
            } => {
                patterns.push(new_pattern);
                patterns
            }
            PatternToken::Some => todo!(),
            PatternToken::None => todo!(),
        };
        RawPattern {
            variant: PatternToken::OneOf {
                subpatterns: patterns,
            },
            range,
        }
    }
}
