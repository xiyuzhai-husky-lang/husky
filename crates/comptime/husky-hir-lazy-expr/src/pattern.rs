use husky_hir_ty::HirType;
use husky_syn_expr::{BeVariablesObelisk, LetVariableObelisk};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyLetVariablesPattern {
    pattern_expr_idx: HirLazyPatternExprIdx,
    // variables: CurrentHirLazySymbolIdxRange,
    ty: Option<HirType>,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_variables_pattern: &LetVariableObelisk,
    ) -> HirLazyLetVariablesPattern {
        HirLazyLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_variables_pattern.pattern_expr_idx()),
            // variables: todo!(),
            ty: let_variables_pattern
                .ty()
                .map(|ty| HirType::from_ethereal(self.expr_term(ty), self.db())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HirLazyBeVariablesPattern {}

impl ToHirLazy for BeVariablesObelisk {
    type Output = HirLazyBeVariablesPattern;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyBeVariablesPattern {}
    }
}
