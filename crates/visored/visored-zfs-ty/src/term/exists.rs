use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsExists(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsExistsData {
    // Add appropriate fields here
}

impl ZfsExists {
    pub fn data(self, db: &::salsa::Db) -> ZfsExistsData {
        match self.0.data(db) {
            ZfsTermData::Exists(data) => data,
            _ => unreachable!(),
        }
    }
}
