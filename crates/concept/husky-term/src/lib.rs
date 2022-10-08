mod abstraction;
mod application;
mod context;
mod cow;
mod curry;
mod decl;
mod display;
mod error;
mod intern;
mod literal;
mod namespace;
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
pub use error::*;
pub use intern::{InternTerm, TermInterner, TermPtr};
pub use literal::TermLiteral;
pub use namespace::TermNamespace;
pub use namespace::{Namespace, NamespacePtr};
pub use query::TermQuery;
pub use ty::Ty;
pub use universe::*;
pub use variable::TermVariable;

use cow::TermCow;
use optional::Optioned;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Term {
    Literal(TermLiteral),
    Namespace(TermNamespace),
    Curry(TermCurry),
    Variable(TermVariable),
    Abstraction(TermAbstraction),
    Application(TermApplication),
    Universe(TermUniverse),
}
