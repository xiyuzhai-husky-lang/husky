use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsLimit(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsLimitData {
    // Add appropriate fields here
}

impl VdZfsLimit {
    pub fn data(self, db: &::salsa::Db) -> &VdZfsLimitData {
        match self.0.data(db) {
            VdZfsTermData::Limit(data) => data,
            _ => unreachable!(),
        }
    }
}
