use super::*;

impl EtherealTerm {
    pub fn curry_parameter_count(self, db: &dyn EtherealTermDb) -> u8 {
        match self {
            EtherealTerm::Curry(term) => curry_parameter_count(db, term),
            _ => 0,
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn curry_parameter_count(db: &dyn EtherealTermDb, term: EtherealTermCurry) -> u8 {
    term.return_ty(db).curry_parameter_count(db) + 1
}
