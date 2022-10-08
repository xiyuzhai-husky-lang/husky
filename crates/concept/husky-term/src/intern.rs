use crate::Term;
use interner::{DefaultInternedPtr, Interner};

pub type TermInterner = Interner<TermPtr>;

pub type TermPtr = DefaultInternedPtr<Term, Term>;

pub fn new_term_interner() -> TermInterner {
    TermInterner::new_empty()
}

pub trait InternTerm {
    fn term_itr(&self) -> &TermInterner;
    fn it_term(&self, term: Term) -> TermPtr {
        self.term_itr().intern(term)
    }
}
