use eterned::db::EternerDb;

use super::{VdTerm, VdTermData, VdTermId, ZfcTerms};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdApplication(VdTermId);

impl std::ops::Deref for VdApplication {
    type Target = VdTermId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdApplicationData {
    pub function: VdTerm,
    pub arguments: ZfcTerms,
}

impl VdApplication {
    pub fn data(self) -> &'static VdApplicationData {
        match self.0.data() {
            VdTermData::Application(data) => data,
            _ => unreachable!(),
        }
    }
}
