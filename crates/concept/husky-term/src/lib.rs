mod abstraction;
mod application;
mod context;
mod cow;
mod curry;
mod decl;
mod display;
mod entity;
mod error;
mod intern;
mod literal;
mod query;
#[cfg(test)]
mod tests;
mod ty;
mod universe;
mod variable;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use context::TermContext;
pub use curry::TermCurry;
pub use decl::*;
pub use entity::*;
pub use error::*;
pub use intern::*;
pub use literal::TermLiteral;
pub use query::TermQuery;
pub use ty::Ty;
pub use universe::*;
pub use variable::TermVariable;

use cow::TermCow;
use optional::Optioned;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Term {
    Literal(TermLiteral),
    Entity(TermEntity),
    Curry(TermCurry),
    Variable(TermVariable),
    Abstraction(TermAbstraction),
    Application(TermApplication),
    Universe(TermUniverse),
}
