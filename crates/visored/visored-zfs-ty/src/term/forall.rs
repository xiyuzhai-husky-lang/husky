use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsForAll(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsForAllData {
    // Add appropriate fields here
}

impl VdZfsForAll {
    pub fn data(self, db: &::salsa::Db) -> VdZfsForAllData {
        match self.0.data(db) {
            VdZfsTermData::ForAll(data) => data,
            _ => unreachable!(),
        }
    }
}
