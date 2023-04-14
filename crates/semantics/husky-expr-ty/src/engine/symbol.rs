use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_current_parameter_symbols(&mut self) {
        for current_symbol_idx in self
            .expr_region_data
            .symbol_region()
            .current_symbol_index_iter()
        {
            let Some(signature) = self
                .signature_term_region
                .term_symbol_region()
                .current_symbol_signature(current_symbol_idx) else {
                return
            };
            if let Some(symbol) = signature.symbol() {
                if let Ok(symbol) = TermSymbol::from_raw(self.db, symbol) {
                    self.symbol_terms
                        .insert_new(current_symbol_idx, symbol.into())
                }
            }
            if let Ok(qualified_ty) =
                FluffyTerm::new_symbol_ty_from_signature(self, current_symbol_idx, signature)
            {
                self.symbol_tys.insert_new(current_symbol_idx, qualified_ty)
            }
        }
    }

    fn parameter_pattern_ty(&self, pattern_expr_idx: PatternExprIdx) -> Term {
        match self
            .expr_region_data
            .symbol_region()
            .regular_parameter_pattern_ty_constraint(pattern_expr_idx)
        {
            Some(_) => todo!(),
            None => todo!(),
        }
        todo!()
    }
}
