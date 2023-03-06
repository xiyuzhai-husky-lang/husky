use super::*;

impl TermApplication {
    pub(super) fn reduce(self, db: &dyn TermDb) -> Term {
        reduce_term_application(db, self)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn reduce_term_application(db: &dyn TermDb, term_application: TermApplication) -> Term {
    todo!()
}
