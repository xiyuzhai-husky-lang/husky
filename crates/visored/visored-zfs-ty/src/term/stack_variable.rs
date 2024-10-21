use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsStackVariable(VdZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VdZfsStackVariableData {
    // Add appropriate fields here
}

impl VdZfsStackVariable {
    pub fn data(self, db: &::salsa::Db) -> VdZfsStackVariableData {
        match self.0.data(db) {
            VdZfsTermData::StackVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
