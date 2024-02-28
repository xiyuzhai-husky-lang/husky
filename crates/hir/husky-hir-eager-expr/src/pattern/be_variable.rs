use super::*;
use husky_syn_expr::BePatternSyndicate;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerBeVariablesPattern {
    pub pattern_expr_idx: HirEagerPatternExprIdx,
}

impl ToHirEager for BePatternSyndicate {
    type Output = HirEagerBeVariablesPattern;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerBeVariablesPattern {
            pattern_expr_idx: builder.new_pattern_expr(self.syn_pattern_root()),
        }
    }
}
