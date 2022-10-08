use crate::*;

impl Ty {
    fn i32(db: &dyn TermQuery) -> TermPtr {
        db.it_term(Term::Entity(TermEntity::i32(db)))
    }
}
