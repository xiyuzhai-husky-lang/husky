use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub(crate) struct PatternSymbolDeclarativeTypeInfo {
    modifier: SymbolModifier,
    base_ty: DeclarativeTerm,
}

impl PatternSymbolDeclarativeTypeInfo {
    fn new(modifier: SymbolModifier, base_ty: DeclarativeTerm) -> Self {
        Self { modifier, base_ty }
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn base_ty(&self) -> DeclarativeTerm {
        self.base_ty
    }
}

impl<'a> DeclarativeTermEngine<'a> {
    pub(super) fn infer_pattern_symbol_tys(
        &mut self,
        syn_pattern_expr_root: impl Into<SynPatternExprRoot>,
    ) {
        let syn_pattern_expr_root = syn_pattern_expr_root.into();
        for (_, pattern_symbol) in self
            .syn_expr_region_data
            .pattern_expr_region()
            .pattern_expr_symbols(syn_pattern_expr_root.syn_pattern_expr_idx())
        {
            self.infer_new_pattern_symbol_ty(*pattern_symbol)
        }
    }

    fn infer_new_pattern_symbol_ty(&mut self, pattern_symbol_idx: SynPatternSymbolIdx) {
        let modifier = self
            .syn_expr_region_data
            .pattern_symbol_modifier(pattern_symbol_idx);
        let base_ty = self.calc_new_pattern_symbol_base_ty(pattern_symbol_idx);
        self.pattern_symbol_ty_infos.insert_new(
            pattern_symbol_idx,
            PatternSymbolDeclarativeTypeInfo::new(modifier, base_ty),
        )
    }

    fn calc_new_pattern_symbol_base_ty(
        &mut self,
        pattern_symbol: SynPatternSymbolIdx,
    ) -> DeclarativeTerm {
        match self.syn_expr_region_data[pattern_symbol] {
            SynPatternSymbol::Atom(pattern_expr) => self
                .get_pattern_expr_ty(pattern_expr)
                .expect("pattern expression type should be inferred at this point"),
        }
    }
}
