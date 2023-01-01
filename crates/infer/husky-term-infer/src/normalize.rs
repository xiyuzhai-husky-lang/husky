use crate::*;
use husky_entity_path::EntityPath;
use husky_expr::*;
use husky_opn_syntax::{BinaryPunctuation, BinaryPureClosedPunctuation, Opn};
use husky_term::Term;
use husky_word::{Identifier, WordDb};

pub(crate) enum NormalizedExpr<'a> {
    Atom(&'a AtomExpr),
    Opn {
        opn_kind: NormalizedOpnKind,
        opds: ExprIdxRange,
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
                opn: ref opn_variant,
                ref opds,
            } => match opn_variant {
                Opn::Binary(opr) => NormalizedExpr::Opn {
                    opn_kind: self.resolve_binary_opr(*opr),
                    opds: opds.clone(),
                },
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(_) => todo!(),
                Opn::CurlBracketed => todo!(),
                Opn::List(_) => todo!(),
                Opn::Field(_) => todo!(),
                Opn::Abstraction => todo!(),
                Opn::Application => todo!(),
            },
            Expr::Bracketed(_) => todo!(),
            Expr::Err(_) => todo!(),
        }
    }

    fn resolve_binary_opr(&self, opr: BinaryPunctuation) -> NormalizedOpnKind {
        panic!("deprecated")
    }
}
