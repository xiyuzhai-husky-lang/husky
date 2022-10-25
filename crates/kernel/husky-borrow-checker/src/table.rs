use crate::*;

pub struct Borrow {
    kind: BorrowKind,
    relevant: bool,
}

pub enum BorrowKind {
    Borrow {
        dependee: VariableIdx,
        dependant: LifetimeIdx,
    },
    BorrowMut {
        dependee: VariableIdx,
        dependant: LifetimeIdx,
    },
    Indirect {
        dependee: LifetimeIdx,
        dependant: SymbolIdx,
    },
}

#[derive(Default)]
pub struct BorrowTable(Vec<Borrow>);

impl BorrowTable {
    pub(crate) fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        self.0.push(Borrow {
            kind: BorrowKind::Borrow {
                dependee: variable,
                dependant: borrower,
            },
            relevant: true,
        })
    }
}
