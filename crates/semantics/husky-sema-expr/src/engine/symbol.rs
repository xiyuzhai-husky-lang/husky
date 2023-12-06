use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn infer_current_parameter_symbols(&mut self) {
        for current_syn_symbol_idx in self
            .syn_expr_region_data
            .symbol_region()
            .current_syn_symbol_indices()
        {
            let Some(signature) = self
                .declarative_term_region
                .term_symbol_region()
                .current_parameter_symbol_signature(current_syn_symbol_idx)
            else {
                return;
            };
            if let Some(symbol) = signature.term_symbol() {
                if let Ok(symbol) = EtherealTermSymbol::from_declarative(self.db, symbol) {
                    self.symbol_terms
                        .insert_new(current_syn_symbol_idx, symbol.into())
                }
            }
            if let Ok(symbol_ty) =
                SymbolType::new_parameter_ty_from_signature(self, current_syn_symbol_idx, signature)
            {
                self.symbol_tys
                    .insert_new(current_syn_symbol_idx, symbol_ty)
            }
        }
    }

    // fn parameter_pattern_ty(&self, pattern_expr_idx: SynPatternExprIdx) -> EtherealTerm {
    //     match self
    //         .expr_region_data
    //         .symbol_region()
    //         .regular_parameter_pattern_ty_constraint(pattern_expr_idx)
    //     {
    //         Some(_) => todo!(),
    //         None => todo!(),
    //     }
    //     todo!()
    // }
}
