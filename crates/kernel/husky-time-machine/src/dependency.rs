use crate::*;

pub struct Dependency {
    kind: DependencyKind,
    relevant: bool,
}

pub enum DependencyKind {
    Borrow {
        dependee: VariableIdx,
        dependant: LifetimeIdx,
    },
    BorrowMut {
        dependee: VariableIdx,
        dependant: LifetimeIdx,
    },
    TypeVariance {
        dependee: LifetimeIdx,
        dependant: ResourceIdx,
    },
}

#[derive(Default)]
pub struct DependencyList(Vec<Dependency>);

impl DependencyList {
    pub(crate) fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        self.0.push(Dependency {
            kind: DependencyKind::Borrow {
                dependee: variable,
                dependant: borrower,
            },
            relevant: true,
        })
    }
}
