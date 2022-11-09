use crate::*;
use interner::{DefaultItd, Internable, Interner};
use optional::{Noned, OptEq};
use std::borrow::Borrow;

pub type TermItd = TermBorrowed<'static>;

impl<'a> Noned for TermBorrowed<'a> {
    fn is_none(&self) -> bool {
        self == &TermBorrowed::Null
    }

    fn get_none() -> Self {
        TermBorrowed::Null
    }
}

impl OptEq for TermItd {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub type TermInterner = Interner<Term>;

impl Internable for Term {
    type BorrowedRaw = TermBorrowedRaw;

    type Borrowed<'a> = TermBorrowed<'a>;

    type Interned = TermItd;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itr() -> Interner<Self> {
        todo!()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        todo!()
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Borrowed<'static> {
        todo!()
    }
}

pub fn new_term_itr() -> TermInterner {
    TermInterner::new_empty()
}

pub trait InternTerm {
    fn term_itr(&self) -> &TermInterner;

    fn it_term(&self, term: Term) -> TermItd {
        self.term_itr().intern(term)
    }
}

impl From<i32> for TermItd {
    fn from(value: i32) -> Self {
        TermItd::Atom(value.into())
    }
}
