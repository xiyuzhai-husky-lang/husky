use super::*;

impl EthTerm {
    // only returns positive
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> i8 {
        match self {
            EthTerm::Curry(term) => term.curry_parameter_count(db),
            _ => 0,
        }
    }
}

impl EthCurry {
    // only returns positive
    pub fn curry_parameter_count(self, db: &::salsa::Db) -> i8 {
        curry_parameter_count(db, self)
    }
}

// only returns positive
#[salsa::tracked]
pub(crate) fn curry_parameter_count(db: &::salsa::Db, term: EthCurry) -> i8 {
    term.return_ty(db).curry_parameter_count(db) + 1
}
