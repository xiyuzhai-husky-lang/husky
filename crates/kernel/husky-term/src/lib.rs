mod abstraction;
mod application;
mod atom;
mod context;
mod cow;
mod curry;
mod db;
mod decl;
mod display;
mod error;
mod intern;
mod menu;
#[cfg(test)]
mod tests;
mod ty;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use atom::*;
pub use context::TermContext;
pub use curry::TermCurry;
pub use db::*;
pub use decl::*;
pub use error::*;
pub use intern::*;
pub use menu::*;
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
        }
    }

    pub fn universe(&self) -> TermUniverse {
        match self {
            Term::Atom(a) => a.universe(),
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
        }
    }
}
