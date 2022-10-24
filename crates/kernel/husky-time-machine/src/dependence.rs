use crate::*;

pub struct Dependency {
    kind: DependencyKind,
    dependee: ResourceIdx,
    dependant: ResourceIdx,
    relevant: bool,
}

pub enum DependencyKind {
    Borrow,
    BorrowMut,
    TypeVariance,
}

#[derive(Default)]
pub struct DependencyList {
    rules: Vec<Dependency>,
}
