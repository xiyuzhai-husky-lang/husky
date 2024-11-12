use super::{VdZfcTerm, VdZfcTermData, VdZfcTermId, ZfcTerms};

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdZfcAbstraction(VdZfcTermId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdZfcAbstractionData {
    pub parameters: VdZfcTerm,
    pub arguments: ZfcTerms,
}

impl VdZfcAbstraction {
    pub fn data(self, db: &::salsa::Db) -> &VdZfcAbstractionData {
        match self.0.data(db) {
            VdZfcTermData::Abstraction(data) => data,
            _ => unreachable!(),
        }
    }
}
