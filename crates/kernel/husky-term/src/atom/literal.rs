use std::ops::Deref;

use ordered_float::OrderedFloat;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermLiteral(TermPtr);

impl std::ops::Deref for TermLiteral {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TermLiteral {
    pub fn data(&self) -> &TermLiteralData {
        match self.deref() {
            Term::Atom(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Universe(_) => todo!(),
        }
    }

    pub fn i32_literal(db: &dyn TermQuery, i: i32, i32: Ty) -> TermPtr {
        db.it_term(Term::Atom(TermAtom::new_literal(
            TermLiteralData::I32(i),
            i32,
        )))
    }

    pub fn i64_literal(db: &dyn TermQuery, i: i64, i64: Ty) -> TermPtr {
        db.it_term(Term::Atom(TermAtom::new_literal(
            TermLiteralData::I64(i),
            i64,
        )))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TermLiteralData {
    Void,
    I32(i32),
    I64(i64),
    Float(OrderedFloat<f64>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    Bits(u64),
    B32(u32),
    B64(u64),
    Bool(bool),
}
