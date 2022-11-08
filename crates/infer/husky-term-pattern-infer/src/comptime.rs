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
            RawExprVariant::Atom(ref atom) => self.infer_atom_const_expr(atom),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.infer_opn_const_expr(opn_variant, opds),
        }
    }

    fn infer_atom_const_expr(
        &self,
        atom: &RawAtomExpr,
    ) -> TermPatternInferResult<Option<ConstExprPattern>> {
        match atom {
            RawAtomExpr::Literal(literal) => self.infer_literal_const_expr(literal),
            RawAtomExpr::Symbol(_) => todo!(),
            RawAtomExpr::Uncertain => todo!(),
        }
    }

    fn infer_literal_const_expr(
        &self,
        literal: &RawLiteralData,
    ) -> TermPatternInferResult<Option<ConstExprPattern>> {
        match literal {
            RawLiteralData::Unit => todo!(),
            RawLiteralData::Integer(_) => todo!(),
            RawLiteralData::I32(_) => todo!(),
            RawLiteralData::I64(_) => todo!(),
            RawLiteralData::Float(_) => todo!(),
            RawLiteralData::F32(_) => todo!(),
            RawLiteralData::F64(_) => todo!(),
            RawLiteralData::Bits(_) => todo!(),
            RawLiteralData::B32(_) => todo!(),
            RawLiteralData::B64(_) => todo!(),
            RawLiteralData::Bool(_) => todo!(),
        }
    }

    fn infer_opn_const_expr(
        &self,
        opn_variant: &RawOpnVariant,
        opds: &RawExprRange,
    ) -> TermPatternInferResult<Option<ConstExprPattern>> {
        todo!()
    }
}
