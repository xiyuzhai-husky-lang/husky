use super::{VdZfsTerm, VdZfsTermData, VdZfsTermId, ZfsTerms};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsApplication(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsApplicationData {
    pub function: VdZfsTerm,
    pub arguments: ZfsTerms,
}

impl VdZfsApplication {
    pub fn data(self, db: &::salsa::Db) -> VdZfsApplicationData {
        match self.0.data(db) {
            VdZfsTermData::Application(data) => data,
            _ => unreachable!(),
        }
    }
}
