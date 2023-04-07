use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_all_parameter_symbols(&mut self) {
        self.infer_inherited_parameter_symbols();
        self.infer_current_parameter_symbols()
    }

    fn infer_inherited_parameter_symbols(&mut self) {
        let Some(parent) = self.expr_region_data.parent()
            else{
                return;
            };
        let parent_symbol_region = parent.data(self.db).symbol_region();
        for (inherited_symbol_idx, inherited_symbol) in self
            .expr_region_data
            .symbol_region()
            .indexed_inherited_symbol_iter()
        {
            match self
                .signature_term_region
                .term_symbol_region()
                .inherited_symbol_signature(inherited_symbol_idx)
            {
                SymbolSignature::ImplicitParamater { .. } => todo!(),
                SymbolSignature::ExplicitParamater { .. } => todo!(),
            }
            // if let Ok(inherited_symbol_signature) = inherited_symbol_signature {
            //     todo!()
            //     // if let Ok(ty) = Term::ty_from_raw(self.db, ty) {
            //     //     self.inherited_symbol_tys
            //     //         .insert_new(inherited_symbol_idx, ty)
            //     // }
            // }
        }
    }

    fn infer_current_parameter_symbols(&mut self) {
        for current_symbol_idx in self
            .expr_region_data
            .symbol_region()
            .current_symbol_index_iter()
        {
            let Some(current_symbol_signature) = self
                .signature_term_region
                .term_symbol_region()
                .current_symbol_signature(current_symbol_idx)
                else {
                    return
                };
            if let (symbol) = current_symbol_signature.symbol() {
                self.current_symbol_qualified_tys
                    .insert_new(current_symbol_idx, ty.into())
            }
            // if let Ok(ty) = current_symbol_signature.ty(self.db) {
            //     if let Ok(ty) = Term::ty_from_raw(self.db, ty.into()) {
            //     }
            // }
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
