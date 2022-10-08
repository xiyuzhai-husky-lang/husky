mod abstraction;
mod application;
mod cow;
mod curry;
mod error;
mod ident;
mod intern;
mod namespace;
#[cfg(test)]
mod tests;
mod ty;
mod universe;
mod variable;

pub use abstraction::TermAbstraction;
pub use application::TermApplication;
pub use curry::TermCurry;
pub use ident::Identifier;
pub use intern::{TermInterner, TermPtr};
pub use namespace::{Namespace, NamespacePtr};
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
