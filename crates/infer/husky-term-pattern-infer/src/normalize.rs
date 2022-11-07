use crate::*;
use husky_entity_path::EntityPathItd;
use husky_expr_syntax::*;
use husky_opn_syntax::{BinaryOpr, PureBinaryOpr, RawOpnVariant};
use husky_term::TermItd;
use husky_word::{Identifier, InternWord};

pub(crate) enum NormalizedExpr<'a> {
    Atom(&'a RawAtom),
    Opn {
        opn_kind: NormalizedOpnKind,
        opds: RawExprRange,
    },
}

pub(crate) enum NormalizedOpnKind {
    ApplyMethod {
        opt_trait_entity: Option<TraitEntity>,
        method_ident: Identifier,
    },
    ScopeResolution,
}

pub struct TraitEntity(TermItd);

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn normalized_expr(&self) -> NormalizedExpr<'a> {
        match self.expr().variant {
            RawExprVariant::Atom(ref atom) => NormalizedExpr::Atom(atom),
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                RawOpnVariant::Binary(opr) => NormalizedExpr::Opn {
                    opn_kind: self.resolve_binary_opr(*opr),
                    opds: opds.clone(),
                },
                RawOpnVariant::Prefix(_) => todo!(),
                RawOpnVariant::Suffix(_) => todo!(),
                RawOpnVariant::List(_) => todo!(),
                RawOpnVariant::Field(_) => todo!(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn resolve_binary_opr(&self, opr: BinaryOpr) -> NormalizedOpnKind {
        match opr {
            BinaryOpr::Pure(pure_opr) => match pure_opr {
                PureBinaryOpr::Add => NormalizedOpnKind::ApplyMethod {
                    opt_trait_entity: Some(TraitEntity(self.term_menu().core_ops_add())),
                    method_ident: self.it_word("add").ident(),
                },
                PureBinaryOpr::And => todo!(),
                PureBinaryOpr::BitAnd => todo!(),
                PureBinaryOpr::BitOr => todo!(),
                PureBinaryOpr::BitXor => todo!(),
                PureBinaryOpr::Div => todo!(),
                PureBinaryOpr::Eq => todo!(),
                PureBinaryOpr::Geq => todo!(),
                PureBinaryOpr::Greater => todo!(),
                PureBinaryOpr::Leq => todo!(),
                PureBinaryOpr::Less => todo!(),
                PureBinaryOpr::Mul => todo!(),
                PureBinaryOpr::Neq => todo!(),
                PureBinaryOpr::Or => todo!(),
                PureBinaryOpr::RemEuclid => todo!(),
                PureBinaryOpr::Power => todo!(),
                PureBinaryOpr::Shl => todo!(),
                PureBinaryOpr::Shr => todo!(),
                PureBinaryOpr::Sub => todo!(),
            },
            BinaryOpr::Assign(_) => todo!(),
            BinaryOpr::ScopeResolution => NormalizedOpnKind::ScopeResolution,
            BinaryOpr::Curry => todo!(),
            BinaryOpr::As => todo!(),
        }
    }
}
