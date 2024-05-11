use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirEagerLetVariablesPattern {
    pub pattern_idx: HirEagerPatternIdx,
    // variables: CurrentHirEagerSymbolIdxRange,
    pub ty: Option<HirType>,
}

impl HirEagerLetVariablesPattern {
    pub fn pattern_idx(self) -> HirEagerPatternIdx {
        self.pattern_idx
    }

    pub fn ty(self) -> Option<HirType> {
        self.ty
    }
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(crate) fn new_let_variables_pattern(
        &mut self,
        let_pattern_sem_obelisk: &LetVariableObelisk,
    ) -> HirEagerLetVariablesPattern {
        HirEagerLetVariablesPattern {
            pattern_idx: self.new_pattern(let_pattern_sem_obelisk.syn_pattern_root()),
            ty: let_pattern_sem_obelisk
                .ty_sem_expr_idx()
                .map(|ty_sem_expr_idx| {
                    HirType::from_eth(self.expr_term(ty_sem_expr_idx), self.db()).unwrap()
                }),
        }
    }
}
