use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_current_symbol_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: &impl ExpectLocalTerm,
        current_symbol_idx: CurrentSymbolIdx,
        current_symbol_kind: CurrentSymbolKind,
    ) -> ExprTypeResult<LocalTerm> {
        let current_symbol_ty = self
            .current_symbol_tys
            .get(current_symbol_idx)
            .copied()
            .ok_or(DerivedExprTypeError::CurrentSymbolTypeError)?;
        Ok(match current_symbol_kind {
            CurrentSymbolKind::ImplicitParameter {
                implicit_parameter_kind,
            } => todo!(),
            CurrentSymbolKind::Parameter { pattern_symbol_idx } => todo!(),
            CurrentSymbolKind::LetVariable { pattern_symbol_idx } => {
                self.local_term_region.intern_unresolved_term(
                    expr_idx,
                    LocalTermData::Qualified {
                        qual: Qual::Todo,
                        base_ty: current_symbol_ty,
                    },
                )
            }
            CurrentSymbolKind::FrameVariable(_) => current_symbol_ty,
        })
    }
}
