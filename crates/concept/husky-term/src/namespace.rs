mod intern;

pub use intern::*;
use optional::Optioned;

use crate::Identifier;

pub enum Namespace {
    Module(Module),
    Package(Package),
    PackageGroup(PackageGroup),
}

pub struct Module {
    parent: NamespacePtr,
    name: Identifier,
}

pub struct Package {
    opt_parent: Optioned<NamespacePtr>,
    name: Identifier,
}

pub struct PackageGroup {
    opt_parent: Optioned<NamespacePtr>,
    name: Identifier,
}
