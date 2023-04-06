use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PatternSymbolTypeInfo {
    ty: RawTerm,
}

impl PatternSymbolTypeInfo {
    fn new(ty: RawTerm) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> RawTerm {
        self.ty
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

    fn infer_new_pattern_symbol_ty(&mut self, pattern_symbol: PatternSymbolIdx) {
        let ty = self.calc_new_pattern_symbol_ty(pattern_symbol);
        self.pattern_symbol_ty_infos
            .insert_new(pattern_symbol, PatternSymbolTypeInfo::new(ty))
    }

    fn calc_new_pattern_symbol_ty(&mut self, pattern_symbol_idx: PatternSymbolIdx) -> RawTerm {
        match self.expr_region_data[pattern_symbol_idx] {
            PatternSymbol::Atom(pattern_expr_idx) => self
                .get_pattern_expr_ty(pattern_expr_idx)
                .expect("pattern expression type should be inferred at this point"),
        }
    }
}
