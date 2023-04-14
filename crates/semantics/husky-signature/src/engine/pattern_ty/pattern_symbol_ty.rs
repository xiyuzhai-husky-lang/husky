use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
pub struct PatternSymbolTypeInfo {
    modifier: SymbolModifier,
    base_ty: RawTerm,
}

impl PatternSymbolTypeInfo {
    fn new(modifier: SymbolModifier, base_ty: RawTerm) -> Self {
        Self { modifier, base_ty }
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn base_ty(&self) -> RawTerm {
        self.base_ty
    }
}

impl<'a> RawTermEngine<'a> {
    pub(super) fn infer_pattern_symbol_tys(&mut self, pattern_expr: PatternExprIdx) {
        for (_, pattern_symbol) in self
            .expr_region_data
            .pattern_expr_region()
            .pattern_expr_symbols(pattern_expr)
        {
            self.infer_new_pattern_symbol_ty(*pattern_symbol)
        }
    }

    fn infer_new_pattern_symbol_ty(&mut self, pattern_symbol_idx: PatternSymbolIdx) {
        let modifier = self
            .expr_region_data
            .pattern_symbol_modifier(pattern_symbol_idx);
        let base_ty = self.calc_new_pattern_symbol_base_ty(pattern_symbol_idx);
        self.pattern_symbol_ty_infos.insert_new(
            pattern_symbol_idx,
            PatternSymbolTypeInfo::new(modifier, base_ty),
        )
    }

    fn calc_new_pattern_symbol_base_ty(&mut self, pattern_symbol: PatternSymbolIdx) -> RawTerm {
        match self.expr_region_data[pattern_symbol] {
            PatternSymbol::Atom(pattern_expr) => self
                .get_pattern_expr_ty(pattern_expr)
                .expect("pattern expression type should be inferred at this point"),
        }
    }
}
