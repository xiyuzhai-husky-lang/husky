use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcAbstractVariable(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcAbstractVariableData {
    // Add appropriate fields here
}

impl VdZfcAbstractVariable {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcAbstractVariableData {
        match self.0.data(db) {
            VdZfcTermData::AbstractVariable(data) => data,
            _ => unreachable!(),
        }
    }
}
