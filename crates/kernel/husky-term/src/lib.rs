mod abstraction;
mod application;
mod atom;
mod context;
mod cow;
mod curry;
mod db;
mod decl;
mod error;
mod intern;
mod menu;
mod subentity;
#[cfg(test)]
mod tests;
mod trai;
mod trait_impl;
mod ty;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use atom::*;
pub use context::*;
pub use curry::*;
pub use db::*;
pub use decl::*;
pub use error::*;
pub use intern::*;
pub use menu::*;
pub use subentity::*;
pub use trai::*;
pub use trait_impl::*;
pub use ty::Ty;

use cow::TermCow;
use optional::Optioned;
#[cfg(test)]
use tests::*;

#[derive(PartialEq, Eq, Hash)]
pub enum Term {
    Atom(TermAtom),
    Curry(TermCurry),
    Abstraction(TermAbstraction),
    Application(TermApplication),
    Subentity(TermSubentity),
    TraitImpl(TermTraitImpl),
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Term(\"{}\")", self)
    }
}

impl Term {
    pub fn ty_itd(&self) -> Option<Ty> {
        match self {
            Term::Atom(a) => a.ty_itd(),
            Term::Curry(c) => Some(c.ty()),
            Term::Abstraction(_) => todo!(),
            Term::Application(a) => a.ty_itd(),
            Term::Subentity(_) => todo!(),
            Term::TraitImpl(_) => todo!(),
        }
    }

    pub fn universe(&self) -> TermUniverse {
        match self {
            Term::Atom(a) => a.universe(),
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::TraitImpl(_) => todo!(),
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Atom(a) => a.fmt(f),
            Term::Curry(c) => c.fmt(f),
            Term::Abstraction(a) => a.fmt(f),
            Term::Application(a) => a.fmt(f),
            Term::Subentity(_) => todo!(),
            Term::TraitImpl(_) => todo!(),
        }
    }
}
