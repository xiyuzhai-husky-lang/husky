use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub(crate) struct PatternSymbolTypeInfo {
    modifier: EphemSymbolModifier,
    base_ty: DeclarativeTerm,
}

impl PatternSymbolTypeInfo {
    fn new(modifier: EphemSymbolModifier, base_ty: DeclarativeTerm) -> Self {
        Self { modifier, base_ty }
    }

    pub fn modifier(&self) -> EphemSymbolModifier {
        self.modifier
    }

    pub fn base_ty(&self) -> DeclarativeTerm {
        self.base_ty
    }
}

impl<'a> DeclarativeTermEngine<'a> {
    pub(super) fn infer_pattern_symbol_tys(&mut self, pattern_expr: SynPatternExprIdx) {
        for (_, pattern_symbol) in self
            .expr_region_data
            .pattern_expr_region()
            .pattern_expr_symbols(pattern_expr)
        {
            self.infer_new_pattern_symbol_ty(*pattern_symbol)
        }
    }

    fn infer_new_pattern_symbol_ty(&mut self, pattern_symbol_idx: SynPatternSymbolIdx) {
        let modifier = self
            .expr_region_data
            .pattern_symbol_modifier(pattern_symbol_idx);
        let base_ty = self.calc_new_pattern_symbol_base_ty(pattern_symbol_idx);
        self.pattern_symbol_ty_infos.insert_new(
            pattern_symbol_idx,
            PatternSymbolTypeInfo::new(modifier, base_ty),
        )
    }

    fn calc_new_pattern_symbol_base_ty(
        &mut self,
        pattern_symbol: SynPatternSymbolIdx,
    ) -> DeclarativeTerm {
        match self.expr_region_data[pattern_symbol] {
            SynPatternSymbol::Atom(pattern_expr) => self
                .get_pattern_expr_ty(pattern_expr)
                .expect("pattern expression type should be inferred at this point"),
        }
    }
}
