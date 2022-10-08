mod abstraction;
mod application;
mod context;
mod cow;
mod curry;
mod error;
mod ident;
mod intern;
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
pub use ident::Identifier;
pub use intern::{InternTerm, TermInterner, TermPtr};
pub use namespace::{Namespace, NamespacePtr};
pub use query::TermQuery;
pub use ty::Ty;
pub use universe::Universe;
pub use variable::TermVariable;

use cow::TermCow;
use optional::Optioned;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Term {
    Type(Universe),
    Curry(TermCurry),
    Variable(TermVariable),
    Abstraction(TermAbstraction),
    Application(TermApplication),
}
