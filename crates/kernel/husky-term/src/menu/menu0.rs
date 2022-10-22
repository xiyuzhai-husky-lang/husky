use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu0 {
    sort: TermItd,
    universe1: TermItd,
}

impl TermMenu0 {
    pub fn new(db: &dyn TermDb) -> Self {
        let sort = db.it_term(TermAtom::new_category(TermCategory::Sort).into());
        let universe1 = db.it_term(TermAtom::new_universe(1).into());
        TermMenu0 { sort, universe1 }
    }

    pub fn sort(&self) -> TermItd {
        self.sort
    }

    pub fn universe1(&self) -> TermItd {
        self.universe1
    }
}
