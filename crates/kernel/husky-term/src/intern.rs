use crate::*;
use interner::{Internable, Interner};
use optional::{Noned, OptEq};
use std::borrow::Borrow;

pub type TermInterner = Interner<Term>;

impl Internable for Term {
    type Ref<'a> = TermRef<'a>;

    type Interned = TermItd;

    fn new_itr() -> Interner<Self> {
        Interner::new_empty()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        match self {
            Term::Atom(atom) => Some(TermItd(TermRef::Atom(*atom))),
            _ => None,
        }
    }

    fn itd_to_borrowed(itd: Self::Interned) -> Self::Ref<'static> {
        todo!()
    }

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        match self {
            Term::Atom(atom) => TermRef::Atom(*atom),
            Term::Curry(curry) => TermRef::Curry(curry),
            Term::Abstraction(abs) => TermRef::Abstraction(abs),
            Term::Application(app) => TermRef::Application(app),
            Term::Subentity(subentity) => TermRef::Subentity(subentity),
            Term::TraitImpl(trait_impl) => TermRef::TraitImpl(trait_impl),
        }
    }

    fn new_itd(&'static self, id: usize) -> Self::Interned {
        TermItd(self.as_ref())
    }

    fn try_direct_from_ref<'a>(r: Self::Ref<'a>) -> Option<Self::Interned> {
        todo!()
    }

    unsafe fn cast_to_static_ref<'a>(r: Self::Ref<'a>) -> Self::Ref<'static> {
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
        TermItd(TermRef::Atom(value.into()))
    }
}
