use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcForAll(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcForAllData {
    // Add appropriate fields here
}

impl VdZfcForAll {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcForAllData {
        match self.0.data(db) {
            VdZfcTermData::ForAll(data) => data,
            _ => unreachable!(),
        }
    }
}
