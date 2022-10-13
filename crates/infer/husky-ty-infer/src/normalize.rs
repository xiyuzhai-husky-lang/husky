use crate::*;
use husky_expr_syntax::*;

pub(crate) enum NormalizedExpr<'a> {
    Atom(&'a RawAtom),
}

impl<'a> TyInferContext<'a> {
    pub(crate) fn normalized_expr(&self) -> NormalizedExpr<'a> {
        match self.expr().variant {
            RawExprVariant::Atom(ref atom) => NormalizedExpr::Atom(atom),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => todo!(),
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }
}
