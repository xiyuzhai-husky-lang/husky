use crate::*;
use interner::{Internable, Interner};
use optional::{Noned, OptEq};
use std::borrow::Borrow;

pub type TermInterner = Interner<TermOwned>;

impl Internable for TermOwned {
    type Borrowed<'a> = TermBorrowed<'a>;

    type Interned = TermItd;

    fn borrow<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itr() -> Interner<Self> {
        Interner::new_empty()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        match self {
            TermOwned::Atom(atom) => Some(TermItd(TermBorrowed::Atom(*atom))),
            _ => None,
        }
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Borrowed<'static> {
        todo!()
    }

    fn to_borrowed<'a>(&'a self) -> Self::Borrowed<'a> {
        todo!()
    }

    fn new_itd(&'static self, id: usize) -> Self::Interned {
        todo!()
    }
}

pub fn new_term_itr() -> TermInterner {
    TermInterner::new_empty()
}

pub trait InternTerm {
    fn term_itr(&self) -> &TermInterner;

    fn it_term(&self, term: TermOwned) -> TermItd {
        self.term_itr().intern(term)
    }
}

impl From<i32> for TermItd {
    fn from(value: i32) -> Self {
        TermItd(TermBorrowed::Atom(value.into()))
    }
}
