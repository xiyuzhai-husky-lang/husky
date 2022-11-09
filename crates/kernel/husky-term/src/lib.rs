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
mod path;
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
use interner::{InternBorrowedRaw, InternedRefWrapper};
pub use menu::*;
pub use subentity::*;
pub use trai::*;
pub use trait_impl::*;
pub use ty::Ty;

// use cow::TermCow;
use husky_entity_path::EntityPathItd;
use optional::Optioned;
#[cfg(test)]
use tests::*;

#[derive(PartialEq, Eq, Hash)]
pub enum TermOwned {
    Atom(TermAtom),               // literal: 1,1.0, true, false; variable, entityPath
    Curry(TermCurry), // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Abstraction(TermAbstraction), // lambda x => expr
    Application(TermApplication), // f x, apply a function to term
    Subentity(TermSubentity), // ::
    TraitImpl(TermTraitImpl), // A as trait
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermBorrowed<'a> {
    Null,
    Atom(TermAtom),       // literal: 1,1.0, true, false; variable, entityPath
    Curry(&'a TermCurry), // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Abstraction(&'a TermAbstraction), // lambda x => expr
    Application(&'a TermApplication), // f x, apply a function to term
    Subentity(&'a TermSubentity), // ::
    TraitImpl(&'a TermTraitImpl), // A as trait
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TermItd(TermBorrowed<'static>);

impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

impl TermItd {
    fn borrowed(self) -> TermBorrowed<'static> {
        self.0
    }
}

impl std::fmt::Debug for TermOwned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Term(\"{}\")", self)
    }
}

impl TermOwned {
    pub fn ty_itd(&self) -> Option<Ty> {
        match self {
            TermOwned::Atom(a) => None,
            TermOwned::Curry(c) => Some(c.ty()),
            TermOwned::Abstraction(_) => todo!(),
            TermOwned::Application(a) => a.ty_itd(),
            TermOwned::Subentity(_) => todo!(),
            TermOwned::TraitImpl(_) => todo!(),
        }
    }

    pub fn universe(&self) -> TermUniverse {
        match self {
            TermOwned::Atom(a) => a.universe(),
            TermOwned::Curry(_) => todo!(),
            TermOwned::Abstraction(_) => todo!(),
            TermOwned::Application(_) => todo!(),
            TermOwned::Subentity(_) => todo!(),
            TermOwned::TraitImpl(_) => todo!(),
        }
    }
}

impl std::fmt::Display for TermOwned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermOwned::Atom(a) => a.fmt(f),
            TermOwned::Curry(c) => c.fmt(f),
            TermOwned::Abstraction(a) => a.fmt(f),
            TermOwned::Application(a) => a.fmt(f),
            TermOwned::Subentity(_) => todo!(),
            TermOwned::TraitImpl(_) => todo!(),
        }
    }
}
