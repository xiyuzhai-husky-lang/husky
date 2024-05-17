use super::*;
use husky_eth_term::term::symbolic_variable::EthSymbolicVariable;

impl<'a> SemExprBuilder<'a> {
    pub(super) fn infer_current_parameter_symbols(&mut self) {
        for (current_variable_idx, current_variable_entry) in self
            .syn_expr_region_data
            .variable_region()
            .indexed_current_variables()
        {
            let Some(signature) = self
                .dec_term_region
                .symbolic_variable_region()
                .current_parameter_variable_signature(current_variable_idx)
            else {
                return;
            };
            if let Some(symbol) = signature.term() {
                if let Ok(symbol) = EthSymbolicVariable::from_dec(self.db, symbol) {
                    self.symbol_terms
                        .insert_new(current_variable_idx, symbol.into())
                }
            }
            if let Ok(symbol_ty) =
                SymbolType::new_parameter_ty_from_signature(self, current_variable_idx, signature)
            {
                self.symbol_tys.insert_new(current_variable_idx, symbol_ty)
            }
        }
    }

    // fn parameter_pattern_ty(&self, pattern_idx: SynPatternIdx) -> EthTerm {
    //     match self
    //         .expr_region_data
    //         .symbol_region()
    //         .regular_parameter_pattern_ty_constraint(pattern_idx)
    //     {
    //         Some(_) => todo!(),
    //         None => todo!(),
    //     }
    //     todo!()
    // }
}
