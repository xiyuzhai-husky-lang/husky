use super::*;
use husky_expr::stmt::ConditionConversion;
use husky_regional_token::RegionalTokenIdx;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemaCondition {
    /// `be` condition with syntactically correct pattern.
    /// This requires special handling for many cases.
    Be {
        src: SemaExprIdx,
        be_regional_token_idx: RegionalTokenIdx,
        target: BePatternSynSyndicate,
    },
    /// all other conditions.
    /// for simplicity, `be` with a syntactically broken pattern is also included in there
    Other {
        sema_expr_idx: SemaExprIdx,
        conversion: ConditionConversion,
    },
}
