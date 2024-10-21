use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsExists(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsExistsData {
    // Add appropriate fields here
}

impl VdZfsExists {
    pub fn data(self, db: &::salsa::Db) -> VdZfsExistsData {
        match self.0.data(db) {
            VdZfsTermData::Exists(data) => data,
            _ => unreachable!(),
        }
    }
}
