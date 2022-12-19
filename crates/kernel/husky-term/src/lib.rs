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

use husky_entity_path::EntityPath;
use husky_toolchain::Toolchain;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = TermDb)]
pub struct TermJar(Term, TermCurryContext, term_menu);

#[salsa::interned(jar = TermJar)]
pub struct Term {
    data: TermData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TermData {
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

impl TermData {
    pub fn universe(&self) -> TermUniverse {
        match self {
            TermData::Atom(a) => a.universe(),
            TermData::Curry(_) => todo!(),
            TermData::Abstraction(_) => todo!(),
            TermData::Application(_) => todo!(),
            TermData::Subentity(_) => todo!(),
            TermData::TraitImpl(_) => todo!(),
        }
    }
}
