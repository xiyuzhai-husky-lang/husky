use crate::Term;
use interner::{DefaultInternedPtr, Interner};

pub type TermInterner = Interner<TermPtr>;

pub type TermPtr = DefaultInternedPtr<Term, Term>;
