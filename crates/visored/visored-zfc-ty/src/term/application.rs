use super::{VdZfcTerm, VdZfcTermData, VdZfcTermId, ZfcTerms};

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcApplication(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcApplicationData {
    pub function: VdZfcTerm,
    pub arguments: ZfcTerms,
}

impl VdZfcApplication {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcApplicationData {
        match self.0.data(db) {
            VdZfcTermData::Application(data) => data,
            _ => unreachable!(),
        }
    }
}
