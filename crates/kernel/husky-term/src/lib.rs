#![doc = include_str!("../README.md")]
// #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

mod abstraction;
mod application;
mod atom;
mod context;
mod cow;
mod curry;
mod db;
mod decl;
mod error;
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
pub use menu::*;
pub use subentity::*;
pub use trai::*;
pub use trait_impl::*;
pub use ty::*;

use husky_entity_path::EntityPath;
use husky_vfs::*;

#[salsa::jar(db = TermDb)]
pub struct TermJar(
    TermCurry,
    TermAbstraction,
    TermApplication,
    TermSubentity,
    TermTraitImpl,
    TermCurryContext,
    Type,
    term_menu,
);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Term {
    // literal: 1,1.0, true, false; variable, entityPath
    Atom(TermAtom),
    // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(TermCurry),
    // lambda x => expr
    Abstraction(TermAbstraction),
    // f x, apply a function to term
    Application(TermApplication),
    // ::
    Subentity(TermSubentity),
    // A as trait
    TraitImpl(TermTraitImpl),
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
