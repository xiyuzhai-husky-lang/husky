use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    sort: TermPtr,
    universe1: TermPtr,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermQuery) -> Self {
        let sort = db.it_term(TermAtom::new_category(TermCategoryKind::Sort).into());
        let universe1 = db.it_term(TermAtom::new_universe(1).into());
        TermMenu0 { sort, universe1 }
    }

    pub fn sort(&self) -> TermPtr {
        self.sort
    }

    pub fn universe1(&self) -> TermPtr {
        self.universe1
    }
}
