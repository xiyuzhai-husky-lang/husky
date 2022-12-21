use crate::*;
use husky_expr::{AtomExpr, ExprIdx, ExprRange};
use husky_primitive_literal_syntax::RawLiteralData;
use husky_print_utils::p;
use husky_symbol_syntax::SymbolKind;
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
            AtomExpr::Symbol(symbol) => match symbol.kind {
                SymbolKind::ModulePath(_) => todo!(),
                SymbolKind::LocalVariable { init_range } => todo!(),
                SymbolKind::FrameVariable { init_range } => todo!(),
                SymbolKind::ThisValue => todo!(),
                SymbolKind::ThisMethod => todo!(),
                SymbolKind::ThisField => todo!(),
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

    fn infer_literal(&self, literal: &RawLiteralData) -> Term {
        let term_menu = self.term_menu();
        match literal {
            RawLiteralData::Unit => todo!(),
            RawLiteralData::Integer(_) => term_menu.i32(),
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
}
