use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsAbstractVariable(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsAbstractVariableData {
    // Add appropriate fields here
}

impl VdZfsAbstractVariable {
    pub fn data(self, db: &::salsa::Db) -> VdZfsAbstractVariableData {
        match self.0.data(db) {
            VdZfsTermData::AbstractVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
