mod abstraction;
mod application;
mod as_trai_subentity;
mod category;
mod constraint;
mod curry;
mod jordan;
mod literal;
mod subentity;
mod universe;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use as_trai_subentity::*;
pub use category::*;
pub use constraint::*;
pub use curry::*;
pub use jordan::*;
pub use literal::*;
pub use subentity::*;
pub use universe::*;

use crate::*;
use husky_entity_path::EntityPath;
use husky_word::Identifier;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Term {
    // atoms
    // literal: 1,1.0, true, false; variable, entityPath
    Literal(TermLiteral),
    Variable(Identifier),
    Lifetime(Identifier),
    Binding(Identifier),
    Entity(EntityPath),
    Category(TermCategory),
    Universe(TermUniverse),
    // X -> Y (a function X to Y, function can be a function pointer or closure or purely conceptual)
    Curry(TermCurry),
    Jordan(TermJordan),
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

impl Term {
    pub fn new_application(db: &dyn TermDb, function: Term, argument: Term) -> Self {
        todo!()
    }
}
