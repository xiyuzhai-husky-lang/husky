use crate::Term;
use interner::{DefaultInternedPtr, Interner};

pub type TermInterner = Interner<TermItd>;

pub type TermItd = DefaultInternedPtr<Term, Term>;

pub fn new_term_itr() -> TermInterner {
    TermInterner::new_empty()
}

pub trait InternTerm {
    fn term_itr(&self) -> &TermInterner;

    fn it_term(&self, term: Term) -> TermItd {
        self.term_itr().intern(term)
    }
}
