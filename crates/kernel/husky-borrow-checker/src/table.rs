use crate::*;

#[derive(Debug)]
pub struct Dependency {
    dependee: BorrowObject,
    dependant: BorrowObject,
}

#[derive(Debug, Default)]
pub struct DependencyTable(Vec<Dependency>);

impl DependencyTable {
    pub(crate) fn add_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        self.0.push(Dependency {
            dependee: variable.into(),
            dependant: borrower.into(),
        })
    }

    fn dependees<'a>(&'a self, dependant: BorrowObject) -> impl Iterator<Item = BorrowObject> + 'a {
        self.0
            .iter()
            .filter_map(move |borrow| match borrow.dependant == dependant {
                true => Some(borrow.dependee),
                false => None,
            })
    }

    pub(crate) fn dependants<'a>(
        &'a self,
        dependee: BorrowObject,
    ) -> impl Iterator<Item = BorrowObject> + 'a {
        self.0
            .iter()
            .filter_map(move |borrow| match borrow.dependee == dependee {
                true => Some(borrow.dependant),
                false => None,
            })
    }
}
