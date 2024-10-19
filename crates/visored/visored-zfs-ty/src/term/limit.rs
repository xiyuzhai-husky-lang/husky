use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsLimit(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsLimitData {
    // Add appropriate fields here
}

impl ZfsLimit {
    pub fn data(self, db: &::salsa::Db) -> ZfsLimitData {
        match self.0.data(db) {
            ZfsTermData::Limit(data) => data,
            _ => unreachable!(),
        }
    }
}
