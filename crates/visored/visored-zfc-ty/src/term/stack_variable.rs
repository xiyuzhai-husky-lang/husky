use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcStackVariable(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcStackVariableData {
    // Add appropriate fields here
}

impl VdZfcStackVariable {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcStackVariableData {
        match self.0.data(db) {
            VdZfcTermData::StackVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
