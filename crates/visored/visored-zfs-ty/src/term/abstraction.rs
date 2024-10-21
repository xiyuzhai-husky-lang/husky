use super::{VdZfsTerm, VdZfsTermData, VdZfsTermId, ZfsTerms};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsAbstraction(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsAbstractionData {
    pub parameters: VdZfsTerm,
    pub arguments: ZfsTerms,
}

impl VdZfsAbstraction {
    pub fn data(self, db: &::salsa::Db) -> &VdZfsAbstractionData {
        match self.0.data(db) {
            VdZfsTermData::Abstraction(data) => data,
            _ => unreachable!(),
        }
    }
}
