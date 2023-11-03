use husky_hir_ty::HirType;
use husky_sema_expr::LetPatternSemaObelisk;
use husky_syn_expr::{BePatternSynObelisk, LetPatternSynObelisk};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerLetVariablesPattern {
    pattern_expr_idx: HirEagerPatternExprIdx,
    // variables: CurrentHirEagerSymbolIdxRange,
    ty: Option<HirType>,
}

impl HirEagerLetVariablesPattern {
    pub fn pattern_expr_idx(self) -> HirEagerPatternExprIdx {
        self.pattern_expr_idx
    }

    pub fn ty(self) -> Option<HirType> {
        self.ty
    }
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_let_variables_pattern(
        &mut self,
        let_pattern_sema_obelisk: &LetPatternSemaObelisk,
    ) -> HirEagerLetVariablesPattern {
        HirEagerLetVariablesPattern {
            pattern_expr_idx: self.new_pattern_expr(let_pattern_sema_obelisk.syn_pattern_root()),
            // variables: todo!(),
            ty: let_pattern_sema_obelisk
                .ty_sema_expr_idx()
                .map(|ty_sema_expr_idx| {
                    HirType::from_ethereal(self.expr_term(ty_sema_expr_idx), self.db())
                }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerBeVariablesPattern {}

impl ToHirEager for BePatternSynObelisk {
    type Output = HirEagerBeVariablesPattern;

    fn to_hir_eager(&self, builder: &mut HirEagerExprBuilder) -> Self::Output {
        HirEagerBeVariablesPattern {}
    }
}
