mod error;
mod intern;

pub use intern::{TyInterner, TyPtr};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Ty {}
