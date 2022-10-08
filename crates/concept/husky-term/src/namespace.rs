mod intern;

pub use intern::*;
use optional::Optioned;

use crate::Identifier;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Namespace {
    Module(Module),
    Package(Package),
    PackageGroup(PackageGroup),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Module {
    parent: NamespacePtr,
    name: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Package {
    opt_parent: Optioned<NamespacePtr>,
    name: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PackageGroup {
    opt_parent: Optioned<NamespacePtr>,
    name: Identifier,
}
