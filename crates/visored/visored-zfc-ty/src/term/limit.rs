use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcLimit(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcLimitData {
    // Add appropriate fields here
}

impl VdZfcLimit {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcLimitData {
        match self.0.data(db) {
            VdZfcTermData::Limit(data) => data,
            _ => unreachable!(),
        }
    }
}
