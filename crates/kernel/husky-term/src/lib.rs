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
pub enum Term {
    Atom(TermAtom),               // literal: 1,1.0, true, false; variable, entityPath
    Curry(TermCurry), // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Abstraction(TermAbstraction), // lambda x => expr
    Application(TermApplication), // f x, apply a function to term
    Subentity(TermSubentity), // ::
    TraitImpl(TermTraitImpl), // A as trait
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermRef<'a> {
    Null,
    Atom(TermAtom),       // literal: 1,1.0, true, false; variable, entityPath
    Curry(&'a TermCurry), // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Abstraction(&'a TermAbstraction), // lambda x => expr
    Application(&'a TermApplication), // f x, apply a function to term
    Subentity(&'a TermSubentity), // ::
    TraitImpl(&'a TermTraitImpl), // A as trait
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TermItd(TermRef<'static>);

#[cfg(target_arch = "x86_64")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { std::mem::transmute::<Self, [u8; 24]>(*self) }.hash(state)
    }
}

#[cfg(target_arch = "x86")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "mips")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "powerpc")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "powerpc64")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "arm")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "aarch64")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

impl TermItd {
    fn borrowed(self) -> TermRef<'static> {
        self.0
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Term(\"{}\")", self)
    }
}

impl Term {
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
