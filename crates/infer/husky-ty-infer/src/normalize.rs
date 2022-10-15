use crate::*;
use husky_expr_syntax::*;
use husky_opn_syntax::RawOpnVariant;

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
            } => match opn_variant {
                RawOpnVariant::Binary(_) => todo!(),
                RawOpnVariant::Prefix(_) => todo!(),
                RawOpnVariant::Suffix(_) => todo!(),
                RawOpnVariant::List(_) => todo!(),
                RawOpnVariant::Field(_) => todo!(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }
}
