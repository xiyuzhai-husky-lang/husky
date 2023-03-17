use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PackageDependency {
    name: Word,
    source: PackageDependencySource,
    options: PackageDependencyOptions,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PackageDependencySource {
    Git,
    Local,
    Registry,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PackageDependencyOptions {
    optional: bool,
}
