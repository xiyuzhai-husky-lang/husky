use crate::*;
use husky_expr::{AtomExpr, ExprIdx, ExprRange};
use husky_print_utils::p;
use husky_symbol::Symbol;
use husky_term::Term;

impl<'a> InferContext<'a> {
    pub(crate) fn infer(&mut self) -> TermInferResult<Term> {
        match self.normalized_expr() {
            NormalizedExpr::Atom(atom) => self.infer_atom(atom),
            NormalizedExpr::Opn { opn_kind, opds } => self.infer_opn(opn_kind, opds),
        }
    }

    fn infer_subexpr(&mut self, subexpr: ExprIdx) -> TermInferResult<Term> {
        self.subexpr_context(subexpr).infer()
    }

    fn infer_atom(&self, atom: &AtomExpr) -> TermInferResult<Term> {
        match atom {
            AtomExpr::Literal(literal) => Ok(self.infer_literal(literal)),
            AtomExpr::Symbol(symbol) => match symbol {
                Symbol::Entity(_) => todo!(),
                Symbol::Variable(_) => todo!(),
                Symbol::Lifetime(_) => todo!(),
                Symbol::Label(_) => todo!(),
            },
            AtomExpr::Unrecognized(_) => Err(TermInferError::IdentUnrecognized),
            AtomExpr::Uncertain(_) => todo!(),
        }
    }

    fn infer_opn(&mut self, opn_kind: NormalizedOpnKind, opds: ExprRange) -> TermInferResult<Term> {
        match opn_kind {
            NormalizedOpnKind::ApplyMethod {
                opt_trait_entity,
                method_ident,
            } => todo!(),
            NormalizedOpnKind::ScopeResolution => todo!(),
        }
        let this_ty = self.infer_subexpr(opds.start());
        p!(this_ty);
        todo!()
    }

    fn infer_literal(&self, literal: &LiteralToken) -> Term {
        let term_menu = self.term_menu();
        match literal {
            LiteralToken::Unit => todo!(),
            LiteralToken::Integer(_) => term_menu.i32(),
            LiteralToken::I32(_) => todo!(),
            LiteralToken::I64(_) => todo!(),
            LiteralToken::Float(_) => todo!(),
            LiteralToken::F32(_) => todo!(),
            LiteralToken::F64(_) => todo!(),
            LiteralToken::Bits(_) => todo!(),
            LiteralToken::B32(_) => todo!(),
            LiteralToken::B64(_) => todo!(),
            LiteralToken::Bool(_) => todo!(),
            LiteralToken::String(_) => todo!(),
        }
    }
}
