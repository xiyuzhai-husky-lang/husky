use eterned::db::EternerDb;

use super::{VdTerm, VdTermData, VdTermId, ZfcTerms};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VdApplication(VdTermId);

impl std::ops::Deref for VdApplication {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VdApplicationData {
    pub function: VdTerm,
    pub arguments: ZfcTerms,
}

impl VdApplication {
    pub fn data(&self, db: &EternerDb) -> &VdApplicationData {
        match self.0.data(db) {
            VdTermData::Application(data) => data,
            _ => unreachable!(),
        }
    }
}
