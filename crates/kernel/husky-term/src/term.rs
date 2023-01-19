mod abstraction;
mod application;
mod as_trai_subentity;
mod category;
mod constraint;
mod curry;
mod durant;
mod literal;
mod subentity;
mod symbol;
mod universe;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use as_trai_subentity::*;
pub use category::*;
pub use constraint::*;
pub use curry::*;
pub use durant::*;
pub use literal::*;
pub use subentity::*;
pub use symbol::*;
pub use universe::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Term {
    // atoms
    // literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Symbol(TermSymbol),
    Entity(EntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(TermCurry),
    Durant(TermDurant),
    // lambda x => expr
    Abstraction(TermAbstraction),
    // f x, apply a function to term
    Application(TermApplication),
    /// ::<ident>
    Subentity(TermSubentity),
    /// (<type> as <trait>)::<ident>
    AsTraitSubentity(TermAsTraitSubentity),
    /// <type> : <trait>
    TraitConstraint(TermTraitConstraint),
}

impl From<TermLiteral> for Term {
    fn from(v: TermLiteral) -> Self {
        Self::Literal(v)
    }
}

impl From<TermDurant> for Term {
    fn from(v: TermDurant) -> Self {
        Self::Durant(v)
    }
}

impl From<TermCurry> for Term {
    fn from(v: TermCurry) -> Self {
        Self::Curry(v)
    }
}

impl From<EntityPath> for Term {
    fn from(v: EntityPath) -> Self {
        Self::Entity(v)
    }
}

impl From<TermSymbol> for Term {
    fn from(v: TermSymbol) -> Self {
        Self::Symbol(v)
    }
}

impl From<TermCategory> for Term {
    fn from(v: TermCategory) -> Self {
        Self::Category(v)
    }
}

impl Term {}
