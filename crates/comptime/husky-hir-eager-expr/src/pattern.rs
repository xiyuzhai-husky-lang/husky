use husky_hir_ty::HirType;
use husky_syn_expr::{BePatternObelisk, LetPatternObelisk};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerLetVariablesPattern {
    pattern_expr_idx: HirEagerPatternExprIdx,
    // variables: CurrentHirEagerSymbolIdxRange,
    ty: Option<HirType>,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_variables_pattern: &LetPatternObelisk,
    ) -> HirEagerLetVariablesPattern {
        HirEagerLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_variables_pattern.syn_pattern_root()),
            // variables: todo!(),
            ty: let_variables_pattern
                .ty()
                .map(|ty| HirType::from_ethereal(self.expr_term(ty), self.db())),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerBeVariablesPattern {}

impl ToHirEager for BePatternObelisk {
    type Output = HirEagerBeVariablesPattern;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerBeVariablesPattern {}
    }
}
