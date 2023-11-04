use husky_hir_ty::HirType;
use husky_sema_expr::LetPatternSemaSyndicate;
use husky_syn_expr::{BePatternSynSyndicate, LetPatternSynSyndicate};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyLetVariablesPattern {
    pattern_expr_idx: HirLazyPatternExprIdx,
    // variables: CurrentHirLazySymbolIdxRange,
    ty: Option<HirType>,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_variables_pattern: &LetPatternSemaSyndicate,
    ) -> HirLazyLetVariablesPattern {
        HirLazyLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_variables_pattern.syn_pattern_root()),
            ty: let_variables_pattern
                .ty_sema_expr_idx()
                .map(|ty_sema_expr_idx| {
                    HirType::from_ethereal(self.expr_term(ty_sema_expr_idx), self.db())
                }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirLazyBeVariablesPattern {}

impl ToHirLazy for BePatternSynSyndicate {
    type Output = HirLazyBeVariablesPattern;

    fn to_hir_lazy(&self, builder: &mut HirLazyExprBuilder) -> Self::Output {
        HirLazyBeVariablesPattern {}
    }
}
