use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsForAll(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsForAllData {
    // Add appropriate fields here
}

impl ZfsForAll {
    pub fn data(self, db: &::salsa::Db) -> ZfsForAllData {
        match self.0.data(db) {
            ZfsTermData::ForAll(data) => data,
            _ => unreachable!(),
        }
    }
}
