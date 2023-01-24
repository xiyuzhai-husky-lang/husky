use super::*;

pub(crate) fn reduced_term(db: &dyn TypeDb, term: Term) -> ReducedTerm {
    // ad hoc
    ReducedTerm(term)
}

pub(crate) struct ReducedTerm(Term);

impl ReducedTerm {
    pub(crate) fn term(&self) -> Term {
        self.0
    }
}
