mod abstraction;
mod application;
mod atom;
mod context;
mod cow;
mod curry;
mod decl;
mod display;
mod error;
mod intern;
mod menu;
mod query;
#[cfg(test)]
mod tests;
mod ty;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use atom::*;
pub use context::TermContext;
pub use curry::TermCurry;
pub use decl::*;
pub use error::*;
pub use intern::*;
pub use menu::*;
pub use query::*;
pub use ty::Ty;

use cow::TermCow;
use optional::Optioned;
#[cfg(test)]
use tests::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Term {
    Atom(TermAtom),
    Curry(TermCurry),
    Abstraction(TermAbstraction),
    Application(TermApplication),
}

impl Term {
    pub fn ty_itd(&self) -> Option<Ty> {
        match self {
            Term::Atom(a) => a.ty_itd(),
            Term::Curry(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(a) => a.ty_itd(),
        }
    }
}
