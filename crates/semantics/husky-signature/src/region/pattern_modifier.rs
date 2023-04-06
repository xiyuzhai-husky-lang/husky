use super::*;
use husky_expr::PatternExprIdx;

impl SignatureRegion {
    pub fn pattern_modifier(&self, pattern_expr: PatternExprIdx) -> PatternModifier {
        self.pattern_modifiers[pattern_expr]
    }
}
