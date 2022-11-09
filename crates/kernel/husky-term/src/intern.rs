use crate::*;
use interner::{DefaultItd, Interned, Interner};
use optional::{Noned, OptEq};
use std::borrow::Borrow;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TermItd {
    Atom(TermAtom),
    Interned(DefaultItd<Term, Term>),
    Null,
}

impl Noned for TermItd {
    fn is_none(&self) -> bool {
        self == &TermItd::Null
    }

    fn get_none() -> Self {
        TermItd::Null
    }
}

impl OptEq for TermItd {
    fn opt_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Interned for TermItd {
    type T = Term;

    type Owned = Term;

    fn new_interned(id: usize, target: &'static Self::T) -> Self {
        todo!()
    }

    fn new_itr() -> Interner<Self> {
        Interner::new(&[])
    }

    fn opt_atom_itd(t: &Self::T) -> Option<Self> {
        match t {
            Term::Atom(atom) => Some(TermItd::Atom(*atom)),
            _ => None,
        }
    }
}

impl Borrow<Term> for TermItd {
    fn borrow(&self) -> &Term {
        todo!()
    }
}

impl std::ops::Deref for TermItd {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        match self {
            TermItd::Atom(_) => todo!(),
            TermItd::Interned(_) => todo!(),
            TermItd::Null => todo!(),
        }
    }
}

pub type TermInterner = Interner<TermItd>;

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
