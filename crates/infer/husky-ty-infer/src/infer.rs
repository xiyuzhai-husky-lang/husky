use crate::*;
use husky_expr_syntax::RawAtom;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_term::Ty;

impl<'a> TyInferContext<'a> {
    pub(crate) fn infer(&mut self) -> Ty {
        match self.normalized_expr() {
            NormalizedExpr::Atom(atom) => self.infer_atom(atom),
        }
    }

    fn infer_atom(&self, atom: &RawAtom) -> Ty {
        match atom {
            RawAtom::Literal(literal) => self.infer_literal(literal),
            RawAtom::Symbol(_) => todo!(),
        }
    }

    fn infer_literal(&self, literal: &RawLiteralData) -> Ty {
        let term_menu = self.term_menu();
        match literal {
            RawLiteralData::Void => todo!(),
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
