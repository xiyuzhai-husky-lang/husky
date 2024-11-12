use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcExists(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcExistsData {
    // Add appropriate fields here
}

impl VdZfcExists {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcExistsData {
        match self.0.data(db) {
            VdZfcTermData::Exists(data) => data,
            _ => unreachable!(),
        }
    }
}
