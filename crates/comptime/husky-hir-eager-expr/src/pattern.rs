use husky_hir_ty::HirType;
use husky_syn_expr::{BeVariablesPattern, LetVariableDecls};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerLetVariablesPattern {
    pattern_expr_idx: HirEagerPatternExprIdx,
    // variables: CurrentHirEagerSymbolIdxRange,
    ty: Option<HirType>,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_variables_pattern: &LetVariableDecls,
    ) -> HirEagerLetVariablesPattern {
        HirEagerLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_variables_pattern.pattern_expr_idx()),
            // variables: todo!(),
            ty: let_variables_pattern
                .ty()
                .map(|ty| HirType::from_ethereal(self.expr_term(ty), self.db())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirEagerBeVariablesPattern {}

impl ToHirEager for BeVariablesPattern {
    type Output = HirEagerBeVariablesPattern;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerBeVariablesPattern {}
    }
}
