use super::*;

impl EtherealTerm {
    // only returns positive
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> i8 {
        match self {
            EtherealTerm::Curry(term) => term.curry_parameter_count(db),
            _ => 0,
        }
    }
}

impl EtherealTermCurry {
    // only returns positive
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> i8 {
        curry_parameter_count(db, self)
    }
}

// only returns positive
#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn curry_parameter_count(db: &::salsa::Db, term: EtherealTermCurry) -> i8 {
    term.return_ty(db).curry_parameter_count(db) + 1
}
