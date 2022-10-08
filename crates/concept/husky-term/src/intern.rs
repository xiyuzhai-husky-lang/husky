use crate::Term;
use interner::{DefaultInternedPtr, Interner};

pub type TermInterner = Interner<TermPtr>;

pub type TermPtr = DefaultInternedPtr<Term, Term>;

pub fn new_term_interner() -> TermInterner {
    TermInterner::new_empty()
}
