use crate::{namespace::TermNamespace, Term, TermLiteral, TermPtr, TermQuery, Ty};

impl Ty {
    fn i32(db: &dyn TermQuery) -> TermPtr {
        db.it_term(Term::Namespace(TermNamespace::i32(db)))
    }
}
