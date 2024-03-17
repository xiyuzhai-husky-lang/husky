use super::*;
use husky_syn_expr::BePatternSyndicate;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerBeVariablesPattern {
    pub pattern: HirEagerPatternIdx,
}

impl ToHirEager for BePatternSyndicate {
    type Output = HirEagerBeVariablesPattern;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerBeVariablesPattern {
            pattern: builder.new_pattern(self.syn_pattern_root()),
        }
    }
}
