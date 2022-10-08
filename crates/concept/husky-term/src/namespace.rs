mod intern;

pub use intern::*;
use optional::Optioned;

use crate::Identifier;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Namespace {
    opt_parent: Optioned<NamespacePtr>,
    name: Identifier,
}
