use super::{ZfsTerm, ZfsTermData, ZfsTermId, ZfsTerms};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsAbstraction(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsAbstractionData {
    pub parameters: ZfsTerm,
    pub arguments: ZfsTerms,
}

impl ZfsAbstraction {
    pub fn data(self, db: &::salsa::Db) -> ZfsAbstractionData {
        match self.0.data(db) {
            ZfsTermData::Abstraction(data) => data,
            _ => unreachable!(),
        }
    }
}
