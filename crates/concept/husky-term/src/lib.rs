mod expr;
mod ident;
mod intern;
mod namespace;
mod ty;
mod universe;

pub use ident::Identifier;
pub use intern::{TermInterner, TermPtr};
pub use namespace::{Namespace, NamespacePtr};
pub use ty::Ty;
pub use universe::Universe;

use optional::Optioned;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Term {
    Type(Universe),
    CurryType { from: Ty, to: Ty },
    Variable(Variable),
    Abstraction(Abstraction),
    Application(Application),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    opt_namespace: Optioned<NamespacePtr>,
    name: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Abstraction {
    x: i32,
    m: TermPtr,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Application {
    m: TermPtr,
    n: TermPtr,
}
