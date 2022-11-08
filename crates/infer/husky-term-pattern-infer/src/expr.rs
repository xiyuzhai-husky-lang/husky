use crate::*;

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn infer_expr_term_pattern(&self) -> TermPatternInferResult<Option<TermPatternItd>> {
        match self.expr().variant {
            RawExprVariant::Atom(ref atom) => self.infer_atom_expr_term_pattern(atom),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn_expr_term_pattern(opn_variant, opds),
        }
    }

    fn infer_atom_expr_term_pattern(
        &self,
        atom: &RawAtomExpr,
    ) -> TermPatternInferResult<Option<TermPatternItd>> {
        todo!()
    }

    fn infer_opn_expr_term_pattern(
        &self,
        opn_variant: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> TermPatternInferResult<Option<TermPatternItd>> {
        todo!()
    }
}
