use super::*;

impl EtherealTerm {
    pub fn total_number_of_curry_parameters(self, db: &dyn EtherealTermDb) -> u8 {
        match self {
            EtherealTerm::Curry(term) => total_number_of_curry_parameters(db, term),
            _ => 0,
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn total_number_of_curry_parameters(
    db: &dyn EtherealTermDb,
    term: EtherealTermCurry,
) -> u8 {
    term.return_ty(db).total_number_of_curry_parameters(db) + 1
}
