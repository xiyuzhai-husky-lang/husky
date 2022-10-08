use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermLiteral {
    data: TermLiteralData,
    ty: Ty,
}

impl TermLiteral {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn data(&self) -> &TermLiteralData {
        &self.data
    }

    pub fn i32_literal(db: &dyn TermQuery, i: i32, i32: Ty) -> TermPtr {
        db.it_term(Term::Literal(TermLiteral {
            data: TermLiteralData::I32(i),
            ty: i32,
        }))
    }

    pub fn i64_literal(db: &dyn TermQuery, i: i64, i64: Ty) -> TermPtr {
        db.it_term(Term::Literal(TermLiteral {
            data: TermLiteralData::I64(i),
            ty: i64,
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermLiteralData {
    I32(i32),
    I64(i64),
    // mom
}
