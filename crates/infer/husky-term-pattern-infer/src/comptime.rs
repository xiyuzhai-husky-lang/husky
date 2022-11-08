use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ConstExprPattern {
    term: TermPatternItd,
    opt_substitution_ctx_idx: Option<TermSubstitutionContextIdx>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TermSubstitutionContextIdx(usize);

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn infer_comptime(&self) -> TermPatternInferResult<Option<ConstExprPattern>> {
        match self.expr().variant {
            RawExprVariant::Atom(ref atom) => self.infer_atom_comptime_info(atom),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn_comptime_info(opn_variant, opds),
        }
    }

    fn infer_atom_comptime_info(
        &self,
        atom: &RawAtomExpr,
    ) -> TermPatternInferResult<Option<ConstExprPattern>> {
        match atom {
            RawAtomExpr::Literal(literal) => todo!(),
            RawAtomExpr::Symbol(_) => todo!(),
            RawAtomExpr::Uncertain => todo!(),
        }
    }

    fn infer_opn_comptime_info(
        &self,
        opn_variant: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> TermPatternInferResult<Option<ConstExprPattern>> {
        todo!()
    }
}
