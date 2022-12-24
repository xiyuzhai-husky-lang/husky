use crate::*;
use husky_entity_path::EntityPath;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr, RawOpnVariant};
use husky_term::Term;
use husky_word::{Identifier, WordDb};

pub(crate) enum NormalizedExpr<'a> {
    Atom(&'a AtomExpr),
    Opn {
        opn_kind: NormalizedOpnKind,
        opds: ExprRange,
    },
}

pub(crate) enum NormalizedOpnKind {
    ApplyMethod {
        opt_trait_entity: Option<TraitEntity>,
        method_ident: Identifier,
    },
    ScopeResolution,
}

pub struct TraitEntity(Term);

impl<'a> InferContext<'a> {
    pub(crate) fn normalized_expr(&self) -> NormalizedExpr<'a> {
        match self.expr() {
            Expr::Atom(ref atom) => NormalizedExpr::Atom(atom),
            Expr::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                RawOpnVariant::Binary(opr) => NormalizedExpr::Opn {
                    opn_kind: self.resolve_binary_opr(*opr),
                    opds: opds.clone(),
                },
                RawOpnVariant::Prefix(_) => todo!(),
                RawOpnVariant::Suffix(_) => todo!(),
                RawOpnVariant::CurlBracketed => todo!(),
                RawOpnVariant::List(_) => todo!(),
                RawOpnVariant::Field(_) => todo!(),
                RawOpnVariant::Abstraction => todo!(),
            },
        }
    }

    fn resolve_binary_opr(&self, opr: BinaryOpr) -> NormalizedOpnKind {
        panic!("deprecated")
    }
}
