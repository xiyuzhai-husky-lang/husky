use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    sort: Term,
    universe1: Term,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermDb) -> Self {
        todo!()
        // let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        // let universe1 = db.it_term(TermAtom::new_universe(1).into());
        // TermMenu0 { sort, universe1 }
    }

    pub fn sort(&self) -> Term {
        self.sort
    }

    pub fn universe1(&self) -> Term {
        self.universe1
    }
}
