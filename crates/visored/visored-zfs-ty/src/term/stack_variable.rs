use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsStackVariable(ZfsTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZfsStackVariableData {
    // Add appropriate fields here
}

impl ZfsStackVariable {
    pub fn data(self, db: &::salsa::Db) -> ZfsStackVariableData {
        match self.0.data(db) {
            ZfsTermData::StackVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
